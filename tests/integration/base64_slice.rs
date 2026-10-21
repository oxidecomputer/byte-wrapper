// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::helpers::draw_related_bytes;
use byte_wrapper::{Base64Slice, Base64Vec};
use hegel::{TestCase, generators as gs};
use std::{
    borrow::{Borrow, Cow},
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, RandomState},
};

const BYTES: [u8; 3] = [1, 2, 3];
const CONST_SLICE: &Base64Slice = Base64Slice::new(&BYTES);
const CONST_EMPTY: &Base64Slice = Base64Slice::new(&[]);
const CONST_COW: Cow<'static, Base64Slice> =
    Cow::Borrowed(Base64Slice::new(&BYTES));
static STATIC_SLICE: &Base64Slice = Base64Slice::new(&BYTES);

#[test]
fn base64_slice_new_preserves_bytes() {
    assert_eq!(CONST_SLICE.as_bytes(), &BYTES);
    assert_eq!(CONST_SLICE.len(), 3);
    assert_eq!(STATIC_SLICE.as_bytes(), &BYTES);

    assert_eq!(CONST_EMPTY.as_bytes(), &[] as &[u8]);
    assert!(CONST_EMPTY.is_empty());

    let backing = [0u8, 1, 2, 3, 4, 5];
    let middle = Base64Slice::new(&backing[2..5]);
    assert_eq!(middle.as_bytes(), &[2, 3, 4]);
    assert!(std::ptr::eq(middle.as_bytes(), &backing[2..5]));
}

#[test]
fn base64_slice_const_matches_parsed() {
    let parsed: Base64Vec = "AQID".parse().expect("parsed");
    assert_eq!(parsed, CONST_SLICE);
    assert_eq!(CONST_SLICE, parsed);
    assert_eq!(*parsed, *CONST_SLICE);
    assert_eq!(CONST_COW, Cow::<Base64Slice>::Owned(parsed));
}

#[test]
fn base64_slice_display() {
    assert_eq!(format!("{CONST_SLICE}"), "AQID");
    assert_eq!(format!("{CONST_EMPTY}"), "");

    let v = Base64Vec::new(vec![1, 2, 3, 4]);
    assert_eq!(format!("{v}"), "AQIDBA==");
    assert_eq!(format!("{}", &*v), "AQIDBA==");

    // Display honors width and alignment.
    assert_eq!(format!("{CONST_SLICE:>6}"), "  AQID");
    assert_eq!(format!("{CONST_SLICE:<6}"), "AQID  ");
    assert_eq!(format!("{v:_^12}"), "__AQIDBA==__");
}

#[test]
fn base64_slice_debug() {
    assert_eq!(format!("{CONST_SLICE:?}"), r#"Base64Slice("AQID")"#);
    assert_eq!(format!("{CONST_EMPTY:?}"), r#"Base64Slice("")"#);
    assert_eq!(
        format!("{:?}", Base64Vec::new(BYTES.to_vec())),
        r#"Base64Vec("AQID")"#,
    );
}

#[test]
fn base64_slice_conversions() {
    let v = Base64Vec::from(vec![1, 2, 3]);
    let slice: &Base64Slice = &v;
    assert_eq!(slice.as_bytes(), &[1, 2, 3]);
    assert_eq!(AsRef::<[u8]>::as_ref(&v), &[1, 2, 3]);
    assert_eq!(AsRef::<Base64Slice>::as_ref(&v), slice);
    assert_eq!(AsRef::<[u8]>::as_ref(slice), &[1, 2, 3]);

    let owned: Base64Vec = slice.to_owned();
    assert_eq!(owned, v);

    let mut v = v;
    v.as_mut()[0] = 0xff;
    v.0.push(4);
    assert_eq!(v.as_bytes(), &[0xff, 2, 3, 4]);
    assert_eq!(Vec::from(v.clone()), vec![0xff, 2, 3, 4]);
    assert_eq!(v.into_inner(), vec![0xff, 2, 3, 4]);
}

#[test]
fn base64_slice_cow() {
    let borrowed: Cow<'static, Base64Slice> = Cow::Borrowed(CONST_SLICE);
    let owned: Cow<'static, Base64Slice> =
        Cow::Owned("AQID".parse().expect("parsed"));
    assert_eq!(borrowed, owned);
    assert_eq!(borrowed.to_string(), owned.to_string());
    assert_eq!(borrowed.clone().into_owned(), owned.clone().into_owned());

    let other: Cow<'static, Base64Slice> = Cow::Owned(Base64Vec::new(vec![1]));
    assert_ne!(borrowed, other);
}

#[test]
fn base64_slice_map_lookup() {
    // `Base64Slice` lets maps keyed by `Base64Vec` be queried with
    // `&Base64Slice`.
    let mut map = HashMap::new();
    map.insert(Base64Vec::new(BYTES.to_vec()), "three");
    map.insert(Base64Vec::default(), "empty");
    assert_eq!(map.get(CONST_SLICE), Some(&"three"));
    assert_eq!(map.get(CONST_EMPTY), Some(&"empty"));
    assert_eq!(map.get(Base64Slice::new(&[1])), None);

    let mut btree = BTreeMap::new();
    btree.insert(Base64Vec::new(vec![2]), 2);
    btree.insert(Base64Vec::new(vec![1, 0xff]), 1);
    btree.insert(Base64Vec::new(vec![]), 0);
    assert_eq!(btree.get(Base64Slice::new(&[1, 0xff])), Some(&1));
    let keys: Vec<String> = btree.keys().map(|k| k.to_string()).collect();
    assert_eq!(keys, ["", "Af8=", "Ag=="]);
}

fn draw_pair(tc: &TestCase) -> (Base64Vec, Base64Vec) {
    let (a, b) = draw_related_bytes(tc);
    (Base64Vec::new(a), Base64Vec::new(b))
}

#[hegel::test]
fn borrow_preserves_eq(tc: TestCase) {
    let (a, b) = draw_pair(&tc);
    let (a_slice, b_slice): (&Base64Slice, &Base64Slice) =
        (a.borrow(), b.borrow());
    assert_eq!(a == b, a_slice == b_slice);
}

#[hegel::test]
fn borrow_preserves_ord(tc: TestCase) {
    let (a, b) = draw_pair(&tc);
    let (a_slice, b_slice): (&Base64Slice, &Base64Slice) =
        (a.borrow(), b.borrow());
    assert_eq!(a.cmp(&b), a_slice.cmp(b_slice));
}

#[hegel::test]
fn borrow_preserves_hash(tc: TestCase) {
    let v = Base64Vec::new(tc.draw(gs::binary()));
    let slice: &Base64Slice = v.borrow();
    let state = RandomState::new();
    assert_eq!(state.hash_one(&v), state.hash_one(slice));
}

#[hegel::test]
fn to_owned_round_trips(tc: TestCase) {
    let bytes = tc.draw(gs::binary());
    let slice = Base64Slice::new(&bytes);
    let owned: Base64Vec = slice.to_owned();
    let reborrowed: &Base64Slice = owned.borrow();
    assert_eq!(reborrowed, slice);
    assert_eq!(owned.as_bytes(), bytes.as_slice());
}

#[hegel::test]
fn owned_and_borrowed_display_agree(tc: TestCase) {
    let v = Base64Vec::new(tc.draw(gs::binary()));
    let slice: &Base64Slice = v.borrow();
    assert_eq!(v.to_string(), slice.to_string());
}

#[hegel::test]
fn from_str_inverts_display(tc: TestCase) {
    let v = Base64Vec::new(tc.draw(gs::binary()));
    let parsed: Base64Vec =
        v.to_string().parse().expect("Display output parses");
    assert_eq!(parsed, v);
}

#[hegel::test]
fn from_str_rejects_non_base64_character(tc: TestCase) {
    let v = Base64Vec::new(tc.draw(gs::binary()));
    let mut encoded = v.to_string();
    let index = tc.draw(gs::integers::<usize>().max_value(encoded.len()));
    let invalid =
        tc.draw(gs::sampled_from(vec!['!', ' ', '-', '_', '.', '\n']));
    encoded.insert(index, invalid);
    encoded
        .parse::<Base64Vec>()
        .expect_err("a non-base64 character is rejected");
}
