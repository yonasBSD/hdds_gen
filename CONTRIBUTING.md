# Contributing to hdds_gen

First off, thank you for considering contributing to hdds_gen! It's people like you that make hdds_gen such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by the [hdds_gen Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Provide the IDL file** that causes the issue (minimal reproducer)
- **Describe the expected vs actual output**
- **Include the full error message**
- **Specify the backend** (rust, c, python, etc.)
- **Environment details**: OS, Rust version, hdds_gen version

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the suggested enhancement
- **Include IDL examples** if proposing new type support
- **Explain why this enhancement would be useful**

### Adding a New Backend

If you want to add a new code generation backend:

1. Study the existing backends in `src/codegen/`
2. Implement the `CodeGenerator` trait
3. Add comprehensive tests with IDL samples
4. Document the generated code format
5. Add examples in `examples/`

### Pull Requests

1. **Fork the repo** and create your branch from `master`
2. **Follow the coding style** (run `cargo fmt` and `cargo clippy`)
3. **Add tests** for any new functionality
4. **Ensure all tests pass**: `cargo test --all`
5. **Test with real IDL files** from `examples/`
6. **Update documentation** if needed

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/hdds_gen.git
cd hdds_gen

# Build
cargo build --release

# Run tests
cargo test --all

# Test IDL parsing
./target/release/hddsgen check examples/HelloWorld.idl

# Test code generation
./target/release/hddsgen gen rust examples/HelloWorld.idl -o /tmp/hello.rs

# Run clippy
cargo clippy --all -- -D warnings

# Format code
cargo fmt --all
```

## Project Structure

```
hdds_gen/
├── src/
│   ├── parser.rs     # IDL 4.2 lexer and parser
│   ├── ast.rs        # Abstract Syntax Tree definitions
│   ├── validate.rs   # Semantic analysis and validation
│   ├── codegen/      # Code generators (rust, c, cpp, python, typescript, c_micro)
│   └── bin/          # CLI binary (hddsgen)
├── examples/         # Sample IDL files
└── tests/            # Integration tests
```

## Coding Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` and fix all warnings
- Document public APIs with rustdoc comments

### Parser Changes

- Maintain compatibility with OMG IDL 4.2 specification
- Add tests for new grammar rules
- Update the grammar documentation

### Backend Changes

- Generated code must be readable and well-formatted
- Include comments in generated code referencing the source IDL
- Test interoperability with target DDS implementations

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues and pull requests after the first line

## License

By contributing, you agree that your contributions will be licensed under the same dual license as the project (Apache-2.0 OR MIT).

## Questions?

Feel free to open an issue with your question or reach out to the maintainers.

Thank you for contributing!
