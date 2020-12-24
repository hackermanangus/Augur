use std::error::Error;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub enum AugurError {
    NonExistentNovel, // Novel doesn't exist in db
    NonExistentGuild, // Guild doesn't exist in db
    UniqueConstraint, // Novel already setup in channel
    NoChapters, // No chapters found for novel
    InvalidLink, // Link provided for novel is invalid
    FailedQuery, // Query to db has failed
    FailedDiscordRequest, // When discord API fails
}



impl Display for AugurError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AugurError::NonExistentNovel => { write!(f, "{}, Novel not found", self)}
            AugurError::NonExistentGuild => { write!(f, "{}, No novels setup in Guild", self)}
            AugurError::UniqueConstraint => { write!(f, "{}, Novel has already been setup in this channel", self)}
            AugurError::NoChapters => { write!(f, "{}, No chapters found for provided novel", self)}
            AugurError::InvalidLink => { write!(f, "{}, Provided novel link is invalid", self)}
            AugurError::FailedQuery => { write!(f, "{}, Something went wrong while querying the database", self)}
            AugurError::FailedDiscordRequest => { write!(f, "{}, Request to Discord API failed", self)}
        }
    }
}

impl Error for AugurError {}

#[derive(Debug)]
pub struct PendingMessage(pub String);

impl Display for PendingMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
