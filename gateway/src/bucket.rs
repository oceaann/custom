use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use tokio::sync::Mutex;
use tokio::time::{Instant, timeout_at};
use twilight_http::Client;
use twilight_model::channel::Message;
use twilight_model::id::Id;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use database::models::config::automod::bucket::get_increase_bucket_amount;
use database::models::config::GuildConfig;
use utils::ok_or_return;
use crate::modules::automod::actions::run_action_bucket;

pub type Bucket = Arc<DashMap<Id<GuildMarker>, Mutex<HashMap<Id<UserMarker>, HashMap<String, usize>>>>>;

pub async fn incr(
    discord_http: &Arc<Client>,
    message: &Message,
    guild_config: &GuildConfig,
    bucket: &Bucket,
    user_id: Id<UserMarker>,
    key: String
) {
    let guild_id = guild_config.guild_id;

    let bucket_data = ok_or_return!(guild_config.get_bucket_action(&key), Some);
    let amount = get_increase_bucket_amount(bucket_data.amount, message);
    if amount == 0 { return }

    let count = get_bucket_count_value(
        bucket, guild_id, user_id, key.to_owned(),
        move |count| *count += amount
    ).await;

    tokio::spawn(
        timeout_at(
            Instant::now() + Duration::from_secs(bucket_data.incr_for),
            decr(bucket.to_owned(), guild_id, user_id, amount, key)
        )
    );

    if count > bucket_data.min as usize {
        for action in &bucket_data.actions {
            run_action_bucket(
                action,
                message,
                discord_http,
                guild_config,
                &bucket_data.reason
            ).await.ok();
        }
    }

}

async fn decr(bucket: Bucket, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>, amount: usize, key: String) {
    get_bucket_count_value(
        &bucket, guild_id, user_id, key,
        move |count| *count -= amount
    ).await;
}

async fn get_bucket_count_value<T>(
    bucket: &Bucket,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
    key: String,
    f: T
) -> usize where T: Fn(&mut usize) {
    let guild_buckets_mutex = bucket.entry(guild_id)
        .or_insert_with(Default::default);
    let mut guild_buckets = guild_buckets_mutex.lock().await;

    let user_buckets = guild_buckets
        .entry(user_id).or_insert_with(HashMap::new);

    let count = user_buckets.entry(key).or_insert(0);
    f(count);

    count.to_owned()
}