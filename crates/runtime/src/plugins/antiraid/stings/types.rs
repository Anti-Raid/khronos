use mlua::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;

/// Represents a sting on AntiRaid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sting {
    /// The sting ID
    pub id: uuid::Uuid,
    /// Src of the sting, this can be useful to store the source of a sting
    pub src: Option<String>,
    /// The number of stings
    pub stings: i32,
    /// The reason for the stings (optional)
    pub reason: Option<String>,
    /// The reason the stings were voided
    pub void_reason: Option<String>,
    /// The guild ID the sting targets
    pub guild_id: serenity::all::GuildId,
    /// The creator of the sting
    pub creator: StingTarget,
    /// The target of the sting
    pub target: StingTarget,
    /// The state of the sting
    pub state: StingState,
    /// When the sting was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the sting expires as a chrono duration
    pub duration: Option<std::time::Duration>,
    /// The data/metadata present within the sting, if any
    pub sting_data: Option<serde_json::Value>,
    /// The handle log encountered while handling the sting
    pub handle_log: serde_json::Value,
}

impl From<Sting> for antiraid_types::stings::Sting {
    fn from(sting: Sting) -> Self {
        antiraid_types::stings::Sting {
            id: sting.id,
            src: sting.src,
            stings: sting.stings,
            reason: sting.reason,
            void_reason: sting.void_reason,
            guild_id: sting.guild_id,
            creator: sting.creator.into(),
            target: sting.target.into(),
            state: sting.state.into(),
            created_at: sting.created_at,
            duration: sting.duration,
            sting_data: sting.sting_data,
            handle_log: sting.handle_log,
        }
    }
}

impl From<antiraid_types::stings::Sting> for Sting {
    fn from(sting: antiraid_types::stings::Sting) -> Self {
        Sting {
            id: sting.id,
            src: sting.src,
            stings: sting.stings,
            reason: sting.reason,
            void_reason: sting.void_reason,
            guild_id: sting.guild_id,
            creator: sting.creator.into(),
            target: sting.target.into(),
            state: sting.state.into(),
            created_at: sting.created_at,
            duration: sting.duration,
            sting_data: sting.sting_data,
            handle_log: sting.handle_log,
        }
    }
}

/// Data required to create a sting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StingCreate {
    /// Src of the sting, this can be useful to store the source of the sting
    pub src: Option<String>,
    /// The number of stings
    pub stings: i32,
    /// The reason for the stings (optional)
    pub reason: Option<String>,
    /// The reason the stings were voided
    pub void_reason: Option<String>,
    /// The guild ID the sting targets
    pub guild_id: serenity::all::GuildId,
    /// The creator of the sting
    pub creator: StingTarget,
    /// The target of the sting
    pub target: StingTarget,
    /// The state of the sting
    pub state: StingState,
    /// When the sting expires as a chrono duration
    pub duration: Option<std::time::Duration>,
    /// The data/metadata present within the sting, if any
    pub sting_data: Option<serde_json::Value>,
}

impl From<StingCreate> for antiraid_types::stings::StingCreate {
    fn from(sting: StingCreate) -> Self {
        antiraid_types::stings::StingCreate {
            src: sting.src,
            stings: sting.stings,
            reason: sting.reason,
            void_reason: sting.void_reason,
            guild_id: sting.guild_id,
            creator: sting.creator.into(),
            target: sting.target.into(),
            state: sting.state.into(),
            duration: sting.duration,
            sting_data: sting.sting_data,
        }
    }
}

impl From<antiraid_types::stings::StingCreate> for StingCreate {
    fn from(sting: antiraid_types::stings::StingCreate) -> Self {
        StingCreate {
            src: sting.src,
            stings: sting.stings,
            reason: sting.reason,
            void_reason: sting.void_reason,
            guild_id: sting.guild_id,
            creator: sting.creator.into(),
            target: sting.target.into(),
            state: sting.state.into(),
            duration: sting.duration,
            sting_data: sting.sting_data,
        }
    }
}

/// A sting target (either user or system)
#[derive(Debug, Clone, Copy)]
pub enum StingTarget {
    /// The sting was created by a user
    User(serenity::all::UserId),
    /// The sting was created by the system
    System,
}

impl From<StingTarget> for antiraid_types::stings::StingTarget {
    fn from(target: StingTarget) -> Self {
        match target {
            StingTarget::User(user_id) => antiraid_types::stings::StingTarget::User(user_id),
            StingTarget::System => antiraid_types::stings::StingTarget::System,
        }
    }
}

impl From<antiraid_types::stings::StingTarget> for StingTarget {
    fn from(target: antiraid_types::stings::StingTarget) -> Self {
        match target {
            antiraid_types::stings::StingTarget::User(user_id) => StingTarget::User(user_id),
            antiraid_types::stings::StingTarget::System => StingTarget::System,
        }
    }
}

impl std::fmt::Display for StingTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StingTarget::User(user_id) => write!(f, "user:{}", user_id),
            StingTarget::System => write!(f, "system"),
        }
    }
}

impl std::str::FromStr for StingTarget {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "system" {
            Ok(StingTarget::System)
        } else {
            let user_id = s
                .strip_prefix("user:")
                .ok_or_else(|| format!("Invalid sting creator: {}", s))?;
            Ok(StingTarget::User(
                user_id
                    .parse()
                    .map_err(|e| format!("Invalid user ID: {}", e))?,
            ))
        }
    }
}

// Serde impls for StingTarget
impl Serialize for StingTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for StingTarget {
    fn deserialize<D>(deserializer: D) -> Result<StingTarget, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StingTarget::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Hash, Default, Debug, Clone, Copy, PartialEq)]
pub enum StingState {
    #[default]
    Active,
    Voided,
    Handled,
}

impl From<StingState> for antiraid_types::stings::StingState {
    fn from(state: StingState) -> Self {
        match state {
            StingState::Active => antiraid_types::stings::StingState::Active,
            StingState::Voided => antiraid_types::stings::StingState::Voided,
            StingState::Handled => antiraid_types::stings::StingState::Handled,
        }
    }
}

impl From<antiraid_types::stings::StingState> for StingState {
    fn from(state: antiraid_types::stings::StingState) -> Self {
        match state {
            antiraid_types::stings::StingState::Active => StingState::Active,
            antiraid_types::stings::StingState::Voided => StingState::Voided,
            antiraid_types::stings::StingState::Handled => StingState::Handled,
        }
    }
}

impl std::fmt::Display for StingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StingState::Active => write!(f, "active"),
            StingState::Voided => write!(f, "voided"),
            StingState::Handled => write!(f, "handled"),
        }
    }
}

impl std::str::FromStr for StingState {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(StingState::Active),
            "voided" => Ok(StingState::Voided),
            "handled" => Ok(StingState::Handled),
            _ => Err(format!("Invalid sting state: {}", s).into()),
        }
    }
}

// Serde impls for StingState
impl Serialize for StingState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for StingState {
    fn deserialize<D>(deserializer: D) -> Result<StingState, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StingState::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// A list of aggregate stings
pub struct StingAggregateSet {
    /// The list of stings
    pub aggregates: Vec<StingAggregate>,
}

impl LuaUserData for StingAggregateSet {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("data", |_lua, this| Ok(this.aggregates.clone()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Len, |_lua, this, _: ()| {
            Ok(this.aggregates.len() as i64)
        });

        methods.add_method("total_stings", |_, this, _: ()| {
            Ok(StingAggregate::total_stings(this.aggregates.clone()))
        });

        methods.add_method("total_stings_per_user", |lua, this, _: ()| {
            let (userid_map, system_stings) =
                StingAggregate::total_stings_per_user(this.aggregates.clone());

            let userid_map = lua.to_value_with(&userid_map, LUA_SERIALIZE_OPTIONS)?;

            Ok((userid_map, system_stings))
        });
    }
}

#[derive(Debug, Clone)]
/// An aggregate of stings
pub struct StingAggregate {
    /// Src of the sting, this can be useful if a module wants to store the source of the sting
    pub src: Option<String>,
    /// The target of the sting
    pub target: StingTarget,
    /// The total number of stings matching this aggregate
    pub total_stings: i64,
}

impl LuaUserData for StingAggregate {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("src", |_, this| Ok(this.src.clone()));
        fields.add_field_method_get("target", |_, this| Ok(this.target.to_string()));
        fields.add_field_method_get("total_stings", |_, this| Ok(this.total_stings));
    }
}

impl From<antiraid_types::stings::StingAggregate> for StingAggregate {
    fn from(aggregate: antiraid_types::stings::StingAggregate) -> Self {
        StingAggregate {
            src: aggregate.src,
            target: aggregate.target.into(),
            total_stings: aggregate.total_stings,
        }
    }
}

impl StingAggregate {
    /// Returns the sum of all total stings in the aggregate
    pub fn total_stings(vec: Vec<StingAggregate>) -> i64 {
        vec.iter().map(|x| x.total_stings).sum()
    }

    /// Returns the total stings per-user
    ///
    /// Returns (user_id_map, system_stings)
    pub fn total_stings_per_user(
        vec: Vec<StingAggregate>,
    ) -> (std::collections::HashMap<serenity::all::UserId, i64>, i64) {
        let mut map = std::collections::HashMap::new();

        let mut system_stings = 0;

        for sting in vec {
            match sting.target {
                StingTarget::System => {
                    system_stings += sting.total_stings;
                }
                StingTarget::User(user_id) => {
                    *map.entry(user_id).or_insert(0) += sting.total_stings;
                }
            }
        }

        // Add system stings to each user
        for (_, total_stings) in map.iter_mut() {
            *total_stings += system_stings;
        }

        (map, system_stings)
    }
}
