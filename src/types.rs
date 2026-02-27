// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

//! IDL type system
//!
//! Defines the type system according to OMG IDL 4.2 specification,
//! including primitive types, annotations, and extensibility modes.

#[derive(Debug, Clone, PartialEq, Eq)]
/// IDL type representation
pub enum IdlType {
    /// Primitive types
    Primitive(PrimitiveType),

    /// User-defined type (by name, resolved later)
    Named(String),

    /// Sequence type with optional bound
    Sequence {
        inner: Box<Self>,
        bound: Option<u32>,
    },

    /// Map type (optional bound for bounded maps)
    Map {
        key: Box<Self>,
        value: Box<Self>,
        bound: Option<u32>,
    },

    /// Array type with fixed size
    Array { inner: Box<Self>, size: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Primitive types in IDL
pub enum PrimitiveType {
    Void,
    Boolean,
    Char,
    WChar,
    Octet,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
    String,
    WString,
    // IDL 4.x fixed-width types
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    /// Fixed-point decimal: fixed<digits, scale>
    Fixed {
        digits: u32,
        scale: u32,
    },
}

impl PrimitiveType {
    /// Get the C++ equivalent type name
    #[must_use]
    pub const fn to_cpp_name(&self) -> &'static str {
        match self {
            Self::Void => "void",
            Self::Boolean => "bool",
            Self::Char => "char",
            Self::WChar => "wchar_t",
            Self::Octet | Self::UInt8 => "uint8_t",
            Self::Short | Self::Int16 => "int16_t",
            Self::UnsignedShort | Self::UInt16 => "uint16_t",
            Self::Long | Self::Int32 => "int32_t",
            Self::UnsignedLong | Self::UInt32 => "uint32_t",
            Self::LongLong | Self::Int64 => "int64_t",
            Self::UnsignedLongLong | Self::UInt64 => "uint64_t",
            Self::Float => "float",
            Self::Double => "double",
            Self::LongDouble | Self::Fixed { .. } => "long double",
            Self::String => "std::string",
            Self::WString => "std::wstring",
            Self::Int8 => "int8_t",
        }
    }

    /// Get the original IDL type name
    #[must_use]
    pub const fn to_idl_string(&self) -> &'static str {
        match self {
            Self::Void => "void",
            Self::Boolean => "boolean",
            Self::Char => "char",
            Self::WChar => "wchar",
            Self::Octet => "octet",
            Self::Short => "short",
            Self::UnsignedShort => "unsigned short",
            Self::Long => "long",
            Self::UnsignedLong => "unsigned long",
            Self::LongLong => "long long",
            Self::UnsignedLongLong => "unsigned long long",
            Self::Float => "float",
            Self::Double => "double",
            Self::LongDouble => "long double",
            Self::String => "string",
            Self::WString => "wstring",
            Self::Int8 => "int8_t",
            Self::Int16 => "int16_t",
            Self::Int32 => "int32_t",
            Self::Int64 => "int64_t",
            Self::UInt8 => "uint8_t",
            Self::UInt16 => "uint16_t",
            Self::UInt32 => "uint32_t",
            Self::UInt64 => "uint64_t",
            Self::Fixed {
                digits: _,
                scale: _,
            } => {
                // Render as fixed<digits,scale>
                // Note: for bounded strings we reuse sequence<char>; here we keep primitive string
                // formatting to preserve intent in comments
                // This string is used by to_idl_string on IdlType::Primitive
                // and shows original IDL form in comments.
                // Use an owned String via formatting caller side when needed.
                // Here we just return a placeholder (will not be used for exact formatting).
                "fixed" // caller not using parameters directly for IDL source
            }
        }
    }
}

impl IdlType {
    /// Convert type to its IDL string representation
    #[must_use]
    pub fn to_idl_string(&self) -> String {
        match self {
            Self::Primitive(p) => p.to_idl_string().to_string(),
            Self::Named(name) => name.clone(),
            Self::Sequence { inner, bound } => bound.as_ref().map_or_else(
                || format!("sequence<{}>", inner.to_idl_string()),
                |b| format!("sequence<{}, {}>", inner.to_idl_string(), b),
            ),
            Self::Map { key, value, bound } => bound.as_ref().map_or_else(
                || format!("map<{}, {}>", key.to_idl_string(), value.to_idl_string()),
                |n| {
                    format!(
                        "map<{}, {}, {}>",
                        key.to_idl_string(),
                        value.to_idl_string(),
                        n
                    )
                },
            ),
            Self::Array { inner, size } => format!("{}[{}]", inner.to_idl_string(), size),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Annotations as defined in IDL 4.2 spec (Section 8.3)
pub enum Annotation {
    /// @key - marks a field as part of the key (Section 8.3.2.1)
    Key,

    /// @id - explicitly sets the member ID
    Id(u32),

    /// @autoid - auto-generate member IDs
    AutoId(AutoIdKind),

    /// @optional - marks a field as optional
    Optional,

    /// @position - explicitly sets field position
    Position(u32),

    /// @value - sets default value
    Value(String),

    /// @extensibility - controls type evolution
    Extensibility(ExtensibilityKind),

    /// @final - shorthand for FINAL extensibility
    Final,

    /// @appendable - shorthand for APPENDABLE extensibility
    Appendable,

    /// @mutable - shorthand for MUTABLE extensibility
    Mutable,

    /// @`must_understand` - field must be understood by reader
    MustUnderstand,

    /// @`default_literal` - default value for discriminator
    DefaultLiteral,

    /// @default - marks default case in union
    Default,

    /// @range - specify value range
    Range { min: String, max: String },

    /// @min - minimum value
    Min(String),

    /// @max - maximum value
    Max(String),

    /// @unit - specify unit of measurement
    Unit(String),

    /// @`bit_bound` - bit bound for integers
    BitBound(u32),

    /// @external - mark as external type
    External,

    /// @nested - mark as nested type
    Nested,

    /// @`data_representation` - select wire representation (e.g., XCDR1, XCDR2, `PLAIN_CDR`, `PLAIN_CDR2`)
    DataRepresentation(String),

    /// @`non_serialized` - mark a member as not serialized
    NonSerialized,

    /// @verbatim - language-specific code injection
    Verbatim {
        language: String,
        placement: String,
        text: String,
    },

    /// @service - mark interface as service
    Service,

    /// @oneway - mark operation as oneway
    Oneway,

    /// @ami - asynchronous method invocation
    Ami,

    /// @topic - marks a struct as a DDS topic type
    Topic,

    /// Custom/unknown annotation
    Custom {
        name: String,
        params: Vec<(String, String)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// `AutoId` kinds (Section 8.3.1.2)
pub enum AutoIdKind {
    Sequential,
    Hash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Extensibility kinds (Section 8.3.1.6-8.3.1.9)
pub enum ExtensibilityKind {
    /// FINAL - no new members can be added
    Final,
    /// APPENDABLE - new members can be appended
    Appendable,
    /// MUTABLE - members can be added/removed freely
    Mutable,
}

impl std::fmt::Display for ExtensibilityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Final => write!(f, "FINAL"),
            Self::Appendable => write!(f, "APPENDABLE"),
            Self::Mutable => write!(f, "MUTABLE"),
        }
    }
}
