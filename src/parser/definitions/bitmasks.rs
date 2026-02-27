// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Bitmask definition parsing.
//!
//! Parses IDL bitmask declarations with flag values.

use crate::ast::{Bitmask, BitmaskFlag};
use crate::error::{ErrorKind, ParseError, Result};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    pub(super) fn parse_bitmask(&mut self) -> Result<Bitmask> {
        self.expect(&TokenKind::Bitmask, "Expected 'bitmask'")?;

        let name = if let TokenKind::Identifier(name) = &self.current_token().kind {
            name.clone()
        } else {
            return Err(ParseError::new(
                ErrorKind::InvalidIdentifier,
                self.current_position(),
                "Expected bitmask name",
            ));
        };
        self.advance();

        self.expect(&TokenKind::LeftBrace, "Expected '{' after bitmask name")?;

        let mut bitmask = Bitmask::new(name);

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let mut annotations = Vec::new();
            while self.check(&TokenKind::Annotation) {
                self.advance();
                annotations.push(self.parse_annotation()?);
            }

            let flag_name = if let TokenKind::Identifier(n) = &self.current_token().kind {
                n.clone()
            } else {
                return Err(ParseError::new(
                    ErrorKind::InvalidIdentifier,
                    self.current_position(),
                    "Expected bitmask flag name",
                ));
            };
            self.advance();

            let mut flag = BitmaskFlag::new(flag_name);
            flag.annotations = annotations;
            bitmask.add_flag(flag);

            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(&TokenKind::RightBrace, "Expected '}' after bitmask body")?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after bitmask")?;
        Ok(bitmask)
    }
}
