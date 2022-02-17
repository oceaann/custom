use std::sync::Arc;
use mongodb::bson::doc;
use twilight_http::Client;
use twilight_model::application::callback::CallbackData;
use twilight_model::application::interaction::application_command::CommandOptionValue;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::channel::message::MessageFlags;
use database::mongodb::MongoDBConnection;
use database::redis::RedisConnection;
use crate::check_type;
use crate::commands::context::InteractionContext;

pub async fn run(interaction: InteractionContext, mongodb: MongoDBConnection, _: RedisConnection, discord_http: Arc<Client>) -> Result<CallbackData, String> {

    let case_id = check_type!(
        interaction.options.get("id").ok_or("There is no case id".to_string())?,
        CommandOptionValue::Integer
    ).ok_or("Case id type not match".to_string())?.clone();

    let removed_case = mongodb.cases.find_one_and_update(
        doc! { "index": case_id, "removed": false }, doc! { "$set": {"removed": true } }, None
    ).await.map_err(|err| format!("{err}"))?.ok_or("Cannot find case with selected id")?;

    Ok(CallbackData {
        allowed_mentions: None,
        components: None,
        content: Some("**Removed case**".to_string()),
        embeds: Some(vec![removed_case.to_embed(discord_http).await?]),
        flags: Some(MessageFlags::EPHEMERAL),
        tts: None
    })

}