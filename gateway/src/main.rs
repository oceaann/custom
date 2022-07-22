mod events;
mod modules;
mod links;
mod bucket;

use std::sync::Arc;
use crate::events::on_event;
use database::mongodb::MongoDBConnection;
use database::redis::RedisConnection;
use dotenv::dotenv;
use database::clients::{Client, DiscordClients, LoadDiscordClients};
use futures_util::StreamExt;
use twilight_gateway::Shard;
use twilight_model::gateway::Intents;
use crate::bucket::Bucket;
use crate::links::ScamLinks;

async fn create_shard(
    (id, value): (String, Client),
    mongodb: MongoDBConnection,
    redis: RedisConnection,
    scam_domains: ScamLinks,
    bucket: Bucket
) {

    let intents = Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGES | Intents::GUILDS | Intents::GUILD_BANS | Intents::GUILD_MEMBERS;

    let (shard, mut events) = Shard::new(value.token.to_owned(), intents).await.unwrap();
    shard.start().await.expect("Failed to start shard");
    println!("Created shard for {}", id);

    while let Some(event) = events.next().await {
        tokio::spawn(on_event(
            event,
            mongodb.to_owned(),
            redis.to_owned(),
            value.http.to_owned(),
            scam_domains.to_owned(),
            bucket.to_owned()
        ));
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let mongodb_url = std::env::var("MONGODB_URL").expect("Cannot load MONGODB_URL from .env");
    let redis_url = std::env::var("REDIS_URL").expect("Cannot load REDIS_URL from .env");

    let mongodb = MongoDBConnection::connect(mongodb_url).await.unwrap();
    let redis = RedisConnection::connect(redis_url).unwrap();

    let scam_domains = ScamLinks::new().await.expect("Cannot load scam links manager");
    scam_domains.connect();

    let bucket: Bucket = Default::default();

    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("Cannot load DISCORD_TOKEN from .env");

    let (discord_clients, _) = DiscordClients::load(
        &mongodb, None, None
    ).await.unwrap();

    for value in discord_clients.iter() {
        tokio::spawn(create_shard(
            (value.key().to_string(), value.to_owned()),
            mongodb.to_owned(),
            redis.to_owned(),
            scam_domains.to_owned(),
            bucket.to_owned()
        ));
    }

    create_shard(
        ("main".to_string(), Client {
            public_key: "".to_string(),
            http: Arc::new(twilight_http::Client::new(discord_token.to_owned())),
            token: discord_token
        }),
        mongodb.to_owned(),
        redis.to_owned(),
        scam_domains.to_owned(),
        bucket.to_owned()
    ).await;
}
