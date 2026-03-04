// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: Apache-2.0

use schemars08::{self as schemars, JsonSchema, schema_for};
use serde_bytefmt::{Base64Vec, HexArray};

#[test]
fn hex_array_schema() {
    let schema = schema_for!(HexArray<16>);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "HexArray_16",
        "type": "string",
        "maxLength": 32,
        "minLength": 32,
        "pattern": "^[0-9a-fA-F]{32}$"
    });
    assert_eq!(actual, expected);
}

#[test]
fn base64_vec_schema() {
    let schema = schema_for!(Base64Vec);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Base64Vec",
        "type": "string",
        "format": "byte",
        "contentEncoding": "base64"
    });
    assert_eq!(actual, expected);
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithHexArrayAttr {
    #[schemars(with = "HexArray<16>")]
    x: [u8; 16],
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithHexArrayDirect {
    x: HexArray<16>,
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithBase64VecAttr {
    #[schemars(with = "Base64Vec")]
    data: Vec<u8>,
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithBase64VecDirect {
    data: Base64Vec,
}

#[test]
fn with_hex_array_attr() {
    let schema = schema_for!(WithHexArrayAttr);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "WithHexArrayAttr",
        "type": "object",
        "required": ["x"],
        "properties": {
            "x": {
                "type": "string",
                "maxLength": 32,
                "minLength": 32,
                "pattern": "^[0-9a-fA-F]{32}$"
            }
        }
    });
    assert_eq!(actual, expected);
}

#[test]
fn with_hex_array_direct() {
    let schema = schema_for!(WithHexArrayDirect);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "WithHexArrayDirect",
        "type": "object",
        "required": ["x"],
        "properties": {
            "x": {
                "type": "string",
                "maxLength": 32,
                "minLength": 32,
                "pattern": "^[0-9a-fA-F]{32}$"
            }
        }
    });
    assert_eq!(actual, expected);
}

#[test]
fn with_base64_vec_attr() {
    let schema = schema_for!(WithBase64VecAttr);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "WithBase64VecAttr",
        "type": "object",
        "required": ["data"],
        "properties": {
            "data": {
                "type": "string",
                "format": "byte",
                "contentEncoding": "base64"
            }
        }
    });
    assert_eq!(actual, expected);
}

#[test]
fn with_base64_vec_direct() {
    let schema = schema_for!(WithBase64VecDirect);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "WithBase64VecDirect",
        "type": "object",
        "required": ["data"],
        "properties": {
            "data": {
                "type": "string",
                "format": "byte",
                "contentEncoding": "base64"
            }
        }
    });
    assert_eq!(actual, expected);
}
