// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Unit tests for the lexer.

#![allow(clippy::pedantic)]
#![allow(clippy::expect_used)]

use super::Lexer;
use crate::token::TokenKind;

#[test]
fn test_basic_tokens() {
    let input = "struct Point { int32_t x; };";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().expect("tokenize basic struct definition");

    assert_eq!(tokens[0].kind, TokenKind::Struct);
    assert_eq!(tokens[2].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[3].kind, TokenKind::Int32);
}

#[test]
fn test_annotation() {
    let input = "@key int32_t id;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer
        .tokenize()
        .expect("tokenize annotation with identifier");

    assert_eq!(tokens[0].kind, TokenKind::Annotation);
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(_)));
}
