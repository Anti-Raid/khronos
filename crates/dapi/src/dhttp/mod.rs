mod httpcall;
mod error;
mod request;
mod routing;

use crate::GuildId;
use crate::MessageId;
use crate::UserId;

pub use self::httpcall::*;
pub use self::error::*;
pub use self::request::*;
pub use self::routing::*;

#[non_exhaustive]
pub enum GuildPagination {
    /// The Id to get the guilds after.
    After(GuildId),
    /// The Id to get the guilds before.
    Before(GuildId),
}

#[non_exhaustive]
pub enum UserPagination {
    /// The Id to get the users after.
    After(UserId),
    /// The Id to get the users before.
    Before(UserId),
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum MessagePagination {
    After(MessageId),
    Around(MessageId),
    Before(MessageId),
}

pub(super) async fn decode_resp<T: serde::de::DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<T, crate::dhttp::error::HttpError> {
    let result = serde_json::from_slice(&resp.bytes().await?)?;
    Ok(result)
}

