use mlua_scheduler::{LuaSchedulerAsync, LuaSchedulerAsyncUserData};
use mluau::prelude::*;
use serenity::futures::StreamExt;
use serenity::futures::stream::FuturesUnordered;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::datetime::TimeDelta;

const MAX_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(7);

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

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("OneshotChannel", lua.create_function(|_, ()| {
        Ok(OneshotChannel::new())
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