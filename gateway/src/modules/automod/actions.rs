use chrono::Utc;
use database::models::config::automod::actions::{Action, Timeout};
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::channel::embed::{Embed, EmbedAuthor};
use twilight_model::channel::Message;
use twilight_model::id::Id;
use twilight_model::id::marker::GuildMarker;
use database::models::config::GuildConfig;
use twilight_model::util::Timestamp;
use utils::avatars::get_avatar_url;
use crate::Bucket;

const CUSTOM_AVATAR: &str = "https://cdn.discordapp.com/attachments/941277994935263302/951521815082180608/713880061635330110.gif";

pub async fn run_action_bucket(
    action: &Action,
    message: &Message,
    discord_http: &Arc<Client>,
    guild_config: &GuildConfig,
    reason: &String
) -> Result<(), ()> {
    let guild_id = message.guild_id.ok_or(())?;
    match action {
        Action::DirectMessage => send_direct_message(message, discord_http, reason).await,
        Action::DeleteMessage => delete_message(message, discord_http).await,
        Action::SendLogs => send_logs(message, discord_http, guild_config, reason).await,
        Action::Timeout(config) => timeout(guild_id, message, discord_http, config).await,
        Action::Kick => kick(guild_id, message, discord_http).await,
        Action::Ban => ban(guild_id, message, discord_http).await,
        _ => Ok(())
    }?;

    Ok(())
}

pub async fn run_action(
    action: &Action,
    message: &Message,
    discord_http: &Arc<Client>,
    bucket: &Bucket,
    guild_config: &GuildConfig,
    reason: &String
) -> Result<(), ()> {
    let guild_id = message.guild_id.ok_or(())?;
    match action {
        Action::DirectMessage => send_direct_message(message, discord_http, reason).await,
        Action::IncreaseBucket(key) => increase_bucket(message, discord_http, bucket, guild_config, key).await,
        Action::DeleteMessage => delete_message(message, discord_http).await,
        Action::SendLogs => send_logs(message, discord_http, guild_config, reason).await,
        Action::Timeout(config) => timeout(guild_id, message, discord_http, config).await,
        Action::Kick => kick(guild_id, message, discord_http).await,
        Action::Ban => ban(guild_id, message, discord_http).await
    }?;

    Ok(())
}

async fn send_direct_message(
    message: &Message,
    discord_http: &Arc<Client>,
    reason: &String
) -> Result<(), ()> {
    let channel = discord_http
        .create_private_channel(message.author.id)
        .exec().await.map_err(|_| ())?
        .model().await.map_err(|_| ())?;

    discord_http
        .create_message(channel.id)
        .embeds(&[Embed {
            author: Some(EmbedAuthor {
                icon_url: Some(CUSTOM_AVATAR.to_string()),
                name: "Custom [AUTOMOD]".to_string(),
                proxy_icon_url: Some(CUSTOM_AVATAR.to_string()),
                url: None,
            }),
            color: None,
            description: Some(reason.to_owned()),
            fields: vec![],
            footer: None,
            image: None,
            kind: "".to_string(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        }]).map_err(|_| ())?.exec().await.map_err(|_| ())?;

    Ok(())
}

async fn increase_bucket(
    message: &Message,
    discord_http: &Arc<Client>,
    bucket: &Bucket,
    guild_config: &GuildConfig,
    key: &String
) -> Result<(), ()> {
    let user_id = message.author.id.to_owned();
    crate::bucket::incr(discord_http, message, guild_config, bucket, user_id, key.to_owned()).await;
    Ok(())
}

async fn delete_message(
    message: &Message,
    discord_http: &Arc<Client>
) -> Result<(), ()> {
    discord_http.delete_message(message.channel_id, message.id).exec().await.ok();
    Ok(())
}

async fn send_logs(
    message: &Message,
    discord_http: &Arc<Client>,
    guild_config: &GuildConfig,
    reason: &String
) -> Result<(), ()> {
    let channel = guild_config.moderation.automod.logs_channel.ok_or(())?;
    let avatar = get_avatar_url(message.author.avatar, message.author.id);
    let embed = Embed {
        author: Some(EmbedAuthor {
            icon_url: Some(avatar.clone()),
            name: format!("{}#{} {}", message.author.name, message.author.discriminator, message.author.id),
            proxy_icon_url: Some(avatar),
            url: None,
        }),
        color: None,
        description: Some(format!(
            "Message ID: {}\nChannel: <#{}>\n Reason: {}",
            message.id, message.channel_id, reason
        )),
        fields: vec![],
        footer: None,
        image: None,
        kind: "".to_string(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: None,
        url: None,
        video: None,
    };
    discord_http.create_message(channel).embeds(&[embed]).map_err(|_| ())?.exec().await.ok();
    Ok(())
}

async fn timeout(
    guild_id: Id<GuildMarker>,
    message: &Message,
    discord_http: &Arc<Client>,
    config: &Timeout
) -> Result<(), ()> {
    let timeout_end = Utc::now().timestamp() + config.duration;
    let timestamp = Timestamp::from_secs(timeout_end).map_err(|_| ())?;

    discord_http
        .update_guild_member(guild_id, message.author.id)
        .communication_disabled_until(Some(timestamp))
        .map_err(|_| ())?.exec().await.map_err(|_| ())?;

    Ok(())
}

async fn kick(
    guild_id: Id<GuildMarker>,
    message: &Message,
    discord_http: &Arc<Client>
) -> Result<(), ()> {
    discord_http.remove_guild_member(guild_id, message.author.id).exec().await.map_err(|_| ())?;
    Ok(())
}

async fn ban(
    guild_id: Id<GuildMarker>,
    message: &Message,
    discord_http: &Arc<Client>
) -> Result<(), ()> {
    discord_http.create_ban(guild_id, message.author.id).exec().await.map_err(|_| ())?;
    Ok(())
}
