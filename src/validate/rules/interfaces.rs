// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Interface validation rules.
//!
//! Validates CORBA-style interface definitions for operation signatures.

#![allow(clippy::redundant_pub_crate)]

use super::super::diagnostics::{Level, ValidationDiag};
use super::helpers::validate_type_usage;
use crate::ast::{Interface, ParamDir};
use crate::types::{IdlType, PrimitiveType};
use std::collections::{HashMap, HashSet};

pub(crate) fn validate_interface(
    i: &Interface,
    typedefs: &HashMap<String, IdlType>,
    exceptions: &HashSet<String>,
    diags: &mut Vec<ValidationDiag>,
) {
    let mut op_names: HashSet<String> = HashSet::new();
    let mut attr_names: HashSet<String> = HashSet::new();

    for a in &i.attributes {
        if !attr_names.insert(a.name.clone()) {
            diags.push(ValidationDiag {
                level: Level::Error,
                message: format!("interface {}: duplicate attribute '{}'", i.name, a.name),
            });
        }
        validate_type_usage(
            &a.ty,
            &format!("interface {}.{} (attribute)", i.name, a.name),
            diags,
            typedefs,
        );
    }

    for op in &i.operations {
        if !op_names.insert(op.name.clone()) {
            diags.push(ValidationDiag {
                level: Level::Error,
                message: format!("interface {}: duplicate operation '{}'", i.name, op.name),
            });
        }
        if attr_names.contains(&op.name) {
            diags.push(ValidationDiag {
                level: Level::Error,
                message: format!(
                    "interface {}: operation '{}' conflicts with attribute of the same name",
                    i.name, op.name
                ),
            });
        }

        if op.oneway {
            let has_non_in = op.params.iter().any(|p| !matches!(p.dir, ParamDir::In));
            let has_raises = !op.raises.is_empty();
            let returns_void = matches!(op.return_type, IdlType::Primitive(PrimitiveType::Void));
            if has_non_in || has_raises || !returns_void {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "interface {}.{}: oneway requires void return, only 'in' params, and no raises",
                        i.name, op.name
                    ),
                });
            }
        } else if !matches!(op.return_type, IdlType::Primitive(PrimitiveType::Void)) {
            validate_type_usage(
                &op.return_type,
                &format!("interface {}.{} (return)", i.name, op.name),
                diags,
                typedefs,
            );
        }

        let mut param_names: HashSet<String> = HashSet::new();
        for p in &op.params {
            if !param_names.insert(p.name.clone()) {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "interface {}.{}: duplicate parameter name '{}'",
                        i.name, op.name, p.name
                    ),
                });
            }
            validate_type_usage(
                &p.ty,
                &format!("interface {}.{} param {}", i.name, op.name, p.name),
                diags,
                typedefs,
            );
        }

        let mut seen_raise: HashSet<String> = HashSet::new();
        for ex in &op.raises {
            if !seen_raise.insert(ex.clone()) {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "interface {}.{}: duplicate raises '{}'",
                        i.name, op.name, ex
                    ),
                });
            }
            if !exceptions.contains(ex) {
                diags.push(ValidationDiag {
                    level: Level::Error,
                    message: format!(
                        "interface {}.{}: raises unknown exception '{}'",
                        i.name, op.name, ex
                    ),
                });
            }
        }
    }
}
