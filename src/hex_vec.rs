// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`HexVec`] newtype wrapper.

use crate::{
    HexSlice, ParseHexError, hex_array::decode_hex_into, hex_slice::HexDisplay,
};
use alloc::{borrow::ToOwned, vec, vec::Vec};
use core::{borrow::Borrow, fmt, str::FromStr};

/// A byte vector that displays and parses as hex.
///
/// `HexVec` wraps `Vec<u8>`, providing [`Display`](fmt::Display),
/// [`FromStr`], [`LowerHex`](fmt::LowerHex), and
/// [`UpperHex`](fmt::UpperHex) implementations that use hexadecimal
/// encoding.
///
/// With the **`serde`** feature enabled, it also implements
/// `Serialize` and `Deserialize` (hex strings in human-readable
/// formats, raw bytes in binary formats), and can be used with
/// `#[serde(with = "HexVec")]` on `Vec<u8>` fields.
///
/// # Examples
///
/// ```
/// use byte_wrapper::HexVec;
///
/// let h = HexVec::new(vec![0x01, 0x02, 0xab, 0xff]);
/// assert_eq!(h.to_string(), "0102abff");
///
/// let parsed: HexVec = "0102abff".parse().unwrap();
/// assert_eq!(parsed, h);
/// ```
#[must_use]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HexVec(pub Vec<u8>);

impl HexVec {
    /// Creates a new `HexVec` from a byte vector.
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Returns the inner byte vector.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    #[inline]
    fn as_hex_slice(&self) -> &HexSlice {
        HexSlice::new(&self.0)
    }
}

impl FromStr for HexVec {
    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.len().is_multiple_of(2) {
            return Err(ParseHexError::OddLength { actual: s.len() });
        }
        let mut out = vec![0u8; s.len() / 2];
        decode_hex_into(s, &mut out)?;
        Ok(Self(out))
    }
}

impl fmt::Debug for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HexVec").field(&HexDisplay(&self.0)).finish()
    }
}

impl fmt::Display for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_hex_slice(), f)
    }
}

impl fmt::LowerHex for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self.as_hex_slice(), f)
    }
}

impl fmt::UpperHex for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self.as_hex_slice(), f)
    }
}

impl core::ops::Deref for HexVec {
    type Target = HexSlice;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_hex_slice()
    }
}

impl Borrow<HexSlice> for HexVec {
    #[inline]
    fn borrow(&self) -> &HexSlice {
        self.as_hex_slice()
    }
}

impl ToOwned for HexSlice {
    type Owned = HexVec;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        HexVec(self.as_bytes().to_vec())
    }
}

impl AsRef<HexSlice> for HexVec {
    #[inline]
    fn as_ref(&self) -> &HexSlice {
        self.as_hex_slice()
    }
}

impl AsRef<[u8]> for HexVec {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for HexVec {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<Vec<u8>> for HexVec {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<HexVec> for Vec<u8> {
    #[inline]
    fn from(hex_vec: HexVec) -> Self {
        hex_vec.0
    }
}

impl PartialEq<HexSlice> for HexVec {
    #[inline]
    fn eq(&self, other: &HexSlice) -> bool {
        self.as_hex_slice() == other
    }
}

impl PartialEq<&HexSlice> for HexVec {
    #[inline]
    fn eq(&self, other: &&HexSlice) -> bool {
        self.as_hex_slice() == *other
    }
}

impl PartialEq<HexVec> for HexSlice {
    #[inline]
    fn eq(&self, other: &HexVec) -> bool {
        self == other.as_hex_slice()
    }
}

impl PartialEq<HexVec> for &HexSlice {
    #[inline]
    fn eq(&self, other: &HexVec) -> bool {
        *self == other.as_hex_slice()
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::HexVec;
    use crate::HexSlice;
    use alloc::vec::Vec;
    use core::fmt;
    use serde_core::{Deserializer, Serialize, Serializer, de::Visitor};

    fn deserialize_hex_vec<'de, D>(deserializer: D) -> Result<HexVec, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_core::de::Error;

        if deserializer.is_human_readable() {
            struct HexVecVisitor;

            impl<'de2> Visitor<'de2> for HexVecVisitor {
                type Value = HexVec;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "a hex string with an even number of hex digits")
                }

                fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    data.parse().map_err(Error::custom)
                }
            }

            deserializer.deserialize_str(HexVecVisitor)
        } else {
            crate::serde_util::deserialize_byte_buf(deserializer).map(HexVec)
        }
    }

    impl HexVec {
        /// Serializes a byte vector as hex if the format is
        /// human-readable, or as raw bytes otherwise.
        ///
        /// Intended for use with `#[serde(with = "HexVec")]`.
        ///
        /// # Examples
        ///
        /// ```
        /// use byte_wrapper::HexVec;
        /// use serde::{Deserialize, Serialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct Blob {
        ///     #[serde(with = "HexVec")]
        ///     data: Vec<u8>,
        /// }
        ///
        /// let b = Blob { data: vec![1, 2, 3] };
        /// let json = serde_json::to_string(&b).unwrap();
        /// assert_eq!(json, r#"{"data":"010203"}"#);
        /// ```
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        pub fn serialize<S>(
            bytes: &[u8],
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            HexSlice::new(bytes).serialize(serializer)
        }

        /// Deserializes a byte vector from hex if the format is
        /// human-readable, or as raw bytes otherwise.
        ///
        /// Intended for use with `#[serde(with = "HexVec")]`.
        ///
        /// # Examples
        ///
        /// ```
        /// use byte_wrapper::HexVec;
        /// use serde::{Deserialize, Serialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct Blob {
        ///     #[serde(with = "HexVec")]
        ///     data: Vec<u8>,
        /// }
        ///
        /// let b: Blob = serde_json::from_str(r#"{"data":"010203"}"#).unwrap();
        /// assert_eq!(b.data, [1, 2, 3]);
        /// ```
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_hex_vec(deserializer).map(HexVec::into_inner)
        }
    }

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl serde_core::Serialize for HexVec {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.as_hex_slice().serialize(serializer)
        }
    }

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl<'de> serde_core::Deserialize<'de> for HexVec {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_hex_vec(deserializer)
        }
    }
}

#[cfg(feature = "schemars08")]
mod schemars_impls {
    use super::HexVec;
    use crate::{HexSlice, schemars_util::x_rust_type_extension};
    use alloc::{boxed::Box, string::String};
    use schemars08::{
        JsonSchema,
        r#gen::SchemaGenerator,
        schema::{InstanceType, Schema, SchemaObject, StringValidation},
    };

    impl JsonSchema for HexVec {
        fn schema_name() -> String {
            "HexVec".into()
        }

        fn is_referenceable() -> bool {
            false
        }

        fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                string: Some(Box::new(StringValidation {
                    pattern: Some("^([0-9a-fA-F]{2})*$".into()),
                    ..Default::default()
                })),
                extensions: x_rust_type_extension("HexVec"),
                ..Default::default()
            })
        }
    }

    // HexSlice shares HexVec's schema (including `x-rust-type` set to
    // `HexVec`) since generated code needs owned types. (This also covers
    // `Cow<'_, HexSlice>`.)
    impl JsonSchema for HexSlice {
        fn schema_name() -> String {
            HexVec::schema_name()
        }

        fn is_referenceable() -> bool {
            HexVec::is_referenceable()
        }

        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            HexVec::json_schema(generator)
        }
    }
}
