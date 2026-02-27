// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Parser for IDL.
//!
//! Builds an Abstract Syntax Tree (AST) from a stream of tokens.

mod annotations;
mod const_expr;
mod definitions;
#[cfg(feature = "interfaces")]
mod interfaces;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use state::Parser;
