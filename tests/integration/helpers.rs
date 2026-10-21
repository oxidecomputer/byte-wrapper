// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use hegel::{TestCase, generators as gs};

/// Draws a pair of byte vectors that are likely to be related to each other.
pub(crate) fn draw_related_bytes(tc: &TestCase) -> (Vec<u8>, Vec<u8>) {
    let a = tc.draw(gs::binary());
    let b = match tc.draw(gs::integers::<u8>().max_value(3)) {
        // The returned pair is equal.
        0 => a.clone(),
        1 => {
            // a is a prefix of b.
            let mut b = a.clone();
            b.extend(tc.draw(gs::binary()));
            b
        }
        2 if !a.is_empty() => {
            // a differs from b at a single byte.
            let mut b = a.clone();
            let index = tc.draw(gs::integers::<usize>().max_value(a.len() - 1));
            b[index] = b[index]
                .wrapping_add(tc.draw(gs::integers::<u8>().min_value(1)));
            b
        }
        // a and b are unrelated.
        _ => tc.draw(gs::binary()),
    };
    (a, b)
}
