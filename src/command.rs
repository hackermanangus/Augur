use std::error::Error;
use std::ops::BitAnd;
use std::sync::Arc;

use twilight_model::gateway::payload::MessageCreate;
use twilight_permission_calculator::prelude::Permissions;

use crate::{Bot, royalroad};
use crate::error::AugurError;
use crate::update::start_update;

pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<&str> = msg.content.as_str().split_whitespace().collect();
    if msg.guild_id.is_none() | args.is_empty() {
        return Ok(());
    }
    match args[0] {
        ">royalroad" => {
            let member = &bot.http.guild_member(msg.guild_id.unwrap(), msg.author.id).await?.ok_or(AugurError::FailedDiscordRequest)?;
            let guild_roles = &bot.http.guild(msg.guild_id.unwrap()).await?.ok_or(AugurError::FailedDiscordRequest)?.roles;

            let mut is_admin = false;

            for role_id in member.roles.clone() {
                let role = guild_roles.get(&role_id).ok_or(AugurError::FailedDiscordRequest)?;
                let role_permissions = role.permissions;

                is_admin = role_permissions.bitand(Permissions::ADMINISTRATOR) == Permissions::ADMINISTRATOR;

                if is_admin { break; }
            }
            if is_admin {
                royalroad::handle(msg.clone(), bot, args).await?
            } else {
                &bot.http.create_message(msg.channel_id).content("Administrator permissions are needed")?.await?;
            }
        }
        ">ping" => {
            &bot.http.create_message(msg.channel_id).content("Pong!")?.await?;
        }
        ">help" => {
            &bot.http.create_message(msg.channel_id).content("```>royalroad add <link>\n>royalroad check\n>royalroad remove <link>```")?.await?;
        }
        ">force-update" => {
            if msg.author.id == bot.owner {
                println!("Executed");
                start_update(bot).await?;
            }
        }
        _ => {}
    }

    Ok(())
}