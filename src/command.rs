use std::error::Error;
use std::sync::Arc;

use twilight_model::gateway::payload::MessageCreate;

use crate::{Bot, royalroad};
use twilight_permission_calculator::prelude::Permissions;
use std::ops::BitAnd;
use crate::error::SimpleError;

pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if msg.guild_id.is_none() {
        return Ok(())
    }
    if msg.content.starts_with(">") {
        let args: Vec<&str> = msg.content.as_str().split_whitespace().collect();
        match args[0] {

            _ if msg.content.starts_with(">royalroad") => {
                let member = &bot.http.guild_member(msg.guild_id.unwrap(), msg.author.id).await?.ok_or(SimpleError::new("Failed to retrieve Member"))?;
                let guild_roles = &bot.http.guild(msg.guild_id.unwrap()).await?.ok_or(SimpleError::new("Failed to retrieve roles for member"))?.roles;

                let mut is_admin = false;

                for role_id in member.roles.clone() {
                    let role = guild_roles.get(&role_id).ok_or(SimpleError::new("Failed to retrieve role permissions"))?;
                    let role_permissions = role.permissions;

                    is_admin = role_permissions.bitand(Permissions::ADMINISTRATOR) == Permissions::ADMINISTRATOR;

                    if is_admin { break }
                }
                if is_admin {
                    royalroad::commands::handle(msg.clone(), bot, args).await?
                } else {
                    &bot.http.create_message(msg.channel_id).content("Administrator permissions are needed")?.await?;
                }
            }
            _ if msg.content.starts_with(">ping") => {
                bot.http.create_message(msg.channel_id).content("Pong!")?.await?;
            }
            _ => {}
        }
    }

    Ok(())
}