use serde::{Serialize, Deserialize};
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, RoleMarker};
use crate::models::config::automod::AutoModeration;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum MuteMode {
    DependOnCommand = 1,
    Timeout = 2,
    Role = 3
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Moderation {
    pub mute_mode: MuteMode,
    pub mute_role: Option<Id<RoleMarker>>,
    pub native_support: bool,
    pub logs_channel: Option<Id<ChannelMarker>>,
    pub dm_case: bool,
    pub automod: AutoModeration
}