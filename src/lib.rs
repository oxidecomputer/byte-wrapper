// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newtype wrappers for byte arrays and vectors with hex and base64
//! formatting.
//!
//! This crate provides wrapper types that display byte data in
//! human-readable encodings. [`HexArray<N>`] encodes fixed-length byte
//! arrays as hex strings, and [`Base64Vec`] encodes variable-length
//! byte vectors as base64 strings.
//!
//! With the `serde` feature, both types implement `Serialize` and
//! `Deserialize`, encoding as human-readable strings (hex or base64) in text
//! formats like JSON, and as efficient raw bytes in binary formats like [CBOR].
//! You do not have to use the newtypes in your own type definitions; you can
//! refer to them via `#[serde(with = "...")]` instead.
//!
//! [CBOR]: https://cbor.io/
//!
//! # Types
//!
//! * [`HexArray<N>`] encodes a fixed-length byte array as a hex
//!   string.
//! * [`Base64Vec`] encodes a variable-length byte vector as a base64
//!   string. (The `alloc` feature is required.)
//!
//! # Examples
//!
//! ```
//! use byte_wrapper::HexArray;
//!
//! let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
//! assert_eq!(h.to_string(), "0102abff");
//!
//! let parsed: HexArray<4> = "0102abff".parse().unwrap();
//! assert_eq!(parsed, h);
//! ```
//!
//! With the **`serde`** feature:
//!
//! ```
//! # #[cfg(feature = "serde")] {
//! use byte_wrapper::HexArray;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Record {
//!     checksum: HexArray<32>,
//! }
//! # }
//! ```
//!
//! Using `#[serde(with = "...")]` on an existing byte array:
//!
//! ```
//! # #[cfg(feature = "serde")] {
//! use byte_wrapper::HexArray;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Record {
//!     #[serde(with = "HexArray::<32>")]
//!     checksum: [u8; 32],
//! }
//! # }
//! ```
//!
//! # Features
//!
//! - **`alloc`**: enables [`Base64Vec`]. *Enabled by default.*
//! - **`serde`**: implements `Serialize` and `Deserialize` for both
//!   types. *Not enabled by default.*
//! - **`schemars08`**: derives `JsonSchema` for both types.
//!   *Not enabled by default.*

#![deny(missing_docs)]
#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod base64_vec;
mod hex_array;

#[cfg(feature = "alloc")]
pub use base64_vec::{Base64Vec, ParseBase64Error};
pub use hex_array::{HexArray, ParseHexError};
