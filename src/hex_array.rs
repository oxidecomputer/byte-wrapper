// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`HexArray`] newtype wrapper.

use crate::{HexSlice, hex_slice::HexDisplay};
use core::{array::TryFromSliceError, error, fmt, str::FromStr};

/// A byte array that displays and parses as hex.
///
/// `HexArray<N>` wraps `[u8; N]`, providing [`Display`](fmt::Display),
/// [`FromStr`], [`LowerHex`](fmt::LowerHex), and
/// [`UpperHex`](fmt::UpperHex) implementations that use hexadecimal
/// encoding.
///
/// With the **`serde`** feature enabled, it also implements
/// `Serialize` and `Deserialize` (hex strings in human-readable
/// formats, raw bytes in binary formats), and can be used with
/// `#[serde(with = "HexArray::<N>")]` on `[u8; N]` fields.
///
/// # Examples
///
/// ```
/// use byte_wrapper::HexArray;
///
/// let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
/// assert_eq!(h.to_string(), "0102abff");
///
/// let parsed: HexArray<4> = "0102abff".parse().unwrap();
/// assert_eq!(parsed, h);
/// ```
#[must_use]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for HexArray<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> HexArray<N> {
    /// Creates a new `HexArray` from a byte array.
    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Returns the inner byte array.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> [u8; N] {
        self.0
    }
}

/// Error returned by [`HexArray::from_str`] and
/// [`HexVec::from_str`](crate::HexVec).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHexError {
    /// The input string had the wrong length.
    InvalidLength {
        /// Expected number of hex characters.
        expected: usize,
        /// Actual number of characters in the input.
        actual: usize,
    },
    /// The input string had an odd number of characters.
    OddLength {
        /// Actual number of characters in the input.
        actual: usize,
    },
    /// The input contained an invalid hex character.
    InvalidHexCharacter {
        /// The invalid character.
        c: char,
        /// Byte index of the invalid character in the input.
        index: usize,
    },
}

impl fmt::Display for ParseHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseHexError::InvalidLength { expected, actual } => {
                write!(
                    f,
                    "expected {} hex characters, got {}",
                    expected, actual,
                )
            }
            ParseHexError::OddLength { actual } => {
                write!(
                    f,
                    "expected an even number of hex characters, got {actual}",
                )
            }
            ParseHexError::InvalidHexCharacter { c, index } => {
                write!(f, "invalid hex character '{}' at index {}", c, index,)
            }
        }
    }
}

impl error::Error for ParseHexError {}

/// Decodes `s` into `out`, requiring that `s` is exactly `2 * out.len()` hex
/// characters.
pub(crate) fn decode_hex_into(
    s: &str,
    out: &mut [u8],
) -> Result<(), ParseHexError> {
    let expected = out.len() * 2;
    if s.len() != expected {
        return Err(ParseHexError::InvalidLength { expected, actual: s.len() });
    }
    hex::decode_to_slice(s, out).map_err(|e| {
        match e {
            hex::FromHexError::InvalidHexCharacter { c, index } => {
                ParseHexError::InvalidHexCharacter { c, index }
            }
            // The length is already validated above, so this
            // branch is unreachable in practice.
            hex::FromHexError::OddLength
            | hex::FromHexError::InvalidStringLength => {
                ParseHexError::InvalidLength { expected, actual: s.len() }
            }
        }
    })
}

impl<const N: usize> FromStr for HexArray<N> {
    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = [0u8; N];
        decode_hex_into(s, &mut out)?;
        Ok(Self(out))
    }
}

impl<const N: usize> fmt::Debug for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HexArray").field(&HexDisplay(&self.0)).finish()
    }
}

impl<const N: usize> fmt::Display for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(HexSlice::new(&self.0), f)
    }
}

impl<const N: usize> fmt::LowerHex for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(HexSlice::new(&self.0), f)
    }
}

impl<const N: usize> fmt::UpperHex for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(HexSlice::new(&self.0), f)
    }
}

impl<const N: usize> core::ops::Deref for HexArray<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> core::ops::DerefMut for HexArray<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for HexArray<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for HexArray<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> From<[u8; N]> for HexArray<N> {
    #[inline]
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> From<HexArray<N>> for [u8; N] {
    #[inline]
    fn from(hex_array: HexArray<N>) -> Self {
        hex_array.0
    }
}

impl<const N: usize> TryFrom<&[u8]> for HexArray<N> {
    type Error = TryFromSliceError;

    #[inline]
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        <[u8; N]>::try_from(slice).map(Self)
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::HexArray;
    use crate::HexSlice;
    use core::fmt;
    use serde_core::{
        Deserializer, Serialize,
        de::{Expected, SeqAccess, Visitor},
    };

    struct HexExpected<const N: usize>;

    impl<const N: usize> Expected for HexExpected<N> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a byte array [u8; {}]", N)
        }
    }

    struct HexStrExpected<const N: usize>;

    impl<const N: usize> Expected for HexStrExpected<N> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a hex string {} hex digits long", N * 2)
        }
    }

    /// Deserializes hex strings (if human-readable) or byte arrays
    /// (if not) to `[u8; N]`.
    fn deserialize<'de, D, const N: usize>(
        deserializer: D,
    ) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_core::de::Error;

        if deserializer.is_human_readable() {
            // hex::FromHex doesn't have an implementation for
            // const-generic N, so do our own thing.
            struct HexVisitor<const N: usize>;

            impl<'de2, const N: usize> Visitor<'de2> for HexVisitor<N> {
                type Value = [u8; N];

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "a hex string {} hex digits long", N * 2)
                }

                fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    let expected_len = N * 2;
                    if data.len() != expected_len {
                        return Err(E::invalid_length(
                            data.len(),
                            &HexStrExpected::<N>,
                        ));
                    }
                    let mut out = [0u8; N];
                    hex::decode_to_slice(data, &mut out)
                        .map_err(Error::custom)?;
                    Ok(out)
                }
            }

            deserializer.deserialize_str(HexVisitor)
        } else {
            struct BytesVisitor<const N: usize>;

            impl<'de2, const N: usize> Visitor<'de2> for BytesVisitor<N> {
                type Value = [u8; N];

                fn expecting(
                    &self,
                    formatter: &mut fmt::Formatter,
                ) -> fmt::Result {
                    write!(formatter, "a byte array [u8; {}]", N)
                }

                fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    v.try_into().map_err(|_| {
                        E::invalid_length(v.len(), &HexExpected::<N>)
                    })
                }

                fn visit_seq<A>(
                    self,
                    mut seq: A,
                ) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de2>,
                {
                    // Reject early if the sequence reports a
                    // wrong length.
                    if let Some(len) = seq.size_hint()
                        && len != N
                    {
                        return Err(Error::invalid_length(
                            len,
                            &HexExpected::<N>,
                        ));
                    }
                    let mut out = [0u8; N];
                    for (i, byte) in out.iter_mut().enumerate() {
                        *byte = seq.next_element()?.ok_or_else(|| {
                            Error::invalid_length(i, &HexExpected::<N>)
                        })?;
                    }
                    // Reject trailing elements rather than
                    // silently discarding them.
                    if seq.next_element::<u8>()?.is_some() {
                        // We don't know the actual length, but
                        // we know it's more than N.
                        return Err(Error::invalid_length(
                            N + 1,
                            &HexExpected::<N>,
                        ));
                    }
                    Ok(out)
                }
            }

            deserializer.deserialize_bytes(BytesVisitor)
        }
    }

    impl<const N: usize> HexArray<N> {
        /// Serializes a byte array as hex in human-readable formats,
        /// or as raw bytes otherwise.
        ///
        /// Intended for use with
        /// `#[serde(with = "HexArray::<N>")]`.
        ///
        /// # Examples
        ///
        /// ```
        /// use byte_wrapper::HexArray;
        /// use serde::{Deserialize, Serialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct Record {
        ///     #[serde(with = "HexArray::<4>")]
        ///     id: [u8; 4],
        /// }
        ///
        /// let r = Record { id: [0x01, 0x02, 0x03, 0x04] };
        /// let json = serde_json::to_string(&r).unwrap();
        /// assert_eq!(json, r#"{"id":"01020304"}"#);
        /// ```
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        pub fn serialize<S>(
            bytes: &[u8; N],
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde_core::Serializer,
        {
            HexSlice::new(bytes).serialize(serializer)
        }

        /// Deserializes a byte array from hex if the format is
        /// human-readable, or as raw bytes otherwise.
        ///
        /// Intended for use with
        /// `#[serde(with = "HexArray::<N>")]`.
        ///
        /// # Examples
        ///
        /// ```
        /// use byte_wrapper::HexArray;
        /// use serde::{Deserialize, Serialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct Record {
        ///     #[serde(with = "HexArray::<4>")]
        ///     id: [u8; 4],
        /// }
        ///
        /// let r: Record = serde_json::from_str(r#"{"id":"01020304"}"#).unwrap();
        /// assert_eq!(r.id, [0x01, 0x02, 0x03, 0x04]);
        /// ```
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize(deserializer)
        }
    }

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl<const N: usize> serde_core::Serialize for HexArray<N> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde_core::Serializer,
        {
            HexSlice::new(&self.0).serialize(serializer)
        }
    }

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl<'de, const N: usize> serde_core::Deserialize<'de> for HexArray<N> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize(deserializer).map(Self)
        }
    }
}

#[cfg(feature = "schemars08")]
mod schemars_impls {
    use super::HexArray;
    use crate::schemars_util::x_rust_type_extension;
    use alloc::{boxed::Box, format, string::String};
    use schemars08::{
        JsonSchema,
        r#gen::SchemaGenerator,
        schema::{InstanceType, Schema, SchemaObject, StringValidation},
    };

    impl<const N: usize> JsonSchema for HexArray<N> {
        fn schema_name() -> String {
            format!("HexArray_{N}")
        }

        fn is_referenceable() -> bool {
            false
        }

        fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
            let hex_len = N * 2;
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                string: Some(Box::new(StringValidation {
                    min_length: Some(hex_len as u32),
                    max_length: Some(hex_len as u32),
                    pattern: Some(format!("^[0-9a-fA-F]{{{hex_len}}}$")),
                })),
                extensions: x_rust_type_extension(&format!("HexArray::<{N}>")),
                ..Default::default()
            })
        }
    }
}
