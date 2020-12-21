use twilight_model::gateway::payload::MessageCreate;
use crate::{Bot, royalroad};
use std::sync::Arc;
use std::error::Error;

pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if msg.content.starts_with(">") {
        let args: Vec<&str>= msg.content.as_str().split_whitespace().collect();
        match args[0]{
        _ if msg.content.starts_with(">royalroad") => {
            royalroad::commands::handle(msg.clone(), bot, args).await?
        }
        _ if msg.content.starts_with(">ping") => {
            bot.http.create_message(msg.channel_id).content("Pong!")?.await?;
        }
        _ => {}
    }}

    Ok(())
}