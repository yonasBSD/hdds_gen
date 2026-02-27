// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Enum validation rules.
//!
//! Validates enum definitions for duplicate variants and annotation usage.

#![allow(clippy::redundant_pub_crate)]

use super::super::diagnostics::{Level, ValidationDiag};
use super::helpers::validate_custom_annotations;
use crate::ast::{AnnotationDecl, Enum};
use crate::types::Annotation;
use std::collections::{HashMap, HashSet};

pub(crate) fn validate_enum(
    e: &Enum,
    ann_index: &HashMap<String, AnnotationDecl>,
    diags: &mut Vec<ValidationDiag>,
) {
    validate_custom_annotations(
        &e.annotations,
        ann_index,
        &format!("enum {}", e.name),
        diags,
    );

    // Validate annotation placement: @external and @non_serialized are invalid on enums
    for ann in &e.annotations {
        match ann {
            Annotation::External => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "enum {}: @external is invalid on enums (valid on struct/union types and members)",
                        e.name
                    ),
                });
            }
            Annotation::NonSerialized => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "enum {}: @non_serialized is invalid on enums (valid on members only)",
                        e.name
                    ),
                });
            }
            _ => {}
        }
    }

    if e.variants.is_empty() {
        diags.push(ValidationDiag {
            level: Level::Error,
            message: format!("enum {}: must declare at least one enumerator", e.name),
        });
        return;
    }

    let mut seen: HashSet<String> = HashSet::new();
    for v in &e.variants {
        if !seen.insert(v.name.clone()) {
            diags.push(ValidationDiag {
                level: Level::Error,
                message: format!("enum {}: duplicate enumerator '{}'", e.name, v.name),
            });
        }
    }
}
