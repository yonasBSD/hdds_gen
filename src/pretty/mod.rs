// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Pretty-printer for IDL AST.
//!
//! Renders an AST back to human-readable IDL source code.
//! Useful for debugging, code formatting, and round-trip testing.

mod bitsets;
mod enums;
mod formatter;
mod helpers;
#[cfg(feature = "interfaces")]
mod interfaces;
mod modules;
mod structs;
#[cfg(test)]
mod tests;
mod unions;

pub use formatter::to_idl;
