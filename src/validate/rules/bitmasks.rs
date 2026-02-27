// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Bitmask validation rules.
//!
//! Validates bitmask definitions for duplicate flags and annotation usage.

#![allow(clippy::redundant_pub_crate)]

use super::super::diagnostics::{Level, ValidationDiag};
use super::helpers::validate_custom_annotations;
use crate::ast::{AnnotationDecl, Bitmask};
use crate::types::Annotation;
use std::collections::{HashMap, HashSet};

pub(crate) fn validate_bitmask(
    b: &Bitmask,
    ann_index: &HashMap<String, AnnotationDecl>,
    diags: &mut Vec<ValidationDiag>,
) {
    validate_custom_annotations(
        &b.annotations,
        ann_index,
        &format!("bitmask {}", b.name),
        diags,
    );

    // Validate annotation placement: @external and @non_serialized are invalid on bitmasks
    for ann in &b.annotations {
        match ann {
            Annotation::External => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "bitmask {}: @external is invalid on bitmasks (valid on struct/union types and members)",
                        b.name
                    ),
                });
            }
            Annotation::NonSerialized => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "bitmask {}: @non_serialized is invalid on bitmasks (valid on members only)",
                        b.name
                    ),
                });
            }
            _ => {}
        }
    }

    // Check for empty bitmask
    if b.flags.is_empty() {
        diags.push(ValidationDiag {
            level: Level::Error,
            message: format!("bitmask {}: must declare at least one flag", b.name),
        });
        return;
    }

    // Check for duplicate flag names
    let mut seen: HashSet<String> = HashSet::new();
    for flag in &b.flags {
        if !seen.insert(flag.name.clone()) {
            diags.push(ValidationDiag {
                level: Level::Error,
                message: format!("bitmask {}: duplicate flag '{}'", b.name, flag.name),
            });
        }

        // Validate flag annotations
        validate_custom_annotations(
            &flag.annotations,
            ann_index,
            &format!("bitmask {}.{}", b.name, flag.name),
            diags,
        );
    }

    // Validate @bit_bound if present
    let mut bit_bound: Option<u32> = None;
    for ann in &b.annotations {
        if let Annotation::BitBound(n) = ann {
            bit_bound = Some(*n);
            break;
        }
    }

    if let Some(bound) = bit_bound {
        // @audit-ok: safe cast - bitmask flag count realistically << u32::MAX
        #[allow(clippy::cast_possible_truncation)]
        let flag_count = b.flags.len() as u32;
        if flag_count > bound {
            diags.push(ValidationDiag {
                level: Level::Warning,
                message: format!(
                    "bitmask {}: has {} flags but @bit_bound({}) only allows {} unique values",
                    b.name, flag_count, bound, bound
                ),
            });
        }
    }
}
