use axum::response::IntoResponse;
use khronos_runtime::lua_promise;
use khronos_runtime::plugins::antiraid::datetime::TimeDelta;
use mlua::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use super::http_client::Headers;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
}

impl FromLua for Method {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => {
                let s = s.to_str()?;

                match s.as_ref() {
                    "GET" => Ok(Method::GET),
                    "POST" => Ok(Method::POST),
                    "PUT" => Ok(Method::PUT),
                    "PATCH" => Ok(Method::PATCH),
                    "DELETE" => Ok(Method::DELETE),
                    "OPTIONS" => Ok(Method::OPTIONS),
                    "HEAD" => Ok(Method::HEAD),
                    "TRACE" => Ok(Method::TRACE),
                    "CONNECT" => Ok(Method::CONNECT),
                    _ => Err(LuaError::external("Unknown method")),
                }
            }
            _ => Err(LuaError::external("Method must be a string")),
        }
    }
}

impl IntoLua for Method {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let s = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
            Method::HEAD => "HEAD",
            Method::TRACE => "TRACE",
            Method::CONNECT => "CONNECT",
        };

        s.into_lua(lua)
    }
}

pub struct ServerUrl {
    pub(crate) url: axum::http::uri::Uri,
}

impl LuaUserData for ServerUrl {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("host", |_, this| Ok(this.url.host().map(|h| h.to_string())));

        fields.add_field_method_get("port", |_, this| Ok(this.url.port().map(|x| x.as_u16())));

        fields.add_field_method_get("scheme", |_, this| {
            Ok(this.url.scheme().map(|s| s.to_string()))
        });

        fields.add_field_method_get("path", |_, this| Ok(this.url.path().to_string()));

        fields.add_field_method_get("query", |_, this| {
            Ok(this.url.query().map(|q| q.to_string()))
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.url.to_string())
        });
    }
}

#[derive(Clone)]
pub struct ServerRequestBody {
    pub(crate) body: Rc<RefCell<Option<axum::body::Body>>>,
}

impl LuaUserData for ServerRequestBody {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("bytes", |_lua, this, limit: Option<usize>| {
            Ok(lua_promise!(this, limit, |lua, this, limit|, {
                let response = {
                    let mut re_guard = this.body.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(mlua::Error::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = axum::body::to_bytes(response, limit.unwrap_or(usize::MAX)).await
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
                bytes.into_lua_multi(&lua)
            }))
        });

        methods.add_method("tobuffer", |_lua, this, limit: Option<usize>| {
            Ok(lua_promise!(this, limit, |lua, this, limit|, {
                let response = {
                    let mut re_guard = this.body.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(mlua::Error::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = axum::body::to_bytes(response, limit.unwrap_or(usize::MAX)).await
                    .map_err(|e| mlua::Error::external(e.to_string()))?;

                let buffer = lua.create_buffer(bytes)?;

                Ok(buffer)
            }))
        });

        methods.add_method("json", |_lua, this, limit: Option<usize>| {
            Ok(lua_promise!(this, limit, |lua, this, limit|, {
                let response = {
                    let mut re_guard = this.body.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(mlua::Error::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = axum::body::to_bytes(response, limit.unwrap_or(usize::MAX)).await
                    .map_err(|e| mlua::Error::external(e.to_string()))?;

                let json: serde_json::Value = serde_json::from_slice(&bytes)
                    .map_err(|e| mlua::Error::external(e.to_string()))?;

                lua.to_value(&json)
            }))
        });
    }
}

pub struct ServerRequest {
    pub method: Method,
    pub url: axum::http::uri::Uri,
    pub headers: axum::http::header::HeaderMap,
    pub body: ServerRequestBody,
}

impl LuaUserData for ServerRequest {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("method", |_lua, this| Ok(this.method));
        fields.add_field_method_get("url", |_lua, this| Ok(this.url.to_string()));
        fields.add_field_method_get("headers", |_lua, this| {
            Ok(Headers {
                headers: this.headers.clone(),
            })
        });

        fields.add_field_method_get("body", |_lua, this| Ok(this.body.clone()));
    }
}

pub enum RoutedRequest {
    Request {
        method: Method,
        url: axum::http::uri::Uri,
        matched_pattern: String,
        headers: axum::http::header::HeaderMap,
        body: axum::body::Body,
        callback: tokio::sync::oneshot::Sender<axum::http::Response<axum::body::Body>>,
    },
    StopServer {},
}

#[derive(Debug, Clone)]
pub struct Router {
    pub bind_addr: (String, u16),
    pub routes: Rc<RefCell<HashMap<(Method, String), LuaThread>>>,
    pub route_timeouts: HashMap<(Method, String), Duration>,
}

impl Router {
    pub fn start_routing(
        match_routes: Vec<(Method, String, Duration)>,
    ) -> tokio::sync::mpsc::UnboundedReceiver<RoutedRequest> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        std::thread::spawn(move || {
            // Create multi-threaded tokio runtime
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async move {
                // Start axum server
                let mut app: axum::Router<()> = axum::Router::new();

                for (method, pattern_outer, timeout) in match_routes {
                    let tx = tx.clone();
                    let pattern = pattern_outer.clone();

                    let route_wrapper = move |req: axum::extract::Request| {
                        let (parts, body) = req.into_parts();

                        async move {
                            let (callback_tx, callback_rx) = tokio::sync::oneshot::channel();

                            let _ = tx.send(RoutedRequest::Request {
                                method,
                                url: parts.uri,
                                matched_pattern: pattern,
                                headers: parts.headers,
                                body,
                                callback: callback_tx,
                            });

                            match tokio::time::timeout(timeout, callback_rx).await {
                                Ok(Ok(resp)) => resp,
                                Ok(Err(err)) => (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    err.to_string(),
                                )
                                    .into_response(),
                                Err(_) => (
                                    axum::http::StatusCode::REQUEST_TIMEOUT,
                                    "Timed out waiting for upstream (after {}ms)",
                                )
                                    .into_response(),
                            }
                        }
                    };

                    match method {
                        Method::GET => {
                            app = app.route(&pattern_outer, axum::routing::get(route_wrapper))
                        }
                        Method::POST => {
                            app = app.route(&pattern_outer, axum::routing::post(route_wrapper))
                        }
                        Method::PUT => {
                            app = app.route(&pattern_outer, axum::routing::put(route_wrapper))
                        }
                        Method::PATCH => {
                            app = app.route(&pattern_outer, axum::routing::patch(route_wrapper))
                        }
                        Method::DELETE => {
                            app = app.route(&pattern_outer, axum::routing::delete(route_wrapper))
                        }
                        Method::OPTIONS => {
                            app = app.route(&pattern_outer, axum::routing::options(route_wrapper))
                        }
                        Method::HEAD => {
                            app = app.route(&pattern_outer, axum::routing::head(route_wrapper))
                        }
                        Method::TRACE => {
                            app = app.route(&pattern_outer, axum::routing::trace(route_wrapper))
                        }
                        Method::CONNECT => {
                            app = app.route(&pattern_outer, axum::routing::connect(route_wrapper))
                        }
                    }
                }
            })
        });

        rx
    }
}

impl LuaUserData for Router {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "set_bind_addr",
            |_lua, this, (addr, port): (String, u16)| {
                this.bind_addr = (addr, port);
                Ok(())
            },
        );

        methods.add_method(
            "route",
            |lua,
             this,
             (method, pattern, th): (
                Method,
                String,
                LuaEither<LuaFunction, LuaThread>,
            )| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert(
                    (
                        method,
                        pattern,
                    ),
                    tx,
                );

                Ok(())
            },
        );

        methods.add_method_mut(
            "timeout",
            |_lua,
             this,
             (method, pattern, duration): (Method, String, LuaUserDataRef<TimeDelta>)| {
                this.route_timeouts.insert(
                    (method, pattern),
                    duration.timedelta.to_std().map_err(LuaError::external)?,
                );
                Ok(())
            },
        );

        methods.add_method(
            "get",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::GET, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "post",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::POST, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "put",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::PUT, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "patch",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::PATCH, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "delete",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::DELETE, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "options",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::OPTIONS, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "head",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::HEAD, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "trace",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::TRACE, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "connect",
            |lua, this, (pattern, th): (String, LuaEither<LuaFunction, LuaThread>)| {
                let tx = match th {
                    LuaEither::Left(f) => lua.create_thread(f)?,
                    LuaEither::Right(t) => t,
                };

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::CONNECT, pattern), tx);

                Ok(())
            },
        );

        methods.add_method("serve", |_lua, this, _g: ()| {
            Ok(lua_promise!(this, _g, |lua, this, _g|, {
                let mut rx = Router::start_routing(
                    this.routes
                        .borrow()
                        .iter()
                        .map(|((method, pattern), _th)| {
                            let duration = this.route_timeouts.get(&(*method, pattern.clone()))
                                .copied()
                                .unwrap_or_else(|| Duration::from_secs(30));
                            (*method, pattern.clone(), duration)
                        })
                        .collect(),
                );

                let taskmgr = mlua_scheduler_ext::Scheduler::get(&lua);

                while let Some(req) = rx.recv().await {
                    match req {
                        RoutedRequest::Request {
                            method,
                            url,
                            matched_pattern,
                            headers,
                            body,
                            callback,
                        } => {
                            let th = {
                                let routes = this.routes.borrow();
                                let th = routes.get(&(method, matched_pattern));
                                th.cloned()
                            };

                            if let Some(th) = th {
                                let request = ServerRequest {
                                    method,
                                    url,
                                    headers,
                                    body: ServerRequestBody {
                                        body: Rc::new(RefCell::new(Some(body))),
                                    },
                                }.into_lua_multi(&lua)?;

                                taskmgr.add_deferred_thread_back(th, request);
                            } else {
                                let _ = callback.send(
                                    (
                                        axum::http::StatusCode::NOT_FOUND,
                                        "No route found for request",
                                    )
                                        .into_response(),
                                );
                            }
                        }
                        RoutedRequest::StopServer {} => break,
                    }
                }

                Ok(())
            }))
        });
    }
}

pub fn http_server(lua: &Lua) -> LuaResult<LuaTable> {
    let http_server = lua.create_table()?;

    http_server.set(
        "url",
        lua.create_function(|_, url: String| {
            Ok(ServerUrl {
                url: url.parse().map_err(LuaError::external)?,
            })
        })?,
    )?;

    http_server.set(
        "new_router",
        lua.create_function(|_lua, (addr, port): (String, u16)| {
            Ok(Router {
                bind_addr: (addr, port),
                routes: Rc::new(RefCell::new(HashMap::new())),
                route_timeouts: HashMap::new(),
            })
        })?,
    )?;

    http_server.set_readonly(true);

    Ok(http_server)
}
