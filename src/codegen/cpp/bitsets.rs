// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Bitset code generation for C++.
//!
//! Generates C++ struct representations of IDL bitset types.

use super::helpers::push_fmt;
use super::CppGenerator;
use crate::ast::Bitset;

#[allow(clippy::too_many_lines)]
pub(super) fn generate_bitset(generator: &CppGenerator, b: &Bitset) -> String {
    let mut output = String::new();
    let indent = generator.indent();
    let bname = &b.name;
    push_fmt(&mut output, format_args!("{indent}// Bitset: {bname}\n"));
    push_fmt(&mut output, format_args!("{indent}struct {bname} {{\n"));
    for f in &b.fields {
        let ty = if f.width > 32 {
            "unsigned long long"
        } else {
            "unsigned int"
        };
        let fname = &f.name;
        let width = f.width;
        push_fmt(
            &mut output,
            format_args!("{indent}    {ty} {fname} : {width};\n"),
        );
    }
    push_fmt(&mut output, format_args!("\n{indent}    // Helpers\n"));
    for f in &b.fields {
        let fname = &f.name;
        let getter = format!("get_{fname}");
        let setter = format!("set_{fname}");
        push_fmt(
            &mut output,
            format_args!(
                "{indent}    inline unsigned long long {getter}() const {{ return static_cast<unsigned long long>({fname}); }}\n"
            ),
        );
        push_fmt(
            &mut output,
            format_args!(
                "{indent}    inline {bname}& {setter}(unsigned long long v) {{ {fname} = static_cast<decltype({fname})>(v); return *this; }}\n"
            ),
        );
    }

    // CDR2 serialization helpers
    push_fmt(
        &mut output,
        format_args!("\n{indent}    // CDR2 serialization helpers\n"),
    );
    push_fmt(
        &mut output,
        format_args!("{indent}    inline std::uint64_t to_uint64() const {{\n"),
    );
    push_fmt(
        &mut output,
        format_args!("{indent}        std::uint64_t packed = 0;\n"),
    );
    push_fmt(
        &mut output,
        format_args!("{indent}        std::size_t bit_offset = 0;\n"),
    );
    for f in &b.fields {
        let fname = &f.name;
        let width = f.width;
        let mask = (1u64 << width) - 1;
        push_fmt(
            &mut output,
            format_args!(
                "{indent}        packed |= (static_cast<std::uint64_t>({fname}) & 0x{mask:X}ULL) << bit_offset; bit_offset += {width};\n"
            ),
        );
    }
    push_fmt(
        &mut output,
        format_args!("{indent}        (void)bit_offset; // suppress unused warning\n"),
    );
    push_fmt(
        &mut output,
        format_args!("{indent}        return packed;\n"),
    );
    push_fmt(&mut output, format_args!("{indent}    }}\n\n"));

    push_fmt(
        &mut output,
        format_args!("{indent}    inline void from_uint64(std::uint64_t packed) {{\n"),
    );
    push_fmt(
        &mut output,
        format_args!("{indent}        std::size_t bit_offset = 0;\n"),
    );
    for f in &b.fields {
        let fname = &f.name;
        let width = f.width;
        let mask = (1u64 << width) - 1;
        push_fmt(
            &mut output,
            format_args!(
                "{indent}        {fname} = static_cast<decltype({fname})>((packed >> bit_offset) & 0x{mask:X}ULL); bit_offset += {width};\n"
            ),
        );
    }
    push_fmt(
        &mut output,
        format_args!("{indent}        (void)bit_offset; // suppress unused warning\n"),
    );
    push_fmt(&mut output, format_args!("{indent}    }}\n"));

    push_fmt(&mut output, format_args!("{indent}}};\n\n"));
    output
}
