// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: Apache-2.0

//! Serialize a byte array as a list of bytes if human-readable, or as hex if not.

use core::{convert::TryInto, fmt};
use serde_core::{
    Deserializer,
    de::{Expected, Visitor},
};

/// Serialization implementations that require the `alloc` feature.
#[cfg(feature = "alloc")]
mod alloc_impls {
    use hex::ToHex;
    use serde_core::Serializer;

    /// Implements serialization for byte arrays to a hex string if human-readable, or as bytes if not.
    ///
    /// This should work transparently with any `[u8; N].
    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let s = bytes.encode_hex::<alloc::string::String>();
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }

    /// Similar to [`serialize`], except to upper-case.
    pub fn serialize_upper<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let s = bytes.encode_hex_upper::<alloc::string::String>();
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impls::*;

/// Deserializes hex strings (if human-readable) or byte arrays (if not) to `[u8; N]`.
pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
where
    D: Deserializer<'de>,
{
    use serde_core::de::Error;

    if deserializer.is_human_readable() {
        // Bleh, hex::FromHex doesn't have an implementation for const-generic N sadly. Do our own
        // thing.
        struct HexVisitor<const N: usize>;

        impl<'de2, const N: usize> Visitor<'de2> for HexVisitor<N> {
            type Value = [u8; N];

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a hex-encoded string {} bytes long", N)
            }

            fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut out = [0u8; N];
                hex::decode_to_slice(data, &mut out).map_err(Error::custom)?;
                Ok(out)
            }

            fn visit_borrowed_str<E>(self, data: &'de2 str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut out = [0u8; N];
                hex::decode_to_slice(data, &mut out).map_err(Error::custom)?;
                Ok(out)
            }
        }

        deserializer.deserialize_str(HexVisitor)
    } else {
        struct BytesVisitor<const N: usize>;

        impl<'de2, const N: usize> Visitor<'de2> for BytesVisitor<N> {
            type Value = [u8; N];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a byte array [u8; {}]", N)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map_err(|_| E::invalid_length(v.len(), &HexExpected::<N>))
            }
        }

        deserializer.deserialize_bytes(BytesVisitor)
    }
}

struct HexExpected<const N: usize>;

impl<const N: usize> Expected for HexExpected<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a byte array [u8; {}]", N)
    }
}
