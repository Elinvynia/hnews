//! Poll struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// A struct representing a poll.
pub struct Poll {
    /// The username of the User that created this poll.
    pub by: String,
    /// The total amount of comments.
    pub comments: u64,
    /// The unique id of the submission.
    pub id: u64,
    pub(crate) kids: Vec<u64>,
    pub(crate) parts: Vec<u64>,
    /// The score of the submission.
    pub score: u64,
    /// The text of the submission.
    pub text: String,
    /// The UNIX timestamp when the submission was made.
    pub time: u64,
    /// The title of the submission.
    pub title: String,
}

impl Poll {
    /// Returns the User that created this poll.
    pub fn by(&self) -> Result<User, HError> {
        Client::get_user(&self.by)
    }

    /// Returns the top-level comments of the poll.
    pub fn comments(&self) -> Result<Vec<Comment>, HError> {
        self.kids
            .iter()
            .map(|kid| Client::get_comment(*kid))
            .collect()
    }

    /// Returns the options of this poll.
    pub fn options(&self) -> Result<Vec<PollOption>, HError> {
        self.parts
            .iter()
            .map(|kid| Client::get_poll_option(*kid))
            .collect()
    }
}

#[non_exhaustive]
#[derive(Debug)]
/// A poll option belonging to a poll.
pub struct PollOption {
    /// Username of the user that created the poll option.
    pub by: String,
    /// Unique ID of the poll option.
    pub id: u64,
    /// The poll it belongs to.
    pub poll: u64,
    /// The score of the poll option.
    pub score: u64,
    /// The text of the poll option.
    pub text: String,
    /// When it was created, unix timestamp.
    pub time: u64,
}

impl PollOption {
    /// Return the User that created the poll option.
    pub fn by(&self) -> Result<User, HError> {
        Client::get_user(&self.by)
    }

    /// Return the Poll this option belongs to.
    pub fn poll(&self) -> Result<Poll, HError> {
        Client::get_poll(self.poll)
    }
}
