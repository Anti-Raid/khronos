mod httpcall;
mod error;
mod request;
mod routing;

use crate::GuildId;
use crate::UserId;

pub use self::httpcall::*;
pub use self::error::*;
pub use self::request::*;
pub use self::routing::*;
pub use crate::types::MessagePagination;

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

pub(super) async fn decode_resp<T: serde::de::DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<T, crate::dhttp::error::HttpError> {
    let result = serde_json::from_slice(&resp.bytes().await?)?;
    Ok(result)
}

