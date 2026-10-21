// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::{Base64Slice, Base64Vec};
use schemars08::{self as schemars, JsonSchema, schema_for};
use std::borrow::Cow;

#[test]
fn base64_vec_schema() {
    let schema = schema_for!(Base64Vec);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Base64Vec",
        "type": "string",
        "format": "byte",
        "contentEncoding": "base64",
        "x-rust-type": {
            "crate": "byte-wrapper",
            "version": "0.2.0",
            "path": "byte_wrapper::Base64Vec"
        }
    });
    assert_eq!(actual, expected);

    let slice_schema = schema_for!(Base64Slice);
    assert_eq!(
        serde_json::to_value(&slice_schema).expect("serialized"),
        actual,
        "Base64Slice and Base64Vec share a schema",
    );
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithBase64SliceCow {
    data: Cow<'static, Base64Slice>,
}

#[test]
fn with_base64_slice_cow() {
    let actual = serde_json::to_value(schema_for!(WithBase64SliceCow))
        .expect("serialized");
    let direct = serde_json::to_value(schema_for!(WithBase64VecDirect))
        .expect("serialized");
    assert_eq!(
        actual["properties"], direct["properties"],
        "a Cow<'static, Base64Slice> field has the same schema as a Base64Vec field",
    );
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
                "contentEncoding": "base64",
                "x-rust-type": {
                    "crate": "byte-wrapper",
                    "version": "0.2.0",
                    "path": "byte_wrapper::Base64Vec"
                }
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
                "contentEncoding": "base64",
                "x-rust-type": {
                    "crate": "byte-wrapper",
                    "version": "0.2.0",
                    "path": "byte_wrapper::Base64Vec"
                }
            }
        }
    });
    assert_eq!(actual, expected);
}
