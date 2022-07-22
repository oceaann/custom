use twilight_model::channel::Message;
use database::models::config::automod::filters::{Attachments, Filter, MessageLength};

pub fn filters_match(filter: &Filter, message: &Message) -> bool {
    match filter {
        Filter::Attachments(config) => attachments(config, message),
        Filter::MessageLength(config) => message_length(config, message),
        Filter::Stickers => stickers(message)
    }
}

fn attachments(config: &Attachments, message: &Message) -> bool {
    min_max_filters(
        config.min.map(usize::from),
        config.max.map(usize::from),
        message.attachments.len()
    )
}

fn message_length(config: &MessageLength, message: &Message) -> bool {
    min_max_filters(
        config.min.map(usize::from),
        config.max.map(usize::from),
        message.content.len()
    )
}

fn stickers(message: &Message) -> bool { !message.sticker_items.is_empty() }

fn min_max_filters(min: Option<usize>, max: Option<usize>, count: usize) -> bool {
    (if let Some(min) = min {
        (min as usize) > count
    } else { false }) || (if let Some(max) = max {
        (max as usize) < count
    } else { false })
}