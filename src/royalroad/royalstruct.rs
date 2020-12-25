use regex::Regex;
use sqlx::{Row, SqlitePool, Error};
use twilight_model::id::{ChannelId, GuildId};
use uuid::Uuid;

use crate::error::AugurError;

// Order of operation is as following
// First we create a new RoyalNovel struct with RoyalNovel::new(novel_link, pool)
// This should automatically check for an existing novel with the same link, and retrieve it's uuid
// It should only retrieve chapters if a new uuid is generated, otherwise blank, add a field that counts for existence

// RoyalGuid::new() takes a RoyalNovel struct, this RoyalNovel struct has to follow the order of operation above
// Otherwise the database will mess up
// Once the RoyalGuild struct is built, we can insert it into the database.
pub struct RoyalGuild {
    pub guild_id: String,
    pub channel_id: String,
    pub novel_id: String,
}

impl RoyalGuild {
    pub fn new(guild: Option<GuildId>, channel: ChannelId, royal_novel: &RoyalNovel) -> RoyalGuild {
        let guild_id = guild.unwrap_or_default().to_string();
        let channel_id = channel.to_string();
        RoyalGuild {
            guild_id,
            channel_id,
            novel_id: royal_novel.novel_id.clone(),
        }
    }
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), AugurError> {
        let mut conn = pool.acquire().await.unwrap();
        let result: Result<_, Error> = sqlx::query("INSERT INTO Guilds (guild_id, channel_id, novel_id)
        VALUES (?, ?, ?)")
            .bind(&self.guild_id)
            .bind(&self.channel_id)
            .bind(&self.novel_id)
            .execute(&mut conn).await;
        result.or(Err(AugurError::UniqueConstraint))?;
        Ok(())
    }
    pub async fn remove(&self, pool: &SqlitePool) -> Result<(), AugurError> {
        let mut conn = pool.acquire().await.unwrap();
        let result: Result<_, Error> = sqlx::query("DELETE FROM Guilds WHERE novel_id = ? AND channel_id=?")
            .bind(&self.novel_id)
            .bind(&self.channel_id)
            .execute(&mut conn).await;
        result.or(Err(AugurError::FailedQuery))?;
        Ok(())
    }
    pub async fn check(guild_id: Option<GuildId>, pool: &SqlitePool) -> Result<Vec<(String, String)>, AugurError> {
        let mut conn = pool.acquire().await.unwrap();
        let guild = guild_id.unwrap().to_string();
        let result = sqlx::query(
            "SELECT DISTINCT novel_link, channel_id FROM Novels, Guilds WHERE guild_id=?")
            .bind(guild)
            .fetch_all(&mut conn)
            .await;
        return match result {
            Ok(this) => {
                if this.is_empty() { return Err(AugurError::NonExistentGuild); }
                Ok(this.into_iter().map(|x| {
                    let y: String = x.get("novel_link");
                    let z: String = x.get("channel_id");
                    (z, y)
                }).collect::<Vec<(String, String)>>())
            }
            Err(_) => {
                Err(AugurError::FailedQuery)
            }
        };
    }
}

pub struct RoyalNovel {
    pub novel_id: String,
    pub novel_link: String,
    pub chapter_id: String,
    pub precedent: bool,
}

impl RoyalNovel {
    pub async fn proc_new(novel_link: String, pool: &SqlitePool) -> Result<RoyalNovel, AugurError> {
        let (novel_id, precedent) = Self::check(&novel_link, pool).await;
        if !precedent {
            return Err(AugurError::NonExistentNovel);
        }
        Ok(RoyalNovel {
            novel_id,
            novel_link,
            chapter_id: "".to_string(),
            precedent,
        })
    }
    pub async fn new(novel_link: String, pool: &SqlitePool) -> Result<RoyalNovel, AugurError> {
        let (novel_id, precedent) = Self::check(&novel_link, pool).await;
        let chapter_id: String;
        if precedent {
            chapter_id = "".to_string();
        } else {
            match Self::get_chapters(&novel_link).await {
                Ok(t) => { chapter_id = t }
                Err(e) => return Err(e)
            }
        };
        Ok(RoyalNovel {
            novel_id,
            novel_link,
            chapter_id,
            precedent,
        })
    }
    pub async fn process(&self, pool: &SqlitePool, guild: Option<GuildId>, channel: ChannelId) -> Result<RoyalGuild, AugurError> {
        if !self.precedent {
            match self.insert(pool).await {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }
        Ok(RoyalGuild::new(guild, channel, self))
    }
    pub async fn get_chapters(novel_link: &String) -> Result<String, AugurError> {
        let page = reqwest::get(novel_link).await;
        let page = page.or(Err(AugurError::InvalidLink))?.text().await.or(Err(AugurError::InvalidLink))?;

        // <td>\s*<a\s*href=["'](?P<chapter_link>[^"']+)["']> this works, something with the old regex broke?
        let re = Regex::new(r#"(?sm)<td>\s*<a\s*href=["'](?P<chapter_link>[^"']+)["']>"#).unwrap();
        let mut temp: String = String::new();
        for capture in re.captures_iter(&page) {
            temp.push_str(&capture["chapter_link"]);
            temp.push_str(" ");
        }
        if temp.is_empty() { return Err(AugurError::NoChapters)} Ok(temp)
    }
    pub async fn check(novel_link: &String, pool: &SqlitePool) -> (String, bool) {
        let row = sqlx::query("SELECT * FROM Novels WHERE novel_link = ?")
            .bind(novel_link)
            .fetch_one(pool).await;

        return match row {
            Ok(row) => {
                let novel_id: &str = row.get("novel_id");
                (novel_id.to_string(), true)
            }
            _ => { (Uuid::new_v4().to_string(), false) }
        };

    }
    async fn insert(&self, pool: &SqlitePool) -> Result<(), AugurError> {
        if !self.precedent {
            let mut conn = pool.acquire().await.unwrap();
            let result: Result<_, Error> = sqlx::query("INSERT INTO Novels (novel_id, novel_link, chapter_id)
        VALUES (?, ?, ?)")
                .bind(&self.novel_id)
                .bind(&self.novel_link)
                .bind(&self.chapter_id)
                .execute(&mut conn).await;
            result.or(Err(AugurError::UniqueConstraint))?;
        }
        Ok(())
    }
    pub fn compare(&self, updated: &RoyalNovel) -> RoyalMessage {
        let one = self.chapter_id.split_whitespace().collect::<Vec<&str>>().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let two = updated.chapter_id.split_whitespace().collect::<Vec<&str>>().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

        let three = two.into_iter()
            .filter_map(|x| if !one.contains(&x) {
                Some(x)
            } else {
                None
            }).collect::<Vec<String>>();

        return RoyalMessage::new(self.novel_id.clone(), self.novel_link.clone(), three);
    }
    pub async fn retrieve_old(pool: &SqlitePool) -> Result<Vec<RoyalNovel>, AugurError> {
        let result = sqlx::query("\
        SELECT novel_id, novel_link, chapter_id FROM Novels").fetch_all(pool).await;

        return match result {
            Ok(k) => {Ok(
                k.into_iter().map(|x| {
                    let novel_id: &str = x.get("novel_id");
                    let novel_link: &str = x.get("novel_link");
                    let chapter_id: &str = x.get("chapter_id");
                    RoyalNovel {
                        novel_id: novel_id.to_string(),
                        novel_link: novel_link.to_string(),
                        chapter_id: chapter_id.to_string(),
                        precedent: true,
                    }
                }).collect::<Vec<RoyalNovel>>())
            }
            Err(_) => Err(AugurError::FailedQuery)
        };
    }
    pub async fn update(&self, pool: &SqlitePool) -> Result<(), AugurError> {
        let result: Result<_, Error> = sqlx::query("\
        UPDATE Novels SET chapter_id = ? WHERE novel_id = ?").bind(&self.chapter_id).bind(&self.novel_id)
            .execute(pool).await;
        result.or(Err(AugurError::FailedQuery))?;
        Ok(())
    }
}

pub struct RoyalMessage {
    pub novel_id: String,
    pub novel_link: String,
    pub chapter_id: Vec<String>,
    pub channel_id: Option<Vec<ChannelId>>,
}

impl RoyalMessage {
    pub fn new(novel_id: String, novel_link: String, chapter_id: Vec<String>) -> RoyalMessage {
        RoyalMessage {
            novel_id,
            novel_link,
            chapter_id,
            channel_id: None,
        }
    }

    pub async fn retrieve_channel_groups(novel_id: String, pool: &SqlitePool) -> Option<Vec<ChannelId>> {
        let result = sqlx::query("\
        SELECT channel_id FROM Guilds WHERE novel_id = ?")
            .bind(novel_id)
            .fetch_all(pool).await;
        return match result {
            Ok(t) => {
                Some(
                t.into_iter().map(|x| {
                    let channel_id: &str = x.get("channel_id");
                    ChannelId::from(channel_id.parse::<u64>().unwrap())
                }).collect::<Vec<ChannelId>>())
            }
            Err(_) => None
        };
    }
}


