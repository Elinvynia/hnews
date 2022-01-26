//! Error struct representing possible failures.

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
/// The error enum representing all possible errors that can originate from this crate.
pub enum HError {
    /// Error originating from the `ureq` crate.
    UReq(ureq::Error),
    /// Error originating from the `miniserde` crate.
    Miniserde(miniserde::Error),
    /// Error originating from `std::io::Error`.
    Io(std::io::Error),
    /// Conversion between returned data and our representation failed.
    ConversionFailed,
}

impl Error for HError {}

impl Display for HError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        use HError::*;
        match self {
            UReq(e) => write!(fmt, "UReq Error: {}", e),
            Miniserde(e) => write!(fmt, "Miniserde Error: {}", e),
            Io(e) => write!(fmt, "Io Error: {}", e),
            ConversionFailed => write!(
                fmt,
                "Conversion between returned data and our representation failed."
            ),
        }
    }
}

impl From<ureq::Error> for HError {
    fn from(err: ureq::Error) -> Self {
        HError::UReq(err)
    }
}

impl From<miniserde::Error> for HError {
    fn from(err: miniserde::Error) -> Self {
        HError::Miniserde(err)
    }
}

impl From<std::io::Error> for HError {
    fn from(err: std::io::Error) -> Self {
        HError::Io(err)
    }
}

macro_rules! convert {
    ($e:expr) => {
        $e.ok_or(HError::ConversionFailed)?
    };
}

macro_rules! convert_default {
    ($e:expr) => {
        $e.unwrap_or_default()
    };
}
