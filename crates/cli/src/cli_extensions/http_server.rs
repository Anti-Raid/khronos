use axum::extract::FromRequestParts;
use axum::response::IntoResponse;
use khronos_runtime::lua_promise;
use khronos_runtime::plugins::antiraid::datetime::TimeDelta;
use khronos_runtime::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use khronos_runtime::primitives::create_userdata_iterator_with_fields;
use khronos_runtime::rt::mlua::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use std::time::Duration;

use super::http_client::Headers;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Method {
    ANY,
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
                    "ANY" | "*" => Ok(Method::ANY),
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
            Method::ANY => "ANY",
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

pub struct ServerResponse {
    pub response: axum::http::Response<axum::body::Body>,
}

impl LuaUserData for ServerResponse {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("status", |_lua, this| Ok(this.response.status().as_u16()));

        fields.add_field_method_set("status", |_lua, this, status: u16| {
            *this.response.status_mut() =
                axum::http::StatusCode::from_u16(status).map_err(LuaError::external)?;
            Ok(())
        });

        fields.add_field_method_get("headers", |_lua, this| {
            Ok(Headers {
                headers: this.response.headers().clone(),
            })
        });

        fields.add_field_method_set("headers", |_lua, this, headers: LuaUserDataRef<Headers>| {
            *this.response.headers_mut() = headers.headers.clone();
            Ok(())
        });

        fields.add_field_method_set(
            "body",
            |_lua, this, body: LuaUserDataRef<ServerRequestBody>| {
                let mut body_guard = body.body.borrow_mut();
                let Some(body) = body_guard.take() else {
                    return Err(LuaError::external("Body has been exhausted"));
                };

                *this.response.body_mut() = body;
                Ok(())
            },
        );

        fields.add_field_method_get("version", |_lua, this| {
            Ok(match this.response.version() {
                axum::http::Version::HTTP_09 => "HTTP/0.9",
                axum::http::Version::HTTP_10 => "HTTP/1.0",
                axum::http::Version::HTTP_11 => "HTTP/1.1",
                axum::http::Version::HTTP_2 => "HTTP/2",
                axum::http::Version::HTTP_3 => "HTTP/3",
                _ => "Unknown",
            })
        });

        fields.add_field_method_set("version", |_lua, this, version: String| {
            *this.response.version_mut() = match version.as_str() {
                "HTTP/0.9" => axum::http::Version::HTTP_09,
                "HTTP/1.0" => axum::http::Version::HTTP_10,
                "HTTP/1.1" => axum::http::Version::HTTP_11,
                "HTTP/2" => axum::http::Version::HTTP_2,
                "HTTP/3" => axum::http::Version::HTTP_3,
                _ => return Err(LuaError::external("Invalid version")),
            };
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<ServerResponse>() {
                return Err(LuaError::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "status", "headers", "body", "version",
                    // Methods
                ],
            )
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
                        return Err(LuaError::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = axum::body::to_bytes(response, limit.unwrap_or(usize::MAX)).await
                    .map_err(|e| LuaError::external(e.to_string()))?;
                bytes.into_lua_multi(&lua)
            }))
        });

        methods.add_method("tobuffer", |_lua, this, limit: Option<usize>| {
            Ok(lua_promise!(this, limit, |lua, this, limit|, {
                let response = {
                    let mut re_guard = this.body.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(LuaError::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = axum::body::to_bytes(response, limit.unwrap_or(usize::MAX)).await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                let buffer = lua.create_buffer(bytes)?;

                Ok(buffer)
            }))
        });

        methods.add_method("json", |_lua, this, limit: Option<usize>| {
            Ok(lua_promise!(this, limit, |lua, this, limit|, {
                let response = {
                    let mut re_guard = this.body.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(LuaError::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = axum::body::to_bytes(response, limit.unwrap_or(usize::MAX)).await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                let json: serde_json::Value = serde_json::from_slice(&bytes)
                    .map_err(|e| LuaError::external(e.to_string()))?;

                lua.to_value_with(&json, LUA_SERIALIZE_OPTIONS)
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<ServerRequestBody>() {
                return Err(LuaError::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    // Methods
                    "bytes", "tobuffer", "json",
                ],
            )
        });
    }
}

#[derive(Debug, Clone)]
pub enum BindAddr {
    #[cfg(unix)]
    Unix {
        path: std::path::PathBuf,
    },
    Tcp {
        addr: std::net::SocketAddr,
    },
}

impl FromLuaMulti for BindAddr {
    fn from_lua_multi(value: LuaMultiValue, _lua: &Lua) -> LuaResult<Self> {
        if value.len() == 1 {
            match value.front().unwrap() {
                LuaValue::String(s) => {
                    let s = s.to_str()?;

                    if s.starts_with("unix:") {
                        #[cfg(unix)]
                        return Ok(BindAddr::Unix {
                            path: if let Some(stripped) = s.strip_prefix("unix:") {
                                stripped.into()
                            } else {
                                return Err(LuaError::external("Invalid Unix socket path"));
                            },
                        });

                        #[cfg(not(unix))]
                        return Err(LuaError::external(
                            "Unix sockets are not supported on this platform",
                        ));
                    } else {
                        return Ok(BindAddr::Tcp {
                            addr: s.parse().map_err(LuaError::external)?,
                        });
                    }
                }
                LuaValue::UserData(ud) => {
                    return Ok(ud.borrow::<BindAddr>()?.clone());
                }
                _ => return Err(LuaError::external("Invalid bind address provided")),
            }
        } else if value.len() == 2 {
            let fv = value.front().unwrap();
            let sv = value.back().unwrap();

            #[allow(clippy::single_match)]
            let addr = match fv {
                LuaValue::String(s) => {
                    let s = s.to_str()?;

                    if s.starts_with("unix:") {
                        return Err(LuaError::external("Invalid bind address provided"));
                    }

                    s.to_string()
                }
                _ => return Err(LuaError::external("Invalid bind address provided")),
            };

            let port = match sv {
                LuaValue::Integer(i) => *i as u16,
                LuaValue::String(s) => s.to_str()?.parse().map_err(LuaError::external)?,
                _ => return Err(LuaError::external("Invalid port provided")),
            };

            return Ok(BindAddr::Tcp {
                addr: format!("{}:{}", addr, port)
                    .to_socket_addrs()
                    .map_err(LuaError::external)?
                    .next()
                    .ok_or_else(|| LuaError::external("Invalid bind address provided"))?,
            });
        }

        Err(LuaError::external("Invalid bind address provided"))
    }
}

impl LuaUserData for BindAddr {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("type", |_lua, this| match this {
            #[cfg(unix)]
            BindAddr::Unix { .. } => Ok("unix".to_string()),
            BindAddr::Tcp { .. } => Ok("tcp".to_string()),
        });

        #[cfg(unix)]
        fields.add_field_method_get("path", |_lua, this| match this {
            BindAddr::Unix { path } => Ok(path.to_string_lossy().to_string()),
            _ => Err(LuaError::external("Not a Unix socket")),
        });

        fields.add_field_method_get("addr", |_lua, this| match this {
            BindAddr::Tcp { addr, .. } => Ok(addr.to_string()),
            _ => Err(LuaError::external("Not a TCP socket")),
        });

        fields.add_field_method_get("port", |_lua, this| match this {
            BindAddr::Tcp { addr, .. } => Ok(addr.port()),
            _ => Err(LuaError::external("Not a TCP socket")),
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| match this {
            #[cfg(unix)]
            BindAddr::Unix { path } => Ok(path.to_string_lossy().to_string()),
            BindAddr::Tcp { addr } => Ok(addr.to_string()),
        });
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<BindAddr>() {
                return Err(LuaError::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "type", "path", "addr", "port",
                    // Methods
                ],
            )
        });
    }
}

pub struct ServerRequest {
    pub route_method: Method,
    pub path:
        Result<axum::extract::RawPathParams, axum::extract::rejection::RawPathParamsRejection>,
    pub parts: axum::http::request::Parts,
    pub body: ServerRequestBody,
}

impl LuaUserData for ServerRequest {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("route_method", |_lua, this| Ok(this.route_method));
        fields.add_field_method_get("method", |_lua, this| Ok(this.parts.method.to_string()));
        fields.add_field_method_get("url", |_lua, this| Ok(this.parts.uri.to_string()));
        fields.add_field_method_get("headers", |_lua, this| {
            Ok(Headers {
                headers: this.parts.headers.clone(),
            })
        });

        fields.add_field_method_get("body", |_lua, this| Ok(this.body.clone()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("path", |lua, this, _g: ()| match &this.path {
            Ok(ref path) => {
                let tab = lua.create_table()?;
                for (key, value) in path.iter() {
                    tab.set(key, value)?;
                }

                Ok(tab)
            }
            Err(e) => Err(LuaError::external(e.to_string())),
        });
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<ServerRequest>() {
                return Err(LuaError::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "method", "url", "headers", "body",
                    // Methods
                ],
            )
        });
    }
}

pub enum RoutedRequest {
    Request {
        method: Method,
        parts: axum::http::request::Parts,
        path_params:
            Result<axum::extract::RawPathParams, axum::extract::rejection::RawPathParamsRejection>,
        matched_pattern: String,
        body: axum::body::Body,
        callback: tokio::sync::oneshot::Sender<axum::http::Response<axum::body::Body>>,
    },
    StopServer {},
}

pub enum LuaServerResponseParsed {
    Response {
        resp: axum::http::Response<axum::body::Body>,
    },
}

impl From<axum::http::Response<axum::body::Body>> for LuaServerResponseParsed {
    fn from(resp: axum::http::Response<axum::body::Body>) -> Self {
        LuaServerResponseParsed::Response { resp }
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    pub stop: Rc<RefCell<Option<tokio::sync::watch::Sender<()>>>>,
    pub bind_addr: BindAddr,
    pub routes: Rc<RefCell<HashMap<(Method, String), LuaFunction>>>,
    pub route_timeouts: HashMap<(Method, String), Duration>,
}

impl Router {
    pub async fn start_routing(
        match_routes: Vec<(Method, String, Duration)>,
        bind: crate::http_binder::CreateRpcServerOptions,
        stop_chan: tokio::sync::watch::Receiver<()>,
    ) -> Result<tokio::sync::mpsc::UnboundedReceiver<RoutedRequest>, khronos_runtime::Error> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let (startup_status_tx, startup_status_rx) = tokio::sync::oneshot::channel();

        std::thread::spawn(move || {
            // Create multi-threaded tokio runtime
            let rt = tokio::runtime::Builder::new_multi_thread()
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
                        let (mut parts, body) = req.into_parts();

                        async move {
                            let path_params =
                                axum::extract::RawPathParams::from_request_parts(&mut parts, &())
                                    .await;

                            let (callback_tx, callback_rx) = tokio::sync::oneshot::channel();

                            let _ = tx.send(RoutedRequest::Request {
                                method,
                                parts,
                                path_params,
                                matched_pattern: pattern,
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
                        Method::ANY => {
                            app = app.route(&pattern_outer, axum::routing::any(route_wrapper))
                        }
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

                // Start the server
                crate::http_binder::start_rpc_server(bind, app, startup_status_tx, stop_chan).await;

                // Send stop signal
                let _ = tx.send(RoutedRequest::StopServer {});
            })
        });

        startup_status_rx
            .await?
            .map_err(|e| format!("failed to spawn server: {}", e))?;

        Ok(rx)
    }

    fn parse_lua_thread_response(
        output: LuaResult<Option<LuaResult<LuaMultiValue>>>,
    ) -> LuaServerResponseParsed {
        match output {
            Ok(Some(result)) => match result {
                Ok(output) => Self::parse_mv_response(output),
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    .into_response()
                    .into(),
            },
            Ok(None) => (axum::http::StatusCode::NO_CONTENT, "")
                .into_response()
                .into(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                .into_response()
                .into(),
        }
    }

    fn parse_mv_response(output: LuaMultiValue) -> LuaServerResponseParsed {
        let Some(response) = output.front() else {
            return (axum::http::StatusCode::NO_CONTENT, "")
                .into_response()
                .into();
        };

        match response {
            LuaValue::UserData(ud) => {
                if let Ok(response) = ud.take::<ServerResponse>() {
                    response.response.into()
                } else {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Invalid response from upstream!",
                    )
                        .into_response()
                        .into()
                }
            }
            LuaValue::String(s) => (axum::http::StatusCode::OK, s.to_string_lossy())
                .into_response()
                .into(),
            r => (axum::http::StatusCode::OK, format!("{:#?}", r))
                .into_response()
                .into(),
        }
    }
}

impl Router {
    fn is_running(&self) -> bool {
        let _g = self.stop.borrow();
        _g.is_some()
    }
}

impl LuaUserData for Router {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("routes", |lua, this| {
            let routes: Vec<_> = this
                .routes
                .borrow()
                .iter()
                .map(|((method, pattern), _th)| {
                    let duration = this
                        .route_timeouts
                        .get(&(*method, pattern.clone()))
                        .copied()
                        .unwrap_or_else(|| Duration::from_secs(30));
                    (*method, pattern.clone(), duration)
                })
                .collect();

            lua.to_value_with(&routes, LUA_SERIALIZE_OPTIONS)
        });

        fields.add_field_method_get("bind_addr", |_lua, this| Ok(this.bind_addr.clone()));

        fields.add_field_method_set(
            "bind_addr",
            |_lua, this, bind_addr: LuaUserDataRef<BindAddr>| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                this.bind_addr = bind_addr.clone();
                Ok(())
            },
        );
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "route",
            |_lua, this, (method, pattern, th): (Method, String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((method, pattern), th);

                Ok(())
            },
        );

        methods.add_method_mut(
            "timeout",
            |_lua,
             this,
             (method, pattern, duration): (Method, String, LuaUserDataRef<TimeDelta>)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                this.route_timeouts.insert(
                    (method, pattern),
                    duration.timedelta.to_std().map_err(LuaError::external)?,
                );
                Ok(())
            },
        );

        methods.add_method("any", |_lua, this, (pattern, tx): (String, LuaFunction)| {
            if this.is_running() {
                return Err(LuaError::external(
                    "Cannot add/change routes while server is running. Stop the server first.",
                ));
            }

            let mut routes = this.routes.borrow_mut();
            routes.insert((Method::ANY, pattern), tx);

            Ok(())
        });

        methods.add_method("get", |_lua, this, (pattern, tx): (String, LuaFunction)| {
            if this.is_running() {
                return Err(LuaError::external(
                    "Cannot add/change routes while server is running. Stop the server first.",
                ));
            }

            let mut routes = this.routes.borrow_mut();
            routes.insert((Method::GET, pattern), tx);

            Ok(())
        });

        methods.add_method(
            "post",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::POST, pattern), tx);

                Ok(())
            },
        );

        methods.add_method("put", |_lua, this, (pattern, tx): (String, LuaFunction)| {
            if this.is_running() {
                return Err(LuaError::external(
                    "Cannot add/change routes while server is running. Stop the server first.",
                ));
            }

            let mut routes = this.routes.borrow_mut();
            routes.insert((Method::PUT, pattern), tx);

            Ok(())
        });

        methods.add_method(
            "patch",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::PATCH, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "delete",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::DELETE, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "options",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::OPTIONS, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "head",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::HEAD, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "trace",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::TRACE, pattern), tx);

                Ok(())
            },
        );

        methods.add_method(
            "connect",
            |_lua, this, (pattern, tx): (String, LuaFunction)| {
                if this.is_running() {
                    return Err(LuaError::external(
                        "Cannot add/change routes while server is running. Stop the server first.",
                    ));
                }

                let mut routes = this.routes.borrow_mut();
                routes.insert((Method::CONNECT, pattern), tx);

                Ok(())
            },
        );

        // Clones a new router
        methods.add_method("clone", |_lua, this, _: ()| {
            Ok(Router {
                stop: Rc::new(RefCell::new(None)),
                bind_addr: this.bind_addr.clone(),
                routes: Rc::clone(&this.routes),
                route_timeouts: this.route_timeouts.clone(),
            })
        });

        // Stops the server. Is a no-op
        methods.add_method_mut("stop", |_lua, this, _: ()| {
            if let Some(stop) = this.stop.take() {
                let _ = stop.send(());
            }
            Ok(())
        });

        // Returns if the server is running
        methods.add_method("is_running", |_lua, this, _: ()| Ok(this.is_running()));

        // Starts serving requests
        methods.add_method_mut("serve", |_lua, this, _g: ()| {
            let (stop_tx, stop_rx) = tokio::sync::watch::channel(());

            {
                let mut _g = this.stop.borrow_mut();
                *_g = Some(stop_tx);
            }

            Ok(lua_promise!(this, _g, stop_rx, |lua, this, _g, stop_rx|, {
                let routes = this.routes
                .borrow()
                .iter()
                .map(|((method, pattern), _th)| {
                    let duration = this.route_timeouts.get(&(*method, pattern.clone()))
                        .copied()
                        .unwrap_or_else(|| Duration::from_secs(30));
                    (*method, pattern.clone(), duration)
                })
                .collect();

                let rx = Router::start_routing(
                    routes,
                    crate::http_binder::CreateRpcServerOptions {
                        bind: match &this.bind_addr {
                            #[cfg(unix)]
                            BindAddr::Unix { path } => {
                                crate::http_binder::CreateRpcServerBind::UnixSocket(
                                    path.clone(),
                                )
                            }
                            BindAddr::Tcp { addr } => {
                                crate::http_binder::CreateRpcServerBind::Address(
                                    *addr,
                                )
                            }
                        },
                    },
                    stop_rx
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()));

                let mut rx = match rx {
                    Ok(rx) => rx,
                    Err(e) => {

                        {
                            let mut _g = this.stop.borrow_mut();
                            *_g = None;
                        }

                        return Err(e);
                    }
                };

                let taskmgr = khronos_runtime::rt::mlua_scheduler_ext::Scheduler::get(&lua);

                while let Some(req) = rx.recv().await {
                    match req {
                        RoutedRequest::Request {
                            method,
                            parts,
                            path_params,
                            matched_pattern,
                            body,
                            callback,
                        } => {
                            let th = {
                                let routes = this.routes.borrow();
                                let th = routes.get(&(method, matched_pattern));
                                th.cloned()
                            };

                            if let Some(th) = th {
                                let th = match lua.create_thread(th) {
                                    Ok(th) => th,
                                    Err(e) => {
                                        let _ = callback.send(
                                            (
                                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                                e.to_string(),
                                            )
                                                .into_response(),
                                        );
                                        continue;
                                    }
                                };

                                let Ok(request) = ServerRequest {
                                    route_method: method,
                                    parts,
                                    path: path_params,
                                    body: ServerRequestBody {
                                        body: Rc::new(RefCell::new(Some(body))),
                                    },
                                }.into_lua_multi(&lua) else {
                                    let _ = callback.send(
                                        (
                                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                            "Failed to create request object",
                                        )
                                            .into_response(),
                                    );
                                    continue;
                                };

                                let taskmgr = taskmgr.clone();
                                tokio::task::spawn_local(async move {
                                    let output = taskmgr
                                    .spawn_thread_and_wait("AxumPath", th, request)
                                    .await;

                                    // Output must be a ServerResponse struct
                                    let parsed_output = Self::parse_lua_thread_response(output);

                                    match parsed_output {
                                        LuaServerResponseParsed::Response { resp } => {
                                            let _ = callback.send(resp);
                                        }
                                    }
                                });
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

                {
                    let mut _g = this.stop.borrow_mut();
                    *_g = None;
                }

                Ok(())
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Router>() {
                return Err(LuaError::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "routes",
                    "bind_addr",
                    // Methods
                    "route",
                    "timeout",
                    "any",
                    "get",
                    "post",
                    "put",
                    "patch",
                    "delete",
                    "options",
                    "head",
                    "trace",
                    "connect",
                    "clone",
                    "stop",
                    "is_running",
                    "serve",
                ],
            )
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
        "bind_addr",
        lua.create_function(|_lua, bind_addr: BindAddr| Ok(bind_addr))?,
    )?;

    http_server.set(
        "new_router",
        lua.create_function(|_lua, bind_addr: BindAddr| {
            Ok(Router {
                stop: Rc::new(RefCell::new(None)), // serve sets this up
                bind_addr,
                routes: Rc::new(RefCell::new(HashMap::new())),
                route_timeouts: HashMap::new(),
            })
        })?,
    )?;

    http_server.set(
        "headers",
        lua.create_function(|_lua, tab: LuaTable| {
            let mut header_map = axum::http::header::HeaderMap::new();
            for key in tab.pairs::<LuaString, LuaEither<LuaString, Vec<LuaString>>>() {
                let (key, value) = key?;

                let key = axum::http::header::HeaderName::from_bytes(&key.as_bytes())
                    .map_err(LuaError::external)?;
                match value {
                    LuaEither::Left(value) => {
                        header_map.insert(
                            key,
                            axum::http::header::HeaderValue::from_bytes(&value.as_bytes())
                                .map_err(LuaError::external)?,
                        );
                    }
                    LuaEither::Right(value) => {
                        if let Some(first) = value.first() {
                            header_map.insert(
                                &key,
                                axum::http::header::HeaderValue::from_bytes(&first.as_bytes())
                                    .map_err(LuaError::external)?,
                            );

                            for v in value.iter().skip(1) {
                                header_map.append(
                                    &key,
                                    axum::http::header::HeaderValue::from_bytes(&v.as_bytes())
                                        .map_err(LuaError::external)?,
                                );
                            }
                        }
                    }
                }
            }

            Ok(Headers {
                headers: header_map,
            })
        })?,
    )?;

    http_server.set(
        "jsonresponse",
        lua.create_function(
            |lua, (status, body, headers): (u16, LuaValue, Option<LuaUserDataRef<Headers>>)| {
                let resp = (
                    axum::http::StatusCode::from_u16(status).map_err(LuaError::external)?,
                    headers.map(|h| h.headers.clone()).unwrap_or_default(),
                    axum::Json(lua.from_value::<serde_json::Value>(body)?),
                )
                    .into_response();

                Ok(ServerResponse { response: resp })
            },
        )?,
    )?;

    http_server.set(
        "fmtresponse",
        lua.create_function(
            |_, (status, body, headers): (u16, LuaValue, Option<LuaUserDataRef<Headers>>)| {
                let resp = (
                    axum::http::StatusCode::from_u16(status).map_err(LuaError::external)?,
                    headers.map(|h| h.headers.clone()).unwrap_or_default(),
                    match body {
                        LuaValue::String(s) => s.to_str()?.to_string(),
                        _ => format!("{:#?}", body),
                    },
                )
                    .into_response();

                Ok(ServerResponse { response: resp })
            },
        )?,
    )?;

    http_server.set(
        "response",
        lua.create_function(
            |_,
             (status, body, headers): (
                u16,
                LuaEither<Vec<u8>, khronos_runtime::rt::mlua::Buffer>,
                Option<LuaUserDataRef<Headers>>,
            )| {
                let resp = (
                    axum::http::StatusCode::from_u16(status).map_err(LuaError::external)?,
                    headers.map(|h| h.headers.clone()).unwrap_or_default(),
                    match body {
                        LuaEither::Left(bytes) => bytes,
                        LuaEither::Right(buffer) => buffer.to_vec(),
                    },
                )
                    .into_response();

                Ok(ServerResponse { response: resp })
            },
        )?,
    )?;

    http_server.set_readonly(true);

    Ok(http_server)
}
