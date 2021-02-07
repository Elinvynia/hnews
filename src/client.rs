//! Static http client for requests.

use crate::prelude::*;
use arc_swap::ArcSwap;
use miniserde::json::from_str;
use once_cell::sync::Lazy;
use std::sync::Arc;
use ureq::Agent;

static STATIC_INSTANCE: Lazy<ArcSwap<Client>> = Lazy::new(|| ArcSwap::from_pointee(Client::new()));

/// The client handling all requests.
#[derive(Debug)]
pub struct Client {
    client: Agent,
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}

impl Client {
    /// Creates a new client instance to be passed around.
    pub fn new() -> Self {
        Self {
            client: Agent::new(),
        }
    }

    /// Returns a static instance for ease of use.
    pub fn instance() -> Arc<Self> {
        STATIC_INSTANCE.load().clone()
    }

    fn get_item(&self, id: u64) -> Result<Item, HError> {
        let response = self.client.get(&Endpoint::item(id)).call()?.into_string()?;

        let item: Item = from_str(&response)?;
        Ok(item)
    }

    /// Returns an [Ask] with the id.
    pub fn get_ask(&self, id: u64) -> Result<Ask, HError> {
        let item = self.get_item(id)?;
        let ask: Ask = item.try_into()?;
        Ok(ask)
    }

    /// Returns a [Comment] with the id.
    pub fn get_comment(&self, id: u64) -> Result<Comment, HError> {
        let item = self.get_item(id)?;
        let comment: Comment = item.try_into()?;
        Ok(comment)
    }

    /// Returns a [Job] with the id.
    pub fn get_job(&self, id: u64) -> Result<Job, HError> {
        let item = self.get_item(id)?;
        let job: Job = item.try_into()?;
        Ok(job)
    }

    /// Returns a [Poll] with the id.
    pub fn get_poll(&self, id: u64) -> Result<Poll, HError> {
        let item = self.get_item(id)?;
        let poll: Poll = item.try_into()?;
        Ok(poll)
    }

    /// Returns a [Poll Option](PollOption) with the id.
    pub fn get_poll_option(&self, id: u64) -> Result<PollOption, HError> {
        let item = self.get_item(id)?;
        let polloption: PollOption = item.try_into()?;
        Ok(polloption)
    }

    /// Returns a [Story] with the id.
    pub fn get_story(&self, id: u64) -> Result<Story, HError> {
        let item = self.get_item(id)?;
        let story: Story = item.try_into()?;
        Ok(story)
    }

    /// Returns a [User] with the specified username.
    pub fn get_user(&self, name: &str) -> Result<User, HError> {
        let response = self
            .client
            .get(&Endpoint::user(name))
            .call()?
            .into_string()?;

        let user: User = from_str(&response)?;
        Ok(user)
    }

    /// Gets up to 200 of the latest [Asks](Ask).
    pub fn get_latest_asks(&self) -> Result<Vec<Ask>, HError> {
        let response = self
            .client
            .get(&Endpoint::askstories())
            .call()?
            .into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        ids.into_iter().map(|id| self.get_ask(id)).collect()
    }

    /// Gets up to 200 of the latest [Stories](Story).
    pub fn get_latest_stories(&self) -> Result<Vec<Story>, HError> {
        let response = self
            .client
            .get(&Endpoint::showstories())
            .call()?
            .into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        ids.into_iter().map(|id| self.get_story(id)).collect()
    }

    /// Gets up to 200 of the latest [Jobs](Job).
    pub fn get_latest_jobs(&self) -> Result<Vec<Job>, HError> {
        let response = self
            .client
            .get(&Endpoint::askstories())
            .call()?
            .into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        ids.into_iter().map(|id| self.get_job(id)).collect()
    }

    /// Returns the latest item id, can be any of: [Story], [Ask], [Job].
    pub fn get_latest(&self) -> Result<u64, HError> {
        let response = self
            .client
            .get(&Endpoint::maxitem())
            .call()?
            .into_string()?;
        let id: u64 = from_str(&response)?;
        Ok(id)
    }

    /// Returns up to 500 of the latest top item ids, can be any of: [Story], [Ask], [Job].
    pub fn get_top(&self) -> Result<Vec<u64>, HError> {
        let response = self
            .client
            .get(&Endpoint::topstories())
            .call()?
            .into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        Ok(ids)
    }

    /// Returns up to 500 of the latest item ids, can be any of: [Story], [Ask], [Job].
    pub fn get_new(&self) -> Result<Vec<u64>, HError> {
        let response = self
            .client
            .get(&Endpoint::newstories())
            .call()?
            .into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        Ok(ids)
    }

    /// Returns up to 500 of the latest best item ids, can be any of: [Story], [Ask], [Job].
    pub fn get_best(&self) -> Result<Vec<u64>, HError> {
        let response = self
            .client
            .get(&Endpoint::beststories())
            .call()?
            .into_string()?;
        let ids: Vec<u64> = from_str(&response)?;
        Ok(ids)
    }

    /// Returns [User] profiles that have changed.
    pub fn get_user_updates(&self) -> Result<Vec<String>, HError> {
        let response = self
            .client
            .get(&Endpoint::updates())
            .call()?
            .into_string()?;
        let update: Update = from_str(&response)?;
        Ok(update.profiles)
    }

    /// Returns item ids that have changed, can be any of: [Story], [Ask], [Job].
    pub fn get_updates(&self) -> Result<Vec<u64>, HError> {
        let response = self
            .client
            .get(&Endpoint::updates())
            .call()?
            .into_string()?;
        let update: Update = from_str(&response)?;
        Ok(update.items)
    }
}
