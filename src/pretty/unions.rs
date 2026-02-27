// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Union pretty-printing.
//!
//! Renders IDL discriminated union definitions back to source format.

use super::helpers::{indent, push_fmt, render_annotation, split_array, type_to_idl};
use crate::ast::{Union, UnionCase, UnionLabel};
use crate::types::Annotation;
use std::fmt::Write;

pub(super) fn push_union(out: &mut String, u: &Union, level: usize) {
    for ann in &u.annotations {
        push_fmt(
            out,
            format_args!("{}{}\n", indent(level), render_annotation(ann)),
        );
    }
    push_fmt(
        out,
        format_args!(
            "{}union {} switch({}) {{\n",
            indent(level),
            u.name,
            type_to_idl(&u.discriminator)
        ),
    );

    for case in &u.cases {
        push_union_case(out, case, level + 1);
    }

    push_fmt(out, format_args!("{}}};\n", indent(level)));
}

fn push_union_case(out: &mut String, case: &UnionCase, level: usize) {
    let mut first = true;
    for lbl in &case.labels {
        match lbl {
            UnionLabel::Value(v) => {
                if first {
                    push_fmt(out, format_args!("{}case {v}: ", indent(level)));
                    first = false;
                } else {
                    push_fmt(out, format_args!("case {v}: "));
                }
            }
            UnionLabel::Default => {
                if first {
                    push_fmt(out, format_args!("{}default: ", indent(level)));
                    first = false;
                } else {
                    out.push_str("default: ");
                }
            }
        }
    }

    push_union_case_field(out, case, level, first);
}

pub(super) fn push_union_case_field(out: &mut String, case: &UnionCase, level: usize, first: bool) {
    if first
        && case
            .field
            .annotations
            .iter()
            .any(|a| matches!(a, Annotation::Default | Annotation::DefaultLiteral))
    {
        push_fmt(out, format_args!("{}default: ", indent(level)));
    }

    let (base, dims) = split_array(&case.field.field_type);
    let mut name_with_dims = case.field.name.clone();
    for d in dims {
        let _ = write!(&mut name_with_dims, "[{d}]");
    }

    push_fmt(
        out,
        format_args!("{} {};\n", type_to_idl(base), name_with_dims),
    );
}
