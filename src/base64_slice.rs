// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`Base64Slice`] newtype wrapper.

use base64::{
    display::Base64Display,
    encoded_len,
    engine::general_purpose::{GeneralPurpose, STANDARD},
};
use core::fmt::{self, Write};

/// A byte slice that displays as base64.
///
/// The borrowed, dynamically-sized equivalent to
/// [`Base64Vec`](crate::Base64Vec), akin to `str`'s relationship to `String`.
///
/// `Base64Slice` wraps `[u8]`, providing a [`Display`](fmt::Display)
/// implementation that uses base64 encoding.
///
/// With the **`serde`** feature enabled, it also implements
/// `Serialize` (base64 strings in human-readable formats, raw bytes in
/// binary formats).
///
/// # Examples
///
/// ```
/// use byte_wrapper::Base64Slice;
///
/// const BYTES: [u8; 3] = [1, 2, 3];
/// const B: &Base64Slice = Base64Slice::new(&BYTES);
/// assert_eq!(B.to_string(), "AQID");
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Base64Slice([u8]);

impl Base64Slice {
    /// Creates a new `Base64Slice` from a byte slice.
    #[inline]
    #[must_use]
    pub const fn new(bytes: &[u8]) -> &Self {
        // SAFETY:
        //
        // * `repr(transparent)` gives `Base64Slice` the layout and validity of
        //   `[u8]`.
        // * An unsized-to-unsized pointer cast retains the length metadata and
        //   provenance.
        // * The output borrow inherits the input's lifetime and shared-ness,
        //   and `Base64Slice` has no interior mutability.
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }

    /// Returns the inner byte slice.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

fn base64_display(bytes: &[u8]) -> Base64Display<'_, 'static, GeneralPurpose> {
    Base64Display::new(bytes, &STANDARD)
}

/// Formats bytes as a quoted base64 string without allocating.
///
/// This matches `String`'s `Debug` impl, since base64 never needs escaping.
pub(crate) struct QuotedBase64<'a>(pub(crate) &'a [u8]);

impl fmt::Debug for QuotedBase64<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        fmt::Display::fmt(&base64_display(self.0), f)?;
        f.write_char('"')
    }
}

/// Forwards at most `remaining` bytes and drops the rest.
///
/// base64 is ASCII, so bytes and chars are the same.
struct Truncate<'a, 'b> {
    inner: &'a mut fmt::Formatter<'b>,
    remaining: usize,
}

impl Write for Truncate<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let take = s.len().min(self.remaining);
        self.remaining -= take;
        self.inner.write_str(&s[..take])
    }
}

impl fmt::Debug for Base64Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Base64Slice").field(&QuotedBase64(&self.0)).finish()
    }
}

impl fmt::Display for Base64Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the displayer by hand, matching how str computes width,
        // fill, etc. We can't use String because this displayer is available in
        // no-alloc contexts.
        let encoded_len = encoded_len(self.0.len(), true)
            .expect("base64 length of an in-memory slice fits in usize");
        let content_len = match f.precision() {
            Some(precision) => precision.min(encoded_len),
            None => encoded_len,
        };
        let padding = match f.width() {
            Some(width) => width.saturating_sub(content_len),
            None => 0,
        };
        let (pre, post) = match f.align() {
            Some(fmt::Alignment::Left) | None => (0, padding),
            Some(fmt::Alignment::Right) => (padding, 0),
            Some(fmt::Alignment::Center) => {
                (padding / 2, padding - padding / 2)
            }
        };

        let fill = f.fill();
        for _ in 0..pre {
            f.write_char(fill)?;
        }
        if content_len == encoded_len {
            fmt::Display::fmt(&base64_display(&self.0), f)?;
        } else {
            let mut truncate = Truncate { inner: f, remaining: content_len };
            write!(truncate, "{}", base64_display(&self.0))?;
        }
        for _ in 0..post {
            f.write_char(fill)?;
        }
        Ok(())
    }
}

impl core::ops::Deref for Base64Slice {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for Base64Slice {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<Base64Slice> for Base64Slice {
    #[inline]
    fn as_ref(&self) -> &Base64Slice {
        self
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::{Base64Slice, base64_display};

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl serde_core::Serialize for Base64Slice {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde_core::Serializer,
        {
            if serializer.is_human_readable() {
                serializer.collect_str(&base64_display(&self.0))
            } else {
                serializer.serialize_bytes(&self.0)
            }
        }
    }
}
