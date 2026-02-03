[![Docs](https://docs.rs/precis-profiles/badge.svg)](https://docs.rs/precis-profiles)
[![Crates.io](https://img.shields.io/crates/v/precis-profiles)](https://crates.io/crates/precis-profiles)

# precis-profiles

PRECIS Framework: Preparation, Enforcement, and Comparison of
Internationalized Strings in Application Protocols as described in
[rfc8264](https://datatracker.ietf.org/doc/html/rfc8264)

This crate implements the next PRECIS profiles:
 * [rfc8265](https://datatracker.ietf.org/doc/html/rfc8265).
   Preparation, Enforcement, and Comparison of Internationalized Strings
   Representing Usernames and Passwords.
 * [rfc8266](https://datatracker.ietf.org/doc/html/rfc8266).
   Preparation, Enforcement, and Comparison of Internationalized Strings
   Representing Nicknames

## Unicode Version

This crate uses a more recent version of Unicode than the RFC specification to provide support for a broader set of characters and modern scripts. This allows the profiles to accept contemporary Unicode characters that may not be present in the RFC-specified Unicode version.

### Compatibility with precis-core

While `precis-core` uses the RFC-specified Unicode version (as maintained by the IANA PRECIS registry), this crate deliberately uses a more recent Unicode version. This design provides:

- **Broader Character Support**: Modern Unicode characters and scripts
- **Better User Experience**: Reduces validation failures for legitimate modern text
- **Forward Compatibility**: Ready for future IANA registry updates

The core library maintains RFC compliance with the IANA-maintained Unicode version, while this profiles crate enhances usability by supporting newer characters. This approach prevents unexpected validation failures when processing code points whose derived properties are not yet defined in the official IANA PRECIS registry.

## Examples
```rust
assert_eq!(Nickname::prepare("Guybrush Threepwood"),
  Ok(Cow::from("Guybrush Threepwood")));
assert_eq!(Nickname::enforce("   Guybrush     Threepwood  "),
  Ok(Cow::from("Guybrush Threepwood")));
assert_eq!(Nickname::compare("Guybrush   Threepwood  ",
  "guybrush threepwood"), Ok(true));
```

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
