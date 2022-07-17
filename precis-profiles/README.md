[![Docs](https://docs.rs/precis-profiles/badge.svg)](https://docs.rs/precis-profiles)

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

```rust
assert_eq!(Nickname::prepare("Guybrush Threepwood"),
  Ok(Cow::from("Guybrush Threepwood")));
assert_eq!(Nickname::enforce("   Guybrush     Threepwood  "),
  Ok(Cow::from("Guybrush Threepwood")));
assert_eq!(Nickname::compare("Guybrush   Threepwood  ",
  "guybrush threepwood"), Ok(true));
```

## `crates.io`

You can use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
precis-profiles = "0.1.6"
```

# Documentation
https://docs.rs/precis-profiles

# License

This project is licensed under either of
* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT) at your option.
