//! Job struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// Job posting submission.
pub struct Job {
    /// Username of the user that posted this.
    pub by: String,
    /// The unique id of this job listing.
    pub id: u64,
    /// The score of this submission.
    pub score: u64,
    /// The text of this job listing.
    pub text: String,
    /// UNIX timestamp when it was posted.
    pub time: u64,
    /// The title of the submission.
    pub title: String,
    /// The URL where the submission leads.
    pub url: String,
}

impl Job {
    /// Returns the User that posted this submission.
    pub fn by(&self) -> Result<User, HError> {
        Client::get_user(&self.by)
    }
}
