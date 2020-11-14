use twilight_model::gateway::payload::MessageCreate;
use std::sync::Arc;
use crate::Bot;
use std::error::Error;
use regex::Regex;
use sqlx::Executor;


pub async fn handle(msg: Box<MessageCreate>, bot: Arc<Bot>, args: Vec<&str>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match args[1] {
        "add" => {
            println!("{:?}", &args);
            let mut db = &mut bot.pool.acquire().await.unwrap();
            let query = sqlx::query("SELECT * FROM Novels WHERE novel_id=?")
                .bind(args[2])
                .execute(&mut db).await;
            let guild_id = msg.guild_id.unwrap().to_string();
            let channel_id = msg.channel_id.to_string();

            match query {
                Ok(inner) => {
                    match inner {
                        1 => {
                        sqlx::query("DELETE FROM Guilds WHERE guild_id=? AND channel_id=? AND novel_id=?")
                        .bind(&guild_id)
                        .bind(&channel_id)
                        .bind(&args[2])
                        .execute(&mut db).await ?;
                        sqlx::query("INSERT INTO Guilds (guild_id , channel_id, novel_id) VALUES (?, ?, ?)")
                        .bind(guild_id)
                        .bind(channel_id)
                        .bind(args[2])
                        .execute(&mut db).await ?;
                        &bot.http.create_message(msg.channel_id).content("Added your guild to the list!") ?.await ?;
                        return Ok(())
                    },
                        0 => {
                            let result = reqwest::get(args[2]).await;
                            let result = match result {
                                Ok(body) => body.text().await?,
                                Err(e) => {
                                    bot.http.create_message(msg.channel_id).content("Unable to find fiction")?.await?;
                                    return Ok(())
                                }
                            };
                            // <td>.*?<a[^<>]*href=["'](?P<chapter_link>[^"']+)["'] regex flor made
                            let re = Regex::new(r#"(?sgm)<td>.*?<a[^<>]*href=["'](?P<chapter_link>[^"']+)["']"#).unwrap();

                            for results in re.captures_iter(&result) {
                                println!("{:?}", &results["chapter_link"] )
                            }
                        },
                        _ => {}
                    }

                },
                Err(_) => {
                    &bot.http.create_message(msg.channel_id).content("Something went wrong, the dev has been notified!")?.await?;
                    return Ok(())
                }
            };

        }
        "remove" => {

        }
        "check" => {

        }
        _ => {}
    }
    Ok(())
}