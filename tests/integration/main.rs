// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(all(feature = "base64", feature = "alloc"))]
mod base64;
#[cfg(all(feature = "schemars08", feature = "base64"))]
mod base64_schemars;
#[cfg(all(feature = "base64", feature = "alloc", feature = "serde"))]
mod base64_serde;
#[cfg(all(feature = "base64", feature = "alloc"))]
mod base64_slice;
#[cfg(feature = "base64")]
mod base64_slice_fmt;
#[cfg(all(feature = "base64", feature = "alloc", feature = "serde"))]
mod base64_slice_serde;
#[cfg(all(feature = "alloc", any(feature = "base64", feature = "hex")))]
mod helpers;
#[cfg(feature = "hex")]
mod hex;
#[cfg(all(feature = "schemars08", feature = "hex"))]
mod hex_schemars;
#[cfg(all(feature = "hex", feature = "serde"))]
mod hex_serde;
#[cfg(all(feature = "hex", feature = "alloc"))]
mod hex_vec;
#[cfg(all(feature = "schemars08", feature = "hex"))]
mod hex_vec_schemars;
#[cfg(all(feature = "hex", feature = "alloc", feature = "serde"))]
mod hex_vec_serde;
#[cfg(feature = "schemars08")]
mod typify_roundtrip;
