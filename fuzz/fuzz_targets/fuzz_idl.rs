// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // Feed arbitrary UTF-8 strings as IDL input
    let mut parser = hddsgen::Parser::new(data);
    if let Ok(ast) = parser.parse() {
        let _ = hddsgen::validate(&ast);
        // If it parses, try pretty-printing roundtrip
        let pretty = hddsgen::idl_pretty(&ast);
        let mut parser2 = hddsgen::Parser::new(&pretty);
        let _ = parser2.parse();
    }
});
