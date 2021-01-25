//! Static http client for requests.

use crate::prelude::*;
use miniserde::json::from_str;

/// The client handling all requests.
#[derive(Debug)]
pub struct Client;

impl Client {
    fn get_item(id: u64) -> Result<Item, HError> {
        let response = ureq::get(&Endpoint::item(id)).call()?.into_string()?;

        let item: Item = from_str(&response)?;
        Ok(item)
    }

    /// Returns an [Ask] with the id.
    pub fn get_ask(id: u64) -> Result<Ask, HError> {
        let item = Client::get_item(id)?;
        let ask: Ask = item.try_into()?;
        Ok(ask)
    }

    /// Returns a [Comment] with the id.
    pub fn get_comment(id: u64) -> Result<Comment, HError> {
        let item = Client::get_item(id)?;
        let comment: Comment = item.try_into()?;
        Ok(comment)
    }

    /// Returns a [Poll] with the id.
    pub fn get_poll(id: u64) -> Result<Poll, HError> {
        let item = Client::get_item(id)?;
        let poll: Poll = item.try_into()?;
        Ok(poll)
    }

    /// Returns a [Poll Option](PollOption) with the id.
    pub fn get_poll_option(id: u64) -> Result<PollOption, HError> {
        let item = Client::get_item(id)?;
        let polloption: PollOption = item.try_into()?;
        Ok(polloption)
    }

    /// Returns a [Story] with the id.
    pub fn get_story(id: u64) -> Result<Story, HError> {
        let item = Client::get_item(id)?;
        let story: Story = item.try_into()?;
        Ok(story)
    }

    /// Returns a [User] with the specified username.
    pub fn get_user(name: &str) -> Result<User, HError> {
        let response = ureq::get(&Endpoint::user(name)).call()?.into_string()?;

        let user: User = from_str(&response)?;
        Ok(user)
    }
}
