// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::{HexSlice, HexVec};
use schemars08::{self as schemars, JsonSchema, schema_for};
use std::borrow::Cow;

fn hex_vec_property() -> serde_json::Value {
    serde_json::json!({
        "type": "string",
        "pattern": "^([0-9a-fA-F]{2})*$",
        "x-rust-type": {
            "crate": "byte-wrapper",
            "version": "0.1.0",
            "path": "byte_wrapper::HexVec"
        }
    })
}

#[test]
fn hex_vec_schema() {
    let schema = schema_for!(HexVec);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let mut expected = hex_vec_property();
    let object = expected.as_object_mut().expect("property is an object");
    object.insert(
        "$schema".into(),
        "http://json-schema.org/draft-07/schema#".into(),
    );
    object.insert("title".into(), "HexVec".into());
    assert_eq!(actual, expected);

    let slice_schema = schema_for!(HexSlice);
    assert_eq!(
        serde_json::to_value(&slice_schema).expect("serialized"),
        actual,
        "HexSlice and HexVec share a schema",
    );
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithHexVecAttr {
    #[schemars(with = "HexVec")]
    data: Vec<u8>,
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithHexVecDirect {
    data: HexVec,
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithHexSliceCow {
    data: Cow<'static, HexSlice>,
}

#[test]
fn with_hex_vec_fields() {
    for (title, schema) in [
        ("WithHexVecAttr", schema_for!(WithHexVecAttr)),
        ("WithHexVecDirect", schema_for!(WithHexVecDirect)),
        ("WithHexSliceCow", schema_for!(WithHexSliceCow)),
    ] {
        let actual = serde_json::to_value(&schema).expect("serialized");
        let expected = serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": title,
            "type": "object",
            "required": ["data"],
            "properties": {
                "data": hex_vec_property(),
            }
        });
        assert_eq!(actual, expected, "schema for {title}");
    }
}
