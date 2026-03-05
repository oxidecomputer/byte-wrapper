// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::{HexArray, ParseHexError};

// -- FromStr tests --

#[test]
fn hex_from_str() {
    let h: HexArray<4> = "01020304".parse().expect("parsed");
    assert_eq!(h, HexArray::new([0x01, 0x02, 0x03, 0x04]));

    // Uppercase input.
    let h: HexArray<2> = "ABCD".parse().expect("parsed uppercase");
    assert_eq!(h, HexArray::new([0xab, 0xcd]));

    // Mixed case.
    let h: HexArray<2> = "aBcD".parse().expect("parsed mixed case");
    assert_eq!(h, HexArray::new([0xab, 0xcd]));

    // Empty.
    let h: HexArray<0> = "".parse().expect("parsed empty");
    assert_eq!(h, HexArray::new([]));
}

#[test]
fn hex_from_str_wrong_length() {
    let err = "0102".parse::<HexArray<4>>().expect_err("too short");
    assert_eq!(err, ParseHexError::InvalidLength { expected: 8, actual: 4 },);
    assert!(
        err.to_string().contains("expected 8 hex characters, got 4"),
        "got: {err}",
    );

    let err = "010203040506".parse::<HexArray<4>>().expect_err("too long");
    assert_eq!(err, ParseHexError::InvalidLength { expected: 8, actual: 12 },);
}

#[test]
fn hex_from_str_invalid_chars() {
    let err = "0g020304".parse::<HexArray<4>>().expect_err("invalid hex char");
    assert_eq!(err, ParseHexError::InvalidHexCharacter { c: 'g', index: 1 },);
    assert!(err.to_string().contains("'g'"), "got: {err}",);
}

// -- TryFrom<&[u8]> tests --

#[test]
fn hex_try_from_slice() {
    let slice: &[u8] = &[0x01, 0x02, 0x03, 0x04];
    let h = HexArray::<4>::try_from(slice).expect("correct length");
    assert_eq!(h, HexArray::new([0x01, 0x02, 0x03, 0x04]));

    // Empty.
    let h = HexArray::<0>::try_from([].as_slice()).expect("empty");
    assert_eq!(h, HexArray::new([]));

    // Too short.
    HexArray::<4>::try_from([0x01, 0x02].as_slice()).expect_err("too short");

    // Too long.
    HexArray::<2>::try_from([0x01, 0x02, 0x03].as_slice())
        .expect_err("too long");
}

// -- Display and Debug formatting tests --

#[test]
fn hex_display() {
    let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
    assert_eq!(format!("{h}"), "0102abff");

    // Empty array.
    let empty = HexArray::new([]);
    assert_eq!(format!("{empty}"), "");

    let h = HexArray::new([0xab, 0xcd]);

    // Right alignment (the default).
    assert_eq!(format!("{h:>10}"), "      abcd");
    assert_eq!(format!("{h:10}"), "      abcd");

    // Left alignment.
    assert_eq!(format!("{h:<10}"), "abcd      ");

    // Center alignment (even and odd padding).
    assert_eq!(format!("{h:^10}"), "   abcd   ");
    assert_eq!(format!("{h:^9}"), "  abcd   ");

    // Custom fill character.
    assert_eq!(format!("{h:_>10}"), "______abcd");
    assert_eq!(format!("{h:_<10}"), "abcd______");
    assert_eq!(format!("{h:_^10}"), "___abcd___");

    // Width smaller than or equal to content: no truncation.
    assert_eq!(format!("{h:2}"), "abcd");
    assert_eq!(format!("{h:4}"), "abcd");
}

#[test]
fn hex_debug() {
    let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
    assert_eq!(format!("{h:?}"), "HexArray(0102abff)");

    // Empty array.
    let empty = HexArray::new([]);
    assert_eq!(format!("{empty:?}"), "HexArray()");

    // Alternate flag.
    let h = HexArray::new([0xab, 0xcd]);
    assert_eq!(format!("{h:#?}"), "HexArray(\n    abcd,\n)");
}

#[test]
fn hex_lower_hex() {
    let h = HexArray::new([0x01, 0xAB, 0xFF]);
    assert_eq!(format!("{h:x}"), "01abff");

    // With padding.
    assert_eq!(format!("{h:>10x}"), "    01abff");
    assert_eq!(format!("{h:<10x}"), "01abff    ");
    assert_eq!(format!("{h:_^10x}"), "__01abff__");

    // Empty.
    let empty = HexArray::new([]);
    assert_eq!(format!("{empty:x}"), "");
}

#[test]
fn hex_upper_hex() {
    let h = HexArray::new([0x01, 0xab, 0xff]);
    assert_eq!(format!("{h:X}"), "01ABFF");

    // With padding.
    assert_eq!(format!("{h:>10X}"), "    01ABFF");
    assert_eq!(format!("{h:<10X}"), "01ABFF    ");
    assert_eq!(format!("{h:_^10X}"), "__01ABFF__");

    // Empty.
    let empty = HexArray::new([]);
    assert_eq!(format!("{empty:X}"), "");
}
