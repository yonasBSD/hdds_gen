// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Helper utilities for pretty-printing.
//!
//! Indentation, type rendering, and annotation formatting.

use crate::types::{Annotation, ExtensibilityKind, IdlType, PrimitiveType};
use std::fmt::Write;

pub(super) fn indent(n: usize) -> String {
    "    ".repeat(n)
}

pub(super) fn type_to_idl(t: &IdlType) -> String {
    match t {
        IdlType::Primitive(PrimitiveType::Fixed { digits, scale }) => {
            format!("fixed<{digits}, {scale}>")
        }
        IdlType::Sequence {
            inner,
            bound: Some(n),
        } => match &**inner {
            IdlType::Primitive(PrimitiveType::Char) => format!("string<{n}>"),
            IdlType::Primitive(PrimitiveType::WChar) => format!("wstring<{n}>"),
            _ => t.to_idl_string(),
        },
        IdlType::Primitive(PrimitiveType::WString) => "wstring".to_string(),
        IdlType::Primitive(PrimitiveType::String) => "string".to_string(),
        _ => t.to_idl_string(),
    }
}

pub(super) fn push_fmt(dst: &mut String, args: std::fmt::Arguments<'_>) {
    let _ = dst.write_fmt(args);
}

pub(super) fn split_array(t: &IdlType) -> (&IdlType, Vec<u32>) {
    let mut dims = Vec::new();
    let mut cur = t;
    while let IdlType::Array { inner, size } = cur {
        dims.push(*size);
        cur = inner.as_ref();
    }
    (cur, dims)
}

pub(super) fn render_annotations_line(anns: &[Annotation]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for a in anns {
        parts.push(render_annotation(a));
    }
    parts.join(" ")
}

pub(super) fn render_annotation(a: &Annotation) -> String {
    match a {
        Annotation::Key => "@key".to_string(),
        Annotation::Optional => "@optional".to_string(),
        Annotation::Id(n) => format!("@id({n})"),
        Annotation::AutoId(kind) => match kind {
            crate::types::AutoIdKind::Sequential => "@autoid(SEQUENTIAL)".to_string(),
            crate::types::AutoIdKind::Hash => "@autoid(HASH)".to_string(),
        },
        Annotation::Position(n) => format!("@position({n})"),
        Annotation::BitBound(n) => format!("@bit_bound({n})"),
        Annotation::Unit(u) => format!("@unit(\"{u}\")"),
        Annotation::Min(v) => format!("@min({v})"),
        Annotation::Max(v) => format!("@max({v})"),
        Annotation::Range { min, max } => format!("@range(min={min}, max={max})"),
        Annotation::Extensibility(kind) => match kind {
            ExtensibilityKind::Final => "@extensibility(FINAL)".to_string(),
            ExtensibilityKind::Appendable => "@extensibility(APPENDABLE)".to_string(),
            ExtensibilityKind::Mutable => "@extensibility(MUTABLE)".to_string(),
        },
        Annotation::Final => "@final".to_string(),
        Annotation::Appendable => "@appendable".to_string(),
        Annotation::Mutable => "@mutable".to_string(),
        Annotation::MustUnderstand => "@must_understand".to_string(),
        Annotation::Default => "@default".to_string(),
        Annotation::Value(v) => format!("@default({v})"),
        Annotation::DefaultLiteral => "@default_literal".to_string(),
        Annotation::Service => "@service".to_string(),
        Annotation::Oneway => "@oneway".to_string(),
        Annotation::Ami => "@ami".to_string(),
        Annotation::External => "@external".to_string(),
        Annotation::Nested => "@nested".to_string(),
        Annotation::DataRepresentation(v) => format!("@data_representation({v})"),
        Annotation::NonSerialized => "@non_serialized".to_string(),
        Annotation::Custom { name, params } => {
            if params.is_empty() {
                format!("@{name}")
            } else {
                let inner: Vec<String> = params
                    .iter()
                    .map(|(k, v)| {
                        if k.is_empty() {
                            v.clone()
                        } else {
                            format!("{k}={v}")
                        }
                    })
                    .collect();
                format!("@{name}({})", inner.join(", "))
            }
        }
        _ => String::new(),
    }
}
