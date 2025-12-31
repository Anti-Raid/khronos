use mlua_scheduler::{LuaSchedulerAsync, LuaSchedulerAsyncUserData};
use mluau::prelude::*;
use serenity::futures::{Stream, StreamExt};
use serenity::futures::stream::FuturesUnordered;
use tokio::sync::Notify;
use tokio_util::time::DelayQueue;
use std::task::{Context, Poll};
use std::{cell::RefCell, pin::Pin};
use std::time::Duration;
use std::rc::{Rc, Weak};

use crate::core::datetime::TimeDelta;

const MAX_TIMEOUT: Duration = Duration::from_secs(7);

pub struct OneshotChannel {
    tx: Rc<RefCell<Option<tokio::sync::oneshot::Sender<LuaValue>>>>,
    rx: Rc<RefCell<Option<tokio::sync::oneshot::Receiver<LuaValue>>>>,
}

impl OneshotChannel {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::oneshot::channel();
        Self {
            tx: Rc::new(RefCell::new(Some(tx))),
            rx: Rc::new(RefCell::new(Some(rx))),
        }
    }
}

impl LuaUserData for OneshotChannel {
    fn add_methods<M: mluau::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("send", |_, this, value: LuaValue| {
            let chan = this.tx.borrow_mut().take()
                .ok_or(mluau::Error::external("Channel already used"))?;

            chan
                .send(value)
                .map_err(|_| mluau::Error::external("Failed to send value: receiver dropped"))?;
            Ok(())
        });

        methods.add_scheduler_async_method("recv", async move |_, this, ()| {
            let rx = this.rx.borrow_mut().take()
                .ok_or(mluau::Error::external("Channel already used"))?;

            let value = rx
                .await
                .map_err(|_| mluau::Error::external("Failed to receive value: sender dropped"))?;
            Ok(value)
        });

        methods.add_scheduler_async_method("recvtimeout", async move |_, this, timeout: LuaUserDataRef<TimeDelta>| {
            let rx = this.rx.borrow_mut().take()
                .ok_or(mluau::Error::external("Channel already used"))?;

            let timeout = timeout.timedelta.to_std().map_err(LuaError::external)?;
            if timeout > MAX_TIMEOUT {
                return Err(LuaError::external("Timeout cannot be greater than the max timeout"));
            }

            let value = tokio::time::timeout(timeout, rx)
                .await
                .map_err(|_| mluau::Error::external("Receive timed out"))?
                .map_err(|_| mluau::Error::external("Failed to receive value: sender dropped"))?;
            
            Ok(value)
        });
    }
}

pub struct KeyHandle {
    queue: Weak<RefCell<DelayQueue<LuaValue>>>,
    key: tokio_util::time::delay_queue::Key,
}

impl LuaUserData for KeyHandle {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("cancel", |_, this, ()| {
            let Some(queue) = this.queue.upgrade() else {
                return Err(LuaError::external("Delay channel has been dropped"));
            };

            let val = match queue.try_borrow_mut()
            .map_err(LuaError::external)?
            .try_remove(&this.key) {
                Some(val) => Ok((true, val.into_inner())),
                None => Ok((false, LuaValue::Nil)),
            };

            val
        });

        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: LuaUserDataRef<KeyHandle>| {
            Ok(this.key == other.key && Weak::ptr_eq(&this.queue, &other.queue))
        });
    }
}

/// A delay channel that can be used to return a value after a delay/expiration
pub struct DelayChannel {
    queue: Rc<RefCell<DelayQueue<LuaValue>>>,
    notify: Rc<Notify>,
}

impl DelayChannel {
    pub fn new() -> Self {
        Self {
            queue: Rc::new(RefCell::new(DelayQueue::new())),
            notify: Rc::new(Notify::new()),
        }
    }
    
    /// Inserts a value into the delay channel with the given delay
    pub fn add(&self, value: LuaValue, delay: Duration) {
        self.queue.borrow_mut().insert(value, delay);
        self.notify.notify_one();
    }

    /// Inserts a value into the delay channel with the given delay
    /// and returns a handle that can be used to cancel it
    pub fn add_with_handle(&self, value: LuaValue, delay: Duration) -> KeyHandle {
        let key = self.queue.borrow_mut().insert(value, delay);
        self.notify.notify_one();
        KeyHandle { key, queue: Rc::downgrade(&self.queue) }
    }

    pub async fn next(&self) -> LuaResult<LuaValue> {
        loop {
            // We recreate the stream wrapper in the loop (it's cheap)
            let mut stream = QueueStream {
                queue: self.queue.clone(),
            };

            // Attempt to get the next expired item
            tokio::select! {
                biased;
                Some(expired) = stream.next() => return Ok(expired.into_inner()),
                _ = self.notify.notified() => {
                    // Notified, loop to recreate the stream and try again
                },
                
            }
        }
    }
}

impl LuaUserData for DelayChannel {
    fn add_methods<M: mluau::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("add", |_, this, (value, delay): (LuaValue, LuaUserDataRef<TimeDelta>)| {
            let delay = delay.timedelta.to_std().map_err(LuaError::external)?;
            Ok(this.add(value, delay))
        });

        methods.add_method("addwithhandle", |_, this, (value, delay): (LuaValue, LuaUserDataRef<TimeDelta>)| {
            let delay = delay.timedelta.to_std().map_err(LuaError::external)?;
            let handle = this.add_with_handle(value, delay);
            Ok(handle)
        });

        methods.add_scheduler_async_method("next", async move |_, this, ()| {
            this.next().await
        });
    }
}

struct QueueStream {
    queue: Rc<RefCell<DelayQueue<LuaValue>>>,
}

impl Stream for QueueStream {
    type Item = tokio_util::time::delay_queue::Expired<LuaValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // We only borrow MUTABLY right here, inside the poll.
        // If the queue is not ready, DelayQueue registers the waker and returns Pending.
        // The borrow is dropped immediately after this line.
        self.queue.borrow_mut().poll_expired(cx)
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("OneshotChannel", lua.create_function(|_, ()| {
        Ok(OneshotChannel::new())
    })?)?;

    module.set("DelayChannel", lua.create_function(|_, ()| {
        Ok(DelayChannel::new())
    })?)?;

    module.set("selectoneshots", lua.create_scheduler_async_function(async |_lua, (channels, timeout): (Vec<LuaUserDataRef<OneshotChannel>>, LuaUserDataRef<TimeDelta>)| {
        let timeout = timeout.timedelta.to_std().map_err(LuaError::external)?;
        if timeout > MAX_TIMEOUT {
            return Err(LuaError::external("Timeout cannot be greater than the max timeout"));
        }
        
        let mut futures_unordered = FuturesUnordered::new();
        for chan in channels {
            let rx = chan.rx.borrow_mut().take()
                .ok_or(mluau::Error::external("Channel already used"))?;

            futures_unordered.push(rx);
        }

        let mut results = Vec::with_capacity(futures_unordered.len());
        loop {
            tokio::select! {
                res = futures_unordered.next() => {
                    match res {
                        Some(Ok(value)) => {
                            results.push(value);
                            if futures_unordered.is_empty() {
                                break;
                            }
                        },
                        Some(Err(_)) => {
                            return Err(mluau::Error::external("Failed to receive value: sender dropped"));
                        },
                        None => break,
                    }
                },
                _ = tokio::time::sleep(timeout) => {
                    return Ok((results, true));
                }
            }
        }

        Ok((results, false))
    })?)?;

    Ok(module)
}