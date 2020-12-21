use std::error::Error;
use std::sync::Arc;

use regex::Regex;
use twilight_model::gateway::payload::MessageCreate;

use crate::Bot;

mod royalroadupdate;

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
                                .execute(&mut db).await?;
                            sqlx::query("INSERT INTO Guilds (guild_id , channel_id, novel_id) VALUES (?, ?, ?)")
                                .bind(guild_id)
                                .bind(channel_id)
                                .bind(args[2])
                                .execute(&mut db).await?;
                            &bot.http.create_message(msg.channel_id).content("Added your guild to the list!")?.await?;
                            return Ok(());
                        }
                        0 => {
                            let result = reqwest::get(args[2]).await;
                            let result = match result {
                                Ok(body) => body.text().await?,
                                Err(_) => {
                                    &bot.http.create_message(msg.channel_id).content("Unable to find fiction")?.await?;
                                    return Ok(());
                                }
                            };
                            // <td>.*?<a[^<>]*href=["'](?P<chapter_link>[^"']+)["'] regex flor made
                            // TODO: <meta name="description" content="(?P<description>[^">]*)["][>] regex I made big brain
                            // TODO: \/chapter\/(?P<chapter_id>[0-9]*)\/ another regex I wrote to get the id
                            let re = Regex::new(r#"(?sm)<td>.*?<a[^<>]*href=["'](?P<chapter_link>[^"']+)["']"#).unwrap();
                            let re_c_id = Regex::new(r#"/chapter/(?P<chapter_id>[0-9]*)/"#).unwrap();
                            let mut truth: bool = true;
                            let mut temp: String = String::new();
                            for capture in re.captures_iter(&result)
                            {
                                let one = re_c_id.captures(&capture["chapter_link"]);
                                if let Some(r) = one {
                                    println!("{}", &r["chapter_id"]);
                                    temp.push_str(&r["chapter_id"]);
                                    temp.push_str(" ");
                                } else {
                                    truth = false;
                                }
                            }
                            return if truth {
                                println!("{:?}", &temp);
                                sqlx::query("INSERT INTO Novels (novel_id, chapter_id) VALUES (?, ?)")
                                    .bind(args[2])
                                    .bind(temp)
                                    .execute(&mut db).await?;
                                sqlx::query("DELETE FROM Guilds WHERE guild_id=? AND channel_id=? AND novel_id=?")
                                    .bind(&guild_id)
                                    .bind(&channel_id)
                                    .bind(&args[2])
                                    .execute(&mut db).await?;
                                sqlx::query("INSERT INTO Guilds (guild_id , channel_id, novel_id) VALUES (?, ?, ?)")
                                    .bind(guild_id)
                                    .bind(channel_id)
                                    .bind(args[2])
                                    .execute(&mut db).await?;
                                &bot.http.create_message(msg.channel_id).content("Added new fiction to the database!")?.await?;
                                Ok(())
                            } else {
                                &bot.http.create_message(msg.channel_id).content("Invalid fiction provided")?.await?;
                                Ok(())
                            }
                        }
                        _ => {}
                    }
                }
                Err(_) => {
                    &bot.http.create_message(msg.channel_id).content("Something went wrong, the dev has been notified!")?.await?;
                    return Ok(());
                }
            };
        }
        "remove" => {
            todo!("Do this after add, shouldn't be too hard")
        }
        "check" => {
            todo!("This is not important, leave it for last")
        }
        _ => {}
    }
    Ok(())
}
