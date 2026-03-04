// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: Apache-2.0

//! Serialize byte arrays and vectors as bytes or as human-readable strings,
//! depending on the format.
//!
//! For example, serialize as hex if serializing to JSON, or as an efficient
//! byte sequence with CBOR.

#![warn(missing_docs)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod base64_vec;
#[cfg(feature = "alloc")]
mod base64_vec_type;
pub mod hex_array;
mod hex_array_type;

#[cfg(feature = "alloc")]
pub use base64_vec_type::Base64Vec;
pub use hex_array_type::HexArray;
