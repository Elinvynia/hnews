//! Job struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// Struct representing a Job posting.
pub struct Job {
    /// The username of the [User] that created this submission.
    pub by: String,
    /// The unique id of this submission.
    pub id: u64,
    /// The score of this submission.
    pub score: i64,
    /// When this submission was made, as a Unix timestamp.
    pub time: u64,
    /// The title of this submission.
    pub title: String,
    /// The URL where this submission leads.
    pub url: String,
}

impl Job {
    /// Returns the [User] that posted this submission.
    pub fn by(&self, client: Client) -> Result<User, HError> {
        client.get_user(&self.by)
    }
}
