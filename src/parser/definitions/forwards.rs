// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Forward declaration parsing.
//!
//! Parses IDL forward declarations for struct and union types.

use crate::ast::{Definition, ForwardDecl, ForwardKind};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    pub(super) fn try_parse_struct_forward(&mut self) -> Option<Definition> {
        let saved = self.current;
        self.advance(); // consume 'struct'

        let forward = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let name_clone = name.clone();
            self.advance();
            if self.check(&TokenKind::Semicolon) {
                self.advance();
                Some(Definition::ForwardDecl(ForwardDecl {
                    kind: ForwardKind::Struct,
                    name: name_clone,
                }))
            } else {
                None
            }
        } else {
            None
        };

        if forward.is_none() {
            self.current = saved;
        }

        forward
    }

    pub(super) fn try_parse_union_forward(&mut self) -> Option<Definition> {
        let saved = self.current;
        self.advance(); // consume 'union'

        let forward = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let name_clone = name.clone();
            self.advance();
            if self.check(&TokenKind::Semicolon) {
                self.advance();
                Some(Definition::ForwardDecl(ForwardDecl {
                    kind: ForwardKind::Union,
                    name: name_clone,
                }))
            } else {
                None
            }
        } else {
            None
        };

        if forward.is_none() {
            self.current = saved;
        }

        forward
    }
}
