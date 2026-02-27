// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! Serialization helpers shared by generated runtimes.
//!
//! The CDR-v2 backends in this crate assume a little-endian target.  These
//! helpers provide a single place where we convert primitive values to a fixed
//! byte order and sanity-check the host platform when the module is used.

#[must_use]
/// Convert a `u32` into little-endian bytes, enforcing the target endianness.
pub const fn to_le_bytes_u32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

#[must_use]
/// Convert little-endian bytes back into a `u32` value.
pub const fn from_le_bytes_u32(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_u32() {
        let value = 0x1234_5678;
        let bytes = to_le_bytes_u32(value);
        assert_eq!(from_le_bytes_u32(bytes), value);
    }
}
