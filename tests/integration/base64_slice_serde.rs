// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::Base64Slice;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithBase64SliceCow {
    data: Cow<'static, Base64Slice>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct WithBase64SliceRef<'a> {
    data: &'a Base64Slice,
}

static FIXTURE: [u8; 16] = hex!("0123456789abcdef0123456789abcdef");
const FIXTURE_SLICE: &Base64Slice = Base64Slice::new(&FIXTURE);

static AS_JSON: &str = r#"{"data":"ASNFZ4mrze8BI0VniavN7w=="}"#;
static AS_CBOR: [u8; 23] =
    hex!("a1646461746150 0123456789abcdef0123456789abcdef");

#[test]
fn base64_slice_serialize() {
    let fixture = WithBase64SliceRef { data: FIXTURE_SLICE };
    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor).expect("serialized CBOR");
    assert_eq!(cbor, AS_CBOR);

    let empty =
        serde_json::to_string(Base64Slice::new(&[])).expect("serialized");
    assert_eq!(empty, r#""""#);
}

#[test]
fn base64_slice_cow_serde() {
    let borrowed = WithBase64SliceCow { data: Cow::Borrowed(FIXTURE_SLICE) };
    let json = serde_json::to_string(&borrowed).expect("serialized");
    assert_eq!(json, AS_JSON);

    let deserialized: WithBase64SliceCow =
        serde_json::from_str(AS_JSON).expect("deserialized Cow");
    assert!(
        matches!(deserialized.data, Cow::Owned(_)),
        "Cow<'static, Base64Slice> deserializes to the owned variant",
    );
    assert_eq!(deserialized, borrowed);

    let top_level: Cow<'static, Base64Slice> =
        serde_json::from_str(r#""AQID""#).expect("deserialized bare Cow");
    assert_eq!(top_level, Cow::Borrowed(Base64Slice::new(&[1, 2, 3])));

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&borrowed, &mut cbor).expect("serialized CBOR");
    assert_eq!(cbor, AS_CBOR);
    let roundtrip: WithBase64SliceCow =
        ciborium::de::from_reader(&cbor[..]).expect("deserialized CBOR");
    assert_eq!(roundtrip, borrowed);
}

#[test]
fn base64_slice_cow_rejects_invalid() {
    for (json, needle) in [
        (r#"{"data":"AQ!D"}"#, "Invalid symbol"),
        (r#"{"data":"A"}"#, "Invalid input length"),
    ] {
        let err = serde_json::from_str::<WithBase64SliceCow>(json)
            .expect_err("invalid base64 rejected for Cow");
        assert!(err.to_string().contains(needle), "{json}: got {err}");
    }
}
