// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Lexer state and core tokenization logic.
//!
//! The `Lexer` struct maintains position state while scanning input.

use crate::error::{ErrorKind, ParseError, Position, Result};
use crate::token::{Token, TokenKind};

/// Lexer state cursoring through the input source.
pub struct Lexer {
    pub(super) input: Vec<char>,
    pub(super) position: usize,
    pub(super) line: usize,
    pub(super) column: usize,
}

impl Lexer {
    /// Create a new lexer from IDL source.
    #[must_use]
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the entire input stream into a vector of tokens.
    ///
    /// # Errors
    ///
    /// Propagates any lexical errors encountered while scanning the source.
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Current cursor position in the source file.
    pub(crate) const fn current_position(&self) -> Position {
        Position::new(self.line, self.column)
    }

    /// Peek at current character without consuming.
    pub(crate) fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Peek ahead *n* characters without consuming.
    pub(crate) fn peek_ahead(&self, n: usize) -> Option<char> {
        self.input.get(self.position + n).copied()
    }

    /// Consume and return the current character.
    pub(crate) fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    /// Whether the cursor has consumed the entire input.
    pub(crate) const fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Skip spaces, tabs, and carriage returns (preserving newlines for preprocessor logic).
    pub(crate) fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if matches!(ch, ' ' | '\t' | '\r') {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip a single-line comment.
    pub(crate) fn skip_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Skip a potentially nested block comment (`/* ... */`).
    pub(crate) fn skip_block_comment(&mut self) -> Result<()> {
        let start_pos = self.current_position();
        self.advance(); // consume '/'
        self.advance(); // consume '*'

        let mut depth = 1usize;

        while !self.is_at_end() {
            let ch = self.peek();
            let next = self.peek_ahead(1);

            if ch == Some('/') && next == Some('*') {
                depth += 1;
                self.advance();
                self.advance();
            } else if ch == Some('*') && next == Some('/') {
                depth -= 1;
                self.advance();
                self.advance();
                if depth == 0 {
                    return Ok(());
                }
            } else {
                self.advance();
            }
        }

        Err(ParseError::new(
            ErrorKind::UnexpectedEof,
            start_pos,
            "Unterminated block comment",
        ))
    }
}
