// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Union definition parsing.
//!
//! Parses IDL discriminated union declarations with case labels.

use crate::ast::{Union, UnionCase, UnionLabel};
use crate::error::{ErrorKind, ParseError, Result};
use crate::token::TokenKind;
use crate::types::{Annotation, IdlType};

use super::Parser;

impl Parser {
    pub(super) fn parse_union(&mut self) -> Result<Union> {
        self.expect(&TokenKind::Union, "Expected 'union'")?;

        let name = self.parse_union_name()?;
        let discriminator = self.parse_union_discriminator()?;
        self.expect(
            &TokenKind::LeftBrace,
            "Expected '{' after union discriminator",
        )?;

        let mut union_def = Union::new(name, discriminator);

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let case = self.parse_union_case()?;
            union_def.add_case(case);
        }

        self.expect(&TokenKind::RightBrace, "Expected '}' after union body")?;
        self.expect(&TokenKind::Semicolon, "Expected ';' after union")?;

        Ok(union_def)
    }

    fn parse_union_name(&mut self) -> Result<String> {
        if let TokenKind::Identifier(name) = &self.current_token().kind {
            let result = name.clone();
            self.advance();
            Ok(result)
        } else {
            Err(ParseError::new(
                ErrorKind::InvalidIdentifier,
                self.current_position(),
                "Expected union name",
            ))
        }
    }

    fn parse_union_discriminator(&mut self) -> Result<IdlType> {
        self.expect(&TokenKind::Switch, "Expected 'switch' after union name")?;
        self.expect(&TokenKind::LeftParen, "Expected '(' after 'switch'")?;

        let discriminator = self.parse_type()?;

        self.expect(
            &TokenKind::RightParen,
            "Expected ')' after discriminator type",
        )?;

        Ok(discriminator)
    }

    fn parse_union_case(&mut self) -> Result<UnionCase> {
        let saw_default_annotation = self.consume_union_case_annotations()?;
        let mut labels = self.parse_union_case_labels()?;

        if labels.is_empty() && saw_default_annotation {
            labels.push(UnionLabel::Default);
        }

        if labels.is_empty() {
            return Err(ParseError::new(
                ErrorKind::InvalidSyntax,
                self.current_position(),
                "Expected case or default label",
            ));
        }

        let field = self.parse_field()?;
        Ok(UnionCase { labels, field })
    }

    fn consume_union_case_annotations(&mut self) -> Result<bool> {
        let mut saw_default = false;
        while self.check(&TokenKind::Annotation) {
            self.advance();
            let annotation = self.parse_annotation()?;
            if matches!(annotation, Annotation::Default | Annotation::DefaultLiteral) {
                saw_default = true;
            }
        }
        Ok(saw_default)
    }

    fn parse_union_case_labels(&mut self) -> Result<Vec<UnionLabel>> {
        let mut labels = Vec::new();
        loop {
            if self.check(&TokenKind::Case) {
                self.advance();
                let value = self.parse_union_case_value()?;
                labels.push(UnionLabel::Value(value));
            } else if self.check(&TokenKind::Default) {
                self.advance();
                self.expect(&TokenKind::Colon, "Expected ':' after 'default'")?;
                labels.push(UnionLabel::Default);
                break;
            } else {
                break;
            }
        }
        Ok(labels)
    }

    fn parse_union_case_value(&mut self) -> Result<String> {
        let value = match self.peek() {
            TokenKind::Identifier(id) => {
                let label = id.clone();
                self.advance();
                label
            }
            TokenKind::IntegerLiteral(n) => {
                let label = n.to_string();
                self.advance();
                label
            }
            _ => {
                return Err(ParseError::new(
                    ErrorKind::InvalidSyntax,
                    self.current_position(),
                    "Expected case label value",
                ));
            }
        };

        self.expect(&TokenKind::Colon, "Expected ':' after case label")?;
        Ok(value)
    }
}
