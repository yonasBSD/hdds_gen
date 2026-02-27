// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Symbol collection for validation.
//!
//! Collects type definitions and annotations for validation lookup.

use crate::ast::{AnnotationDecl, Definition, IdlFile};
use crate::types::IdlType;
use std::collections::HashMap;
#[cfg(feature = "interfaces")]
use std::collections::HashSet;

pub(super) fn collect_annotation_decls(ast: &IdlFile) -> HashMap<String, AnnotationDecl> {
    let mut map = HashMap::new();
    collect_annotation_decl_walk(&ast.definitions, &mut map);
    map
}

fn collect_annotation_decl_walk(defs: &[Definition], map: &mut HashMap<String, AnnotationDecl>) {
    for d in defs {
        match d {
            Definition::AnnotationDecl(ad) => {
                map.insert(ad.name.clone(), ad.clone());
            }
            Definition::Module(m) => {
                collect_annotation_decl_walk(&m.definitions, map);
            }
            _ => {}
        }
    }
}

pub(super) fn collect_typedefs(ast: &IdlFile, prefix: &str, out: &mut HashMap<String, IdlType>) {
    for d in &ast.definitions {
        match d {
            Definition::Typedef(td) => {
                let fq = if prefix.is_empty() {
                    td.name.clone()
                } else {
                    format!("{}::{}", prefix, td.name)
                };
                out.insert(fq.clone(), td.base_type.clone());
                out.insert(td.name.clone(), td.base_type.clone());
            }
            Definition::Module(m) => {
                let new_prefix = if prefix.is_empty() {
                    m.name.clone()
                } else {
                    format!("{}::{}", prefix, m.name)
                };
                collect_typedefs(
                    &IdlFile {
                        definitions: m.definitions.clone(),
                    },
                    &new_prefix,
                    out,
                );
            }
            _ => {}
        }
    }
}

pub(super) fn resolve_typedef(name: &str, typedefs: &HashMap<String, IdlType>) -> Option<IdlType> {
    if let Some(t) = typedefs.get(name) {
        return Some(t.clone());
    }
    if let Some(last) = name.rsplit("::").next() {
        if let Some(t) = typedefs.get(last) {
            return Some(t.clone());
        }
    }
    None
}

#[cfg(feature = "interfaces")]
pub(super) fn collect_exceptions(ast: &IdlFile) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();
    collect_exceptions_walk(&ast.definitions, "", &mut set);
    set
}

#[cfg(feature = "interfaces")]
fn collect_exceptions_walk(defs: &[Definition], prefix: &str, set: &mut HashSet<String>) {
    for d in defs {
        match d {
            Definition::Exception(e) => {
                let fq = if prefix.is_empty() {
                    e.name.clone()
                } else {
                    format!("{}::{}", prefix, e.name)
                };
                set.insert(fq);
                set.insert(e.name.clone());
            }
            Definition::Module(m) => {
                let new_prefix = if prefix.is_empty() {
                    m.name.clone()
                } else {
                    format!("{}::{}", prefix, m.name)
                };
                collect_exceptions_walk(&m.definitions, &new_prefix, set);
            }
            _ => {}
        }
    }
}
