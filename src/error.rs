// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Error types for IDL parsing
//!
//! Provides comprehensive error handling with source location information.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Position in source code
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
/// Parse error with location information
pub struct ParseError {
    pub kind: ErrorKind,
    pub position: Position,
    pub message: String,
}

impl ParseError {
    #[must_use]
    pub fn new(kind: ErrorKind, position: Position, message: impl Into<String>) -> Self {
        Self {
            kind,
            position,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] at {}: {}", self.kind, self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Type of parse error
pub enum ErrorKind {
    /// Unexpected token
    UnexpectedToken,
    /// Unknown type name
    UnknownType,
    /// Unknown annotation
    UnknownAnnotation,
    /// Invalid syntax
    InvalidSyntax,
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid identifier
    InvalidIdentifier,
    /// Duplicate definition
    DuplicateDefinition,
    /// Preprocessor error
    PreprocessorError,
    /// Other error
    Other,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken => write!(f, "Unexpected token"),
            Self::UnknownType => write!(f, "Unknown type"),
            Self::UnknownAnnotation => write!(f, "Unknown annotation"),
            Self::InvalidSyntax => write!(f, "Invalid syntax"),
            Self::UnexpectedEof => write!(f, "Unexpected end of file"),
            Self::InvalidIdentifier => write!(f, "Invalid identifier"),
            Self::DuplicateDefinition => write!(f, "Duplicate definition"),
            Self::PreprocessorError => write!(f, "Preprocessor error"),
            Self::Other => write!(f, "Parse error"),
        }
    }
}

/// Result type alias for parser operations
pub type Result<T> = std::result::Result<T, ParseError>;
