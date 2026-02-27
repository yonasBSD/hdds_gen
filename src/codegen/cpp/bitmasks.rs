// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Bitmask code generation for C++.
//!
//! Generates C++ enum class with bitwise operators for IDL bitmask types.

use super::helpers::push_fmt;
use super::CppGenerator;
use crate::ast::Bitmask;
use crate::types::Annotation;

pub(super) fn generate_bitmask(generator: &CppGenerator, m: &Bitmask) -> String {
    let mut output = String::new();
    write_bitmask_enum(generator, m, &mut output);
    write_bitmask_operators(generator, &m.name, &mut output);
    output.push('\n');
    output
}

fn write_bitmask_enum(generator: &CppGenerator, m: &Bitmask, out: &mut String) {
    let indent = generator.indent();
    let mname = &m.name;
    push_fmt(out, format_args!("{indent}// Bitmask: {mname}\n"));
    push_fmt(out, format_args!("{indent}using {mname} = uint64_t;\n"));
    push_fmt(
        out,
        format_args!("{indent}enum class {mname} : uint64_t {{\n"),
    );

    let mut next_pos: u32 = 0;
    for (i, flag) in m.flags.iter().enumerate() {
        let mut pos = None;
        for ann in &flag.annotations {
            if let Annotation::Position(p) = ann {
                pos = Some(*p);
                break;
            }
        }
        let bit = pos.unwrap_or_else(|| {
            let p = next_pos;
            next_pos += 1;
            p
        });
        let comma = if i + 1 == m.flags.len() { "" } else { "," };
        let fname = &flag.name;
        push_fmt(
            out,
            format_args!("{indent}    {fname} = (1ull << {bit}){comma}\n"),
        );
    }
    push_fmt(out, format_args!("{indent}}};\n"));
}

fn write_bitmask_operators(generator: &CppGenerator, name: &str, out: &mut String) {
    for symbol in ['|', '&', '^'] {
        write_binary_operator(generator, name, out, symbol);
    }
    write_unary_operator(generator, name, out);
    for symbol in ['|', '&', '^'] {
        write_assign_operator(generator, name, out, symbol);
    }
    out.push('\n');
}

fn write_binary_operator(generator: &CppGenerator, name: &str, out: &mut String, symbol: char) {
    push_fmt(
        out,
        format_args!(
            "{}inline constexpr {t} operator{symbol}({t} a, {t} b) {{ return static_cast<{t}>(static_cast<uint64_t>(a) {symbol} static_cast<uint64_t>(b)); }}\n",
            generator.indent(),
            t = name
        ),
    );
}

fn write_unary_operator(generator: &CppGenerator, name: &str, out: &mut String) {
    push_fmt(
        out,
        format_args!(
            "{}inline constexpr {t} operator~({t} a) {{ return static_cast<{t}>(~static_cast<uint64_t>(a)); }}\n",
            generator.indent(),
            t = name
        ),
    );
}

fn write_assign_operator(generator: &CppGenerator, name: &str, out: &mut String, symbol: char) {
    push_fmt(
        out,
        format_args!(
            "{}inline {t}& operator{symbol}=({t}& a, {t} b) {{ a = a {symbol} b; return a; }}\n",
            generator.indent(),
            t = name
        ),
    );
}
