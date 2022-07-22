use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, RoleMarker, UserMarker};
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ignore {
    pub channels: Vec<Id<ChannelMarker>>,
    pub channels_ignore_mode: IgnoreMode,
    pub roles: Vec<Id<RoleMarker>>,
    pub users: Vec<Id<UserMarker>>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum IgnoreMode {
    /// Automod checks only channels from list
    WhileList = 1,
    /// Automod don't checks channels from list
    BlackList = 2
}