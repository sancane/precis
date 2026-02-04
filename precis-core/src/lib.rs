//! PRECIS Framework: Preparation, Enforcement, and Comparison of
//! Internationalized Strings in Application Protocols as described in
//! [`rfc8264`](https://datatracker.ietf.org/doc/html/rfc8264)
//!
//! This crate implements the PRECIS base string classes and tables
//! that profiles can use for their implementation. The crate `precis-profiles`
//! provides a list of implemented profiles that applications can use.
//!
//! # Unicode Version
//!
//! This crate uses the Unicode version specified in the PRECIS RFC and maintained
//! by the [`IANA` PRECIS registry](https://www.iana.org/assignments/precis-tables/precis-tables.xhtml).
//! The core library strictly adheres to the RFC specification and will continue to use
//! the `IANA`-maintained Unicode version until the registry is updated.
//!
//! This approach ensures:
//! - **RFC Compliance**: Full conformance with the PRECIS specification
//! - **Predictable Behavior**: Consistent validation results across implementations
//! - **Interoperability**: Compatibility with other RFC-compliant PRECIS implementations
//!
//! ## Note on Unicode Version Differences
//!
//! The `precis-profiles` crate uses a more recent version of Unicode to allow a broader
//! set of characters. While this may seem inconsistent, it is a deliberate design decision:
//!
//! - **precis-core**: Uses the RFC-specified Unicode version (`IANA` registry)
//! - **precis-profiles**: Uses a more recent Unicode version (supports modern characters and scripts)
//!
//! This design prevents unexpected validation failures when processing code points whose
//! derived properties are not yet defined in the `IANA` registry, while maintaining strict
//! RFC compliance at the core level.

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
