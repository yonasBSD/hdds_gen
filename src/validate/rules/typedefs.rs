// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Typedef validation rules.
//!
//! Validates typedef definitions for annotation usage.

#![allow(clippy::redundant_pub_crate)]

use super::super::diagnostics::{Level, ValidationDiag};
use super::helpers::validate_custom_annotations;
use crate::ast::{AnnotationDecl, Typedef};
use crate::types::Annotation;
use std::collections::HashMap;

pub(crate) fn validate_typedef(
    t: &Typedef,
    ann_index: &HashMap<String, AnnotationDecl>,
    diags: &mut Vec<ValidationDiag>,
) {
    validate_custom_annotations(
        &t.annotations,
        ann_index,
        &format!("typedef {}", t.name),
        diags,
    );

    // Validate annotation placement: @external and @non_serialized are invalid on typedefs
    for ann in &t.annotations {
        match ann {
            Annotation::External => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "typedef {}: @external is invalid on typedefs (valid on struct/union types and members)",
                        t.name
                    ),
                });
            }
            Annotation::NonSerialized => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "typedef {}: @non_serialized is invalid on typedefs (valid on members only)",
                        t.name
                    ),
                });
            }
            _ => {}
        }
    }
}
