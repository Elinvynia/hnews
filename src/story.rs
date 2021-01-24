//! Story struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// Struct representing a Story submission.
pub struct Story {
    /// The username of the [User] that created this story.
    pub by: String,
    /// The total amount of comments.
    pub comments: u64,
    /// The unique id of this submission.
    pub id: u64,
    pub(crate) kids: Vec<u64>,
    /// The score of this submission.
    pub score: u64,
    /// The UNIX timestamp of this submission.
    pub time: u64,
    /// The title of this submission.
    pub title: String,
    /// The URL where this submission leads.
    pub url: String,
}

impl Story {
    /// Returns the [User] that created this Story.
    pub fn by(&self) -> Result<User, HError> {
        Client::get_user(&self.by)
    }

    /// Returns the list of top-level [Comments](Comment) of this Story.
    pub fn comments(&self) -> Result<Vec<Comment>, HError> {
        self.kids.iter().map(|kid| Client::get_comment(*kid)).collect()
    }
}
