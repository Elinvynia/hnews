#![doc(hidden)]

use crate::prelude::*;
use miniserde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Item {
    pub(crate) id: u64,
    pub(crate) deleted: Option<bool>,
    #[serde(rename = "type")]
    pub(crate) kind: Option<String>,
    pub(crate) by: Option<String>,
    pub(crate) time: Option<u64>,
    pub(crate) text: Option<String>,
    pub(crate) dead: Option<bool>,
    pub(crate) parent: Option<u64>,
    pub(crate) poll: Option<u64>,
    pub(crate) kids: Option<Vec<u64>>,
    pub(crate) url: Option<String>,
    pub(crate) score: Option<i64>,
    pub(crate) title: Option<String>,
    pub(crate) parts: Option<Vec<u64>>,
    pub(crate) descendants: Option<u64>,
}

impl TryFrom<Item> for Ask {
    type Error = HError;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let kind = convert!(item.kind);
        if kind != "story" {
            return Err(HError::ConversionFailed);
        }
        Ok(Ask {
            by: convert!(item.by),
            comments: convert!(item.descendants),
            id: item.id,
            kids: convert_default!(item.kids),
            score: convert!(item.score),
            text: convert!(item.text),
            time: convert!(item.time),
            title: convert!(item.title),
        })
    }
}

impl TryFrom<Item> for Comment {
    type Error = HError;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let kind = convert!(item.kind);
        if kind != "comment" {
            return Err(HError::ConversionFailed);
        }
        Ok(Comment {
            by: convert!(item.by),
            id: item.id,
            kids: convert_default!(item.kids),
            parent: convert!(item.parent),
            text: convert!(item.text),
            time: convert!(item.time),
        })
    }
}

impl TryFrom<Item> for Job {
    type Error = HError;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let kind = convert!(item.kind);
        if kind != "job" {
            return Err(HError::ConversionFailed);
        }
        Ok(Job {
            by: convert!(item.by),
            id: item.id,
            score: convert!(item.score),
            time: convert!(item.time),
            title: convert!(item.title),
            url: convert!(item.url),
        })
    }
}

impl TryFrom<Item> for Poll {
    type Error = HError;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let kind = convert!(item.kind);
        if kind != "poll" {
            return Err(HError::ConversionFailed);
        }
        Ok(Poll {
            by: convert!(item.by),
            comments: convert!(item.descendants),
            id: item.id,
            kids: convert_default!(item.kids),
            parts: convert!(item.parts),
            score: convert!(item.score),
            text: convert!(item.text),
            time: convert!(item.time),
            title: convert!(item.title),
        })
    }
}

impl TryFrom<Item> for PollOption {
    type Error = HError;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let kind = convert!(item.kind);
        if kind != "pollopt" {
            return Err(HError::ConversionFailed);
        }
        Ok(PollOption {
            by: convert!(item.by),
            id: item.id,
            poll: convert!(item.poll),
            score: convert!(item.score),
            text: convert!(item.text),
            time: convert!(item.time),
        })
    }
}

impl TryFrom<Item> for Story {
    type Error = HError;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let kind = convert!(item.kind);
        if kind != "story" {
            return Err(HError::ConversionFailed);
        }
        Ok(Story {
            by: convert!(item.by),
            comments: convert!(item.descendants),
            id: item.id,
            kids: convert_default!(item.kids),
            score: convert!(item.score),
            time: convert!(item.time),
            title: convert!(item.title),
            url: convert!(item.url),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Update {
    pub(crate) items: Vec<u64>,
    pub(crate) profiles: Vec<String>,
}
