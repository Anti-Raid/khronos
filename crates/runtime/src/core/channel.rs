use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;
use serenity::futures::{Stream, StreamExt};
use tokio_util::time::DelayQueue;
use std::cell::Cell;
use std::task::{Context, Poll, Waker};
use std::{cell::RefCell, pin::Pin};
use std::time::{Duration, Instant};
use std::rc::{Rc, Weak};

use crate::core::datetime::TimeDelta;

const MAX_TIMEOUT: Duration = Duration::from_secs(7);

const NUM_LEVELS: usize = 6;
const MAX_DURATION_UNSIGNED: u64 = (1 << (6 * NUM_LEVELS)) - 1;
const MAX_DURATION_OBJ_STD: Duration = Duration::from_millis(MAX_DURATION_UNSIGNED-5000);

#[derive(Clone)]
pub struct StreamSender {
    pub tx: tokio::sync::mpsc::UnboundedSender<LuaValue>,
}

impl LuaUserData for StreamSender {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("send", |_, this, value: LuaValue| {
            let _ = this.tx.send(value);
            Ok(())
        });

        methods.add_method("sendstrict", |_, this, value: LuaValue| {
            this.tx.send(value)
            .map_err(|_| LuaError::external("Failed to send message: receiver dropped"))?;
            Ok(())
        });

        methods.add_method("isclosed", |_, this, _: ()| {
            Ok(this.tx.is_closed())
        });
    }
}

pub struct StreamReceiver {
    pub rx: tokio::sync::mpsc::UnboundedReceiver<LuaValue>,
}

impl LuaUserData for StreamReceiver {
    fn add_methods<M: mluau::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method_mut("recv", async move |_, mut this, ()| {
            let value = this.rx
                .recv()
                .await;
            Ok(value)
        });

        methods.add_scheduler_async_method_mut("recvtimeout", async move |_, mut this, timeout: LuaUserDataRef<TimeDelta>| {
            let timeout = timeout.timedelta.to_std().map_err(LuaError::external)?;
            if timeout > MAX_TIMEOUT {
                return Err(LuaError::external("Timeout cannot be greater than the max timeout"));
            }

            let value = tokio::time::timeout(timeout, this.rx.recv())
                .await
                .ok();
            
            Ok(value)
        });

        methods.add_scheduler_async_method_mut("recvbatch", async move |_, mut this, (expected_count, timeout): (usize, LuaUserDataRef<TimeDelta>)| {
            let timeout_dur = timeout.timedelta.to_std().map_err(LuaError::external)?;
            if timeout_dur > MAX_TIMEOUT {
                return Err(LuaError::external("Timeout cannot be greater than the max timeout"));
            }

            let mut results = Vec::with_capacity(expected_count);

            // Calculate the absolute deadline for the entire batch operation
            let deadline = tokio::time::Instant::now() + timeout_dur;

            // Keep reading from our single channel until we hit our target count
            while results.len() < expected_count {
                // Try to grab the next message before the overarching deadline hits
                match tokio::time::timeout_at(deadline, this.rx.recv()).await {
                    Ok(Some(value)) => results.push(value),
                    Ok(None) => return Ok((results, "closed")),                     
                    Err(_) => return Ok((results, "timeout"))
                }
            }

            // Returns a Luau tuple: ( {results...}, timed_out_boolean )
            Ok((results, "completed"))
        });
    }
}

pub struct LuaStream {
    tx: StreamSender,
    rx: StreamReceiver,
}

impl LuaStream {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            tx: StreamSender { tx },
            rx: StreamReceiver { rx },
        }
    }
}

impl IntoLua for LuaStream {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let tab = lua.create_table_with_capacity(0, 2)?;
        tab.set("sender", self.tx)?;
        tab.set("receiver", self.rx)?;
        tab.set_readonly(true);
        Ok(LuaValue::Table(tab))
    }
}

// The key may change if the item is reinserted, so we use a Cell to allow mutability
type SharedKey = Rc<Cell<Option<tokio_util::time::delay_queue::Key>>>;

struct Item {
    value: LuaValue,
    final_expiry: std::time::Instant,
    key: SharedKey
}

pub struct KeyHandle {
    queue: Weak<RefCell<DelayQueue<Item>>>,
    key: SharedKey,
}

impl LuaUserData for KeyHandle {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("cancel", |_, this, ()| {
            let Some(queue) = this.queue.upgrade() else {
                return Err(LuaError::external("Delay channel has been dropped"));
            };

            let Some(key) = this.key.get() else {
                return Ok((false, LuaValue::Nil)); // Already removed
            };


            let val = match queue.try_borrow_mut()
            .map_err(LuaError::external)?
            .try_remove(&key) {
                Some(val) => {
                    this.key.set(None); // Clear the key since it's been removed
                    Ok((true, val.into_inner().value))
                },
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
    queue: Rc<RefCell<DelayQueue<Item>>>,
    waiting_add: Rc<RefCell<Option<Waker>>>, // used to wake up the stream when a new item is added
}

impl DelayChannel {
    pub fn new() -> Self {
        Self {
            queue: Rc::new(RefCell::new(DelayQueue::new())),
            waiting_add: Rc::new(RefCell::new(None)),
        }
    }

    fn get_safe_delay(delay: Duration) -> Duration {
        if delay > MAX_DURATION_OBJ_STD { MAX_DURATION_OBJ_STD } else { delay }
    }
    
    /// Inserts a value into the delay channel with the given delay
    /// and returns a handle that can be used to cancel it
    pub fn add(&self, value: LuaValue, delay: Duration) -> LuaResult<KeyHandle> {
        let final_expiry = Instant::now() + delay;
        let safe_delay = Self::get_safe_delay(delay);
        let key_cell = Rc::new(Cell::new(None));
        let key = self.queue.borrow_mut().insert(Item { value, final_expiry, key: key_cell.clone() }, safe_delay);
        key_cell.set(Some(key)); // Store the key in the cell for later retrieval
        if let Some(waker) = self.waiting_add.borrow_mut().take() {
            waker.wake();
        }
        Ok(KeyHandle { key: key_cell, queue: Rc::downgrade(&self.queue) })
    }

    pub async fn next(&self) -> LuaResult<LuaValue> {
        let mut stream = QueueStream {
            queue: self.queue.clone(),
            waiting_add: self.waiting_add.clone(),
        };

        // Attempt to get the next expired item
        match StreamExt::next(&mut stream).await {
            Some(value) => return Ok(value),
            None => {
                // This should never happen, but just in case
                return Err(LuaError::external("Delay channel closed unexpectedly"));
            }
        }
    }
}

impl LuaUserData for DelayChannel {
    fn add_methods<M: mluau::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("add", |_, this, (value, delay): (LuaValue, LuaUserDataRef<TimeDelta>)| {
            let delay = delay.timedelta.to_std().map_err(LuaError::external)?;
            this.add(value, delay)
        });

        methods.add_method("clear", |_, this, (): ()| {
            Ok(this.queue.try_borrow_mut().map_err(LuaError::external)?.clear())
        });

        methods.add_scheduler_async_method("next", async move |_, this, ()| {
            this.next().await
        });

        methods.add_meta_method(LuaMetaMethod::Len, |_, this, _: ()| {
            Ok(this.queue.try_borrow().map_err(LuaError::external)?.len())
        });
    }
}

struct QueueStream {
    queue: Rc<RefCell<DelayQueue<Item>>>,
    waiting_add: Rc<RefCell<Option<Waker>>>,
}

impl Stream for QueueStream {
    type Item = LuaValue;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut queue = self.queue.borrow_mut();
        
        loop {
            match queue.poll_expired(cx) {
                Poll::Ready(Some(item)) => {
                    let item = item.into_inner();
                    let now = Instant::now();
                    let key_cell = item.key.clone();

                    if now < item.final_expiry {
                        // If we woke up too early, reinsert with the remaining time
                        // and keep looping
                        //
                        // The loop will hit poll_expired again, which registers the waker 
                        // for this new item (or the next earliest item).
                        let remaining = item.final_expiry - now;
                        let safe_delay = DelayChannel::get_safe_delay(remaining);
                        let new_key = queue.insert(item, safe_delay);
                        key_cell.set(Some(new_key)); // Update the key in the cell
                        continue;
                    } else {
                        // We've actually expired here, return the value
                        key_cell.set(None); // Clear the key since it's been removed
                        return Poll::Ready(Some(item.value));
                    }
                },
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => { 
                    // We want to wait for new items to be added
                    // Store the waker so `add` can wake us up later
                    *self.waiting_add.borrow_mut() = Some(cx.waker().clone());
                    return Poll::Pending
                }
            }
        }
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("Stream", lua.create_function(|_, ()| {
        Ok(LuaStream::new())
    })?)?;

    module.set("DelayChannel", lua.create_function(|_, ()| {
        Ok(DelayChannel::new())
    })?)?;

    Ok(module)
}