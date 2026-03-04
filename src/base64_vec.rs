// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Serialize a byte vector as base64 if human-readable, or as bytes if not.

use alloc::vec::Vec;
use base64::Engine;
use core::fmt;
use serde_core::{Deserializer, Serializer, de::Visitor};

/// Implements serialization for byte vectors to a base64 string if
/// human-readable, or as bytes if not.
pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if serializer.is_human_readable() {
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        serializer.serialize_str(&encoded)
    } else {
        serializer.serialize_bytes(bytes)
    }
}

/// Deserializes base64 strings (if human-readable) or byte arrays (if not) to
/// `Vec<u8>`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_core::de::Error;

    if deserializer.is_human_readable() {
        struct Base64Visitor;

        impl<'de2> Visitor<'de2> for Base64Visitor {
            type Value = Vec<u8>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a base64-encoded string")
            }

            fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                base64::engine::general_purpose::STANDARD
                    .decode(data)
                    .map_err(Error::custom)
            }

            fn visit_borrowed_str<E>(
                self,
                data: &'de2 str,
            ) -> Result<Self::Value, E>
            where
                E: Error,
            {
                base64::engine::general_purpose::STANDARD
                    .decode(data)
                    .map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(Base64Visitor)
    } else {
        struct BytesVisitor;

        impl<'de2> Visitor<'de2> for BytesVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v.to_vec())
            }
        }

        deserializer.deserialize_bytes(BytesVisitor)
    }
}
