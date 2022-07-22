use serde::{Serialize, Deserialize};
use twilight_model::channel::Message;
use crate::models::config::automod::actions::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "count")]
pub enum IncreaseBucketAmount {
    Stickers,
    Attachments,
    Mentions(MentionsCount),
    Static(u8)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionsCount {
    count_replay: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BucketAction {
    pub attached_rule_name: String,
    pub amount: IncreaseBucketAmount,
    pub actions: Vec<Action>,
    pub reason: String,
    pub incr_for: u64,
    /// minimal value required to run action
    pub min: u8
}

pub fn get_increase_bucket_amount(amount: IncreaseBucketAmount, message: &Message) -> usize {
    match amount {
        IncreaseBucketAmount::Stickers => message.sticker_items.len(),
        IncreaseBucketAmount::Attachments => message.attachments.len(),
        IncreaseBucketAmount::Mentions(count) => {
            let mentions = message.mentions.len();
            if !count.count_replay {
                if let Some(reference) = &message.referenced_message {
                    let include_mention = message.mentions.iter()
                        .find(|m| m.id == reference.author.id).is_some();
                    if include_mention { mentions - 1 } else { mentions }
                } else { mentions }
            } else { mentions }
        },
        IncreaseBucketAmount::Static(value) => value as usize
    }
}