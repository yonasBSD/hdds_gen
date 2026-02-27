// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Feed raw bytes to the lexer - should never panic
    if let Ok(s) = std::str::from_utf8(data) {
        let mut lexer = hddsgen::parser::Parser::new(s);
        // Just tokenize, ignore errors
        let _ = lexer.parse();
    }
});
