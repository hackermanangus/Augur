use std::error::Error;
use std::sync::Arc;

use twilight_model::gateway::payload::MessageCreate;

use crate::Bot;
use crate::royalroad::royalstruct::{RoyalNovel, RoyalGuild};
use crate::error::SimpleError;


pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match args[1] {
        "add" => {
            let novel = match RoyalNovel::new(args[2].to_string(), &bot.pool).await {
                Ok(novel) => novel,
                Err(e) =>  {
                    &bot.http.create_message(msg.channel_id).content(e.to_string())?.await?;
                    return Ok(())
                }
            };
            let guild = match novel.process(&bot.pool, msg.guild_id, msg.channel_id).await {
                Ok(guild) => {guild}
                Err(e) => {
                    &bot.http.create_message(msg.channel_id).content(e.to_string())?.await?;
                    return Ok(())
                }
            };
            return match guild.insert(&bot.pool).await {
                Ok(_) => {
                    &bot.http.create_message(msg.channel_id).content("Success")?.await?;
                    Ok(())
                }
                Err(e) => {
                    &bot.http.create_message(msg.channel_id).content(e.to_string())?.await?;
                    Ok(())
                }
            };


        },
        "remove" => {
            let novel = match RoyalNovel::proc_new(args[2].to_string(), &bot.pool).await {
                Ok(novel) => novel,
                Err(e) => {&bot.http.create_message(msg.channel_id).content(e.to_string())?.await?; return Ok(())}
            };
            let guild = RoyalGuild::new(msg.guild_id, msg.channel_id, &novel);
            return match guild.remove(&bot.pool).await {
                Ok(_) => {&bot.http.create_message(msg.channel_id).content(format!("Cleared any existing instances of {} from <#{}>", novel.novel_link, guild.channel_id))?.await?; Ok(())},
                Err(e) => { &bot.http.create_message(msg.channel_id).content(e.to_string())?.await?;
                Ok(())}
            }
        },
        "check" => {
            todo!("This is not important, leave it for last")
        },
        _ => {}
    }
    Ok(())
}
