// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::{Base64Vec, ParseBase64Error};

// -- FromStr tests --

#[test]
fn base64_from_str() {
    let b: Base64Vec = "AQID".parse().expect("parsed");
    assert_eq!(*b, [1, 2, 3]);

    // With padding.
    let b: Base64Vec = "AQIDBA==".parse().expect("parsed padded");
    assert_eq!(*b, [1, 2, 3, 4]);

    // Empty.
    let b: Base64Vec = "".parse().expect("parsed empty");
    assert!(b.is_empty());
}

#[test]
fn base64_from_str_invalid_byte() {
    let err = "AQ!D".parse::<Base64Vec>().expect_err("invalid byte");
    assert_eq!(
        err,
        ParseBase64Error::InvalidByte { offset: 2, byte: b'!' },
        "got: {err}",
    );
    assert!(err.to_string().contains("offset 2"), "got: {err}",);
}

#[test]
fn base64_from_str_invalid_length() {
    // A single base64 character is not a valid encoding.
    let err = "A".parse::<Base64Vec>().expect_err("invalid length");
    assert_eq!(err, ParseBase64Error::InvalidLength { length: 1 });
}

// -- Display and Debug formatting tests --

#[test]
fn base64_display() {
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b}"), "AQID");

    // With padding.
    let b = Base64Vec::new(vec![1, 2, 3, 4]);
    assert_eq!(format!("{b}"), "AQIDBA==");

    // Empty.
    let b = Base64Vec::new(vec![]);
    assert_eq!(format!("{b}"), "");

    // Width and alignment.
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b:>10}"), "      AQID");
    assert_eq!(format!("{b:<10}"), "AQID      ");
    assert_eq!(format!("{b:^10}"), "   AQID   ");
    assert_eq!(format!("{b:_>10}"), "______AQID");

    // Width smaller than content: no truncation.
    assert_eq!(format!("{b:2}"), "AQID");
}

#[test]
fn base64_debug() {
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b:?}"), r#"Base64Vec("AQID")"#);

    // Empty.
    let b = Base64Vec::new(vec![]);
    assert_eq!(format!("{b:?}"), r#"Base64Vec("")"#);

    // Alternate flag.
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b:#?}"), "Base64Vec(\n    \"AQID\",\n)",);
}
