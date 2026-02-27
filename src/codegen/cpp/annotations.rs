// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Annotation declaration handling for C++.
//!
//! Placeholder for custom annotation support in generated C++ code.

use super::helpers::push_fmt;
use super::CppGenerator;
use crate::ast::AnnotationDecl;

pub(super) fn generate_annotation_decl(generator: &CppGenerator, ann: &AnnotationDecl) -> String {
    let mut output = String::new();
    push_fmt(
        &mut output,
        format_args!(
            "{}// Annotation {} ({} member{}) not emitted in C++ backend yet.\n\n",
            generator.indent(),
            ann.name,
            ann.members.len(),
            if ann.members.len() == 1 { "" } else { "s" }
        ),
    );
    output
}
