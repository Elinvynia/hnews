//! Ask struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// A question posted by a [User].
pub struct Ask {
    /// Username of the [User] that posted this submission.
    pub by: String,
    /// The total amount of comments.
    pub comments: u64,
    /// Unique id of the submission.
    pub id: u64,
    pub(crate) kids: Vec<u64>,
    /// The score of the submission.
    pub score: u64,
    /// The text of the submission.
    pub text: String,
    /// Unix timestamp of the submission.
    pub time: u64,
    /// The title of the submission.
    pub title: String,
}

impl Ask {
    /// Returns the [User] that made this submission.
    pub fn by(&self) -> Result<User, HError> {
        Client::get_user(&self.by)
    }

    /// Retrieves the top level [comments](Comment) of the submission.
    pub fn comments(&self) -> Result<Vec<Comment>, HError> {
        self.kids
            .iter()
            .map(|kid| Client::get_comment(*kid))
            .collect()
    }
}
