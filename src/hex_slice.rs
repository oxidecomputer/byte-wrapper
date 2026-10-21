// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`HexSlice`] newtype wrapper.

use core::fmt::{self, Write};

/// A byte slice that displays as hex.
///
/// The borrowed, dynamically-sized equivalent to
/// [`HexVec`](crate::HexVec), akin to `str`'s relationship to `String`.
///
/// `HexSlice` wraps `[u8]`, providing [`Display`](fmt::Display),
/// [`LowerHex`](fmt::LowerHex), and [`UpperHex`](fmt::UpperHex)
/// implementations that use hexadecimal encoding.
///
/// With the **`serde`** feature enabled, it also implements
/// `Serialize` (hex strings in human-readable formats, raw bytes in
/// binary formats).
///
/// # Examples
///
/// ```
/// use byte_wrapper::HexSlice;
///
/// const BYTES: [u8; 4] = [0x01, 0x02, 0xab, 0xff];
/// const H: &HexSlice = HexSlice::new(&BYTES);
/// assert_eq!(H.to_string(), "0102abff");
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HexSlice([u8]);

impl HexSlice {
    /// Creates a new `HexSlice` from a byte slice.
    #[inline]
    #[must_use]
    pub const fn new(bytes: &[u8]) -> &Self {
        // SAFETY:
        //
        // * `repr(transparent)` gives `HexSlice` the layout and validity of
        //   `[u8]`.
        // * An unsized-to-unsized pointer cast retains the length metadata and
        //   provenance.
        // * The output borrow inherits the input's lifetime and shared-ness,
        //   and `HexSlice` has no interior mutability.
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }

    /// Returns the inner byte slice.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Formats a byte slice as lower-case hex, ignoring width and fill.
///
/// This is used both for serialization (via `Display`) and as the
/// inner value in the hex types' `Debug` output (via `Debug`).
pub(crate) struct HexDisplay<'a>(pub(crate) &'a [u8]);

impl fmt::Display for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Writes hex bytes with padding/alignment support.
///
/// `write_byte` is called for each byte in the slice to allow
/// callers to choose between lowercase and uppercase hex.
fn fmt_hex_padded(
    bytes: &[u8],
    f: &mut fmt::Formatter<'_>,
    write_byte: fn(&mut fmt::Formatter<'_>, u8) -> fmt::Result,
) -> fmt::Result {
    let content_len = bytes.len() * 2;

    match f.width() {
        Some(width) if width > content_len => {
            let padding = width - content_len;
            let fill = f.fill();
            let (pre, post) = match f.align() {
                Some(fmt::Alignment::Left) => (0, padding),
                Some(fmt::Alignment::Right) | None => (padding, 0),
                Some(fmt::Alignment::Center) => {
                    (padding / 2, padding - padding / 2)
                }
            };
            for _ in 0..pre {
                f.write_char(fill)?;
            }
            for &byte in bytes {
                write_byte(f, byte)?;
            }
            for _ in 0..post {
                f.write_char(fill)?;
            }
            Ok(())
        }
        Some(_) | None => {
            for &byte in bytes {
                write_byte(f, byte)?;
            }
            Ok(())
        }
    }
}

impl fmt::Debug for HexSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HexSlice").field(&HexDisplay(&self.0)).finish()
    }
}

impl fmt::Display for HexSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::LowerHex for HexSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_hex_padded(&self.0, f, |f, b| write!(f, "{b:02x}"))
    }
}

impl fmt::UpperHex for HexSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_hex_padded(&self.0, f, |f, b| write!(f, "{b:02X}"))
    }
}

impl core::ops::Deref for HexSlice {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for HexSlice {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<HexSlice> for HexSlice {
    #[inline]
    fn as_ref(&self) -> &HexSlice {
        self
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::{HexDisplay, HexSlice};

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl serde_core::Serialize for HexSlice {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde_core::Serializer,
        {
            if serializer.is_human_readable() {
                serializer.collect_str(&HexDisplay(&self.0))
            } else {
                serializer.serialize_bytes(&self.0)
            }
        }
    }
}
