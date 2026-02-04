[![Docs](https://docs.rs/precis-core/badge.svg)](https://docs.rs/precis-core)
[![Crates.io](https://img.shields.io/crates/v/precis-core)](https://crates.io/crates/precis-core)

# precis-core

PRECIS Framework: Preparation, Enforcement, and Comparison of
Internationalized Strings in Application Protocols as described in
[`rfc8264`](https://datatracker.ietf.org/doc/html/rfc8264)

The core library of the PRECIS Framework. The base string classes `IdentifierClass`
and `FreeFormClass` are implemented here as defined in
[rfc8264](https://datatracker.ietf.org/doc/html/rfc8264).
This crate provides the `APIs` required for profiles to be implemented.

The crate [precis-profiles](https://docs.rs/precis-profiles)
provides a list of implemented profiles that applications can use.

## Unicode Version

This crate uses the Unicode version specified in the PRECIS RFC and maintained by the [IANA PRECIS registry](https://www.iana.org/assignments/precis-tables/precis-tables.xhtml). The core library strictly adheres to the RFC specification and will continue to use the IANA-maintained Unicode version until the registry is updated.

### Why the RFC-specified Unicode version?

- **RFC Compliance**: Full conformance with the PRECIS specification
- **Predictable Behavior**: Consistent validation results across implementations
- **Interoperability**: Compatibility with other RFC-compliant PRECIS implementations

### Note on Unicode Version Differences

The `precis-profiles` crate uses a more recent Unicode version to support a broader set of modern characters. This is a deliberate design decision to enhance usability while maintaining RFC compliance at the core level. See the [precis-profiles documentation](https://docs.rs/precis-profiles) for more details.

# Contributing

Patches and feedback are welcome.

# Donations

If you find this project helpful, you may consider making a donation:

<img src="https://www.bitcoinqrcodemaker.com/api/?style=bitcoin&amp;address=bc1qx258lwvgzlg5zt2xsns2nr75dhvxuzk3wkqmnh" height="150" width="150" alt="Bitcoin QR Code">
<img src="https://www.bitcoinqrcodemaker.com/api/?style=ethereum&amp;address=0xefa6404e5A50774117fd6204cbD33cf4454c67Fb" height="150" width="150" alt="Ethereum QR Code">

# License

This project is licensed under either of
* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](https://opensource.org/licenses/MIT)

[![say thanks](https://img.shields.io/badge/Say%20Thanks-👍-1EAEDB.svg)](https://github.com/sancane/precis/stargazers)
