// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: Apache-2.0

use hex_literal::hex;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize)]
#[cfg_attr(feature = "alloc", derive(serde::Serialize))]
struct MyStruct {
    #[serde(deserialize_with = "serde_bytefmt::hex_array::deserialize")]
    #[cfg_attr(
        feature = "alloc",
        serde(serialize_with = "serde_bytefmt::hex_array::serialize")
    )]
    x: [u8; 16],
}

/// Test that `HexArray` works with `#[serde(with = "...")]`.
#[cfg(feature = "alloc")]
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct WithHexArrayAttr {
    #[serde(with = "serde_bytefmt::HexArray::<16>")]
    x: [u8; 16],
}

/// Test using `HexArray` directly as a field type.
#[cfg(feature = "alloc")]
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct WithHexArrayDirect {
    x: serde_bytefmt::HexArray<16>,
}

static FIXTURE: MyStruct = MyStruct {
    x: hex!("0123456789abcdef0123456789abcdef"),
};

static AS_JSON: &str = r#"{"x":"0123456789abcdef0123456789abcdef"}"#;
static AS_CBOR: [u8; 20] = hex!("a16178500123456789abcdef0123456789abcdef");

#[cfg(feature = "alloc")]
#[test]
fn hex_serialize() {
    assert_eq!(
        serde_json::to_string(&FIXTURE).expect("serializing as JSON succeeded"),
        AS_JSON,
        "JSON matched",
    );
    let mut cbor_actual: Vec<u8> = Vec::new();
    ciborium::ser::into_writer(&FIXTURE, &mut cbor_actual).expect("writing to vec<u8> succeeded");

    assert_eq!(cbor_actual, AS_CBOR, "CBOR matched");
}

#[test]
fn hex_deserialize() {
    let json_actual: MyStruct =
        serde_json::from_str(AS_JSON).expect("deserializing from JSON succeeded");
    assert_eq!(FIXTURE, json_actual, "deserializing from JSON matched");

    let cbor_actual: MyStruct =
        ciborium::de::from_reader(&AS_CBOR[..]).expect("deserializing from CBOR succeeded");
    assert_eq!(FIXTURE, cbor_actual, "deserializing from CBOR succeeded");
}

#[cfg(feature = "alloc")]
#[test]
fn hex_array_with_attr() {
    let fixture = WithHexArrayAttr {
        x: hex!("0123456789abcdef0123456789abcdef"),
    };

    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let roundtrip: WithHexArrayAttr = serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);
}

#[cfg(feature = "alloc")]
#[test]
fn hex_array_direct() {
    let fixture = WithHexArrayDirect {
        x: serde_bytefmt::HexArray::new(hex!("0123456789abcdef0123456789abcdef")),
    };

    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let roundtrip: WithHexArrayDirect = serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);
}
