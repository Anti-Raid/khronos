//! HTTP Client Extensions (cli.httpclient)

mod dns;

use std::error::Error;
use std::sync::Arc;

use crate::core::datetime::TimeDelta;
use crate::primitives::context::TemplateContext;
use crate::traits::context::KhronosContext;
use crate::traits::httpclientprovider::HTTPClientProvider;
use crate::{
    plugins::antiraid::LUA_SERIALIZE_OPTIONS,
};
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

pub struct Url {
    pub(crate) url: reqwest::Url,
}

impl LuaUserData for Url {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("host", |_, this| {
            Ok(this.url.host_str().map(|h| h.to_string()))
        });

        fields.add_field_method_get("port", |_, this| Ok(this.url.port()));

        fields.add_field_method_get("scheme", |_, this| Ok(this.url.scheme().to_string()));

        fields.add_field_method_get("path", |_, this| Ok(this.url.path().to_string()));

        fields.add_field_method_get("query", |_, this| {
            Ok(this.url.query().map(|q| q.to_string()))
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
                .map_err(LuaError::external)?;
            let value = this
                .headers
                .get(&key)
                .map(|v| v.to_str().unwrap().to_string());
            Ok(value)
        });

        methods.add_method_mut("set", |_, this, (key, value): (String, String)| {
            let key = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(LuaError::external)?;
            let value =
                reqwest::header::HeaderValue::from_str(&value).map_err(LuaError::external)?;
            this.headers.insert(key, value);
            Ok(())
        });

        methods.add_method_mut("remove", |_, this, key: String| {
            let key = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(LuaError::external)?;
            this.headers.remove(&key);
            Ok(())
        });

        methods.add_method("headers", |lua, this, _: ()| {
            let headers = this.to_headers_list();
            let value = lua.to_value_with(&headers, LUA_SERIALIZE_OPTIONS)?;
            Ok(value)
        });
    }
}

pub struct Request<T: KhronosContext> {
    pub(crate) client: reqwest::Client,
    pub(crate) request: reqwest::Request,
    pub(crate) httpclient_provider: T::HTTPClientProvider,
}

impl<T: KhronosContext> Request<T> {
    /// Given a domain `domain`, returns a vector of every 'domain' in the domain
    ///
    /// E.g. discord.com would return ["discord.com"]
    /// www.discord.com would return ["www.discord.com", "discord.com"]
    /// cdn.blah.discord.com would return ["cdn.blah.discord.com", "blah.discord.com", "discord.com"]
    fn extract_domain_parts(domain: &str) -> Vec<String> {
        let mut domains = vec![domain.to_string()];
        let mut parts: Vec<&str> = domain.split('.').collect();
        while parts.len() > 1 {
            parts.remove(0);
            domains.push(parts.join("."));
        }
        domains
    }

    /// Validates the URL against the HTTP client provider's rules
    fn check_url(&self, url: &reqwest::Url) -> Result<(), LuaError> {
        let base = url.scheme();

        let Some(host) = url.host_str() else {
            return Err(LuaError::external("URL does not have a valid host"));
        };

        const LOCALHOSTS: [&str; 5] = [
            "localhost",
            "127.0.0.1",
            "[::1]",   // IPv6 localhost
            "::1",     // IPv6 localhost without brackets
            "0.0.0.0", // IPv4 wildcard address
        ];

        if self.httpclient_provider.allow_localhost() && LOCALHOSTS.contains(&host) {
            if base != "http" && base != "https" {
                return Err(LuaError::external(
                    "Localhost requests must use http or https",
                ));
            }

            return Ok(()); // Allow localhost if configured
        } else {
            if LOCALHOSTS.contains(&host) {
                return Err(LuaError::external("Localhost requests are not allowed"));
            }

            if base != "https" && base != "http" {
                return Err(LuaError::external("Only HTTP/HTTPS requests are allowed"));
            }

            if url.port().is_some() {
                return Err(LuaError::external("Ports are not allowed in URLs"));
            }
        }

        let domain = url
            .domain()
            .ok_or_else(|| LuaError::external("URL does not have a valid domain"))?;

        // Check if the domain is whitelisted (whitelist only applies if there is a whitelist)
        if !self.httpclient_provider.domain_whitelist().is_empty()
            && !self
                .httpclient_provider
                .domain_whitelist()
                .contains(&domain.to_string())
        {
            return Err(LuaError::external(format!(
                "Domain {domain} is not whitelisted",
            )));
        }

        // Check if the domain is blacklisted
        let domain_parts = Self::extract_domain_parts(domain);

        for part in &domain_parts {
            if self.httpclient_provider.domain_blacklist().contains(part) {
                return Err(LuaError::external(format!("Domain {part} is blacklisted",)));
            }
        }

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for Request<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "Request");

        fields.add_field_method_get("method", |_, this| {
            let req_guard = &this.request;
            Ok(req_guard.method().as_str().to_string())
        });

        fields.add_field_method_set("method", |_, this, method: String| {
            let method =
                reqwest::Method::from_bytes(method.as_bytes()).map_err(LuaError::external)?;
            let req_guard = &mut this.request;
            *req_guard.method_mut() = method;
            Ok(())
        });

        fields.add_field_method_get("url", |_, this| {
            let req_guard = &this.request;
            Ok(Url {
                url: req_guard.url().clone(),
            })
        });

        fields.add_field_method_set("url", |_, this, url: LuaUserDataRef<Url>| {
            let req_guard = &mut this.request;
            *req_guard.url_mut() = url.url.clone();
            Ok(())
        });

        fields.add_field_method_get("headers", |_, this| {
            let req_guard = &this.request;
            let headers = req_guard.headers().clone();
            Ok(Headers { headers })
        });

        fields.add_field_method_set("headers", |_, this, headers: LuaUserDataRef<Headers>| {
            let req_guard = &mut this.request;
            *req_guard.headers_mut() = headers.headers.clone();
            Ok(())
        });

        fields.add_field_method_get("body_bytes", |lua, this| {
            let req_guard = &this.request;

            let Some(body) = req_guard.body() else {
                return Ok(LuaValue::Nil);
            };

            let bytes = body.as_bytes();

            let value = lua.to_value_with(&bytes, LUA_SERIALIZE_OPTIONS)?;
            Ok(value)
        });

        fields.add_field_method_set("body_bytes", |lua, this, body: LuaValue| {
            let req_guard = &mut this.request;
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
                LuaValue::Buffer(b) => {
                    let body = reqwest::Body::from(b.to_vec());
                    *req_guard.body_mut() = Some(body);
                }
                _ => {
                    return Err(LuaError::external("Invalid body type"));
                }
            };
            Ok(())
        });

        fields.add_field_method_get("timeout", |_, this| {
            let req_guard = &this.request;
            let timeout = req_guard.timeout();

            if let Some(timeout) = timeout {
                Ok(Some(TimeDelta {
                    timedelta: chrono::Duration::from_std(*timeout).map_err(LuaError::external)?,
                }))
            } else {
                Ok(None)
            }
        });

        fields.add_field_method_set("timeout", |_, this, timeout: LuaUserDataRef<TimeDelta>| {
            let req_guard = &mut this.request;
            *req_guard.timeout_mut() =
                Some(timeout.timedelta.to_std().map_err(LuaError::external)?);
            Ok(())
        });

        fields.add_field_method_get("version", |_, this| {
            let req_guard = &this.request;
            let version = req_guard.version();
            Ok(format!("{version:?}"))
        });

        fields.add_field_method_set("version", |_, this, version: String| {
            let version = match version.as_str() {
                "HTTP/0.9" => reqwest::Version::HTTP_09,
                "HTTP/1.0" => reqwest::Version::HTTP_10,
                "HTTP/1.1" => reqwest::Version::HTTP_11,
                "HTTP/2.0" => reqwest::Version::HTTP_2,
                "HTTP/3.0" => reqwest::Version::HTTP_3,
                _ => return Err(LuaError::external("Invalid version")),
            };

            let req_guard = &mut this.request;
            *req_guard.version_mut() = version;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method_mut("send", async |_lua, mut this, _g: ()| {
            this.httpclient_provider
                .attempt_action("send", this.request.url().as_str())
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.check_url(this.request.url())
                .map_err(LuaError::external)?;

            let builder = {
                let method = this.request.method().clone();
                let url = this.request.url().clone();
                let headers = this.request.headers().clone();
                let body = this.request.body_mut().take();
                let timeout = this.request.timeout();

                let mut builder = this.client.request(method, url).headers(headers);

                if let Some(body) = body {
                    builder = builder.body(body);
                }

                if let Some(timeout) = timeout {
                    builder = builder.timeout(*timeout);
                } else {
                    builder = builder.timeout(DEFAULT_TIMEOUT);
                }

                builder
            };

            let response = builder.send().await.map_err(|e| {
                LuaError::external(format!("{}: {:?}", e, e.source()))
            })?;

            Ok(Response::new(response))
        });
    }

    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

pub struct Response {
    pub(crate) response: Option<reqwest::Response>,
}

impl Response {
    fn new(response: reqwest::Response) -> Self {
        Self {
            response: Some(response),
        }
    }
}

impl LuaUserData for Response {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("url", |_, this| {
            let Some(response) = this.response.as_ref() else {
                return Err(LuaError::external("Response has been exhausted"));
            };
            Ok(Url {
                url: response.url().clone(),
            })
        });

        fields.add_field_method_get("status", |_, this| {
            let Some(response) = this.response.as_ref() else {
                return Err(LuaError::external("Response has been exhausted"));
            };
            Ok(response.status().as_u16())
        });

        fields.add_field_method_get("content_length", |_, this| {
            let Some(response) = this.response.as_ref() else {
                return Err(LuaError::external("Response has been exhausted"));
            };
            Ok(response.content_length().map(|l| l as i64).unwrap_or(-1))
        });

        fields.add_field_method_get("headers", |_, this| {
            let Some(response) = this.response.as_ref() else {
                return Err(LuaError::external("Response has been exhausted"));
            };

            let headers = response.headers().clone();
            Ok(Headers { headers })
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method_mut("text", async |_lua, mut this, _g: ()| {
            let response = {
                let Some(response) = this.response.take() else {
                    return Err(LuaError::external("Response has been exhausted"));
                };
                response
            };

            let text = response.text().await.map_err(LuaError::external)?;
            Ok(text)
        });

        methods.add_scheduler_async_method_mut("json", async |lua, mut this, _g: ()| {
            let response = {
                let Some(response) = this.response.take() else {
                    return Err(LuaError::external("Response has been exhausted"));
                };
                response
            };

            let json = response
                .json::<serde_json::Value>()
                .await
                .map_err(LuaError::external)?;

            let lua_value = lua.to_value_with(&json, LUA_SERIALIZE_OPTIONS)?;

            Ok(lua_value)
        });

        methods.add_scheduler_async_method_mut("bytes", async |lua, mut this, _g: ()| {
            let response = {
                let Some(response) = this.response.take() else {
                    return Err(LuaError::external("Response has been exhausted"));
                };
                response
            };

            let bytes = response.bytes().await.map_err(LuaError::external)?;
            lua.create_buffer(bytes)
        });
    }

    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaTable> {
    let Some(httpclient_provider) = token.context.httpclient_provider() else {
        return Err(LuaError::external(
            "The httpclient plugin is not supported in this context",
        ));
    };

    let http_client = lua.create_table()?;

    let client = reqwest::Client::builder()
        .user_agent("Anti-Raid/Khronos (v7.0.0)")
        .redirect(reqwest::redirect::Policy::none()) // No redirects due to security concerns + code maintainability, the user should manually follow them if they want to
        .timeout(DEFAULT_TIMEOUT)
        //.https_only(!httpclient_provider.allow_localhost()) // Enforce HTTPS
        .dns_resolver(Arc::new(
            dns::HickoryDnsResolver::new(httpclient_provider.allow_localhost())
                .map_err(LuaError::external)?,
        ))
        .build()
        .map_err(LuaError::external)?;

    http_client.set(
        "new_request",
        lua.create_function(move |_lua, (method, url): (String, String)| {
            let url = reqwest::Url::parse(&url).map_err(LuaError::external)?;
            let method =
                reqwest::Method::from_bytes(method.as_bytes()).map_err(LuaError::external)?;
            Ok(Request::<T> {
                client: client.clone(),
                request: reqwest::Request::new(method, url),
                httpclient_provider: httpclient_provider.clone(),
            })
        })?,
    )?;

    http_client.set(
        "new_headers",
        lua.create_function(|_, _: ()| {
            Ok(Headers {
                headers: reqwest::header::HeaderMap::new(),
            })
        })?,
    )?;

    http_client.set_readonly(true);

    Ok(http_client)
}
