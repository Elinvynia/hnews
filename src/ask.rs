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
    /// The unique id of this submission.
    pub id: u64,
    pub(crate) kids: Vec<u64>,
    /// The score of this submission.
    pub score: i64,
    /// The text of this submission.
    pub text: String,
    /// When this submission was made, as a Unix timestamp.
    pub time: u64,
    /// The title of this submission.
    pub title: String,
}

impl Ask {
    /// Returns the [User] that made this submission.
    pub fn by(&self, client: Client) -> Result<User, HError> {
        client.get_user(&self.by)
    }

    /// Retrieves the top level [comments](Comment) of the submission.
    pub fn comments(&self, client: Client) -> Result<Vec<Comment>, HError> {
        self.kids
            .iter()
            .map(|kid| client.get_comment(*kid))
            .collect()
    }
}
