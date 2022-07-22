use std::collections::HashMap;
use crate::models::config::automod::actions::Action;
use crate::models::config::automod::filters::Filter;
use serde::{Serialize, Deserialize};
use crate::models::config::automod::checks::Check;
use serde_repr::{Serialize_repr, Deserialize_repr};
use twilight_model::id::Id;
use twilight_model::id::marker::ChannelMarker;
use crate::models::config::automod::bucket::BucketAction;
use crate::models::config::automod::ignore::Ignore;

pub mod actions;
pub mod checks;
pub mod filters;
pub mod bucket;
pub mod ignore;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoModeration {
    pub rules: Vec<AutoModeratorRule>,
    pub bucket_actions: HashMap<String, BucketAction>,
    pub logs_channel: Option<Id<ChannelMarker>>,
    pub ignore: Option<Ignore>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum BasicAutoModerationRule {
    ScamLinks = 1,
    Mentions = 2,
    CapsLock = 3,
    Invites = 4
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoModeratorRule {
    // Only for dashboard configuration
    pub basic_type: Option<BasicAutoModerationRule>,
    pub name: String,
    // Automod settings
    pub filters: Vec<Filter>,
    pub checks: Vec<Check>,
    pub actions: Vec<Action>,
    pub ignore: Option<Ignore>,
    /// Reason shown in DM messages, logs and channel message
    pub reason: String
}