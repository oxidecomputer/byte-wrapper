# AGENTS.md

This file provides guidance to LLMs when working with code in this repository.

## Build and test commands

```shell
cargo build
cargo nextest run                  # all tests
cargo clippy --all-targets         # lint
cargo xfmt                         # check formatting (max_width=80, edition 2024)
just powerset nextest run          # test all feature combinations via cargo-hack
just generate-readmes              # regenerate readme from lib.rs
```

## Architecture

Single-crate library providing newtype wrappers for byte arrays and vectors with hex and base64 formatting:

- **`HexArray<N>`** — wraps `[u8; N]`, displays as hex. With the `serde` feature, serializes as hex in human-readable formats and raw bytes in binary formats.
- **`Base64Vec`** — wraps `Vec<u8>`, displays as base64. With the `serde` feature, serializes as base64 in human-readable formats and raw bytes in binary formats. Requires the `alloc` feature.
- Both types support `#[serde(with = "...")]` for use on raw `[u8; N]` / `Vec<u8>` fields (requires the `serde` feature).

## Key conventions

- MSRV is 1.85 (Rust edition 2024).
- `no_std` by default; `alloc` feature enables `Base64Vec`; `serde` feature enables serde support.
- `#![deny(missing_docs)]` enforced.
- README generated from rustdoc via `cargo-sync-rdme`.
- Each type lives in a private module and is re-exported from the crate root.
