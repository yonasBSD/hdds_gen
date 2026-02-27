// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Interface pretty-printing.
//!
//! Renders IDL interface definitions back to source format.

use super::helpers::{indent, push_fmt, type_to_idl};
use crate::ast::{Interface, ParamDir};

pub(super) fn push_interface(out: &mut String, itf: &Interface, level: usize) {
    if let Some(base) = &itf.base {
        push_fmt(
            out,
            format_args!("{}interface {} : {} {{\n", indent(level), itf.name, base),
        );
    } else {
        push_fmt(
            out,
            format_args!("{}interface {} {{\n", indent(level), itf.name),
        );
    }

    for attr in &itf.attributes {
        let kw = if attr.readonly {
            "readonly attribute"
        } else {
            "attribute"
        };
        push_fmt(
            out,
            format_args!(
                "{}{} {} {};\n",
                indent(level + 1),
                kw,
                type_to_idl(&attr.ty),
                attr.name
            ),
        );
    }

    for op in &itf.operations {
        let ret = type_to_idl(&op.return_type);
        let oneway = if op.oneway { "oneway " } else { "" };
        push_fmt(
            out,
            format_args!("{}{}{} {}(", indent(level + 1), oneway, ret, op.name),
        );

        let mut first = true;
        for param in &op.params {
            if first {
                first = false;
            } else {
                out.push_str(", ");
            }
            let dir = match param.dir {
                ParamDir::In => "in",
                ParamDir::Out => "out",
                ParamDir::InOut => "inout",
            };
            push_fmt(
                out,
                format_args!("{} {} {}", dir, type_to_idl(&param.ty), param.name),
            );
        }
        out.push(')');

        if !op.raises.is_empty() {
            out.push_str(" raises(");
            for (idx, name) in op.raises.iter().enumerate() {
                if idx > 0 {
                    out.push_str(", ");
                }
                out.push_str(name);
            }
            out.push(')');
        }

        out.push_str(";\n");
    }

    push_fmt(out, format_args!("{}}};\n", indent(level)));
}

pub(super) fn push_exception(out: &mut String, ex: &crate::ast::Exception, level: usize) {
    push_fmt(
        out,
        format_args!("{}exception {} {{\n", indent(level), ex.name),
    );
    for member in &ex.members {
        push_fmt(
            out,
            format_args!(
                "{}{} {};\n",
                indent(level + 1),
                type_to_idl(&member.field_type),
                member.name
            ),
        );
    }
    push_fmt(out, format_args!("{}}};\n", indent(level)));
}
