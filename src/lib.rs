#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! hnews is a synchronous Rust library for the Hacker News API.
//!
//! It is extremely lightweight (thanks to ureq and miniserde) but still provides enough basics to get you started processing data.
//!
//!
//! [ci]: https://github.com/Elinvynia/hnews/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/hnews/Rust/master?style=flat-square
//! [docs]: https://docs.rs/hnews
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/hnews
//! [crate-version]: https://img.shields.io/crates/v/hnews.svg?style=flat-square

pub mod ask;
pub mod client;
pub mod comment;
pub(crate) mod endpoint;
#[macro_use]
pub mod error;
pub mod item;
pub mod job;
pub mod poll;
pub mod prelude;
pub mod story;
pub mod user;
