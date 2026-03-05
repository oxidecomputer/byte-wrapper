// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "alloc")]
mod base64;
// HexArray works without alloc, but the test harness requires require an
// allocator.
#[cfg(feature = "alloc")]
mod hex;
#[cfg(feature = "schemars08")]
mod schemars;
