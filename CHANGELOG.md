# v2.0.9 — 2025-09-14

Release by: Larry Wren

## Documentation

- Crate-level docs: rewritten for clarity with Quickstart, MCCP boundary handling, recipes (NAWS, TTYPE, GMCP), feature flags, and FAQ.
- Module docs: added to `events`, `compatibility`, and `telnet` with runnable examples so docs.rs renders a complete picture of the API.
- README: modernized and corrected — added install snippet, badges (crates.io, docs.rs), accurate code samples, links to examples, and contributor guidance.
- Examples: new `examples/basic.rs` and `examples/tokio_client.rs` (async Tokio integration). Fixed doctests to work with `bytes::Bytes` conversions and non_exhaustive enums.

## Tooling

- clippy pedantic: resolved doc_markdown lint; repo is clean with `cargo clippy --all-targets --all-features -D warnings`.

## Notes

- No API or behavior changes. Patch release focused on documentation quality and developer experience.

# v2.0.8 — 2025-09-13

Release by: Larry Wren

## Bug fixes

* Parser: Correctly handle truncated subnegotiations where the first byte after
  `SB` is `IAC` (e.g., `IAC SB IAC SE`). The empty subnegotiation is now emitted
  and any trailing bytes are preserved and delivered as normal data. This brings
  behavior in line with RFC 854 expectations and the reference implementation.
* Improved resilience around stray `SE` bytes during subnegotiation parsing so
  non-IAC-terminated `SE` is treated as data, avoiding loss of bytes in edge cases.

## Documentation

* Added crate-level documentation with a Quickstart example and `no_std` notes.
* Enabled docs.rs config (`all-features = true`) and `doc_cfg` for clearer feature docs.

## Tooling & CI

* Added a lightweight CI workflow running format, clippy (with `-D warnings`),
  tests (all features), and a `no_std` check.
* Disabled fuzzing on push/PR; fuzz workflow is now manual via `workflow_dispatch`.

## Developer Experience

* Compat benches: `arbitrary` feature is enabled by default and benches are
  gated with `required-features`, simplifying `cargo bench` usage.
* Workspaces/publish: marked `compat` as `publish = false` and adjusted
  workspace defaults so `cargo publish --dry-run` works cleanly from the repo root
  without needing `-p`.
* General clippy cleanups (needless borrows, acronyms, clone-on-copy, format args).

## Compatibility & MSRV

* MSRV raised to Rust 1.66 to align with tooling and docs configuration.
* BREAKING (source): `events::TelnetEvents` is now `#[non_exhaustive]` to allow
  future extension without further breakage. Migration: add a wildcard arm to
  exhaustive `match` statements over `TelnetEvents`.

---

Initial release of `libmudtelnet` - a fork of `libtelnet-rs`.

## Bug fixes

* A `SE` byte that isn't preceded by `IAC` is now properly handled as a normal
  byte during Telnet subnegotiation processing.
* Fixed a panic when Telnet option code 0xFF is negotiated, and a truncated
  subnegotiation (e.g. `IAC SB IAC SE`) is received.
* Multiple escaped `IAC` bytes (`e.g. IAC IAC IAC IAC`) are now properly
 unescaped (e.g. `IAC IAC`) instead of truncated (e.g. `IAC`).

## Features

* Many API types now derive helpful traits (`Debug`, `Eq`, etc.).
* An optional `arbitrary` crate feature is now available to enable generating
  arbitrary `event::TelnetIAC`, `event::TelnetNegotiation` and `event::TelnetSubnegotiation`
  instances for testing.
* **Added missing Telnet option constants**: `CHARSET` (42), `MSDP` (69), `MXP` (91), and `ATCP` (200)
  are now available in `telnet::op_option`.
* **Added MSDP subnegotiation tag constants**: New `telnet::msdp` module provides canonical byte
  constants for MSDP protocol parsing (`VAR`, `VAL`, `TABLE_OPEN`, `TABLE_CLOSE`, `ARRAY_OPEN`, `ARRAY_CLOSE`).

## Misc

* CI improvements.
* Fuzz testing.
* Simple benchmarking.
* Small performance and safety improvements (avoiding direct indexing, etc.).

# v2.0.0

As of version 2.0, `libtelnet_rs` has switched over to using the `bytes`
[crate](https://crates.io/crates/bytes).

With this change, the method signatures for most methods that return `Vec<u8>`
now return `Bytes` instead.

For most situations where a `Vec<u8>` would be required, the returned value can
simply be changed to utilize `Bytes::to_vec()`.

To make usage of the new dependency a little bit easier, it is also re-exported
as `libtelnet_rs::bytes`.
