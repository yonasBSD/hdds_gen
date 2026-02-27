// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Diagnostic types for semantic validation.
//!
//! Defines warning and error levels with structured diagnostic messages.

#[derive(Debug, Clone, PartialEq, Eq)]
/// Severity level reported by the validator.
pub enum Level {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
/// Diagnostic emitted during semantic validation.
pub struct ValidationDiag {
    pub level: Level,
    pub message: String,
}
