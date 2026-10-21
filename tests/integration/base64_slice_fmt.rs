// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for base64 slice formatting.
//!
//! These tests only need the `base64` feature, so they also cover scenarios
//! where the crate's `alloc` feature is disabled.

use byte_wrapper::Base64Slice;
use hegel::{TestCase, generators as gs};

const BYTES: [u8; 4] = [1, 2, 3, 4];
const SLICE: &Base64Slice = Base64Slice::new(&BYTES);

#[test]
fn display_matches_known_encoding() {
    assert_eq!(format!("{SLICE}"), "AQIDBA==");
    assert_eq!(format!("{}", Base64Slice::new(&[])), "");
    assert_eq!(format!("{}", Base64Slice::new(&[0xfb, 0xff])), "+/8=");
}

#[test]
fn display_honors_flags() {
    assert_eq!(format!("{SLICE:12}"), "AQIDBA==    ");
    assert_eq!(format!("{SLICE:>12}"), "    AQIDBA==");
    assert_eq!(format!("{SLICE:_^13}"), "__AQIDBA==___");
    assert_eq!(format!("{SLICE:4}"), "AQIDBA==");
    assert_eq!(format!("{SLICE:.3}"), "AQI");
    assert_eq!(format!("{SLICE:.20}"), "AQIDBA==");
    assert_eq!(format!("{SLICE:>6.3}"), "   AQI");
    assert_eq!(format!("{SLICE:.0}"), "");
}

#[test]
fn debug_quotes_encoding() {
    assert_eq!(format!("{SLICE:?}"), r#"Base64Slice("AQIDBA==")"#);
    assert_eq!(format!("{SLICE:#?}"), "Base64Slice(\n    \"AQIDBA==\",\n)",);
}

// Our hand-written Base64Slice Display must agree with str's Display for all
// possible parameters.
#[hegel::test]
fn display_flags_match_str(tc: TestCase) {
    let bytes = tc.draw(gs::binary());
    let slice = Base64Slice::new(&bytes);
    let encoded = slice.to_string();
    let w = tc.draw(gs::integers::<usize>().max_value(64));
    let p = tc.draw(gs::integers::<usize>().max_value(64));

    assert_eq!(format!("{slice:w$}"), format!("{encoded:w$}"));
    assert_eq!(format!("{slice:<w$}"), format!("{encoded:<w$}"));
    assert_eq!(format!("{slice:>w$}"), format!("{encoded:>w$}"));
    assert_eq!(format!("{slice:*^w$}"), format!("{encoded:*^w$}"));
    assert_eq!(format!("{slice:.p$}"), format!("{encoded:.p$}"));
    assert_eq!(format!("{slice:>w$.p$}"), format!("{encoded:>w$.p$}"));
    assert_eq!(format!("{slice:-^w$.p$}"), format!("{encoded:-^w$.p$}"));
}

mod reference {
    // Same name as the real type, so derived Debug output is directly
    // comparable.
    #[derive(Debug)]
    pub(crate) struct Base64Slice(
        #[expect(dead_code, reason = "only read by the derived Debug impl")]
        pub(crate) String,
    );
}

#[hegel::test]
fn debug_matches_string_debug(tc: TestCase) {
    let bytes = tc.draw(gs::binary());
    let slice = Base64Slice::new(&bytes);
    let expected = reference::Base64Slice(slice.to_string());
    assert_eq!(format!("{slice:?}"), format!("{expected:?}"));
    assert_eq!(format!("{slice:#?}"), format!("{expected:#?}"));
}
