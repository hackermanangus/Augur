use std::error::Error;
use std::sync::Arc;

use twilight_model::gateway::payload::MessageCreate;

use crate::Bot;
use crate::error::PendingMessage;
use crate::royalroad::royalstruct::RoyalNovel;

pub async fn add(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<PendingMessage, Box<dyn Error + Send + Sync>> {
    let novel = match RoyalNovel::new(args[2].to_string(), &bot.pool).await {
        Ok(novel) => novel,
        Err(e) => {
            return Ok(PendingMessage(e.to_string()));
        }
    };
    let guild = match novel.process(&bot.pool, msg.guild_id, msg.channel_id).await {
        Ok(guild) => { guild }
        Err(e) => {
            return Ok(PendingMessage(e.to_string()));
        }
    };
    return match guild.insert(&bot.pool).await {
        Ok(_) => {
            Ok(PendingMessage("Setup has been successful".to_string()))
        }
        Err(e) => {
            Ok(PendingMessage(e.to_string()))
        }
    };
}