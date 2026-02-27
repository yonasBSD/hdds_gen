// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Validation rules for specific IDL constructs.
//!
//! Submodules implement semantic checks for structs, enums, unions, etc.

#![allow(clippy::redundant_pub_crate)]

pub(crate) mod bitmasks;
pub(crate) mod bitsets;
pub(crate) mod enums;
mod helpers;
pub(crate) mod structs;
pub(crate) mod typedefs;
pub(crate) mod unions;

#[cfg(feature = "interfaces")]
pub(crate) mod interfaces;

pub(super) use bitmasks::validate_bitmask;
pub(super) use bitsets::validate_bitset;
pub(super) use enums::validate_enum;
pub(super) use structs::validate_struct;
pub(super) use typedefs::validate_typedef;
pub(super) use unions::validate_union;

#[cfg(feature = "interfaces")]
pub(super) use interfaces::validate_interface;
