// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Enum pretty-printing.
//!
//! Renders IDL enum definitions back to source format.

use super::helpers::{indent, push_fmt, render_annotation};
use crate::ast::Enum;

pub(super) fn push_enum(out: &mut String, e: &Enum, level: usize) {
    for ann in &e.annotations {
        push_fmt(
            out,
            format_args!("{}{}\n", indent(level), render_annotation(ann)),
        );
    }
    push_fmt(out, format_args!("{}enum {} {{\n", indent(level), e.name));

    let max_name_len = e
        .variants
        .iter()
        .filter(|v| v.value.is_some())
        .map(|v| v.name.len())
        .max()
        .unwrap_or(0);

    for (idx, v) in e.variants.iter().enumerate() {
        let sep = if idx + 1 == e.variants.len() { "" } else { "," };
        match v.value {
            Some(val) => {
                let pad = if max_name_len > v.name.len() {
                    " ".repeat(max_name_len - v.name.len())
                } else {
                    String::new()
                };
                push_fmt(
                    out,
                    format_args!("{}{}{} = {}{}\n", indent(level + 1), v.name, pad, val, sep),
                );
            }
            None => push_fmt(
                out,
                format_args!("{}{}{}\n", indent(level + 1), v.name, sep),
            ),
        }
    }

    push_fmt(out, format_args!("{}}};\n", indent(level)));
}
