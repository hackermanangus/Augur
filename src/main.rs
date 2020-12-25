use std::{env, error::Error};
use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::stream::StreamExt;
use tokio::time::Duration;
use twilight_cache_inmemory::{EventType, InMemoryCache};
use twilight_gateway::{cluster::{Cluster, ShardScheme}, Event};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;

use crate::update::start_update;
use twilight_model::id::UserId;

mod db;
mod command;
mod royalroad;
mod error;
pub mod update;

pub struct Bot {
    pub http: HttpClient,
    pub pool: SqlitePool,
    pub owner: UserId
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv().ok();
    let token = env::var("TEST_TOKEN")?;
    let owner = UserId(env::var("OWNER")?.parse().unwrap());

    // This is the default scheme. It will automatically create as many
    // shards as is suggested by Discord.
    let scheme = ShardScheme::Auto;

    // Use intents to only receive guild message events.
    let cluster = Cluster::builder(&token, Intents::GUILD_MESSAGES)
        .shard_scheme(scheme)
        .build()
        .await?;

    // Start up the cluster.
    let cluster_spawn = cluster.clone();

    // Start all shards in the cluster in the background.
    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // HTTP is separate from the gateway, so create a new client.
    let http = HttpClient::new(&token);
    let mut pool = db::connect_database().await.expect("Unable to connect to database");
    db::setup_database(&mut pool).await.unwrap();
    let bot = Arc::new(Bot {
        http,
        pool,
        owner
    });


    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = InMemoryCache::builder()
        .event_types(
            EventType::MESSAGE_CREATE
        )
        .build();

    let mut events = cluster.events();
    let other_bot = bot.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::delay_for(Duration::from_secs(600)).await;
            let n = start_update(bot.clone()).await;
            match n {
                Ok(_) => continue,
                Err(e) => {println!("{:?}", e); continue}
            }
        }
    });
    // Process each event as they come in.
    while let Some((shard_id, event)) = events.next().await {
        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, other_bot.clone()));
    }


    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    bot: Arc<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) => {
            command::handle(msg, bot).await?;
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}