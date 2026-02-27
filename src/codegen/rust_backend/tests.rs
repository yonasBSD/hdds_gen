// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Unit tests for the Rust code generator.

#![allow(clippy::expect_used)]

use super::*;
use crate::ast::{
    BitfieldDecl, Bitset, Definition, Enum, EnumVariant, Field, Struct, Typedef, Union, UnionCase,
    UnionLabel,
};
use crate::types::{Annotation, IdlType, PrimitiveType};
use std::error::Error;

type TestResult<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn rust_bitset_generates_helpers() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut bs = Bitset::new("Reg");
    let f1 = BitfieldDecl::new(3, "mode");
    let mut f2 = BitfieldDecl::new(5, "value");
    f2.annotations.push(Annotation::Position(4));
    bs.add_field(f1);
    bs.add_field(f2);
    file.add_definition(Definition::Bitset(bs));

    let r#gen = RustGenerator::new();
    let code = r#gen.generate(&file)?;
    assert!(code.contains("pub struct Reg { pub bits: u64 }"));
    assert!(code.contains("impl Reg {"));
    assert!(code.contains("MODE_SHIFT"));
    assert!(code.contains("MODE_MASK"));
    assert!(code.contains("VALUE_SHIFT"));
    assert!(code.contains("VALUE_MASK"));
    assert!(code.contains("pub fn mode(&self) -> u64"));
    assert!(code.contains("pub fn set_mode(&mut self, value: u64)"));
    assert!(code.contains("pub fn with_mode(mut self, value: u64) -> Self"));
    assert!(code.contains("pub const fn zero() -> Self"));
    assert!(code.contains("pub const fn from_bits(bits: u64) -> Self"));
    assert!(code.contains("pub const fn bits(&self) -> u64"));
    Ok(())
}

#[test]
fn typedef_fixed_alias_rust() -> TestResult<()> {
    let mut file = IdlFile::new();
    let td = Typedef {
        name: "Currency".into(),
        base_type: IdlType::Primitive(PrimitiveType::Fixed {
            digits: 10,
            scale: 2,
        }),
        annotations: vec![],
    };
    file.add_definition(Definition::Typedef(td));

    let r#gen = RustGenerator::new();
    let code = r#gen.generate(&file)?;
    assert!(code.contains("pub struct Fixed<"));
    assert!(code.contains("pub type Currency = Fixed<10, 2>;"));
    assert!(code.contains("from_parts"));
    assert!(code.contains("impl<const D: u32, const S: u32> core::fmt::Display for Fixed"));
    Ok(())
}

#[test]
fn codegen_struct_inheritance_rust() -> TestResult<()> {
    let mut file = IdlFile::new();

    let mut base = Struct::new("Base");
    base.add_field(Field::new("id", IdlType::Primitive(PrimitiveType::Int32)));
    let mut derived = Struct::new("Derived");
    derived.base_struct = Some("Base".to_string());
    derived.add_field(Field::new(
        "name",
        IdlType::Primitive(PrimitiveType::String),
    ));

    file.add_definition(Definition::Struct(base));
    file.add_definition(Definition::Struct(derived));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;
    assert!(out.contains("pub struct Derived {"));
    assert!(out.contains("pub base: Base,"));
    assert!(out.contains("pub id: i32"));
    assert!(out.contains("pub name: String"));
    Ok(())
}

#[test]
fn union_with_default_generates_default_impl() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut union = Union::new("Choice", IdlType::Primitive(PrimitiveType::Int32));

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Default],
        field: Field::new("none", IdlType::Primitive(PrimitiveType::String)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("1".into())],
        field: Field::new("number", IdlType::Primitive(PrimitiveType::Int32)),
    });

    file.add_definition(Definition::Union(union));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    assert!(out.contains("impl Default for Choice"));
    assert!(out.contains("Self::None(String::new())"));
    Ok(())
}

#[test]
fn union_default_case_encodes_zero_discriminant() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut union = Union::new("Message", IdlType::Primitive(PrimitiveType::Int32));

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("1".into())],
        field: Field::new("text", IdlType::Primitive(PrimitiveType::String)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Default],
        field: Field::new("unknown", IdlType::Primitive(PrimitiveType::Int32)),
    });

    file.add_definition(Definition::Union(union));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    // The default case discriminant must encode as 0, not as the case index (1)
    assert!(
        out.contains("0 as i32"),
        "default case should encode discriminant as 0, got:\n{out}"
    );
    Ok(())
}

#[test]
fn union_default_case_avoids_collision_with_case_zero() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut union = Union::new("Event", IdlType::Primitive(PrimitiveType::Int32));

    // case 0 is explicitly used
    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("0".into())],
        field: Field::new("zero_val", IdlType::Primitive(PrimitiveType::Int32)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("1".into())],
        field: Field::new("one_val", IdlType::Primitive(PrimitiveType::Int32)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Default],
        field: Field::new("other", IdlType::Primitive(PrimitiveType::Octet)),
    });

    file.add_definition(Definition::Union(union));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    // Default must NOT use 0 (collision with case 0) or 1 (collision with case 1)
    // It should pick 2
    assert!(
        out.contains("2 as i32"),
        "default case should pick discriminant 2 to avoid collision with 0 and 1, got:\n{out}"
    );
    Ok(())
}

#[test]
fn union_default_case_bool_discriminant() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut union = Union::new("Toggle", IdlType::Primitive(PrimitiveType::Boolean));

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("TRUE".into())],
        field: Field::new("on_value", IdlType::Primitive(PrimitiveType::Int32)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Default],
        field: Field::new("off_value", IdlType::Primitive(PrimitiveType::Int32)),
    });

    file.add_definition(Definition::Union(union));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    // Boolean discriminant default should encode as "false"
    assert!(
        out.contains("false"),
        "boolean default case should encode discriminant as false, got:\n{out}"
    );
    Ok(())
}

#[test]
fn external_field_generates_box_type() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut s = Struct::new("Node");

    s.add_field(Field::new("id", IdlType::Primitive(PrimitiveType::Int32)));
    s.add_field(
        Field::new("child", IdlType::Named("Node".into())).with_annotation(Annotation::External),
    );
    s.add_field(
        Field::new("label", IdlType::Primitive(PrimitiveType::String))
            .with_annotation(Annotation::External)
            .with_annotation(Annotation::Optional),
    );

    file.add_definition(Definition::Struct(s));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    // @external field -> Box<T>
    assert!(
        out.contains("pub child: Box<Node>"),
        "external field should be Box<T>, got:\n{out}"
    );

    // @external + @optional -> Option<Box<T>>
    assert!(
        out.contains("pub label: Option<Box<String>>"),
        "external+optional field should be Option<Box<T>>, got:\n{out}"
    );

    // Decoder must wrap in Box::new()
    assert!(
        out.contains("Box::new(child"),
        "decoder should wrap external field in Box::new(), got:\n{out}"
    );

    // Builder should accept T and wrap in Box::new in build()
    assert!(
        out.contains("Box::new(self.child"),
        "builder should wrap external field in Box::new(), got:\n{out}"
    );

    Ok(())
}

#[test]
fn external_sequence_generates_box_vec() -> TestResult<()> {
    let mut file = IdlFile::new();
    let mut s = Struct::new("Container");

    s.add_field(
        Field::new(
            "items",
            IdlType::Sequence {
                inner: std::boxed::Box::new(IdlType::Primitive(PrimitiveType::Int32)),
                bound: None,
            },
        )
        .with_annotation(Annotation::External),
    );

    file.add_definition(Definition::Struct(s));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    assert!(
        out.contains("pub items: Box<Vec<i32>>"),
        "external sequence should be Box<Vec<T>>, got:\n{out}"
    );

    Ok(())
}

#[test]
fn union_default_case_enum_discriminant() -> TestResult<()> {
    let mut file = IdlFile::new();

    // Define enum Mode { X, Y }
    let mut mode_enum = Enum::new("Mode");
    mode_enum.add_variant(EnumVariant {
        name: "X".into(),
        value: None,
    });
    mode_enum.add_variant(EnumVariant {
        name: "Y".into(),
        value: None,
    });
    file.add_definition(Definition::Enum(mode_enum));

    // Union switch(Mode) with case X, case Y, default
    let mut union = Union::new("U2", IdlType::Named("Mode".into()));

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("X".into())],
        field: Field::new("x_val", IdlType::Primitive(PrimitiveType::Int32)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Value("Y".into())],
        field: Field::new("y_val", IdlType::Primitive(PrimitiveType::Int32)),
    });

    union.add_case(UnionCase {
        labels: vec![UnionLabel::Default],
        field: Field::new("other", IdlType::Primitive(PrimitiveType::Octet)),
    });

    file.add_definition(Definition::Union(union));

    let r#gen = RustGenerator::new();
    let out = r#gen.generate(&file)?;

    // X=0, Y=1, so default must pick 2 to avoid collision
    assert!(
        out.contains("2 as i32"),
        "enum default case should pick discriminant 2 to avoid collision with X(0) and Y(1), got:\n{out}"
    );
    Ok(())
}
