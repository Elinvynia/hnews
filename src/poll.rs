//! Poll struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// A struct representing a Poll.
pub struct Poll {
    /// The username of the User that created this submission.
    pub by: String,
    /// The total amount of comments.
    pub comments: u64,
    /// The unique id of this submission.
    pub id: u64,
    pub(crate) kids: Vec<u64>,
    pub(crate) parts: Vec<u64>,
    /// The score of this submission.
    pub score: i64,
    /// The text of this submission.
    pub text: String,
    /// When this submission was made, as a Unix timestamp.
    pub time: u64,
    /// The title of this submission.
    pub title: String,
}

impl Poll {
    /// Returns the [User] that created this poll.
    pub fn by(&self, client: Client) -> Result<User, HError> {
        client.get_user(&self.by)
    }

    /// Returns the top-level [Comments](Comment) of the poll.
    pub fn comments(&self, client: Client) -> Result<Vec<Comment>, HError> {
        self.kids
            .iter()
            .map(|kid| client.get_comment(*kid))
            .collect()
    }

    /// Returns the [Poll Options](PollOption) of this poll.
    pub fn options(&self, client: Client) -> Result<Vec<PollOption>, HError> {
        self.parts
            .iter()
            .map(|kid| client.get_poll_option(*kid))
            .collect()
    }
}

#[non_exhaustive]
#[derive(Debug)]
/// A poll option belonging to a poll.
pub struct PollOption {
    /// Username of the [User] that created this poll option.
    pub by: String,
    /// Unique id of this poll option.
    pub id: u64,
    /// The [Poll] it belongs to.
    pub poll: u64,
    /// The score of this poll option.
    pub score: i64,
    /// The text of this poll option.
    pub text: String,
    /// When this option was made, as a Unix timestamp.
    pub time: u64,
}

impl PollOption {
    /// Return the [User] that created this poll option.
    pub fn by(&self, client: Client) -> Result<User, HError> {
        client.get_user(&self.by)
    }

    /// Return the [Poll] this option belongs to.
    pub fn poll(&self, client: Client) -> Result<Poll, HError> {
        client.get_poll(self.poll)
    }
}
