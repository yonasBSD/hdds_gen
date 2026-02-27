// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Fixed-point decimal support for Rust.
//!
//! Emits the `Fixed<D, S>` struct template for IDL fixed types.

use super::RustGenerator;

impl RustGenerator {
    pub(super) fn emit_fixed_support(output: &mut String) {
        output.push_str("// Fixed-point decimal with const generics (digits, scale)\n");
        output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        output.push_str("pub struct Fixed<const D: u32, const S: u32>(pub i128);\n");
        output.push_str("impl<const D: u32, const S: u32> Fixed<D, S> {\n");
        output.push_str("    #[inline] pub const fn from_raw(v: i128) -> Self { Self(v) }\n");
        output.push_str("    #[inline] pub const fn raw(&self) -> i128 { self.0 }\n");
        output.push_str(
            "    #[inline] pub const fn pow10() -> i128 { let mut p: i128 = 1; let mut i = 0; while i < S { p *= 10; i += 1; } p }\n",
        );
        output.push_str("    /// Build from integer and fractional parts (frac in [0, 10^S))\n");
        output.push_str(
            "    pub const fn from_parts(int: i128, frac: i128) -> Self { let base = Self::pow10(); let sign = if int < 0 { -1 } else { 1 }; Self(int * base + (sign as i128) * frac) }\n",
        );
        output.push_str(
            "    #[inline] pub fn to_f64(self) -> f64 { (self.0 as f64) / (Self::pow10() as f64) }\n",
        );
        output.push_str("}\n\n");
        output.push_str("impl<const D: u32, const S: u32> core::fmt::Display for Fixed<D, S> {\n");
        output.push_str(
            "    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n",
        );
        output.push_str(
            "        let base = Self::pow10();\n        let neg = self.0 < 0;\n        let mut v = if neg { -self.0 } else { self.0 };\n        let intp = v / base;\n        let fracp = (v % base) as u128;\n        if S == 0 { if neg { write!(f, \"-{}\", intp) } else { write!(f, \"{}\", intp) } } else { let width = S as usize; if neg { write!(f, \"-{}.{:0width$}\", intp, fracp) } else { write!(f, \"{}.{:0width$}\", intp, fracp) } }\n    }\n}",
        );
        output.push('\n');
        output.push_str("impl<const D: u32, const S: u32> Cdr2Encode for Fixed<D, S> {\n");
        output.push_str(
            "    fn encode_cdr2_le(&self, dst: &mut [u8]) -> Result<usize, CdrError> {\n",
        );
        output.push_str("        if dst.len() < 16 { return Err(CdrError::BufferTooSmall); }\n");
        output.push_str("        dst[..16].copy_from_slice(&self.raw().to_le_bytes());\n");
        output.push_str("        Ok(16)\n");
        output.push_str("    }\n");
        output.push_str("    fn max_cdr2_size(&self) -> usize { 16 }\n");
        output.push_str("}\n\n");
        output.push_str("impl<const D: u32, const S: u32> Cdr2Decode for Fixed<D, S> {\n");
        output.push_str("    fn decode_cdr2_le(src: &[u8]) -> Result<(Self, usize), CdrError> {\n");
        output.push_str("        if src.len() < 16 { return Err(CdrError::UnexpectedEof); }\n");
        output.push_str(
            "        let raw = {\n            let mut __hdds_tmp = [0u8; 16];\n            __hdds_tmp.copy_from_slice(&src[..16]);\n            i128::from_le_bytes(__hdds_tmp)\n        };\n",
        );
        output.push_str("        Ok((Fixed::<D, S>::from_raw(raw), 16))\n");
        output.push_str("    }\n");
        output.push_str("}\n");
        output.push('\n');
        output.push_str("impl<const D: u32, const S: u32> Default for Fixed<D, S> {\n");
        output.push_str("    fn default() -> Self { Self::from_raw(0) }\n");
        output.push_str("}\n");
    }
}
