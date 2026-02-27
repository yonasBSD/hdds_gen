// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Bitmask code generation for Rust.
//!
//! Generates Rust type aliases and constants for IDL bitmask types.

use super::{push_fmt, RustGenerator};
use crate::ast::Bitmask;
use crate::types::Annotation;

impl RustGenerator {
    pub(super) fn generate_bitmask(&self, m: &Bitmask) -> String {
        let mut output = String::new();
        let indent = self.indent();
        let name = &m.name;
        push_fmt(
            &mut output,
            format_args!("{indent}pub type {name} = u64;\n"),
        );

        let mut next_pos: u32 = 0;
        for flag in &m.flags {
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
            let prefix = m.name.to_uppercase();
            let flag_upper = flag.name.to_uppercase();
            push_fmt(
                &mut output,
                format_args!("{indent}pub const {prefix}_{flag_upper}: {name} = 1u64 << {bit};\n"),
            );
        }
        output.push('\n');
        output
    }
}
