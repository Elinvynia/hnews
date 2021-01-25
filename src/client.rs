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

    /// Returns a [Job] with the id.
    pub fn get_job(id: u64) -> Result<Job, HError> {
        let item = Client::get_item(id)?;
        let job: Job = item.try_into()?;
        Ok(job)
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

    /// Gets up to 200 of the latest [Asks](Ask).
    pub fn get_latest_asks() -> Result<Vec<Ask>, HError> {
        let response = ureq::get(&Endpoint::askstories()).call()?.into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        ids.into_iter().map(Client::get_ask).collect()
    }

    /// Gets up to 200 of the latest [Stories](Story).
    pub fn get_latest_stories() -> Result<Vec<Story>, HError> {
        let response = ureq::get(&Endpoint::showstories()).call()?.into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        ids.into_iter().map(Client::get_story).collect()
    }

    /// Gets up to 200 of the latest [Jobs](Job).
    pub fn get_latest_jobs() -> Result<Vec<Job>, HError> {
        let response = ureq::get(&Endpoint::askstories()).call()?.into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        ids.into_iter().map(Client::get_job).collect()
    }

    /// Returns the latest item id, can be any of: [Story], [Ask], [Job].
    pub fn get_latest() -> Result<u64, HError> {
        let response = ureq::get(&Endpoint::maxitem()).call()?.into_string()?;
        let id: u64 = from_str(&response)?;
        Ok(id)
    }

    /// Returns up to 500 of the latest top item ids, can be any of: [Story], [Ask], [Job].
    pub fn get_top() -> Result<Vec<u64>, HError> {
        let response = ureq::get(&Endpoint::topstories()).call()?.into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        Ok(ids)
    }

    /// Returns up to 500 of the latest item ids, can be any of: [Story], [Ask], [Job].
    pub fn get_new() -> Result<Vec<u64>, HError> {
        let response = ureq::get(&Endpoint::newstories()).call()?.into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        Ok(ids)
    }

    /// Returns up to 500 of the latest best item ids, can be any of: [Story], [Ask], [Job].
    pub fn get_best() -> Result<Vec<u64>, HError> {
        let response = ureq::get(&Endpoint::beststories()).call()?.into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        Ok(ids)
    }

    /// Returns [User] profiles that have changed.
    pub fn get_user_updates() -> Result<Vec<String>, HError> {
        let response = ureq::get(&Endpoint::updates()).call()?.into_string()?;
        let update: Update = from_str(&response)?;
        Ok(update.profiles)
    }

    /// Returns item ids that have changed, can be any of: [Story], [Ask], [Job].
    pub fn get_updates() -> Result<Vec<u64>, HError> {
        let response = ureq::get(&Endpoint::updates()).call()?.into_string()?;
        let update: Update = from_str(&response)?;
        Ok(update.items)
    }
}
