// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Token types for IDL lexical analysis
//!
//! Defines all token types according to OMG IDL 4.2 specification.

use crate::error::Position;

#[derive(Debug, Clone, PartialEq)]
/// Token with position information
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub position: Position,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, position: Position) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// All token types in IDL
pub enum TokenKind {
    // Keywords - Base constructs
    Module,
    Struct,
    Typedef,
    Enum,
    Union,
    Switch,
    Case,
    Default,
    Const,
    // Interfaces (feature: interfaces)
    Interface,
    Exception,
    Oneway,
    Attribute,
    Readonly,
    In,
    Out,
    Inout,
    Raises,

    // Keywords - Primitive types
    Void,
    Boolean,
    Char,
    Octet,
    Short,
    Long,
    LongLong,
    Unsigned,
    UnsignedShort,
    UnsignedLong,
    UnsignedLongLong,
    Float,
    Double,
    String,
    // Wide types and extended primitives
    WChar,
    WString,

    // IDL 4.x types
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,

    // Collection types
    Sequence,
    Map,

    // Fixed-point decimal
    Fixed,

    // Bitfield types (IDL 4.2)
    Bitset,
    Bitmask,

    // Annotations (IDL 4.x)
    Annotation, // The '@' prefix

    // Identifiers and literals
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),

    // Operators and delimiters
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftAngle,    // <
    RightAngle,   // >
    Semicolon,    // ;
    Comma,        // ,
    Colon,        // :
    DoubleColon,  // ::
    Equal,        // =

    // Operators for constant expressions
    Plus,            // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Percent,         // %
    Ampersand,       // &
    Pipe,            // |
    Caret,           // ^
    Bang,            // !
    ShiftLeft,       // <<
    ShiftRight,      // >>
    DoubleAmpersand, // &&
    DoublePipe,      // ||

    // Preprocessor directives
    PreprocessorDefine,
    PreprocessorInclude,
    PreprocessorIfdef,
    PreprocessorIfndef,
    PreprocessorElse,
    PreprocessorElif,
    PreprocessorEndif,
    PreprocessorUndef,
    PreprocessorPragma,

    // Special
    Eof,
    Newline,
}

impl TokenKind {
    /// Check if token is a primitive type keyword
    #[must_use]
    pub const fn is_primitive_type(&self) -> bool {
        matches!(
            self,
            Self::Void
                | Self::Boolean
                | Self::Char
                | Self::Octet
                | Self::Short
                | Self::Long
                | Self::LongLong
                | Self::UnsignedShort
                | Self::UnsignedLong
                | Self::UnsignedLongLong
                | Self::Float
                | Self::Double
                | Self::String
                | Self::Int8
                | Self::Int16
                | Self::Int32
                | Self::Int64
                | Self::UInt8
                | Self::UInt16
                | Self::UInt32
                | Self::UInt64
        )
    }

    /// Check if token is a collection type keyword
    #[must_use]
    pub const fn is_collection_type(&self) -> bool {
        matches!(self, Self::Sequence | Self::Map)
    }
}

const KEYWORD_TABLE: &[(&str, TokenKind)] = &[
    // Base constructs
    ("module", TokenKind::Module),
    ("struct", TokenKind::Struct),
    ("typedef", TokenKind::Typedef),
    ("enum", TokenKind::Enum),
    ("union", TokenKind::Union),
    ("switch", TokenKind::Switch),
    ("case", TokenKind::Case),
    ("default", TokenKind::Default),
    ("const", TokenKind::Const),
    // Interfaces (feature gated elsewhere but tokens remain)
    ("interface", TokenKind::Interface),
    ("exception", TokenKind::Exception),
    ("oneway", TokenKind::Oneway),
    ("attribute", TokenKind::Attribute),
    ("readonly", TokenKind::Readonly),
    ("in", TokenKind::In),
    ("out", TokenKind::Out),
    ("inout", TokenKind::Inout),
    ("raises", TokenKind::Raises),
    // Primitive types
    ("void", TokenKind::Void),
    ("boolean", TokenKind::Boolean),
    ("bool", TokenKind::Boolean),
    ("char", TokenKind::Char),
    ("octet", TokenKind::Octet),
    ("short", TokenKind::Short),
    ("long", TokenKind::Long),
    ("unsigned", TokenKind::Unsigned),
    ("float", TokenKind::Float),
    ("double", TokenKind::Double),
    ("string", TokenKind::String),
    ("wchar", TokenKind::WChar),
    ("wstring", TokenKind::WString),
    ("fixed", TokenKind::Fixed),
    // IDL 4.x primitive aliases
    ("int8_t", TokenKind::Int8),
    ("int8", TokenKind::Int8),
    ("int16_t", TokenKind::Int16),
    ("int16", TokenKind::Int16),
    ("int32_t", TokenKind::Int32),
    ("int32", TokenKind::Int32),
    ("int", TokenKind::Int32),
    ("int64_t", TokenKind::Int64),
    ("int64", TokenKind::Int64),
    ("uint8_t", TokenKind::UInt8),
    ("uint8", TokenKind::UInt8),
    ("uint16_t", TokenKind::UInt16),
    ("uint16", TokenKind::UInt16),
    ("uint32_t", TokenKind::UInt32),
    ("uint32", TokenKind::UInt32),
    ("uint64_t", TokenKind::UInt64),
    ("uint64", TokenKind::UInt64),
    // Collections
    ("sequence", TokenKind::Sequence),
    ("map", TokenKind::Map),
    // Bitfield types
    ("bitset", TokenKind::Bitset),
    ("bitmask", TokenKind::Bitmask),
];

impl TokenKind {
    /// Convert keyword string to `TokenKind`
    #[must_use]
    pub fn from_keyword(s: &str) -> Option<Self> {
        KEYWORD_TABLE
            .iter()
            .find_map(|(keyword, kind)| (*keyword == s).then(|| kind.clone()))
    }
}
