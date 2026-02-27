// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Enum code generation for C++.
//!
//! Generates C++ enum class definitions from IDL enum types.

use super::helpers::push_fmt;
use super::CppGenerator;
use crate::ast::Enum;

pub(super) fn generate_enum(generator: &CppGenerator, e: &Enum) -> String {
    let mut output = String::new();
    let indent = generator.indent();
    let name = &e.name;
    push_fmt(&mut output, format_args!("{indent}enum class {name} {{\n"));

    for variant in &e.variants {
        let vname = &variant.name;
        if let Some(val) = variant.value {
            push_fmt(&mut output, format_args!("{indent}    {vname} = {val},\n"));
        } else {
            push_fmt(&mut output, format_args!("{indent}    {vname},\n"));
        }
    }

    push_fmt(&mut output, format_args!("{indent}}};\n\n"));
    output
}
