![Build](https://github.com/sancane/precis/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/sancane/precis/branch/main/graph/badge.svg?token=EZM6KCSLZC)](https://codecov.io/gh/sancane/precis)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/sancane/precis)](https://rust-reportcard.xuri.me/report/github.com/sancane/precis)

# Rust PRECIS Framework library

PRECIS Framework: Preparation, Enforcement, and Comparison of
Internationalized Strings in Application Protocols as described in
[rfc8264](https://datatracker.ietf.org/doc/html/rfc8264)

This workspace implements the next crates:

## [precis-tools](precis-tools/README.md)

[![Docs](https://docs.rs/precis-tools/badge.svg)](https://docs.rs/precis-tools)
[![Crates.io](https://img.shields.io/crates/v/precis-tools)](https://crates.io/crates/precis-tools)

This crate contains all the tools and parsers to generate PRECIS
tables from the Unicode Character Database [`UCD`](https://unicode.org).
This crate is only used to generate code required by
[precis-core](https://docs.rs/precis-core) and
[precis-profiles](https://docs.rs/precis-profiles) crates.
It contains the main dependencies in order to download `UCD` files,
parse them and generate Rust code.

## [precis-core](precis-core/README.md)

[![Docs](https://docs.rs/precis-core/badge.svg)](https://docs.rs/precis-core)
[![Crates.io](https://img.shields.io/crates/v/precis-core)](https://crates.io/crates/precis-core)

The core library of the PRECIS Framework. The base string classes `IdentifierClass`
and `FreeFormClass` are implemented here as defined in
[rfc8264](https://datatracker.ietf.org/doc/html/rfc8264).
This crate provides the `APIs` required for profiles to be implemented.
You mostly won't require this crate unless you are implementing a new profile.

## [precis-profiles](precis-profiles/README.md)

[![Docs](https://docs.rs/precis-profiles/badge.svg)](https://docs.rs/precis-profiles)
[![Crates.io](https://img.shields.io/crates/v/precis-profiles)](https://crates.io/crates/precis-profiles)

This crate implements the next PRECIS profiles:
 * [rfc8265](https://datatracker.ietf.org/doc/html/rfc8265).
   Preparation, Enforcement, and Comparison of Internationalized Strings
   Representing Usernames and Passwords.
 * [rfc8266](https://datatracker.ietf.org/doc/html/rfc8266).
   Preparation, Enforcement, and Comparison of Internationalized Strings
   Representing Nicknames

PRECIS profiles provides an API that allows application to prepare, enforce and compare
internationalized strings.

### Example

```rust
// create OpaqueString profile
let profile = OpaqueString::new();

// prepare string
assert_eq!(profile.prepare("I'm Guybrush Threepwood, Mighty Pirate ☠"),
    Ok(Cow::from("I'm Guybrush Threepwood, Mighty Pirate ☠")));

// enforce string
assert_eq!(profile.enforce("Look behind you, a three-headed monkey!🐒"),
    Ok(Cow::from("Look behind you, a three-headed monkey!🐒")));

// compare strings
assert_eq!(profile.compare("That’s the second biggest 🐵 I’ve ever seen!",
    "That’s the second biggest 🐵 I’ve ever seen!"), Ok(true));
```

If you find yourself continuously creating and destroying profiles to perform
any of the operation described for internationalized strings. You can make use
of the `PrecisFastInvocation` trait.
Profiles implementing this trait will allow you to prepare, enforce or compare
internationalized strings without having to instantiate a specific profile.
Profiles usually use a static instance allocated with
[`lazy_static`](https://docs.rs/lazy_static/)

### Example

```rust
assert_eq!(Nickname::prepare("Guybrush Threepwood"),
  Ok(Cow::from("Guybrush Threepwood")));
assert_eq!(Nickname::enforce("   Guybrush     Threepwood  "),
  Ok(Cow::from("Guybrush Threepwood")));
assert_eq!(Nickname::compare("Guybrush   Threepwood  ",
  "guybrush threepwood"), Ok(true));
```

# Common features

* **networking** - Enables crates to download Unicode Character Database [`UCD`](https://unicode.org) files from the network. This is helpful for developing and testing new Unicode releases, but this feature can not be enabled to generate the library documentation when the crate is uploaded to [`crates.io`](https://crates.io). For security reasons, network access is not allowed to crates to avoid compromising the security of docs.rs itself.

# Known limitations

PRECIS recommends using `toLowerCase`() operation as defined in the Unicode Standard
[Unicode](http://www.unicode.org/versions/latest/). This implementation uses the 
one provided by Rust standard library 
[`to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase).
This operation performs an unconditional mapping without tailoring. That is, the 
conversion is independent of context and language.

# Contributing

Patches and feedback are welcome.

# Donations

If you find this project helpful, you may consider making a donation:

[![Donate with Bitcoin](https://en.cryptobadges.io/badge/micro/1EK28M4ht6qu7xFahTxuquXPzZSjCSGVBM)](https://en.cryptobadges.io/donate/1EK28M4ht6qu7xFahTxuquXPzZSjCSGVBM)
[![Donate with Ethereum](https://en.cryptobadges.io/badge/micro/0xefa6404e5A50774117fd6204cbD33cf4454c67Fb)](https://en.cryptobadges.io/donate/0xefa6404e5A50774117fd6204cbD33cf4454c67Fb)


# License

This project is licensed under either of
* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT) at your option.

[![say thanks](https://img.shields.io/badge/Say%20Thanks-👍-1EAEDB.svg)](https://github.com/sancane/precis/stargazers)
