use std::error::Error;
use std::sync::Arc;

use chrono::{SecondsFormat, Utc};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
use twilight_model::gateway::payload::MessageCreate;

use crate::Bot;
use crate::royalroad::add::add;
use crate::royalroad::check::check;
use crate::royalroad::remove::remove;

pub mod royalstruct;
mod remove;
mod check;
mod add;

pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let embed = EmbedBuilder::new()
        .title("Result")?
        .timestamp(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true))
        .color(0x80_00_80)?;

    match args[1] {
        "add" => {
            if args.len() < 3 {
                let embed = embed.description("`EmptyArgument`: No provided arguments")?.field(EmbedFieldBuilder::new("Usage", ">royalroad add <link>")?).build()?;
                &bot.http.create_message(msg.channel_id).embed(embed)?.await?;
            } else {
                let message = add(msg.clone(), bot.clone(), args).await?;
                let embed = embed.description(message.0)?.build()?;
                &bot.http.create_message(msg.channel_id).embed(embed)?.await?;
            }
        }
        "remove" => {
            if args.len() < 3 {
                let embed = embed.description("`EmptyArgument`: No provided arguments")?.field(EmbedFieldBuilder::new("Usage", ">royalroad remove <link>")?).build()?;
                &bot.http.create_message(msg.channel_id).embed(embed)?.await?;
            } else {
                let message = remove(msg.clone(), bot.clone(), args).await?;
                let embed = embed.description(message.0)?.build()?;
                &bot.http.create_message(msg.channel_id).embed(embed)?.await?;
            }
        }
        "check" => {
            let message = check(msg.clone(), bot.clone()).await?;
            let embed = embed.description(message.0)?.build()?;
            &bot.http.create_message(msg.channel_id).embed(embed)?.await?;
        }
        _ => {
            let embed = embed.description("`Unknown subcommand`")?.build()?;
            &bot.http.create_message(msg.channel_id).embed(embed)?.await?;
        }
    }
    Ok(())
}
