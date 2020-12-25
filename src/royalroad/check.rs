use std::error::Error;
use std::sync::Arc;

use twilight_model::gateway::payload::MessageCreate;

use crate::Bot;
use crate::error::PendingMessage;
use crate::royalroad::royalstruct::RoyalGuild;

pub async fn check(msg: Box<MessageCreate>, bot: Arc<Bot>) -> Result<PendingMessage, Box<dyn Error + Send + Sync>> {
    let result = RoyalGuild::check(msg.guild_id, &bot.pool).await;
    let contain = match result {
        Ok(contain) => contain,
        Err(e) => {
            return Ok(PendingMessage(e.to_string()));
        }
    };
    let mut temp: String = "".to_string();
    contain.into_iter().map(|(x, y)| {
        temp.push_str(&format!("<#{}> currently houses <{}>\n", x, y));
    }).for_each(drop);
    return Ok(PendingMessage(temp));
}