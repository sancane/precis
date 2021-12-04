//! PRECIS Framework: Preparation, Enforcement, and Comparison of
//! Internationalized Strings in Application Protocols as described in
//! [`rfc8264`](https://datatracker.ietf.org/doc/html/rfc8264)
//!
//! This crate implements the PRECIS base string classes and tables
//! that profiles can use for their implementation. The crate `precis-profiles`
//! provides a list of implemented profiles that applications can use.
//!
//! # `crates.io`
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! precis-core = "0.1.4"
//! ```

#![deny(missing_docs)]

include!(concat!(env!("OUT_DIR"), "/public.rs"));

mod common;

pub mod context;

pub use crate::error::CodepointInfo;
pub use crate::error::Error;
pub use crate::error::UnexpectedError;
pub use crate::stringclasses::FreeformClass;
pub use crate::stringclasses::IdentifierClass;
pub use crate::stringclasses::StringClass;

mod error;
pub mod profile;
pub mod stringclasses;
