//! HTTP Client Extensions (cli.httpclient)

use khronos_runtime::{lua_promise, primitives::create_userdata_iterator_with_fields};
use mlua::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub struct Url {
    pub(crate) url: reqwest::Url,
}

impl LuaUserData for Url {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("host", |_, this| {
            Ok(this.url.host_str().unwrap().to_string())
        });

        fields.add_field_method_get("port", |_, this| {
            Ok(this.url.port().unwrap_or(80).to_string())
        });

        fields.add_field_method_get("scheme", |_, this| Ok(this.url.scheme().to_string()));

        fields.add_field_method_get("path", |_, this| Ok(this.url.path().to_string()));

        fields.add_field_method_get("query", |_, this| {
            Ok(this.url.query().map(|q| q.to_string()).unwrap_or_default())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.url.as_str().to_string())
        });
    }
}

pub struct Headers {
    pub(crate) headers: reqwest::header::HeaderMap,
}

impl Headers {
    fn to_headers_list(&self) -> Vec<(String, String, Vec<u8>)> {
        self.headers
            .iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_string(),
                    v.to_str().unwrap_or_default().to_string(),
                    v.as_bytes().to_vec(),
                )
            })
            .collect()
    }
}

impl LuaUserData for Headers {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("get", |_, this, key: String| {
            let key = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(mlua::Error::external)?;
            let value = this
                .headers
                .get(&key)
                .map(|v| v.to_str().unwrap().to_string());
            Ok(value)
        });

        methods.add_method_mut("set", |_, this, (key, value): (String, String)| {
            let key = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(mlua::Error::external)?;
            let value =
                reqwest::header::HeaderValue::from_str(&value).map_err(mlua::Error::external)?;
            this.headers.insert(key, value);
            Ok(())
        });

        methods.add_method_mut("remove", |_, this, key: String| {
            let key = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(mlua::Error::external)?;
            this.headers.remove(&key);
            Ok(())
        });

        methods.add_method("headers", |lua, this, _: ()| {
            let headers = this.to_headers_list();
            let value = lua.to_value(&headers)?;
            Ok(value)
        });
    }
}

#[derive(Clone)]
pub struct Request {
    pub(crate) client: reqwest::Client,
    pub(crate) request: Rc<RefCell<reqwest::Request>>,
}

impl LuaUserData for Request {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("method", |_, this| {
            let req_guard = this.request.borrow();
            Ok(req_guard.method().as_str().to_string())
        });

        fields.add_field_method_set("method", |_, this, method: String| {
            let method =
                reqwest::Method::from_bytes(method.as_bytes()).map_err(mlua::Error::external)?;
            let mut req_guard = this.request.borrow_mut();
            *req_guard.method_mut() = method;
            Ok(())
        });

        fields.add_field_method_get("url", |_, this| {
            let req_guard = this.request.borrow();
            Ok(Url {
                url: req_guard.url().clone(),
            })
        });

        fields.add_field_method_set("url", |_, this, url: LuaUserDataRef<Url>| {
            let mut req_guard = this.request.borrow_mut();
            *req_guard.url_mut() = url.url.clone();
            Ok(())
        });

        fields.add_field_method_get("headers", |_, this| {
            let req_guard = this.request.borrow();
            let headers = req_guard.headers().clone();
            Ok(Headers { headers })
        });

        fields.add_field_method_set("headers", |_, this, headers: LuaUserDataRef<Headers>| {
            let mut req_guard = this.request.borrow_mut();
            *req_guard.headers_mut() = headers.headers.clone();
            Ok(())
        });

        fields.add_field_method_get("body_bytes", |lua, this| {
            let req_guard = this.request.borrow();

            let Some(body) = req_guard.body() else {
                return Ok(LuaValue::Nil);
            };

            let bytes = body.as_bytes();

            let value = lua.to_value(&bytes)?;
            Ok(value)
        });

        fields.add_field_method_set("body_bytes", |lua, this, body: LuaValue| {
            let mut req_guard = this.request.borrow_mut();
            match body {
                LuaValue::Nil => {
                    req_guard.body_mut().take();
                }
                LuaValue::String(s) => {
                    let bytes = s.as_bytes().to_vec();
                    let body = reqwest::Body::from(bytes);
                    *req_guard.body_mut() = Some(body);
                }
                LuaValue::Table(_) => {
                    let body: Vec<u8> = lua.from_value(body)?;
                    let body = reqwest::Body::from(body);
                    *req_guard.body_mut() = Some(body);
                }
                _ => {
                    return Err(mlua::Error::external("Invalid body type"));
                }
            };
            Ok(())
        });

        fields.add_field_method_get("timeout", |_, this| {
            let req_guard = this.request.borrow();
            let timeout = req_guard.timeout();

            if let Some(timeout) = timeout {
                Ok(Some(
                    khronos_runtime::plugins::antiraid::datetime::TimeDelta {
                        timedelta: chrono::Duration::from_std(*timeout)
                            .map_err(mlua::Error::external)?,
                    },
                ))
            } else {
                Ok(None)
            }
        });

        fields.add_field_method_set("timeout", |_, this, timeout: LuaUserDataRef<khronos_runtime::plugins::antiraid::datetime::TimeDelta>| {
            let mut req_guard = this.request.borrow_mut();
            *req_guard.timeout_mut() = Some(timeout.timedelta.to_std().map_err(mlua::Error::external)?);
            Ok(())
        });

        fields.add_field_method_get("version", |_, this| {
            let req_guard = this.request.borrow();
            let version = req_guard.version();
            Ok(format!("{:?}", version))
        });

        fields.add_field_method_set("version", |_, this, version: String| {
            let version = match version.as_str() {
                "HTTP/0.9" => reqwest::Version::HTTP_09,
                "HTTP/1.0" => reqwest::Version::HTTP_10,
                "HTTP/1.1" => reqwest::Version::HTTP_11,
                "HTTP/2.0" => reqwest::Version::HTTP_2,
                "HTTP/3.0" => reqwest::Version::HTTP_3,
                _ => return Err(mlua::Error::external("Invalid version")),
            };

            let mut req_guard = this.request.borrow_mut();
            *req_guard.version_mut() = version;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("send", |_lua, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                let builder = {
                    let mut req_guard = this.request.borrow_mut();

                    let method = req_guard.method().clone();
                    let url = req_guard.url().clone();
                    let headers = req_guard.headers().clone();
                    let body = req_guard.body_mut().take();
                    let timeout = req_guard.timeout();

                    let mut builder = this.client.request(method, url)
                        .headers(headers);

                    if let Some(body) = body {
                        builder = builder.body(body);
                    }

                    if let Some(timeout) = timeout {
                        builder = builder.timeout(*timeout);
                    }

                    builder
                };

                let response = builder
                    .send()
                    .await
                    .map_err(mlua::Error::external)?;

                Ok(Response::new(response))
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Request>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "method",
                    "url",
                    "headers",
                    "body_bytes",
                    "timeout",
                    "version",
                    // Methods
                    "send",
                ],
            )
        });
    }
}

#[derive(Clone)]
pub struct Response {
    pub(crate) response: Rc<RefCell<Option<reqwest::Response>>>,
}

impl Response {
    fn new(response: reqwest::Response) -> Self {
        Self {
            response: Rc::new(RefCell::new(Some(response))),
        }
    }
}

impl LuaUserData for Response {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("url", |_, this| {
            let re_guard = this.response.borrow();
            let Some(response) = re_guard.as_ref() else {
                return Err(mlua::Error::external("Response has been exhausted"));
            };
            Ok(Url {
                url: response.url().clone(),
            })
        });

        fields.add_field_method_get("status", |_, this| {
            let re_guard = this.response.borrow();
            let Some(response) = re_guard.as_ref() else {
                return Err(mlua::Error::external("Response has been exhausted"));
            };
            Ok(response.status().as_u16())
        });

        fields.add_field_method_get("content_length", |_, this| {
            let re_guard = this.response.borrow();
            let Some(response) = re_guard.as_ref() else {
                return Err(mlua::Error::external("Response has been exhausted"));
            };
            Ok(response.content_length().map(|l| l as i64).unwrap_or(-1))
        });

        fields.add_field_method_get("headers", |_, this| {
            let re_guard = this.response.borrow();
            let Some(response) = re_guard.as_ref() else {
                return Err(mlua::Error::external("Response has been exhausted"));
            };

            let headers = response.headers().clone();
            Ok(Headers { headers })
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("text", |_lua, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                let response = {
                    let mut re_guard = this.response.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(mlua::Error::external("Response has been exhausted"));
                    };
                    response
                };

                let text = response.text().await.map_err(mlua::Error::external)?;
                Ok(text)
            }))
        });

        methods.add_method("json", |_lua, this, _g: ()| {
            Ok(lua_promise!(this, _g, |lua, this, _g|, {
                let response = {
                    let mut re_guard = this.response.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(mlua::Error::external("Response has been exhausted"));
                    };
                    response
                };

                let json = response.json::<serde_json::Value>().await.map_err(mlua::Error::external)?;
                
                let lua_value = lua.to_value(&json)?;

                Ok(lua_value)
            }))
        });

        methods.add_method("bytes", |_lua, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                let response = {
                    let mut re_guard = this.response.borrow_mut();
                    let Some(response) = re_guard.take() else {
                        return Err(mlua::Error::external("Response has been exhausted"));
                    };
                    response
                };

                let bytes = response.bytes().await.map_err(mlua::Error::external)?;
                Ok(bytes.to_vec())
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Response>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "url",
                    "status",
                    "content_length",
                    "headers",
                    // Methods
                    "text",
                    "json",
                    "bytes",
                ],
            )
        });
    }
}

pub fn http_client(lua: &Lua) -> LuaResult<LuaTable> {
    let http_client = lua.create_table()?;

    let client = reqwest::Client::new();

    http_client.set(
        "new_request",
        lua.create_function(move |_lua, (method, url): (String, String)| {
            let url = reqwest::Url::parse(&url).map_err(mlua::Error::external)?;
            let method = reqwest::Method::from_bytes(method.as_bytes()).map_err(mlua::Error::external)?;
            Ok(Request {
                client: client.clone(),
                request: Rc::new(RefCell::new(reqwest::Request::new(method, url))),
            })
        })?
    )?;

    Ok(http_client)
}