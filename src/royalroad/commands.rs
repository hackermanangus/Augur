use std::error::Error;
use std::sync::Arc;

use twilight_model::gateway::payload::MessageCreate;

use crate::Bot;
use crate::royalroad::add::add;
use crate::royalroad::remove::remove;
use crate::royalroad::check::check;

pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match args[1] {
        "add" => {
            let message = add(msg.clone(), bot.clone(), args).await?;
            &bot.http.create_message(msg.channel_id).content(message.0)?.await?;
        }
        "remove" => {
            let message = remove(msg.clone(), bot.clone(), args).await?;
            &bot.http.create_message(msg.channel_id).content(message.0)?.await?;
        }
        "check" => {
            let message = check(msg.clone(), bot.clone()).await?;
            &bot.http.create_message(msg.channel_id).content(message.0)?.await?;
        }
        _ => {
            &bot.http.create_message(msg.channel_id).content("Unknown sub-command")?.await?;
        }
    }
    Ok(())
}
