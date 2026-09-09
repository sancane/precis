# Changelog

All notable changes to the `precis-wasm` package are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this package adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Its version tracks the `precis-wasm` crate version in `Cargo.toml`.

## [Unreleased]

## [0.1.0]

### Added

- WebAssembly bindings for the PRECIS profiles: `Nickname` (RFC 8266),
  `OpaqueString`, `UsernameCaseMapped`, and `UsernameCasePreserved` (RFC 8265),
  each exposing `prepare`, `enforce`, and `compare`.
- TypeScript wrappers with `string → string` signatures over the raw
  `wasm-bindgen` glue (whose types are `any`).
- A single package that serves browsers/bundlers and Node.js through
  conditional `exports`: an ESM entry (`precis.mjs`, requires `await init()`)
  and a CommonJS entry (`precis-node.js`, initialized automatically).

[Unreleased]: https://github.com/sancane/precis/compare/main...HEAD
[0.1.0]: https://github.com/sancane/precis/releases/tag/precis-wasm-v0.1.0
