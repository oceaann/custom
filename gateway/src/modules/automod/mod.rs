pub mod actions;
mod checks;
mod filters;

use std::sync::Arc;
use database::models::config::automod::ignore::{Ignore, IgnoreMode};
use twilight_http::Client;
use twilight_model::channel::Message;
use database::mongodb::MongoDBConnection;
use crate::modules::automod::actions::run_action;
use crate::{Bucket, ScamLinks};
use self::filters::filters_match;
use self::checks::checks_match;

/// Returns true when message shouldn't be checked by automod
fn is_ignored(message: &Message, ignore_rule: &Option<Ignore>) -> bool {
    let ignore_rule = match ignore_rule {
        Some(rule) => rule,
        None => return false
    };

    let member = match &message.member {
        Some(member) => member,
        None => return false
    };

    for role in &member.roles {
        if ignore_rule.roles.contains(role) { return true }
    }

    if ignore_rule.users.contains(&message.author.id) { return true }

    let is_whitelist = ignore_rule.channels_ignore_mode == IgnoreMode::WhileList;
    let contains_channel = ignore_rule.channels.contains(&message.channel_id);

    is_whitelist && contains_channel || !(is_whitelist || contains_channel)
}

pub async fn run(
    message: Message,
    mongodb: MongoDBConnection,
    discord_http: Arc<Client>,
    scam_domains: ScamLinks,
    bucket: Bucket
) -> Result<(), ()> {
    let guild_id = message.guild_id.ok_or(())?;
    let guild_config = mongodb.get_config(guild_id).await.map_err(|_| ())?;

    if message.content.is_empty() || message.author.bot {
        return Ok(())
    }

    if is_ignored(&message, &guild_config.moderation.automod.ignore) { return Ok(()) }

    for automod in &guild_config.moderation.automod.rules {

        if is_ignored(&message, &automod.ignore) { continue }

        for filter in &automod.filters {
            let is_filtered = filters_match(filter, &message);
            if is_filtered { return Ok(()) }
        }

        for check in &automod.checks {
            let is_allowed = checks_match(
                check, &message.content, &scam_domains
            ).await?;
            if !is_allowed { return Ok(()) }
        }

        for action in &automod.actions {
            run_action(
                action,
                &message,
                &discord_http,
                &bucket,
                &guild_config,
                &automod.reason,
            ).await.ok();
        }

    }

    Ok(())
}