// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! `TypeDescriptor` generation for C
//!
//! Generates metadata structures for DDS types, including:
//! - Type name
//! - Type kind (struct, enum, etc.)
//! - Field descriptions (name, type, `@key` status)
//! - Type hash computation

use crate::ast::Struct;
use crate::types::{Annotation, IdlType, PrimitiveType};
use std::fmt::Write;

/// Convert type name to lowercase `snake_case` for C function names
fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Generate `TypeDescriptor` struct and functions for a struct
#[must_use]
pub fn generate_type_descriptor(s: &Struct) -> String {
    let type_name = &s.name;
    let snake_name = to_snake_case(type_name);

    // Collect @key fields
    let key_field_count = s
        .fields
        .iter()
        .filter(|f| f.annotations.iter().any(|a| matches!(a, Annotation::Key)))
        .count();

    let mut out = String::new();

    // Header comment
    let _ = writeln!(out, "/* TypeDescriptor for {type_name} */\n");

    // Field descriptor struct (only if not already defined)
    out.push_str("#ifndef CDR_FIELD_DESCRIPTOR_DEFINED\n");
    out.push_str("#define CDR_FIELD_DESCRIPTOR_DEFINED\n");
    out.push_str("typedef struct cdr_field_descriptor {\n");
    out.push_str("    const char* name;      /* Field name */\n");
    out.push_str("    const char* type_name; /* Field type as string */\n");
    out.push_str("    uint8_t is_key;        /* 1 if @key annotated */\n");
    out.push_str("    uint8_t type_kind;     /* Type kind enum */\n");
    out.push_str("    uint32_t offset;       /* Offset in struct (for simple types) */\n");
    out.push_str("} cdr_field_descriptor_t;\n");
    out.push_str("#endif /* CDR_FIELD_DESCRIPTOR_DEFINED */\n\n");

    // Type kind enum (only if not already defined)
    out.push_str("#ifndef CDR_TYPE_KIND_DEFINED\n");
    out.push_str("#define CDR_TYPE_KIND_DEFINED\n");
    out.push_str("typedef enum cdr_type_kind {\n");
    out.push_str("    CDR_TYPE_PRIMITIVE = 0,\n");
    out.push_str("    CDR_TYPE_STRING = 1,\n");
    out.push_str("    CDR_TYPE_SEQUENCE = 2,\n");
    out.push_str("    CDR_TYPE_ARRAY = 3,\n");
    out.push_str("    CDR_TYPE_STRUCT = 4,\n");
    out.push_str("    CDR_TYPE_ENUM = 5,\n");
    out.push_str("    CDR_TYPE_UNION = 6,\n");
    out.push_str("    CDR_TYPE_MAP = 7,\n");
    out.push_str("} cdr_type_kind_t;\n");
    out.push_str("#endif /* CDR_TYPE_KIND_DEFINED */\n\n");

    // TypeDescriptor struct (only if not already defined)
    out.push_str("#ifndef CDR_TYPE_DESCRIPTOR_DEFINED\n");
    out.push_str("#define CDR_TYPE_DESCRIPTOR_DEFINED\n");
    out.push_str("typedef struct cdr_type_descriptor {\n");
    out.push_str("    const char* name;                  /* Type name */\n");
    out.push_str("    cdr_type_kind_t kind;              /* Type kind */\n");
    out.push_str("    uint32_t field_count;              /* Number of fields */\n");
    out.push_str("    uint32_t key_field_count;          /* Number of @key fields */\n");
    out.push_str("    const cdr_field_descriptor_t* fields; /* Field descriptors */\n");
    out.push_str("    uint8_t type_hash[16];             /* Type identifier hash */\n");
    out.push_str("} cdr_type_descriptor_t;\n");
    out.push_str("#endif /* CDR_TYPE_DESCRIPTOR_DEFINED */\n\n");

    // Field descriptors array
    let _ = writeln!(
        out,
        "static const cdr_field_descriptor_t {snake_name}_fields[] = {{"
    );
    for field in &s.fields {
        let is_key = i32::from(
            field
                .annotations
                .iter()
                .any(|a| matches!(a, Annotation::Key)),
        );
        let type_kind = get_type_kind(&field.field_type);
        let type_name_str = field.field_type.to_idl_string();
        let _ = writeln!(
            out,
            "    {{ \"{}\", \"{}\", {}, {}, 0 }},",
            field.name, type_name_str, is_key, type_kind
        );
    }
    out.push_str("};\n\n");

    // Type hash computation (FNV-1a based on type structure)
    let type_hash = compute_type_hash(s);
    let _ = writeln!(
        out,
        "/* Type hash for {} (computed from type structure) */",
        type_name
    );
    let _ = write!(
        out,
        "static const uint8_t {snake_name}_type_hash[16] = {{\n    "
    );
    for (i, byte) in type_hash.iter().enumerate() {
        if i > 0 && i % 8 == 0 {
            out.push_str("\n    ");
        }
        let _ = write!(out, "0x{:02x}", byte);
        if i < 15 {
            out.push_str(", ");
        }
    }
    out.push_str("\n};\n\n");

    // TypeDescriptor instance
    let _ = writeln!(
        out,
        "static const cdr_type_descriptor_t {snake_name}_type_descriptor = {{"
    );
    let _ = writeln!(out, "    \"{type_name}\",");
    out.push_str("    CDR_TYPE_STRUCT,\n");
    let _ = writeln!(out, "    {},", s.fields.len());
    let _ = writeln!(out, "    {key_field_count},");
    let _ = writeln!(out, "    {snake_name}_fields,");
    out.push_str("    { /* type_hash copied below */ }\n");
    out.push_str("};\n\n");

    // Getter function
    let _ = writeln!(
        out,
        "static inline const cdr_type_descriptor_t* {snake_name}_get_type_descriptor(void) {{"
    );
    let _ = writeln!(out, "    return &{snake_name}_type_descriptor;");
    out.push_str("}\n\n");

    // Type name getter
    let _ = writeln!(
        out,
        "static inline const char* {snake_name}_type_name(void) {{"
    );
    let _ = writeln!(out, "    return \"{type_name}\";");
    out.push_str("}\n\n");

    // Has key function
    let _ = writeln!(out, "static inline int {snake_name}_has_key(void) {{");
    let _ = writeln!(out, "    return {};", i32::from(key_field_count > 0));
    out.push_str("}\n\n");

    // Compute key function (if there are @key fields)
    let _ = writeln!(out, "/* Compute instance key hash for {type_name} */");
    let _ = writeln!(
        out,
        "static inline int {snake_name}_compute_key(const {type_name}* value, uint8_t key_hash[16]) {{"
    );
    out.push_str("    if (!value || !key_hash) return -1;\n");

    let key_fields: Vec<_> = s
        .fields
        .iter()
        .filter(|f| f.annotations.iter().any(|a| matches!(a, Annotation::Key)))
        .collect();

    if key_fields.is_empty() {
        out.push_str("    /* No @key fields - clear hash */\n");
        out.push_str("    memset(key_hash, 0, 16);\n");
        out.push_str("    return 0; /* No key */\n");
    } else {
        out.push_str("    /* FNV-1a hash of @key fields */\n");
        out.push_str("    uint64_t hash = 14695981039346656037ULL;\n");
        for field in &key_fields {
            out.push_str("    {\n");
            let _ = writeln!(
                out,
                "        const uint8_t* ptr = (const uint8_t*)&value->{};",
                field.name
            );
            let size = get_primitive_size(&field.field_type);
            let _ = writeln!(out, "        for (size_t i = 0; i < {}; ++i) {{", size);
            out.push_str("            hash ^= ptr[i];\n");
            out.push_str("            hash *= 1099511628211ULL;\n");
            out.push_str("        }\n");
            out.push_str("    }\n");
        }
        out.push_str("    /* Expand to 16 bytes */\n");
        out.push_str("    memset(key_hash, 0, 16);\n");
        out.push_str("    memcpy(key_hash, &hash, sizeof(hash));\n");
        out.push_str("    hash *= 1099511628211ULL;\n");
        out.push_str("    memcpy(key_hash + 8, &hash, sizeof(hash));\n");
        out.push_str("    return 1; /* Has key */\n");
    }
    out.push_str("}\n\n");

    out
}

/// Get the type kind enum value as a string
const fn get_type_kind(t: &IdlType) -> &'static str {
    match t {
        IdlType::Primitive(PrimitiveType::String | PrimitiveType::WString) => "CDR_TYPE_STRING",
        IdlType::Primitive(_) => "CDR_TYPE_PRIMITIVE",
        IdlType::Sequence { .. } => "CDR_TYPE_SEQUENCE",
        IdlType::Array { .. } => "CDR_TYPE_ARRAY",
        IdlType::Map { .. } => "CDR_TYPE_MAP",
        IdlType::Named(_) => "CDR_TYPE_STRUCT", // Could be struct or enum
    }
}

/// Get size of a primitive type for key hashing
const fn get_primitive_size(t: &IdlType) -> &'static str {
    match t {
        IdlType::Primitive(p) => match p {
            PrimitiveType::Boolean
            | PrimitiveType::Char
            | PrimitiveType::Octet
            | PrimitiveType::Int8
            | PrimitiveType::UInt8 => "1",
            PrimitiveType::Short
            | PrimitiveType::Int16
            | PrimitiveType::UnsignedShort
            | PrimitiveType::UInt16 => "2",
            PrimitiveType::Long
            | PrimitiveType::Int32
            | PrimitiveType::UnsignedLong
            | PrimitiveType::UInt32
            | PrimitiveType::Float
            | PrimitiveType::WChar => "4",
            PrimitiveType::LongLong
            | PrimitiveType::Int64
            | PrimitiveType::UnsignedLongLong
            | PrimitiveType::UInt64
            | PrimitiveType::Double
            | PrimitiveType::LongDouble => "8",
            PrimitiveType::Fixed { .. } => "16",
            PrimitiveType::String | PrimitiveType::WString => "sizeof(void*)",
            PrimitiveType::Void => "0",
        },
        _ => "sizeof(void*)", // For complex types
    }
}

/// Compute a type hash based on type structure (FNV-1a)
fn compute_type_hash(s: &Struct) -> [u8; 16] {
    let mut hash: u64 = 14_695_981_039_346_656_037;

    // Hash the type name
    for byte in s.name.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }

    // Hash each field
    for field in &s.fields {
        // Field name
        for byte in field.name.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(1_099_511_628_211);
        }
        // Field type
        for byte in field.field_type.to_idl_string().as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(1_099_511_628_211);
        }
        // @key annotation
        let is_key = field
            .annotations
            .iter()
            .any(|a| matches!(a, Annotation::Key));
        hash ^= u64::from(is_key);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }

    // Expand to 16 bytes
    let mut result = [0u8; 16];
    result[0..8].copy_from_slice(&hash.to_le_bytes());
    hash = hash.wrapping_mul(1_099_511_628_211);
    result[8..16].copy_from_slice(&hash.to_le_bytes());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Field;

    #[test]
    fn test_type_descriptor_generation() {
        let s = Struct {
            name: "TestMessage".to_string(),
            fields: vec![
                Field {
                    name: "id".to_string(),
                    field_type: IdlType::Primitive(PrimitiveType::UInt32),
                    annotations: vec![Annotation::Key],
                },
                Field {
                    name: "data".to_string(),
                    field_type: IdlType::Primitive(PrimitiveType::String),
                    annotations: vec![],
                },
            ],
            annotations: vec![],
            base_struct: None,
            extensibility: None,
        };

        let output = generate_type_descriptor(&s);
        assert!(output.contains("cdr_type_descriptor_t"));
        assert!(output.contains("test_message_fields"));
        assert!(output.contains("test_message_compute_key"));
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("XMLParser"), "x_m_l_parser");
        assert_eq!(to_snake_case("simple"), "simple");
    }
}
