use std::{env, error::Error};
use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::stream::StreamExt;
use tokio::time::Duration;
use twilight_cache_inmemory::{EventType, InMemoryCache};
use twilight_gateway::{cluster::{Cluster, ShardScheme}, Event};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;

use crate::royalroad::royalstruct::{RoyalMessage, RoyalNovel};

mod db;
mod command;
mod royalroad;
mod error;

pub struct Bot {
    pub http: HttpClient,
    pub pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv().ok();
    let token = env::var("TEST_TOKEN")?;

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
            println!("launching");
            let novels = match RoyalNovel::retrieve_old(&bot.pool).await {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e.to_string());
                    break;
                }
            };
            for novel in novels.into_iter() {
                let chapter_id = match RoyalNovel::get_chapters(&novel.novel_link.clone()).await {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{}", e.to_string());
                        "".to_string()
                    }
                };
                let new_novel = RoyalNovel {
                    novel_id: novel.novel_id.clone(),
                    novel_link: novel.novel_link.clone(),
                    chapter_id,
                    precedent: true,
                };
                let _ = new_novel.update(&bot.pool).await;
                let message = novel.compare(&new_novel);
                let channels = RoyalMessage::retrieve_channel_groups(novel.novel_id.clone(), &bot.pool).await;
                if channels.is_none() {
                    break;
                } else {
                    let vec_channel = channels.unwrap();

                    println!("->{:?}", &vec_channel);
                    for channel in vec_channel.into_iter() {
                        message.chapter_id.as_slice().chunks(5);
                        for slice in message.chapter_id.chunks(5) {
                            let compounded_msg = slice.iter().map(|x| {
                                //println!("{}", &x);
                                format!("https://royalroad.com{}\n", x)
                            }).collect::<String>();
                            &bot.http.create_message(channel).content(&compounded_msg).unwrap().await;
                        }
                    }
                }
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