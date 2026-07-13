use std::sync::Arc;

use reqwest::header::{
    AUTHORIZATION,
    CONTENT_LENGTH,
    CONTENT_TYPE,
    HeaderValue,
    USER_AGENT,
};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::ApplicationId;
use crate::dhttp::ErrorResponse;

use super::{HttpCall, HttpError};

/// The maximum unicode code points allowed within a message by Discord.
pub const MESSAGE_CODE_LIMIT: usize = 2000;

/// The [UserAgent] sent along with every request.
///
/// [UserAgent]: ::reqwest::header::USER_AGENT
pub const USER_AGENT_STR: &str = concat!(
    "DiscordBot (https://github.com/Anti-Raid/khronos, ",
    env!("CARGO_PKG_VERSION"),
    ")"
);

pub enum ClientKind {
    Bot { token: String },
    Oauth2 { token: String }
}

struct ClientInner {
    token: HeaderValue,
    discord: String,
    client: reqwest::Client,
    app_id: ApplicationId
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

impl Client {
    pub fn new(discord: String, kind: ClientKind, client: reqwest::Client, app_id: ApplicationId) -> Self {
        let token = match kind {
            ClientKind::Bot { token } => format!("Bot {token}"),
            ClientKind::Oauth2 { token } => format!("Bearer {token}")
        };
        Self { inner: Arc::new(ClientInner { token: HeaderValue::from_str(&token).unwrap(), discord: discord.into(), client, app_id }) }
    }

    pub fn nest(&self, discord: String, kind: ClientKind, client: reqwest::Client) -> Self {
        let token = match kind {
            ClientKind::Bot { token } => format!("Bot {token}"),
            ClientKind::Oauth2 { token } => format!("Bearer {token}")
        };
        Self { inner: Arc::new(ClientInner { token: HeaderValue::from_str(&token).unwrap(), discord: discord.into(), client, app_id: self.inner.app_id }) }
    }

    pub fn app_id(&self) -> ApplicationId {
        self.inner.app_id
    }

    /// Make an http call to discord
    pub async fn call<'a, T: DeserializeOwned>(&self, call: HttpCall<'a>) -> Result<Option<T>, HttpError> {
        let req = call.into_url_and_body();
        let mut headers = req.headers.unwrap_or_default();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(
            AUTHORIZATION,
            self.inner.token.clone(),
        );

        let furl = format!("{}/{}", self.inner.discord, req.url);
        let fmethod = req.method;
        let mut reqw = self.inner.client.request(fmethod.clone(), furl);
        if let Some(body) = req.body {
            headers.insert(CONTENT_LENGTH, body.len().into());
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            reqw = reqw.body(body);
        } else {
            headers.insert(CONTENT_LENGTH, 0.into()); 
        }

        let reqw = reqw.headers(headers).build()?;
        let resp = self.inner.client.execute(reqw).await?;

        if resp.status().is_success() {
            if resp.status() == StatusCode::NO_CONTENT {
                Ok(None)
            } else {
                let json = resp.json().await?;
                Ok(Some(json))
            }
        } else {
            Err(HttpError::UnsuccessfulRequest(
                ErrorResponse::from_response(resp, fmethod).await,
            ))
        }
    }
}
