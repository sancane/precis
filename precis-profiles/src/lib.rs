//! PRECIS Framework: Preparation, Enforcement, and Comparison of
//! Internationalized Strings in Application Protocols as described in
//! [`rfc8264`](https://datatracker.ietf.org/doc/html/rfc8264)
//!
//! This crate implements the next PRECIS profiles:
//! * [`rfc8265`](https://datatracker.ietf.org/doc/html/rfc8265).
//!   Preparation, Enforcement, and Comparison of Internationalized Strings
//!   Representing Usernames and Passwords.
//! * [`rfc8266`](https://datatracker.ietf.org/doc/html/rfc8266).
//!   Preparation, Enforcement, and Comparison of Internationalized Strings
//!   Representing Nicknames
//!
//! ```rust
//! extern crate precis_profiles;
//!
//! use precis_core::profile::PrecisFastInvocation;
//! use precis_profiles::Nickname;
//! use std::borrow::Cow;
//!
//! fn main() {
//!    assert_eq!(Nickname::prepare("Guybrush Threepwood"),
//!      Ok(Cow::from("Guybrush Threepwood")));
//!    assert_eq!(Nickname::enforce("   Guybrush     Threepwood  "),
//!      Ok(Cow::from("Guybrush Threepwood")));
//!    assert_eq!(Nickname::compare("Guybrush   Threepwood  ",
//!      "guybrush threepwood"), Ok(true));
//! }
//! ```
//!
//! # `crates.io`
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! precis-profiles = "0.1.3"
//! ```

include!(concat!(env!("OUT_DIR"), "/unicode_version.rs"));

mod bidi;
mod common;
mod nicknames;
mod passwords;
mod usernames;

pub use crate::nicknames::Nickname;
pub use crate::passwords::OpaqueString;
pub use crate::usernames::UsernameCaseMapped;
pub use crate::usernames::UsernameCasePreserved;
