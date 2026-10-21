// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::{HexSlice, HexVec};
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithHexVecAttr {
    #[serde(with = "HexVec")]
    data: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithHexVecDirect {
    data: HexVec,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithHexSliceCow {
    data: Cow<'static, HexSlice>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct WithHexSliceRef<'a> {
    data: &'a HexSlice,
}

static FIXTURE: [u8; 16] = hex!("0123456789abcdef0123456789abcdef");
const FIXTURE_SLICE: &HexSlice = HexSlice::new(&FIXTURE);

static AS_JSON: &str = r#"{"data":"0123456789abcdef0123456789abcdef"}"#;
static AS_JSON_UPPER: &str = r#"{"data":"0123456789ABCDEF0123456789ABCDEF"}"#;
static AS_CBOR: [u8; 23] =
    hex!("a1646461746150 0123456789abcdef0123456789abcdef");

#[test]
fn hex_vec_serde_roundtrip() {
    let fixture = WithHexVecAttr { data: FIXTURE.to_vec() };

    let json =
        serde_json::to_string(&fixture).expect("serializing as JSON succeeded");
    assert_eq!(json, AS_JSON, "JSON matched");

    let json_roundtrip: WithHexVecAttr = serde_json::from_str(&json)
        .expect("JSON roundtrip deserialization succeeded");
    assert_eq!(fixture, json_roundtrip, "JSON roundtrip matched");

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor)
        .expect("serializing as CBOR succeeded");
    assert_eq!(cbor, AS_CBOR, "CBOR matched");

    let cbor_roundtrip: WithHexVecAttr = ciborium::de::from_reader(&cbor[..])
        .expect("CBOR roundtrip deserialization succeeded");
    assert_eq!(fixture, cbor_roundtrip, "CBOR roundtrip matched");
}

#[test]
fn hex_vec_direct() {
    let fixture = WithHexVecDirect { data: HexVec::new(FIXTURE.to_vec()) };

    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let roundtrip: WithHexVecDirect =
        serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor).expect("serialized CBOR");
    assert_eq!(cbor, AS_CBOR);
    let roundtrip: WithHexVecDirect =
        ciborium::de::from_reader(&cbor[..]).expect("deserialized CBOR");
    assert_eq!(fixture, roundtrip);
}

#[test]
fn hex_vec_serde_empty() {
    let fixture = WithHexVecDirect { data: HexVec::default() };
    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, r#"{"data":""}"#);
    let roundtrip: WithHexVecDirect =
        serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);

    let attr: WithHexVecAttr =
        serde_json::from_str(r#"{"data":""}"#).expect("deserialized attr");
    assert_eq!(attr, WithHexVecAttr { data: vec![] });
}

#[test]
fn hex_vec_serde_uppercase() {
    let direct: WithHexVecDirect =
        serde_json::from_str(AS_JSON_UPPER).expect("deserialized uppercase");
    assert_eq!(direct.data, FIXTURE_SLICE);
    assert_eq!(
        serde_json::to_string(&direct).expect("serialized"),
        AS_JSON,
        "serialization is lowercase",
    );

    let attr: WithHexVecAttr = serde_json::from_str(AS_JSON_UPPER)
        .expect("deserialized uppercase with attr");
    assert_eq!(attr.data, FIXTURE);
}

#[test]
fn hex_vec_serde_rejects_invalid() {
    for (json, needle) in [
        (r#"{"data":"012"}"#, "expected an even number of hex characters"),
        (r#"{"data":"0g"}"#, "invalid hex character 'g' at index 1"),
    ] {
        let err = serde_json::from_str::<WithHexVecDirect>(json)
            .expect_err("invalid hex rejected");
        assert!(err.to_string().contains(needle), "{json}: got {err}");

        let err = serde_json::from_str::<WithHexVecAttr>(json)
            .expect_err("invalid hex rejected with attr");
        assert!(err.to_string().contains(needle), "{json}: got {err}");

        let err = serde_json::from_str::<WithHexSliceCow>(json)
            .expect_err("invalid hex rejected for Cow");
        assert!(err.to_string().contains(needle), "{json}: got {err}");
    }
}

#[test]
fn hex_vec_json_array_rejected() {
    let json = r#"{"data":[1,2,3,4]}"#;

    let err = serde_json::from_str::<WithHexVecAttr>(json)
        .expect_err("JSON array should not deserialize as HexVec");
    let msg = err.to_string();
    assert!(msg.contains("hex string"), "got: {msg}");

    let err = serde_json::from_str::<WithHexVecDirect>(json)
        .expect_err("JSON array should not deserialize as direct HexVec");
    let msg = err.to_string();
    assert!(msg.contains("hex string"), "got: {msg}");
}

#[test]
fn hex_vec_deserialize_from_seq() {
    let cbor_array = hex!("a1 6464617461 84 01020304");

    let direct: WithHexVecDirect = ciborium::de::from_reader(&cbor_array[..])
        .expect("deserialized HexVec from CBOR array");
    assert_eq!(direct.data, HexVec::new(vec![1, 2, 3, 4]));

    let with_attr: WithHexVecAttr = ciborium::de::from_reader(&cbor_array[..])
        .expect("deserialized Vec<u8> with HexVec from CBOR array");
    assert_eq!(with_attr.data, [1, 2, 3, 4]);
}

#[test]
fn hex_slice_serialize() {
    let fixture = WithHexSliceRef { data: FIXTURE_SLICE };
    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor).expect("serialized CBOR");
    assert_eq!(cbor, AS_CBOR);

    let empty = serde_json::to_string(HexSlice::new(&[])).expect("serialized");
    assert_eq!(empty, r#""""#);
}

#[test]
fn hex_slice_cow_serde() {
    let borrowed = WithHexSliceCow { data: Cow::Borrowed(FIXTURE_SLICE) };
    let json = serde_json::to_string(&borrowed).expect("serialized");
    assert_eq!(json, AS_JSON);

    let deserialized: WithHexSliceCow =
        serde_json::from_str(AS_JSON_UPPER).expect("deserialized Cow");
    assert!(
        matches!(deserialized.data, Cow::Owned(_)),
        "Cow<'static, HexSlice> deserializes to the owned variant",
    );
    assert_eq!(deserialized, borrowed);

    let top_level: Cow<'static, HexSlice> =
        serde_json::from_str(r#""abcd""#).expect("deserialized bare Cow");
    assert_eq!(top_level, Cow::Borrowed(HexSlice::new(&[0xab, 0xcd])));

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&borrowed, &mut cbor).expect("serialized CBOR");
    assert_eq!(cbor, AS_CBOR);
    let roundtrip: WithHexSliceCow =
        ciborium::de::from_reader(&cbor[..]).expect("deserialized CBOR");
    assert_eq!(roundtrip, borrowed);
}

/// Tests that deserializing large byte buffers works with ciborium.
///
/// ciborium's deserialize_bytes only accepts definite-length byte strings that
/// fit its 4096-byte scratch buffer, so we must call `deserialize_byte_buf`
/// instead.
#[test]
fn hex_vec_cbor_large_and_indefinite_length() {
    let large: Vec<u8> = (0..=255).cycle().take(4097).collect();
    let attr = WithHexVecAttr { data: large.clone() };
    let direct = WithHexVecDirect { data: HexVec::new(large) };

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&attr, &mut cbor)
        .expect("serializing large value as CBOR succeeded");
    let attr_roundtrip: WithHexVecAttr = ciborium::de::from_reader(&cbor[..])
        .expect("large CBOR byte string deserialized via attr");
    assert_eq!(attr, attr_roundtrip);
    let direct_roundtrip: WithHexVecDirect =
        ciborium::de::from_reader(&cbor[..])
            .expect("large CBOR byte string deserialized directly");
    assert_eq!(direct, direct_roundtrip);

    // This is the CBOR encoding for {"data": (_ h'0102' h'03')}, an
    // indefinite-length byte string split into two chunks.
    let indefinite = hex!("a1 6464617461 5f 42 0102 41 03 ff");
    let attr: WithHexVecAttr = ciborium::de::from_reader(&indefinite[..])
        .expect("indefinite-length CBOR byte string deserialized via attr");
    assert_eq!(attr.data, [1, 2, 3]);
    let direct: WithHexVecDirect = ciborium::de::from_reader(&indefinite[..])
        .expect("indefinite-length CBOR byte string deserialized directly");
    assert_eq!(direct.data, HexVec::new(vec![1, 2, 3]));
}
