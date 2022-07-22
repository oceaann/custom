use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use twilight_model::id::Id;
use twilight_model::id::marker::{ApplicationMarker, GuildMarker};
use crate::models::config::activity::{Levels, Top};
use crate::models::config::automod::AutoModeration;
use crate::models::config::automod::bucket::BucketAction;
use crate::models::config::moderation::{Moderation, MuteMode};

pub mod moderation;
pub mod activity;
pub mod automod;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildConfig {
    pub guild_id: Id<GuildMarker>,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub enabled: HashMap<String, bool>,
    pub moderation: Moderation,
    pub premium: bool,
    pub levels: Levels,
    pub top: Top
}

impl GuildConfig {
    pub fn default(guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            application_id: None,
            enabled: HashMap::new(),
            moderation: Moderation {
                mute_mode: MuteMode::Timeout,
                mute_role: None,
                native_support: false,
                logs_channel: None,
                dm_case: false,
                automod: AutoModeration {
                    rules: vec![],
                    bucket_actions: Default::default(),
                    logs_channel: None,
                    ignore: None
                }
            },
            premium: false,
            levels: Levels {
                xp_timeout: 0,
                xp_min: 0,
                xp_max: 0
            },
            top: Top {
                week: false,
                day: false,
                webhook_url: "".to_string()
            }
        }
    }

    pub fn get_bucket_action(&self, key: &String) -> Option<BucketAction> {
        self.moderation.automod.bucket_actions.get(key.as_str()).cloned()
    }
}

