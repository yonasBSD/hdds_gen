// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Lexical analyzer (tokenizer) for IDL.
//!
//! The module is split into smaller pieces to keep each file focused:
//! - `state.rs`: the `Lexer` struct and low-level cursor utilities.
//! - `numbers.rs`: numeric literal parsing helpers.
//! - `scanner.rs`: high-level token production routines.

mod numbers;
mod scanner;
mod state;

#[cfg(test)]
mod tests;

pub use state::Lexer;
