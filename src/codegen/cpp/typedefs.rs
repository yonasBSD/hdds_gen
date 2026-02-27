// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Typedef code generation for C++.
//!
//! Generates C++ using declarations from IDL typedefs.

use super::helpers::type_to_cpp;
use super::CppGenerator;
use crate::ast::Typedef;

pub(super) fn generate_typedef(generator: &CppGenerator, t: &Typedef) -> String {
    let indent = generator.indent();
    let name = &t.name;
    let base = type_to_cpp(&t.base_type);
    format!("{indent}using {name} = {base};\n\n")
}
