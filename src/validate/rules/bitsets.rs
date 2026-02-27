// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Bitset validation rules.
//!
//! Validates bitset definitions for overlapping bit ranges and annotation usage.

#![allow(clippy::redundant_pub_crate)]

use super::super::diagnostics::{Level, ValidationDiag};
use super::helpers::validate_custom_annotations;
use crate::ast::{AnnotationDecl, Bitset};
use crate::types::Annotation;
use std::collections::HashMap;

pub(crate) fn validate_bitset(
    b: &Bitset,
    ann_index: &HashMap<String, AnnotationDecl>,
    diags: &mut Vec<ValidationDiag>,
) {
    validate_custom_annotations(
        &b.annotations,
        ann_index,
        &format!("bitset {}", b.name),
        diags,
    );

    // Validate annotation placement: @external and @non_serialized are invalid on bitsets
    for ann in &b.annotations {
        match ann {
            Annotation::External => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "bitset {}: @external is invalid on bitsets (valid on struct/union types and members)",
                        b.name
                    ),
                });
            }
            Annotation::NonSerialized => {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "bitset {}: @non_serialized is invalid on bitsets (valid on members only)",
                        b.name
                    ),
                });
            }
            _ => {}
        }
    }

    let mut positions: Vec<(u32, u32, &str)> = Vec::new();
    let mut next_pos: u32 = 0;
    for f in &b.fields {
        validate_custom_annotations(
            &f.annotations,
            ann_index,
            &format!("bitset {}.{}", b.name, f.name),
            diags,
        );
        let mut pos_ann: Option<u32> = None;
        for ann in &f.annotations {
            if let Annotation::Position(p) = ann {
                pos_ann = Some(*p);
                break;
            }
        }
        let start = pos_ann.unwrap_or_else(|| {
            let p = next_pos;
            next_pos = next_pos.saturating_add(f.width);
            p
        });
        positions.push((start, f.width, &f.name));
    }

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (s1, w1, n1) = positions[i];
            let (s2, w2, n2) = positions[j];
            let e1 = s1.saturating_add(w1);
            let e2 = s2.saturating_add(w2);
            let overlap = s1 < e2 && s2 < e1;
            if overlap {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "Bitset {}: fields '{}'@[{},{}) and '{}'@[{},{}) overlap",
                        b.name, n1, s1, e1, n2, s2, e2
                    ),
                });
            }
        }
    }

    let mut bit_bound: Option<u32> = None;
    for ann in &b.annotations {
        if let Annotation::BitBound(n) = ann {
            bit_bound = Some(*n);
            break;
        }
    }
    if let Some(bound) = bit_bound {
        let max_end = positions
            .iter()
            .map(|(s, w, _)| s.saturating_add(*w))
            .max()
            .unwrap_or(0);
        if max_end > bound {
            diags.push(ValidationDiag {
                level: Level::Error,
                message: format!(
                    "Bitset {} exceeds @bit_bound({}): max end bit is {}",
                    b.name, bound, max_end
                ),
            });
        }
    }
}
