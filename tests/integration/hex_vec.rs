// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::helpers::draw_related_bytes;
use byte_wrapper::{HexSlice, HexVec, ParseHexError};
use hegel::{TestCase, generators as gs};
use std::{
    borrow::{Borrow, Cow},
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, RandomState},
};

const BYTES: [u8; 4] = [0x01, 0x02, 0xab, 0xff];
const CONST_SLICE: &HexSlice = HexSlice::new(&BYTES);
const CONST_EMPTY: &HexSlice = HexSlice::new(&[]);
const CONST_COW: Cow<'static, HexSlice> = Cow::Borrowed(HexSlice::new(&BYTES));
static STATIC_SLICE: &HexSlice = HexSlice::new(&BYTES);

#[test]
fn hex_slice_new_preserves_bytes() {
    assert_eq!(CONST_SLICE.as_bytes(), &BYTES);
    assert_eq!(CONST_SLICE.len(), 4);
    assert_eq!(STATIC_SLICE.as_bytes(), &BYTES);

    assert_eq!(CONST_EMPTY.as_bytes(), &[] as &[u8]);
    assert!(CONST_EMPTY.is_empty());

    let backing = [0u8, 1, 2, 3, 4, 5];
    let middle = HexSlice::new(&backing[2..5]);
    assert_eq!(middle.as_bytes(), &[2, 3, 4]);
    assert!(std::ptr::eq(middle.as_bytes(), &backing[2..5]));
}

#[test]
fn hex_slice_const_matches_parsed() {
    let parsed: HexVec = "0102abff".parse().expect("parsed");
    assert_eq!(parsed, CONST_SLICE);
    assert_eq!(CONST_SLICE, parsed);
    assert_eq!(*parsed, *CONST_SLICE);
    assert_eq!(CONST_COW, Cow::<HexSlice>::Owned(parsed));
}

// -- FromStr tests --

#[test]
fn hex_vec_from_str() {
    let h: HexVec = "01020304".parse().expect("parsed");
    assert_eq!(h, HexVec::new(vec![0x01, 0x02, 0x03, 0x04]));

    // Uppercase input.
    let h: HexVec = "ABCD".parse().expect("parsed uppercase");
    assert_eq!(h, HexVec::new(vec![0xab, 0xcd]));
    assert_eq!(h.to_string(), "abcd");

    // Mixed case.
    let h: HexVec = "aBcD".parse().expect("parsed mixed case");
    assert_eq!(h, HexVec::new(vec![0xab, 0xcd]));

    // Empty.
    let h: HexVec = "".parse().expect("parsed empty");
    assert_eq!(h, HexVec::new(vec![]));
    assert!(h.is_empty());
}

#[test]
fn hex_vec_from_str_roundtrip() {
    for bytes in
        [vec![], vec![0x00], vec![0xff, 0x00, 0x7f], (0..=255).collect()]
    {
        let h = HexVec::new(bytes);
        let s = h.to_string();
        assert_eq!(s.len(), h.len() * 2);
        let parsed: HexVec = s.parse().expect("parsed Display output");
        assert_eq!(parsed, h, "roundtrip of {s}");
    }
}

#[test]
fn hex_vec_from_str_odd_length() {
    for (input, actual) in [("0", 1), ("012", 3), ("0g1", 3)] {
        let err = input.parse::<HexVec>().expect_err("odd length");
        assert_eq!(err, ParseHexError::OddLength { actual }, "input {input}");
    }

    let err = "012".parse::<HexVec>().expect_err("odd length");
    assert_eq!(
        err.to_string(),
        "expected an even number of hex characters, got 3",
    );
}

#[test]
fn hex_vec_from_str_invalid_chars() {
    let err = "0g020304".parse::<HexVec>().expect_err("invalid hex char");
    assert_eq!(err, ParseHexError::InvalidHexCharacter { c: 'g', index: 1 });
    assert!(err.to_string().contains("'g'"), "got: {err}");

    let err = "01 2".parse::<HexVec>().expect_err("space");
    assert_eq!(err, ParseHexError::InvalidHexCharacter { c: ' ', index: 2 });

    let err = "0x01".parse::<HexVec>().expect_err("0x prefix");
    assert_eq!(err, ParseHexError::InvalidHexCharacter { c: 'x', index: 1 });
}

// -- Display and Debug formatting tests --

#[test]
fn hex_vec_display() {
    let h = HexVec::new(vec![0x01, 0x02, 0xab, 0xff]);
    assert_eq!(format!("{h}"), "0102abff");
    assert_eq!(format!("{}", &*h), "0102abff");
    assert_eq!(format!("{CONST_SLICE}"), "0102abff");

    // Empty.
    let empty = HexVec::default();
    assert_eq!(format!("{empty}"), "");
    assert_eq!(format!("{CONST_EMPTY}"), "");

    let h = HexVec::new(vec![0xab, 0xcd]);
    let s = HexSlice::new(&[0xab, 0xcd]);

    // Right alignment (the default).
    assert_eq!(format!("{h:>10}"), "      abcd");
    assert_eq!(format!("{s:10}"), "      abcd");

    // Left alignment.
    assert_eq!(format!("{h:<10}"), "abcd      ");
    assert_eq!(format!("{s:<10}"), "abcd      ");

    // Center alignment (even and odd padding).
    assert_eq!(format!("{h:^10}"), "   abcd   ");
    assert_eq!(format!("{s:^9}"), "  abcd   ");

    // Custom fill character.
    assert_eq!(format!("{h:_>10}"), "______abcd");

    // Width smaller than or equal to content: no truncation.
    assert_eq!(format!("{h:2}"), "abcd");
    assert_eq!(format!("{s:4}"), "abcd");
}

#[test]
fn hex_vec_lower_upper_hex() {
    let h = HexVec::new(vec![0x01, 0xab, 0xff]);
    assert_eq!(format!("{h:x}"), "01abff");
    assert_eq!(format!("{h:X}"), "01ABFF");
    assert_eq!(format!("{h:_^10X}"), "__01ABFF__");
    assert_eq!(format!("{:x}", &*h), "01abff");
    assert_eq!(format!("{:>10X}", &*h), "    01ABFF");
}

#[test]
fn hex_vec_debug() {
    let h = HexVec::new(vec![0x01, 0x02, 0xab, 0xff]);
    assert_eq!(format!("{h:?}"), "HexVec(0102abff)");
    assert_eq!(format!("{CONST_SLICE:?}"), "HexSlice(0102abff)");

    // Empty.
    assert_eq!(format!("{:?}", HexVec::default()), "HexVec()");
    assert_eq!(format!("{CONST_EMPTY:?}"), "HexSlice()");

    // Alternate flag.
    let h = HexVec::new(vec![0xab, 0xcd]);
    assert_eq!(format!("{h:#?}"), "HexVec(\n    abcd,\n)");
}

#[test]
fn hex_vec_conversions() {
    let h = HexVec::from(vec![1, 2, 3]);
    let slice: &HexSlice = &h;
    assert_eq!(slice.as_bytes(), &[1, 2, 3]);
    assert_eq!(AsRef::<[u8]>::as_ref(&h), &[1, 2, 3]);
    assert_eq!(AsRef::<HexSlice>::as_ref(&h), slice);
    assert_eq!(AsRef::<[u8]>::as_ref(slice), &[1, 2, 3]);

    let owned: HexVec = slice.to_owned();
    assert_eq!(owned, h);

    let mut h = h;
    h.as_mut()[0] = 0xff;
    h.0.push(4);
    assert_eq!(h.to_string(), "ff020304");
    assert_eq!(Vec::from(h.clone()), vec![0xff, 2, 3, 4]);
    assert_eq!(h.into_inner(), vec![0xff, 2, 3, 4]);
}

#[test]
fn hex_slice_cow() {
    let borrowed: Cow<'static, HexSlice> = Cow::Borrowed(CONST_SLICE);
    let owned: Cow<'static, HexSlice> =
        Cow::Owned("0102ABFF".parse().expect("parsed"));
    assert_eq!(borrowed, owned);
    assert_eq!(borrowed.to_string(), owned.to_string());
    assert_eq!(borrowed.clone().into_owned(), owned.clone().into_owned());

    let other: Cow<'static, HexSlice> = Cow::Owned(HexVec::new(vec![0x01]));
    assert_ne!(borrowed, other);
}

#[test]
fn hex_slice_map_lookup() {
    // `HexSlice` lets maps keyed by `HexVec` be queried with `&HexSlice`.
    let mut map = HashMap::new();
    map.insert(HexVec::new(BYTES.to_vec()), "four");
    map.insert(HexVec::default(), "empty");
    assert_eq!(map.get(CONST_SLICE), Some(&"four"));
    assert_eq!(map.get(CONST_EMPTY), Some(&"empty"));
    assert_eq!(map.get(HexSlice::new(&[0x01])), None);

    let mut btree = BTreeMap::new();
    btree.insert(HexVec::new(vec![0x02]), 2);
    btree.insert(HexVec::new(vec![0x01, 0xff]), 1);
    btree.insert(HexVec::new(vec![]), 0);
    assert_eq!(btree.get(HexSlice::new(&[0x01, 0xff])), Some(&1));
    let keys: Vec<String> = btree.keys().map(|k| k.to_string()).collect();
    assert_eq!(keys, ["", "01ff", "02"]);
}

fn draw_pair(tc: &TestCase) -> (HexVec, HexVec) {
    let (a, b) = draw_related_bytes(tc);
    (HexVec::new(a), HexVec::new(b))
}

#[hegel::test]
fn borrow_preserves_eq(tc: TestCase) {
    let (a, b) = draw_pair(&tc);
    let (a_slice, b_slice): (&HexSlice, &HexSlice) = (a.borrow(), b.borrow());
    assert_eq!(a == b, a_slice == b_slice);
}

#[hegel::test]
fn borrow_preserves_ord(tc: TestCase) {
    let (a, b) = draw_pair(&tc);
    let (a_slice, b_slice): (&HexSlice, &HexSlice) = (a.borrow(), b.borrow());
    assert_eq!(a.cmp(&b), a_slice.cmp(b_slice));
}

#[hegel::test]
fn borrow_preserves_hash(tc: TestCase) {
    let v = HexVec::new(tc.draw(gs::binary()));
    let slice: &HexSlice = v.borrow();
    let state = RandomState::new();
    assert_eq!(state.hash_one(&v), state.hash_one(slice));
}

#[hegel::test]
fn to_owned_round_trips(tc: TestCase) {
    let bytes = tc.draw(gs::binary());
    let slice = HexSlice::new(&bytes);
    let owned: HexVec = slice.to_owned();
    let reborrowed: &HexSlice = owned.borrow();
    assert_eq!(reborrowed, slice);
    assert_eq!(owned.as_bytes(), bytes.as_slice());
}

#[hegel::test]
fn owned_and_borrowed_formatting_agree(tc: TestCase) {
    let v = HexVec::new(tc.draw(gs::binary()));
    let slice: &HexSlice = v.borrow();
    assert_eq!(v.to_string(), slice.to_string());
    assert_eq!(format!("{v:x}"), format!("{slice:x}"));
    assert_eq!(format!("{v:X}"), format!("{slice:X}"));
}

#[hegel::test]
fn from_str_inverts_display(tc: TestCase) {
    let v = HexVec::new(tc.draw(gs::binary()));
    let parsed: HexVec = v.to_string().parse().expect("Display output parses");
    assert_eq!(parsed, v);
}

#[hegel::test]
fn from_str_accepts_uppercase(tc: TestCase) {
    let v = HexVec::new(tc.draw(gs::binary()));
    let parsed: HexVec =
        format!("{v:X}").parse().expect("UpperHex output parses");
    assert_eq!(parsed, v);
}

#[hegel::test]
fn from_str_rejects_odd_length(tc: TestCase) {
    let v = HexVec::new(tc.draw(gs::binary()));
    let mut encoded = v.to_string();
    encoded.push(tc.draw(gs::sampled_from(vec!['0', 'a', 'F'])));
    assert_eq!(
        encoded.parse::<HexVec>(),
        Err(ParseHexError::OddLength { actual: encoded.len() }),
    );
}

#[hegel::test]
fn from_str_rejects_non_hex_character(tc: TestCase) {
    let v = HexVec::new(tc.draw(gs::binary().min_size(1)));
    let encoded = v.to_string();
    let index = tc.draw(gs::integers::<usize>().max_value(encoded.len() - 1));
    let c = tc.draw(gs::sampled_from(vec!['g', 'G', 'x', ' ', '-', '+', '\n']));
    let mut corrupted = encoded.into_bytes();
    corrupted[index] = c as u8;
    let corrupted =
        String::from_utf8(corrupted).expect("ASCII stays valid UTF-8");
    assert_eq!(
        corrupted.parse::<HexVec>(),
        Err(ParseHexError::InvalidHexCharacter { c, index }),
    );
}
