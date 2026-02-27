// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Type reference validation.
//!
//! Validates that all user-defined type references can be resolved.

use super::diagnostics::{Level, ValidationDiag};
use crate::ast::{Definition, IdlFile};
use crate::types::IdlType;

enum NameRes {
    Ok,
    Unresolved,
    Ambiguous(Vec<String>),
}

pub(super) fn validate_references(ast: &IdlFile, diags: &mut Vec<ValidationDiag>) {
    use std::collections::{HashMap, HashSet};
    // Build a map of fully qualified names to a marker
    let mut defs: HashSet<String> = HashSet::new();
    collect_definitions(ast, "", &mut defs);

    // Track unresolved and ambiguous names (de-dup)
    let mut unresolved: HashSet<String> = HashSet::new();
    let mut ambiguous: HashMap<String, Vec<String>> = HashMap::new();

    walk_references(ast, &defs, &mut unresolved, &mut ambiguous, "");
    for name in unresolved {
        diags.push(ValidationDiag {
            level: Level::Error,
            message: format!("Unresolved type reference: {name}"),
        });
    }
    for (name, cands) in ambiguous {
        let list = cands.join(", ");
        diags.push(ValidationDiag {
            level: Level::Error,
            message: format!("Ambiguous type reference: {name} (candidates: {list})"),
        });
    }
}

fn check_reference_type(
    t: &IdlType,
    defs: &std::collections::HashSet<String>,
    unresolved: &mut std::collections::HashSet<String>,
    ambiguous: &mut std::collections::HashMap<String, Vec<String>>,
    context_module: &str,
) {
    match t {
        IdlType::Named(name) => match resolve_name_ctx(name, defs, context_module) {
            NameRes::Ok => {}
            NameRes::Unresolved => {
                unresolved.insert(name.clone());
            }
            NameRes::Ambiguous(cands) => {
                ambiguous.entry(name.clone()).or_insert(cands);
            }
        },
        IdlType::Map { key, value, .. } => {
            check_reference_type(key, defs, unresolved, ambiguous, context_module);
            check_reference_type(value, defs, unresolved, ambiguous, context_module);
        }
        IdlType::Sequence { inner, .. } | IdlType::Array { inner, .. } => {
            check_reference_type(inner, defs, unresolved, ambiguous, context_module);
        }
        IdlType::Primitive(_) => {}
    }
}

fn walk_references(
    file: &IdlFile,
    defs: &std::collections::HashSet<String>,
    unresolved: &mut std::collections::HashSet<String>,
    ambiguous: &mut std::collections::HashMap<String, Vec<String>>,
    context_module: &str,
) {
    for d in &file.definitions {
        match d {
            Definition::Struct(s) => {
                for f in &s.fields {
                    check_reference_type(
                        &f.field_type,
                        defs,
                        unresolved,
                        ambiguous,
                        context_module,
                    );
                }
            }
            Definition::Union(u) => {
                check_reference_type(
                    &u.discriminator,
                    defs,
                    unresolved,
                    ambiguous,
                    context_module,
                );
                for c in &u.cases {
                    check_reference_type(
                        &c.field.field_type,
                        defs,
                        unresolved,
                        ambiguous,
                        context_module,
                    );
                }
            }
            Definition::Typedef(t) => {
                check_reference_type(&t.base_type, defs, unresolved, ambiguous, context_module);
            }
            Definition::Module(m) => {
                let new_ctx = if context_module.is_empty() {
                    m.name.clone()
                } else {
                    format!("{}::{}", context_module, m.name)
                };
                walk_references(
                    &IdlFile {
                        definitions: m.definitions.clone(),
                    },
                    defs,
                    unresolved,
                    ambiguous,
                    &new_ctx,
                );
            }
            _ => {}
        }
    }
}

fn collect_definitions(ast: &IdlFile, prefix: &str, defs: &mut std::collections::HashSet<String>) {
    for d in &ast.definitions {
        match d {
            Definition::Module(m) => {
                let new_prefix = if prefix.is_empty() {
                    m.name.clone()
                } else {
                    format!("{}::{}", prefix, m.name)
                };
                collect_definitions(
                    &IdlFile {
                        definitions: m.definitions.clone(),
                    },
                    &new_prefix,
                    defs,
                );
            }
            Definition::Struct(s) => {
                let name = if prefix.is_empty() {
                    s.name.clone()
                } else {
                    format!("{}::{}", prefix, s.name)
                };
                let _ = defs.insert(name);
            }
            Definition::Enum(e) => {
                let name = if prefix.is_empty() {
                    e.name.clone()
                } else {
                    format!("{}::{}", prefix, e.name)
                };
                let _ = defs.insert(name);
            }
            Definition::Union(u) => {
                let name = if prefix.is_empty() {
                    u.name.clone()
                } else {
                    format!("{}::{}", prefix, u.name)
                };
                let _ = defs.insert(name);
            }
            Definition::Typedef(t) => {
                let name = if prefix.is_empty() {
                    t.name.clone()
                } else {
                    format!("{}::{}", prefix, t.name)
                };
                let _ = defs.insert(name);
            }
            Definition::Bitset(b) => {
                let name = if prefix.is_empty() {
                    b.name.clone()
                } else {
                    format!("{}::{}", prefix, b.name)
                };
                let _ = defs.insert(name);
            }
            Definition::Bitmask(m) => {
                let name = if prefix.is_empty() {
                    m.name.clone()
                } else {
                    format!("{}::{}", prefix, m.name)
                };
                let _ = defs.insert(name);
            }
            _ => {}
        }
    }
}

fn resolve_name_ctx(
    name: &str,
    defs: &std::collections::HashSet<String>,
    context_module: &str,
) -> NameRes {
    // Fully-qualified? Must match exactly
    if name.contains("::") {
        return if defs.contains(name) {
            NameRes::Ok
        } else {
            NameRes::Unresolved
        };
    }
    // Unqualified: filter by last segment, then rank by common module prefix with context
    let last = name;
    let mut matches: Vec<String> = defs
        .iter()
        .filter(|k| k.rsplit("::").next() == Some(last))
        .cloned()
        .collect();
    match matches.len() {
        0 => NameRes::Unresolved,
        1 => NameRes::Ok,
        _ => {
            let ctx: Vec<&str> = if context_module.is_empty() {
                Vec::new()
            } else {
                context_module.split("::").collect()
            };
            matches.sort_by(|a, b| {
                let sa = common_prefix_len(&ctx, a);
                let sb = common_prefix_len(&ctx, b);
                sb.cmp(&sa).then_with(|| a.cmp(b))
            });
            NameRes::Ambiguous(matches)
        }
    }
}

fn common_prefix_len(ctx: &[&str], fqn: &str) -> usize {
    let mut score = 0usize;
    for (i, seg) in ctx.iter().enumerate() {
        if let Some(part) = fqn.split("::").nth(i) {
            if part == *seg {
                score = i + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    score
}
