// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Helper utilities for Rust code generation.
//!
//! Common functions for type mapping, formatting, and AST traversal.

#![allow(clippy::redundant_pub_crate)]

use super::RustGenerator;
use crate::ast::{Definition, Enum, Field, IdlFile, Struct};
use crate::types::{Annotation, AutoIdKind, IdlType, PrimitiveType};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

pub(crate) fn push_fmt(dst: &mut String, args: std::fmt::Arguments<'_>) {
    let _ = dst.write_fmt(args);
}

thread_local! {
    // @audit-ok: thread_local! guarantees thread-safety, RefCell is safe here
    static MUTABLE_TYPES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());

    /// Index of all type definitions (structs + enums) for XTypes inline resolution.
    /// Maps unqualified name -> (definition, optional module name).
    // @audit-ok: thread_local! RefCell is inherently thread-safe (one instance per thread)
    static TYPE_DEFS: RefCell<HashMap<String, (TypeDef, Option<String>)>> =
        RefCell::new(HashMap::new());
}

/// Lightweight clone of struct/enum AST nodes for `XTypes` inline `TypeObject` generation.
#[derive(Clone)]
pub(super) enum TypeDef {
    Struct(Struct),
    Enum(Enum),
}

pub(super) fn set_mutable_types(set: HashSet<String>) {
    MUTABLE_TYPES.with(|cell| *cell.borrow_mut() = set);
}

/// Populate the type definition index from the AST.
pub(super) fn set_type_defs(ast: &IdlFile) {
    fn walk(
        defs: &[Definition],
        module: Option<&str>,
        acc: &mut HashMap<String, (TypeDef, Option<String>)>,
    ) {
        for def in defs {
            match def {
                Definition::Struct(s) => {
                    acc.insert(
                        s.name.clone(),
                        (TypeDef::Struct(s.clone()), module.map(String::from)),
                    );
                }
                Definition::Enum(e) => {
                    acc.insert(
                        e.name.clone(),
                        (TypeDef::Enum(e.clone()), module.map(String::from)),
                    );
                }
                Definition::Module(m) => walk(&m.definitions, Some(&m.name), acc),
                _ => {}
            }
        }
    }
    let mut map = HashMap::new();
    walk(&ast.definitions, None, &mut map);
    TYPE_DEFS.with(|cell| *cell.borrow_mut() = map);
}

/// Look up a type definition by unqualified name.
pub(super) fn lookup_type_def(name: &str) -> Option<(TypeDef, Option<String>)> {
    TYPE_DEFS.with(|cell| cell.borrow().get(name).cloned())
}

pub(super) fn is_named_mutable(name: &str) -> bool {
    MUTABLE_TYPES.with(|cell| cell.borrow().contains(name))
}

pub(super) fn collect_mutable_types(ast: &IdlFile) -> HashSet<String> {
    fn walk(defs: &[Definition], acc: &mut HashSet<String>) {
        for def in defs {
            match def {
                Definition::Struct(s) => {
                    let is_mut = matches!(
                        s.extensibility,
                        Some(
                            crate::types::ExtensibilityKind::Mutable
                                | crate::types::ExtensibilityKind::Appendable
                        )
                    ) || s.annotations.iter().any(|a| {
                        matches!(
                            a,
                            Annotation::Extensibility(
                                crate::types::ExtensibilityKind::Mutable
                                    | crate::types::ExtensibilityKind::Appendable
                            ) | Annotation::Mutable
                                | Annotation::Appendable
                        )
                    });
                    if is_mut {
                        acc.insert(s.name.clone());
                    }
                }
                Definition::Module(m) => walk(&m.definitions, acc),
                _ => {}
            }
        }
    }

    let mut set = HashSet::new();
    walk(&ast.definitions, &mut set);
    set
}

pub(super) fn is_mutable_struct(s: &Struct) -> bool {
    s.extensibility.is_some()
        && matches!(
            s.extensibility,
            Some(crate::types::ExtensibilityKind::Mutable)
        )
        || s.annotations.iter().any(|a| {
            matches!(
                a,
                Annotation::Extensibility(crate::types::ExtensibilityKind::Mutable)
                    | Annotation::Mutable
            )
        })
}

/// Compact `PL_CDR2` mutable struct:
/// - Marked `@mutable` (or extensibility Mutable)
/// - All fields are non-optional primitives (no sequences/maps/arrays/named)
///
/// `FastDDS` encodes these as a flat sequence of `EMHEADER1` + payload without
/// an inner `DHEADER`. Sequences of such structs then add a `DHEADER` per element.
pub(super) fn is_compact_mutable_struct(s: &Struct) -> bool {
    if !is_mutable_struct(s) {
        return false;
    }

    s.fields
        .iter()
        .all(|f| !f.is_optional() && matches!(f.field_type, IdlType::Primitive(_)))
}

/// Check if a type is a bounded string (string<N> -> Sequence<Char, Some(N)>)
/// or bounded wstring (wstring<N> -> Sequence<`WChar`, Some(N)>)
pub(super) fn is_bounded_string(ty: &IdlType) -> bool {
    matches!(
        ty,
        IdlType::Sequence {
            inner,
            bound: Some(_),
        } if matches!(**inner, IdlType::Primitive(PrimitiveType::Char | PrimitiveType::WChar))
    )
}

impl RustGenerator {
    pub(super) fn type_to_rust(idl_type: &IdlType) -> String {
        match idl_type {
            IdlType::Primitive(p) => match p {
                PrimitiveType::Void => "()".to_string(),
                PrimitiveType::Boolean => "bool".to_string(),
                PrimitiveType::Char | PrimitiveType::WChar => "char".to_string(),
                PrimitiveType::Octet | PrimitiveType::UInt8 => "u8".to_string(),
                PrimitiveType::Short | PrimitiveType::Int16 => "i16".to_string(),
                PrimitiveType::UnsignedShort | PrimitiveType::UInt16 => "u16".to_string(),
                PrimitiveType::Long | PrimitiveType::Int32 => "i32".to_string(),
                PrimitiveType::UnsignedLong | PrimitiveType::UInt32 => "u32".to_string(),
                PrimitiveType::LongLong | PrimitiveType::Int64 => "i64".to_string(),
                PrimitiveType::UnsignedLongLong | PrimitiveType::UInt64 => "u64".to_string(),
                PrimitiveType::Float => "f32".to_string(),
                PrimitiveType::Double | PrimitiveType::LongDouble => "f64".to_string(),
                PrimitiveType::String | PrimitiveType::WString => "String".to_string(),
                PrimitiveType::Int8 => "i8".to_string(),
                PrimitiveType::Fixed { digits, scale } => format!("Fixed<{digits}, {scale}>"),
            },
            IdlType::Named(name) => name.clone(),
            IdlType::Sequence { inner, .. } => {
                // Check if this is a bounded string (string<N> -> sequence<char, N>)
                // or bounded wstring (wstring<N> -> sequence<wchar, N>)
                if matches!(
                    **inner,
                    IdlType::Primitive(PrimitiveType::Char | PrimitiveType::WChar)
                ) {
                    return "String".to_string();
                }
                let inner_type = Self::type_to_rust(inner);
                format!("Vec<{inner_type}>")
            }
            IdlType::Map { key, value, .. } => {
                let key = Self::type_to_rust(key);
                let value = Self::type_to_rust(value);
                format!("std::collections::HashMap<{key}, {value}>")
            }
            IdlType::Array { inner, size } => {
                let inner = Self::type_to_rust(inner);
                format!("[{inner}; {size}]")
            }
        }
    }

    /// Calculate CDR2 alignment for a given IDL type
    ///
    /// Returns alignment in bytes (1, 2, 4, or 8).
    /// Primitives align to their natural size, sequences/strings align to 4 (u32 prefix).
    pub(super) fn cdr2_alignment(idl_type: &IdlType) -> usize {
        match idl_type {
            IdlType::Primitive(p) => match p {
                PrimitiveType::Void
                | PrimitiveType::Octet
                | PrimitiveType::UInt8
                | PrimitiveType::Int8
                | PrimitiveType::Boolean
                | PrimitiveType::Char => 1,
                PrimitiveType::Short
                | PrimitiveType::UnsignedShort
                | PrimitiveType::Int16
                | PrimitiveType::UInt16 => 2,
                PrimitiveType::Long
                | PrimitiveType::UnsignedLong
                | PrimitiveType::Int32
                | PrimitiveType::UInt32
                | PrimitiveType::Float
                | PrimitiveType::WChar
                | PrimitiveType::String
                | PrimitiveType::WString
                | PrimitiveType::Fixed { .. } => 4,
                PrimitiveType::LongLong
                | PrimitiveType::UnsignedLongLong
                | PrimitiveType::Int64
                | PrimitiveType::UInt64
                | PrimitiveType::Double
                | PrimitiveType::LongDouble => 8,
            },
            IdlType::Array { inner, .. } => Self::cdr2_alignment(inner),
            IdlType::Sequence { .. } | IdlType::Map { .. } | IdlType::Named(_) => 4,
        }
    }

    /// Calculate fixed size for primitives (None for variable-size types)
    pub(super) fn cdr2_fixed_size(idl_type: &IdlType) -> Option<usize> {
        match idl_type {
            IdlType::Primitive(p) => match p {
                PrimitiveType::Octet
                | PrimitiveType::UInt8
                | PrimitiveType::Int8
                | PrimitiveType::Boolean
                | PrimitiveType::Char => Some(1),
                PrimitiveType::Short
                | PrimitiveType::UnsignedShort
                | PrimitiveType::Int16
                | PrimitiveType::UInt16 => Some(2),
                PrimitiveType::Long
                | PrimitiveType::UnsignedLong
                | PrimitiveType::Int32
                | PrimitiveType::UInt32
                | PrimitiveType::Float => Some(4),
                PrimitiveType::LongLong
                | PrimitiveType::UnsignedLongLong
                | PrimitiveType::Int64
                | PrimitiveType::UInt64
                | PrimitiveType::Double
                | PrimitiveType::LongDouble => Some(8),
                PrimitiveType::Fixed { .. } => Some(16),
                _ => None, // String, WString are variable size
            },
            IdlType::Array { inner, size } => {
                Self::cdr2_fixed_size(inner).map(|element_size| element_size * (*size as usize))
            }
            _ => None, // Sequences, Maps, Named types are variable size
        }
    }

    /// Check if a type is a single-byte type that can be directly memcpy'd
    /// (only u8, i8, bool - NOT floats, i32, etc.)
    pub(super) const fn is_byte_copyable(idl_type: &IdlType) -> bool {
        matches!(
            idl_type,
            IdlType::Primitive(
                PrimitiveType::Octet
                    | PrimitiveType::UInt8
                    | PrimitiveType::Int8
                    | PrimitiveType::Boolean
            )
        )
    }

    /// Check if a type is likely a primitive or primitive typedef (supports `to_le_bytes()`)
    /// Used for generating encoding code for arrays with named element types.
    pub(super) fn is_primitive_like(idl_type: &IdlType) -> Option<usize> {
        match idl_type {
            IdlType::Primitive(PrimitiveType::Fixed { .. }) => None,
            IdlType::Primitive(_) => Self::cdr2_fixed_size(idl_type),
            IdlType::Named(name) => match name.as_str() {
                "I8" | "Int8" | "U8" | "UInt8" | "Octet" => Some(1),
                "I16" | "Int16" | "Short" | "U16" | "UInt16" | "UShort" => Some(2),
                "I32" | "Int32" | "Long" | "U32" | "UInt32" | "ULong" | "F32" | "Float" => Some(4),
                "I64" | "Int64" | "LongLong" | "U64" | "UInt64" | "ULongLong" | "F64"
                | "Double" => Some(8),
                _ => None,
            },
            _ => None,
        }
    }

    /// Compute `MemberId` for mutable structs.
    ///
    /// Priority:
    /// - `@id` on the field
    /// - `@autoid(SEQUENTIAL)` on the struct -> use declaration order
    /// - default/`@autoid(HASH)` -> FNV-1a 32-bit on field name & `0x0FFF_FFFF`
    ///
    /// Algorithm: FNV-1a 32-bit (`XTypes` ss7.3.1.2.1.2), masked to 28 bits.
    pub(crate) fn compute_member_id(s: &Struct, idx: usize, field: &Field) -> u32 {
        for ann in &field.annotations {
            if let Annotation::Id(id) = ann {
                return *id;
            }
        }

        let autoid_seq = s
            .annotations
            .iter()
            .any(|a| matches!(a, Annotation::AutoId(AutoIdKind::Sequential)));
        if autoid_seq {
            // @audit-ok: safe cast - field index in struct always << u32::MAX
            #[allow(clippy::cast_possible_truncation)]
            return idx as u32;
        }

        Self::fnv1a_member_id(&field.name)
    }

    /// FNV-1a 32-bit hash masked to 28 bits, per `XTypes` ss7.3.1.2.1.2.
    pub(crate) fn fnv1a_member_id(name: &str) -> u32 {
        let mut h: u32 = 0x811c_9dc5; // FNV offset basis
        for b in name.as_bytes() {
            h ^= u32::from(*b);
            h = h.wrapping_mul(0x0100_0193); // FNV prime
        }
        h & 0x0fff_ffff
    }
}

pub(super) fn uses_fixed(ast: &IdlFile) -> bool {
    fn type_has_fixed(t: &IdlType) -> bool {
        match t {
            IdlType::Primitive(PrimitiveType::Fixed { .. }) => true,
            IdlType::Sequence { inner, .. } | IdlType::Array { inner, .. } => type_has_fixed(inner),
            IdlType::Map { key, value, .. } => type_has_fixed(key) || type_has_fixed(value),
            _ => false,
        }
    }

    for def in &ast.definitions {
        match def {
            Definition::Struct(s) => {
                if s.fields.iter().any(|f| type_has_fixed(&f.field_type)) {
                    return true;
                }
            }
            Definition::Typedef(t) => {
                if type_has_fixed(&t.base_type) {
                    return true;
                }
            }
            Definition::Union(u) => {
                if u.cases.iter().any(|c| type_has_fixed(&c.field.field_type)) {
                    return true;
                }
            }
            Definition::Module(m) => {
                let sub = IdlFile {
                    definitions: m.definitions.clone(),
                };
                if uses_fixed(&sub) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Convert `SCREAMING_SNAKE_CASE` or `snake_case` to `PascalCase` for Rust enum variants.
///
/// Examples:
/// - `RED` -> `Red`
/// - `GREEN_LIGHT` -> `GreenLight`
/// - `ALREADY_Pascal` -> `AlreadyPascal`
pub(super) fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                let mut result = String::new();
                result.push_str(&first.to_uppercase().to_string());
                result.push_str(&chars.as_str().to_lowercase());
                result
            })
        })
        .collect()
}
