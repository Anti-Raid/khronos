use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use serenity::all::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParseValue {
    Everyone,
    Users,
    Roles,
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#allowed-mentions-object).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateAllowedMentions {
    pub parse: ArrayVec<ParseValue, 3>,
    pub users: Vec<UserId>,
    pub roles: Vec<RoleId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_user: Option<bool>,
}
