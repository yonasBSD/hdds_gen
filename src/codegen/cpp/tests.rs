// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Unit tests for the C++ code generator.

#![allow(clippy::expect_used)]

use super::CppGenerator;
use crate::ast::{Definition, Field, IdlFile, Struct};
use crate::codegen::CodeGenerator;
use crate::types::{IdlType, PrimitiveType};

#[test]
fn cpp_emits_fixed_template_with_raw_storage() {
    let mut file = IdlFile::new();
    let mut s = Struct::new("Account");
    s.add_field(Field::new(
        "balance",
        IdlType::Primitive(PrimitiveType::Fixed {
            digits: 12,
            scale: 4,
        }),
    ));
    file.add_definition(Definition::Struct(s));

    let generator = CppGenerator::new();
    let code = generator.generate(&file).expect("generate C++ header");

    assert!(
        code.contains("struct Fixed"),
        "expected Fixed template to be emitted"
    );
    assert!(
        code.contains("std::int64_t high;"),
        "Fixed template should expose high word storage"
    );
    assert!(
        code.contains("std::uint64_t low;"),
        "Fixed template should expose low word storage"
    );
    assert!(
        code.contains("static_assert(sizeof(Fixed<1, 0>) == 16"),
        "Fixed template must guarantee 16 byte layout"
    );
    assert!(
        code.contains("Fixed<12, 4> balance = 0;"),
        "struct field should map to Fixed<digits, scale>"
    );
    assert!(
        code.contains("to_le_bytes"),
        "Fixed helper should expose to_le_bytes for Phase 8 encoding parity"
    );
    assert!(
        code.contains("from_le_bytes"),
        "Fixed helper should expose from_le_bytes for Phase 8 decoding parity"
    );
}

#[test]
fn cpp_struct_phase8_types_rendered() {
    let mut file = IdlFile::new();
    let mut s = Struct::new("Phase8");
    s.add_field(Field::new(
        "title",
        IdlType::Primitive(PrimitiveType::WString),
    ));
    s.add_field(Field::new(
        "glyphs",
        IdlType::Sequence {
            inner: Box::new(IdlType::Primitive(PrimitiveType::WChar)),
            bound: None,
        },
    ));
    s.add_field(Field::new(
        "dictionary",
        IdlType::Map {
            key: Box::new(IdlType::Primitive(PrimitiveType::WString)),
            value: Box::new(IdlType::Primitive(PrimitiveType::Fixed {
                digits: 10,
                scale: 2,
            })),
            bound: None,
        },
    ));
    file.add_definition(Definition::Struct(s));

    let generator = CppGenerator::new();
    let code = generator.generate(&file).expect("generate C++ header");

    assert!(
        code.contains("std::wstring title;"),
        "wstring fields should map to std::wstring"
    );
    assert!(
        code.contains("std::vector<wchar_t> glyphs;"),
        "sequence<wchar> should map to std::vector<wchar_t>"
    );
    assert!(
        code.contains("std::map<std::wstring, Fixed<10, 2>> dictionary;"),
        "map<wstring, fixed> should map to std::map<std::wstring, Fixed<10, 2>>"
    );
}
