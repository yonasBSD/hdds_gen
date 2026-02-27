// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Definition parsers for IDL constructs.
//!
//! Submodules handle structs, enums, unions, typedefs, and other definitions.

#![allow(clippy::redundant_pub_crate)]

pub(crate) mod bitmasks;
pub(crate) mod bitsets;
pub(crate) mod consts;
pub(crate) mod enums;
pub(crate) mod forwards;
pub(crate) mod module;
pub(crate) mod structs;
pub(crate) mod typedefs;
pub(crate) mod unions;

pub(crate) use module::merge_module_into;

use crate::ast::Definition;
use crate::error::{ErrorKind, ParseError, Result};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// Parse any top-level definition (module, struct, union, etc.).
    pub(super) fn parse_definition(&mut self) -> Result<Definition> {
        if let Some(annotation_decl) = self.try_parse_annotation_declaration()? {
            return Ok(annotation_decl);
        }

        let annotations = self.collect_leading_annotations()?;

        let definition = match self.peek() {
            TokenKind::Module => self.parse_module().map(Definition::Module),
            TokenKind::Struct => self
                .try_parse_struct_forward()
                .map_or_else(|| self.parse_struct().map(Definition::Struct), Ok),
            TokenKind::Typedef => self.parse_typedef().map(Definition::Typedef),
            TokenKind::Enum => self.parse_enum().map(Definition::Enum),
            TokenKind::Union => self
                .try_parse_union_forward()
                .map_or_else(|| self.parse_union().map(Definition::Union), Ok),
            TokenKind::Bitset => self.parse_bitset().map(Definition::Bitset),
            TokenKind::Bitmask => self.parse_bitmask().map(Definition::Bitmask),
            #[cfg(feature = "interfaces")]
            TokenKind::Interface => self.parse_interface().map(Definition::Interface),
            #[cfg(feature = "interfaces")]
            TokenKind::Exception => self.parse_exception().map(Definition::Exception),
            TokenKind::Const => self.parse_const().map(Definition::Const),
            _ => Err(ParseError::new(
                ErrorKind::UnexpectedToken,
                self.current_position(),
                format!("Expected definition, found {:?}", self.peek()),
            )),
        }?;

        Ok(Self::attach_definition_annotations(definition, annotations))
    }
}
