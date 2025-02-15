use serde::{Deserialize, Serialize};
use serenity::all::*;

/// [Discord docs](https://discord.com/developers/docs/resources/channel#overwrite-object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionOverwriteType {
    pub id: TargetId,
    #[serde(rename = "type")]
    pub kind: u8,
}
