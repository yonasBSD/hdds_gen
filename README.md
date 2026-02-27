<p align="center">
  <img src="hdds-logo.png" alt="HDDS" width="160">
</p>
# hdds_gen

[![CI](https://git.hdds.io/hdds/hdds_gen/actions/workflows/ci.yml/badge.svg)](https://git.hdds.io/hdds/hdds_gen/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![IDL](https://img.shields.io/badge/OMG%20IDL-4.2-green.svg)](https://www.omg.org/spec/IDL/)
[![Backends](https://img.shields.io/badge/backends-6-blue.svg)](#code-generation-backends)
[![Lines](https://img.shields.io/badge/lines-~22k-informational.svg)](#statistics)

High-assurance OMG IDL 4.2 parser and multi-language code generator for DDS (Data Distribution Service) applications.

## Overview

`hdds_gen` is a Rust-based toolchain that parses OMG IDL 4.2 files and generates serialization code for multiple target languages. It provides CDR2 (Common Data Representation version 2) encode/decode implementations suitable for DDS middleware interoperability.

**Key capabilities:**

- Full OMG IDL 4.2 parser with preprocessor support
- Code generation for 6 target backends
- Semantic validation with detailed diagnostics
- Pretty-printer for IDL formatting
- CLI tool with subcommands for parsing, generation, validation, and formatting

## Supported IDL Types

### Primitive Types

| IDL Type | Description |
|----------|-------------|
| `boolean` | Boolean value |
| `char`, `wchar` | 8-bit and wide characters |
| `octet` | 8-bit unsigned |
| `short`, `unsigned short` | 16-bit signed/unsigned |
| `long`, `unsigned long` | 32-bit signed/unsigned |
| `long long`, `unsigned long long` | 64-bit signed/unsigned |
| `float`, `double`, `long double` | Floating-point types |
| `string`, `wstring` | Unbounded strings |
| `string<N>`, `wstring<N>` | Bounded strings |
| `int8`, `int16`, `int32`, `int64` | Fixed-width signed integers |
| `uint8`, `uint16`, `uint32`, `uint64` | Fixed-width unsigned integers |
| `fixed<D,S>` | Fixed-point decimal (D digits, S scale) |
| `void` | Void type (for operations) |

### Constructed Types

| Type | Description |
|------|-------------|
| `struct` | Aggregated data structure with optional inheritance |
| `enum` | Enumeration with optional explicit values |
| `union` | Discriminated union with case labels |
| `typedef` | Type alias with annotation support |
| `bitset` | Packed bit fields with explicit widths |
| `bitmask` | Named flag constants |
| `const` | Constant definitions |
| `module` | Namespace scoping |
| `forward declaration` | Forward struct/union declarations |
| `@annotation` | Custom annotation declarations |

### Container Types

| Type | Description |
|------|-------------|
| `sequence<T>` | Unbounded sequence |
| `sequence<T, N>` | Bounded sequence (max N elements) |
| `T[N]` | Fixed-size array |
| `map<K, V>` | Unbounded map |
| `map<K, V, N>` | Bounded map (max N entries) |

### Interfaces (Feature-Gated)

With `--features interfaces`:

| Type | Description |
|------|-------------|
| `interface` | Interface with operations and attributes |
| `exception` | Exception type declarations |
| `oneway` | One-way operations |
| `in/out/inout` | Parameter direction qualifiers |
| `raises` | Exception specifications |

## Supported Annotations

### DDS/XTYPES Standard Annotations

| Annotation | Target | Description |
|------------|--------|-------------|
| `@key` | Field | Marks field as part of topic key |
| `@optional` | Field | Field may be absent |
| `@id(N)` | Field | Explicit member ID |
| `@autoid(SEQUENTIAL\|HASH)` | Type | Auto-generate member IDs |
| `@extensibility(FINAL\|APPENDABLE\|MUTABLE)` | Type | Type evolution policy |
| `@final` | Type | Shorthand for FINAL extensibility |
| `@appendable` | Type | Shorthand for APPENDABLE extensibility |
| `@mutable` | Type | Shorthand for MUTABLE extensibility |
| `@must_understand` | Field | Reader must understand this field |
| `@nested` | Type | Nested type (no topic) |
| `@external` | Field | External reference |
| `@default_literal` | Enum | Default discriminator value |
| `@default` | Union case | Default union case |
| `@position(N)` | Bitset/Bitmask | Explicit bit position |
| `@bit_bound(N)` | Enum/Bitmask | Maximum bit width |
| `@data_representation(XCDR1\|XCDR2)` | Type | Wire format selection |
| `@non_serialized` | Field | Exclude from serialization |

### Documentation Annotations

| Annotation | Description |
|------------|-------------|
| `@unit("...")` | Unit of measurement |
| `@min(N)` | Minimum value constraint |
| `@max(N)` | Maximum value constraint |
| `@range(min=N, max=M)` | Value range constraint |
| `@value(...)` | Default value |
| `@verbatim(...)` | Language-specific code injection |

### Interface Annotations

| Annotation | Description |
|------------|-------------|
| `@service` | Mark interface as service |
| `@oneway` | One-way operation (no reply) |
| `@ami` | Asynchronous method invocation |

### Custom Annotations

User-defined annotations via `@annotation` declarations with typed members and default values.

## Code Generation Backends

### Rust (`rust`)

- Idiomatic Rust structs with `#[derive(Debug, Clone, PartialEq)]`
- CDR2 serialization via `Cdr2Encode` / `Cdr2Decode` traits
- `Option<T>` for `@optional` fields
- `Vec<T>` for sequences, `HashMap<K,V>` for maps
- Module namespacing preserved
- PL-CDR2 support for mutable/appendable types

### C++ (`cpp`)

- C++17 compatible headers
- STL containers (`std::vector`, `std::map`, `std::array`, `std::string`)
- Inline CDR2 encode/decode methods
- Namespace wrapping via `--namespace-cpp`
- Compatible with FastDDS, Cyclone DDS, RTI Connext

### C (`c`)

- C99/C11 compatible header-only output
- Static inline encode/decode functions
- Struct definitions with explicit padding
- Type descriptors for runtime introspection

### Python (`python`)

- Python 3.7+ with `@dataclass` decorators
- Type hints via `typing` module
- `IntEnum` for enumerations
- CDR2 `encode_cdr2_le()` / `decode_cdr2_le()` methods
- `compute_key()` for `@key` field hashing

### Micro (`micro`) - no_std Rust

- `#![no_std]` compatible for embedded targets
- Uses `heapless::Vec` and `heapless::String`
- Inline CDR encode/decode (no trait dispatch)
- Configurable buffer sizes
- Target: bare-metal Rust with `hdds-micro` crate

### C-Micro (`c-micro`) - Header-Only C for MCUs

- C89/C99 compatible, no dynamic allocation
- Fixed-size buffers with compile-time bounds
- Target: STM32, AVR, PIC, ESP32, any MCU with C compiler
- Minimal runtime footprint

## CLI Usage

```bash
# Install
cargo install --path .

# Parse and validate
hddsgen parse input.idl
hddsgen parse input.idl --pretty      # Pretty-print parsed IDL
hddsgen parse input.idl --json        # JSON diagnostics

# Check (validation only, CI-friendly exit codes)
hddsgen check input.idl
hddsgen check input.idl --json

# Generate code
hddsgen gen rust input.idl -o output.rs
hddsgen gen cpp input.idl -o output.hpp
hddsgen gen c input.idl -o output.h
hddsgen gen python input.idl -o output.py
hddsgen gen micro input.idl -o output.rs
hddsgen gen c-micro input.idl -o output.h

# Generate with namespace (C++)
hddsgen gen cpp input.idl --namespace-cpp MyApp::Types -o output.hpp

# Generate full project with examples
hddsgen gen rust input.idl --example --out-dir ./my_project
hddsgen gen cpp input.idl --example --out-dir ./my_project --build-system cmake

# Format IDL
hddsgen fmt input.idl -o formatted.idl

# Include directories for #include resolution
hddsgen parse main.idl -I ./includes -I /usr/share/idl
```

### Subcommands

| Command | Description |
|---------|-------------|
| `parse` | Parse and validate IDL, optionally pretty-print |
| `gen` | Generate code for target language |
| `check` | Validate only (returns non-zero on error) |
| `fmt` | Reformat IDL via pretty-printer |

### Generation Options

| Option | Description |
|--------|-------------|
| `-o, --out <FILE>` | Output file (stdout if omitted) |
| `--out-dir <DIR>` | Output directory for module files |
| `--namespace-cpp <NS>` | C++ namespace (e.g., `A::B::C`) |
| `--example` | Generate full project with publisher/subscriber examples |
| `--build-system <TYPE>` | Build system: `cargo`, `cmake`, `make` |
| `--hdds-path <PATH>` | Path to hdds crate for Rust examples |
| `-I, --include <DIR>` | Include directory for `#include` resolution |

## Preprocessor

Full C-style preprocessor with:

- `#include "file.idl"` and `#include <file.idl>`
- `#define NAME value` and `#define MACRO(args) body`
- `#ifdef`, `#ifndef`, `#if`, `#elif`, `#else`, `#endif`
- `#undef`
- Cycle detection for include guards
- Macro expansion with function-like macros
- Token pasting (`##`) and stringification (`#`)

## Validation Rules

The validator enforces semantic correctness:

### Struct Rules
- No duplicate field names
- Valid type references
- `@key` only on serializable fields
- Extensibility annotation conflicts

### Enum Rules
- No duplicate enumerator names
- Explicit values within `@bit_bound` limits

### Union Rules
- No duplicate case labels
- At most one `default` case
- Valid discriminator type
- `@default` annotation consistency

### Bitset Rules
- Bit positions must not overlap
- Total width within bounds
- Valid `@position` annotations

### Interface Rules (with feature)
- No duplicate operation names
- No operation/attribute name collisions
- Valid parameter types
- `raises` references existing exceptions
- `oneway` operations must return `void`

### Custom Annotations
- Parameters match declared annotation members
- Required parameters provided

## Project Architecture

```
src/
  lib.rs              # Public API exports
  ast.rs              # Abstract Syntax Tree types
  types.rs            # IDL type system (primitives, annotations)
  token.rs            # Lexer token definitions
  error.rs            # Error types and handling

  lexer/              # Lexical analysis
    mod.rs            # Lexer entry point
    scanner.rs        # Character scanning
    numbers.rs        # Numeric literal parsing
    state.rs          # Lexer state machine

  parser/             # Syntax analysis
    mod.rs            # Parser entry point
    annotations.rs    # Annotation parsing
    const_expr.rs     # Constant expression evaluation
    interfaces.rs     # Interface parsing (feature-gated)
    types.rs          # Type parsing
    definitions/      # Definition parsers
      structs.rs
      enums.rs
      unions.rs
      bitsets.rs
      bitmasks.rs
      typedefs.rs
      consts.rs
      module.rs
      forwards.rs

  validate/           # Semantic validation
    mod.rs            # Validation entry point
    engine.rs         # Validation orchestration
    rules/            # Validation rules
      structs.rs
      enums.rs
      unions.rs
      bitsets.rs
      interfaces.rs
    diagnostics.rs    # Diagnostic types
    references.rs     # Reference resolution

  codegen/            # Code generation
    mod.rs            # Backend trait and registry
    rust_backend/     # Rust code generator
    cpp/              # C++ code generator
    c/                # C code generator
    python.rs         # Python code generator
    micro/            # no_std Rust generator
    c_micro/          # Header-only C for MCUs
    examples.rs       # Example code generation
    examples_project.rs  # Full project scaffolding

  pretty/             # Pretty-printer
    mod.rs            # Formatter entry point
    formatter.rs      # Core formatting logic
    structs.rs        # Struct formatting
    enums.rs          # Enum formatting
    unions.rs         # Union formatting
    bitsets.rs        # Bitset formatting
    modules.rs        # Module formatting
    interfaces.rs     # Interface formatting (feature-gated)

  bin/
    hddsgen.rs        # CLI entry point
    hddsgen/
      commands.rs     # Subcommand implementations
      preprocessor.rs # Preprocessor implementation

tests/                # Integration tests
examples/             # Example IDL files
  canonical/          # Reference test cases
  invalid/            # Expected-failure cases
  include/            # Include resolution tests
  macros/             # Preprocessor tests
  interfaces/         # Interface feature tests
```

## Build and Test

```bash
# Build
make build              # Debug build
make release            # Release build

# Test
make test               # Unit tests
make validate-ci        # Full CI validation suite

# Code quality
make fmt                # Format code
make clippy             # Run linter
make doc                # Generate documentation

# Install
make install            # Install to ~/.cargo/bin
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `interfaces` | Enable interface/exception parsing and pretty-printing |

```bash
# Build with interfaces support
cargo build --features interfaces
```

## Examples

### Basic IDL

```idl
@extensibility(APPENDABLE)
struct HelloWorld {
    unsigned long index;
    string message;
};
```

### Advanced IDL

```idl
module Comp {
    @appendable
    struct Msg {
        @key int32_t id;
        @optional string content;
        string<16> name;
        sequence<int32_t, 10> values;
    };

    enum Color { Red = 0, Green = 1, Blue = 2 };

    typedef map<string, int32_t, 100> ConfigMap;

    bitset Flags {
        bitfield<3> mode;
        bitfield<5> value, @position(4);
    };

    bitmask Permissions { Read, Write, Execute };

    union Data switch(int32_t) {
        case 1: int32_t integer;
        default: octet raw;
    };

    const int32_t MAGIC = 42;
};
```

### Generated Rust Usage

```rust
use hdds::{Cdr2Encode, Cdr2Decode};

let msg = Comp::Msg {
    id: 1,
    content: Some("Hello".to_string()),
    name: "test".to_string(),
    values: vec![1, 2, 3],
};

let mut buffer = [0u8; 256];
let size = msg.encode_cdr2_le(&mut buffer)?;
let (decoded, _) = Comp::Msg::decode_cdr2_le(&buffer)?;
assert_eq!(msg, decoded);
```

## Statistics

- ~22,000 lines of Rust code
- 6 code generation backends
- 60+ example IDL files
- Comprehensive validation suite

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Copyright (c) 2025-2026 naskel.com

## Repository

https://git.hdds.io/hdds/hdds_gen.git
