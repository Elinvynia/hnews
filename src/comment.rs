//! Comment struct and associated methods.

use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug)]
/// Struct representing a comment made on a submission.
pub struct Comment {
    /// The username of the [User] that made this comment.
    pub by: String,
    /// The unique id of this comment.
    pub id: u64,
    pub(crate) kids: Vec<u64>,
    /// The id of the parent this comment belongs to, either another [Comment] or one of: [Story], [Ask], [Poll]
    pub parent: u64,
    /// The text of the comment.
    pub text: String,
    /// When this comment was made, as a Unix timestamp.
    pub time: u64,
}

impl Comment {
    /// Returns the [User] that made this comment.
    pub fn by(&self, client: &Client) -> Result<User, HError> {
        client.get_user(&self.by)
    }

    /// Returns the top-level replies to this comment.
    pub fn replies(&self, client: &Client) -> Result<Vec<Comment>, HError> {
        self.kids
            .iter()
            .map(|kid| client.get_comment(*kid))
            .collect()
    }
}
