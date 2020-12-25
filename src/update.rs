use std::sync::Arc;

use crate::Bot;
use crate::error::AugurError;
use crate::royalroad::royalstruct::{RoyalMessage, RoyalNovel};

pub async fn start_update(bot: Arc<Bot>) -> Result<(), AugurError> {
    println!("Starting Update Sequence");
    let novels = RoyalNovel::retrieve_old(&bot.pool).await?;

    for novel in novels.into_iter() {
        let chapter_id = RoyalNovel::get_chapters(&novel.novel_link.clone()).await.unwrap_or("".to_string());
        let new_novel = RoyalNovel {
            novel_id: novel.novel_id.clone(),
            novel_link: novel.novel_link.clone(),
            chapter_id,
            precedent: true,
        };
        let _ = new_novel.update(&bot.pool).await;
        let message = novel.compare(&new_novel);
        let channels = RoyalMessage::retrieve_channel_groups(novel.novel_id.clone(), &bot.pool).await;
        if channels.is_none() {
            break;
        } else {
            let vec_channel = channels.unwrap();
            println!("{:?}", &vec_channel);
            for channel in vec_channel.into_iter() {
                message.chapter_id.as_slice().chunks(5);
                for slice in message.chapter_id.chunks(5) {
                    let compounded_msg = slice.iter().map(|x| {
                        //println!("{}", &x);
                        format!("https://royalroad.com{}\n", x)
                    }).collect::<String>();
                    &bot.http.create_message(channel).content(&compounded_msg).unwrap().await;
                }
            }
        }
    }
    Ok(())
}