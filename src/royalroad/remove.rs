use crate::error::PendingMessage;
use std::error::Error;
use twilight_model::gateway::payload::MessageCreate;
use std::sync::Arc;
use crate::Bot;
use crate::royalroad::royalstruct::{RoyalNovel, RoyalGuild};

pub async fn remove(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<PendingMessage, Box<dyn Error + Send + Sync>> {
    let novel = match RoyalNovel::proc_new(args[2].to_string(), &bot.pool).await {
        Ok(novel) => novel,
        Err(e) => {
            return Ok(PendingMessage(e.to_string()));
        }
    };
    let guild = RoyalGuild::new(msg.guild_id, msg.channel_id, &novel);
    return match guild.remove(&bot.pool).await {
        Ok(_) => {
            Ok(PendingMessage(format!("Cleared any existing instances of <{}> from <#{}>", novel.novel_link, guild.channel_id)))
        }
        Err(e) => {
            Ok(PendingMessage(e.to_string()))
        }
    };
}