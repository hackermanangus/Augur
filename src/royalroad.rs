use twilight_model::gateway::payload::MessageCreate;
use std::sync::Arc;
use crate::Bot;
use std::error::Error;
use regex::Regex;
use sqlx::Executor;

struct Novel {
    novel_id: String,
    chapter_id: String
}

pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match args[1] {
        "add" => {
            let mut db = &mut bot.pool.acquire().await.unwrap();
            let query = sqlx::query_as!(Novel,"SELECT * FROM Novels WHERE novel_id=?", args[1] )
                .fetch_one(&mut db)
                .await?;
            let guild_id = msg.guild_id.unwrap().to_string();
            let channel_id = msg.channel_id.to_string();

            let check = match query {

                Some(_) => { sqlx::query!("UPDATE Guilds SET guild_id=?, channel_id=?, novel_id=?", (guild_id, channel_id, args[2])) },


            };
            let result = reqwest::get(args[2]).await;
            let result = match result {
                Ok(body) => {},
                Err(e) => {
                    bot.http.create_message(msg.channel_id).content("Unable to find fiction")?.await?;
                    return Ok(())
                }
            };
            // <td>.*?<a[^<>]*href=["'](?P<chapter_link>[^"']+)["'] regex flor made
            let re = Regex::new(r#"(?sgm)<td>.*?<a[^<>]*href=["'](?P<chapter_link>[^"']+)["']/sgm"#).unwrap();

            let regex_result = re.captures_iter(&result);
        }
        "remove" => {

        }
        "check" => {

        }
        _ => {}
    }
    Ok(())
}