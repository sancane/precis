![Tests](https://github.com/sancane/precis/actions/workflows/rust_tests.yml/badge.svg)
[![codecov](https://codecov.io/gh/sancane/precis/branch/main/graph/badge.svg?token=EZM6KCSLZC)](https://codecov.io/gh/sancane/precis)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/sancane/precis)](https://rust-reportcard.xuri.me/report/github.com/sancane/precis)
[![dependency status](https://deps.rs/repo/github/sancane/precis/status.svg)](https://deps.rs/repo/github/sancane/precis)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/sancane/precis?utm_source=badge)


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
[`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)

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

# Development

## Testing

The project has comprehensive test coverage with multiple testing approaches:

- **Unit tests**: tests covering core functionality
- **Property-based tests**: tests with [proptest](https://proptest-rs.github.io/proptest/) generating thousands of random inputs. See [PROPTEST_GUIDE.md](PROPTEST_GUIDE.md) for details.
- **Integration tests**: RFC test vectors and edge cases
- **Doc tests**: tests in documentation examples

```bash
# Run all tests
cargo test

# Run property-based tests
cargo test proptest

# Run with increased cases
PROPTEST_CASES=10000 cargo test proptest
```

See [PROPTEST_GUIDE.md](PROPTEST_GUIDE.md) and [PROPTEST_CI.md](PROPTEST_CI.md) for complete testing documentation.

## Fuzzing

The project supports fuzzing with cargo-fuzz to discover edge cases, panics, and bugs. Comprehensive fuzz targets available across core classes and all profiles:

```bash
# Install cargo-fuzz (requires nightly)
cargo install cargo-fuzz

# Run fuzzing (5 seconds quick test)
cd precis-profiles
cargo +nightly fuzz run nickname_enforce -- -max_total_time=5

# List available targets
cd precis-profiles && cargo +nightly fuzz list  # Profile targets
cd precis-core && cargo +nightly fuzz list      # Core class targets
```

**Profile targets** (prepare, enforce, compare for all profiles):
- **Nickname**: `nickname_enforce`, `nickname_prepare`, `nickname_compare`, `nickname_arbitrary`
- **OpaqueString**: `opaque_string_enforce`, `opaque_string_prepare`, `opaque_string_compare`
- **UsernameCaseMapped**: `username_casemapped`, `username_casemapped_prepare`, `username_casemapped_compare`
- **UsernameCasePreserved**: `username_casepreserved`, `username_casepreserved_prepare`, `username_casepreserved_compare`

**Core class targets** (FreeformClass, IdentifierClass):
- `freeform_class_allows`, `freeform_class_get_value`, `freeform_class_codepoint`
- `identifier_class_allows`, `identifier_class_get_value`, `identifier_class_codepoint`

**CI Integration:**
- ✅ **ClusterFuzzLite** runs automatically on every PR
- ✅ 5 minutes fuzzing per target
- ✅ Catches bugs before merge
- ✅ No setup required

See [FUZZING.md](FUZZING.md) for complete fuzzing guide.

## Benchmarking

The project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for performance benchmarking, integrated with [CodSpeed](https://codspeed.io/) for continuous performance tracking:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench stringclass_bench
cargo bench --bench profiles_bench

# Compare with baseline
cargo bench -- --save-baseline main
# ... make changes ...
cargo bench -- --baseline main
```

**CodSpeed Integration:**
- ✅ Automatic performance tracking on every PR
- ✅ Regression detection
- ✅ Historical performance trends

See [BENCHMARKING.md](BENCHMARKING.md) for complete benchmarking guide and [CODSPEED.md](CODSPEED.md) for CodSpeed setup.

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
