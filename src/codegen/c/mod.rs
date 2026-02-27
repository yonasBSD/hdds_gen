// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! C code generator (header-only)
//!
//! MVP: generate C89/C11-compatible header with types (structs/enums/typedefs).
//! CDR2 encode/decode will be added in subsequent phases.

#![allow(
    clippy::useless_format,
    clippy::single_char_add_str,
    clippy::format_push_string,
    clippy::uninlined_format_args,
    clippy::too_many_lines,
    clippy::single_match_else,
    clippy::if_not_else
)]

use crate::ast::{Definition, IdlFile};
use crate::codegen::CodeGenerator;
use crate::error::Result;
use crate::types::{IdlType, PrimitiveType};

mod codec;
mod definitions;
mod header;
mod helpers;
mod index;
#[cfg(test)]
mod tests;
pub mod type_descriptor;

use index::DefinitionIndex;

/// C language standard version for code generation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CStandard {
    /// C89/C90 (ANSI C) - variables at block start, no `_Static_assert`
    C89,
    /// C99 (default) - mixed declarations, for-loop initializers
    #[default]
    C99,
    /// C11 - adds `_Static_assert`
    C11,
}

/// C code generator used by the CLI.
pub struct CGenerator {
    indent_level: usize,
    c_standard: CStandard,
}

impl CGenerator {
    /// Creates a new C generator with default C99 standard.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            indent_level: 0,
            c_standard: CStandard::C99,
        }
    }

    /// Create a generator targeting a specific C standard.
    #[must_use]
    pub const fn with_standard(standard: CStandard) -> Self {
        Self {
            indent_level: 0,
            c_standard: standard,
        }
    }

    /// Returns the C standard being used.
    #[must_use]
    pub const fn standard(&self) -> CStandard {
        self.c_standard
    }

    /// Returns true if targeting C89 (ANSI C).
    #[must_use]
    pub const fn is_c89(&self) -> bool {
        matches!(self.c_standard, CStandard::C89)
    }

    pub(super) fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub(super) fn type_to_c(t: &IdlType) -> String {
        match t {
            IdlType::Primitive(p) => match p {
                PrimitiveType::Void => "void".to_string(),
                PrimitiveType::Boolean => "bool".to_string(),
                PrimitiveType::Char => "char".to_string(),
                PrimitiveType::WChar => "wchar_t".to_string(),
                PrimitiveType::Octet | PrimitiveType::UInt8 => "uint8_t".to_string(),
                PrimitiveType::Short | PrimitiveType::Int16 => "int16_t".to_string(),
                PrimitiveType::UnsignedShort | PrimitiveType::UInt16 => "uint16_t".to_string(),
                PrimitiveType::Long | PrimitiveType::Int32 => "int32_t".to_string(),
                PrimitiveType::UnsignedLong | PrimitiveType::UInt32 => "uint32_t".to_string(),
                PrimitiveType::LongLong | PrimitiveType::Int64 => "int64_t".to_string(),
                PrimitiveType::UnsignedLongLong | PrimitiveType::UInt64 => "uint64_t".to_string(),
                PrimitiveType::Float => "float".to_string(),
                PrimitiveType::Double | PrimitiveType::LongDouble => "double".to_string(),
                PrimitiveType::Fixed { digits, scale } => {
                    format!("cdr_fixed128_t /* fixed<{digits}, {scale}> */")
                }
                PrimitiveType::String => "char*".to_string(),
                PrimitiveType::WString => "wchar_t*".to_string(),
                PrimitiveType::Int8 => "int8_t".to_string(),
            },
            IdlType::Named(n) => n.clone(),
            IdlType::Sequence { inner, .. } => {
                format!(
                    "/* sequence<{}> */ struct {{ {}* data; uint32_t len; }}",
                    inner.to_idl_string(),
                    Self::type_to_c(inner)
                )
            }
            IdlType::Map { key, value, .. } => {
                format!(
                    "/* map<{},{}> */ struct {{ {}* keys; {}* values; uint32_t len; }}",
                    key.to_idl_string(),
                    value.to_idl_string(),
                    Self::type_to_c(key),
                    Self::type_to_c(value)
                )
            }
            IdlType::Array { inner, size } => {
                format!("{}[{}]", Self::type_to_c(inner), size)
            }
        }
    }
}

impl CodeGenerator for CGenerator {
    fn generate(&self, ast: &IdlFile) -> Result<String> {
        let include_fixed = uses_fixed_c(ast);
        let (idx, flat) = DefinitionIndex::from_file(ast);

        let mut out = String::new();
        out.push_str(&Self::header_prelude(include_fixed, self.c_standard));
        out.push_str(&self.emit_type_definitions(&flat));
        out.push_str(&Self::emit_struct_helpers_section(
            &flat,
            &idx,
            self.c_standard,
        ));
        out.push_str(&Self::emit_union_helpers_section(
            &flat,
            &idx,
            self.c_standard,
        ));

        Ok(out)
    }
}

impl Default for CGenerator {
    fn default() -> Self {
        Self::new()
    }
}

fn uses_fixed_c(ast: &IdlFile) -> bool {
    fn type_has_fixed(ty: &IdlType) -> bool {
        match ty {
            IdlType::Primitive(PrimitiveType::Fixed { .. }) => true,
            IdlType::Sequence { inner, .. } | IdlType::Array { inner, .. } => type_has_fixed(inner),
            IdlType::Map { key, value, .. } => type_has_fixed(key) || type_has_fixed(value),
            _ => false,
        }
    }

    for def in &ast.definitions {
        match def {
            Definition::Struct(s) => {
                if s.fields.iter().any(|f| type_has_fixed(&f.field_type)) {
                    return true;
                }
            }
            Definition::Union(u) => {
                if u.cases.iter().any(|c| type_has_fixed(&c.field.field_type)) {
                    return true;
                }
            }
            Definition::Typedef(t) => {
                if type_has_fixed(&t.base_type) {
                    return true;
                }
            }
            Definition::Module(m) => {
                let sub_ast = IdlFile {
                    definitions: m.definitions.clone(),
                };
                if uses_fixed_c(&sub_ast) {
                    return true;
                }
            }
            Definition::Bitset(_)
            | Definition::Bitmask(_)
            | Definition::Enum(_)
            | Definition::Const(_)
            | Definition::ForwardDecl(_)
            | Definition::AnnotationDecl(_) => {}
            #[cfg(feature = "interfaces")]
            Definition::Interface(_) => {}
            #[cfg(feature = "interfaces")]
            Definition::Exception(_) => {}
        }
    }

    false
}
