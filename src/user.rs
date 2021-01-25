//! User struct and associated methods.

use crate::prelude::*;
use miniserde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
/// Struct representing a User.
pub struct User {
    /// Username of the user, unique and case-sensitive.
    pub id: String,
    /// User creation time as a Unix timestamp.
    pub created: u64,
    /// The total karma of the user.
    pub karma: u64,
    /// Optional description of the user.
    pub about: String,
    pub(crate) submitted: Vec<u64>,
}

impl User {
    /// Returns a list of [Comments](Comment) this user has posted.
    pub fn comments(&self) -> Result<Vec<Comment>, HError> {
        let mut comments = vec![];
        for x in self.submitted.iter() {
            match Client::get_comment(*x) {
                Ok(s) => comments.push(s),
                Err(_) => continue,
            }
        }
        Ok(comments)
    }

    /// Returns a list of [Polls](Poll) this user has posted.
    pub fn polls(&self) -> Result<Vec<Poll>, HError> {
        let mut polls = vec![];
        for x in self.submitted.iter() {
            match Client::get_poll(*x) {
                Ok(s) => polls.push(s),
                Err(_) => continue,
            }
        }
        Ok(polls)
    }

    /// Returns a list of [Stories](Story) this user has posted.
    pub fn stories(&self) -> Result<Vec<Story>, HError> {
        let mut stories = vec![];
        for x in self.submitted.iter() {
            match Client::get_story(*x) {
                Ok(s) => stories.push(s),
                Err(_) => continue,
            }
        }
        Ok(stories)
    }
}
