// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Typedef code generation for Rust.
//!
//! Generates Rust type aliases from IDL typedefs.

use super::RustGenerator;
use crate::ast::Typedef;

impl RustGenerator {
    pub(super) fn generate_typedef(&self, t: &Typedef) -> String {
        let indent = self.indent();
        let name = &t.name;
        let base = Self::type_to_rust(&t.base_type);
        format!("{indent}pub type {name} = {base};\n\n")
    }
}
