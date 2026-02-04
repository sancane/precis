//! PRECIS Framework: Preparation, Enforcement, and Comparison of
//! Internationalized Strings in Application Protocols as described in
//! [`rfc8264`](https://datatracker.ietf.org/doc/html/rfc8264)
//!
//! This crate implements the next PRECIS profiles:
//! * [`rfc8265`](https://datatracker.ietf.org/doc/html/rfc8265).
//!   Preparation, Enforcement, and Comparison of Internationalized Strings
//!   Representing `Usernames` and `Passwords`.
//! * [`rfc8266`](https://datatracker.ietf.org/doc/html/rfc8266).
//!   Preparation, Enforcement, and Comparison of Internationalized Strings
//!   Representing Nicknames
//!
//! ```rust
//! # use precis_core::profile::PrecisFastInvocation;
//! # use precis_profiles::Nickname;
//! # use std::borrow::Cow;
//! assert_eq!(Nickname::prepare("Guybrush Threepwood"),
//!   Ok(Cow::from("Guybrush Threepwood")));
//! assert_eq!(Nickname::enforce("   Guybrush     Threepwood  "),
//!   Ok(Cow::from("Guybrush Threepwood")));
//! assert_eq!(Nickname::compare("Guybrush   Threepwood  ",
//!   "guybrush threepwood"), Ok(true));
//! ```
//!
//! # Unicode Version
//!
//! This crate uses a more recent version of Unicode than the RFC specification to provide
//! support for a broader set of characters and modern scripts used by applications. This
//! allows the profiles to accept contemporary Unicode characters that may not be present
//! in the RFC-specified Unicode version.
//!
//! ## Compatibility with precis-core
//!
//! While `precis-core` uses the RFC-specified Unicode version (as maintained by the `IANA`
//! PRECIS registry), this crate deliberately uses a more recent Unicode version. This design
//! decision provides:
//!
//! - **Broader Character Support**: Modern Unicode characters and scripts
//! - **Better User Experience**: Reduces validation failures for legitimate modern text
//! - **Forward Compatibility**: Ready for future `IANA` registry updates
//!
//! The core library maintains RFC compliance by using the `IANA`-maintained Unicode version,
//! while this profiles crate enhances usability by supporting newer characters. This approach
//! avoids unexpected validation failures when processing code points whose derived properties
//! are not yet defined in the official `IANA` PRECIS registry.

#![deny(missing_docs)]

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
pub use precis_core;
