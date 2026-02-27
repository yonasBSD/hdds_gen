## Description
Brief description of changes.

## Related Issue
Fixes #(issue number)

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] New backend (adds code generation for a new language)
- [ ] Parser enhancement (new IDL 4.2 feature support)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Checklist
- [ ] My code follows the project's style guidelines (`cargo fmt`)
- [ ] I have run `cargo clippy` and fixed all warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] All new and existing tests pass (`cargo test --all`)
- [ ] I have tested with real IDL files from `examples/`
- [ ] I have updated the documentation accordingly
- [ ] Generated code compiles and works correctly

## Testing
Describe how you tested these changes:

```bash
# Commands used to test
hddsgen gen rust examples/HelloWorld.idl -o /tmp/test.rs
```

## Sample IDL Tested
```idl
// IDL used for testing
```

## Generated Code Sample
```rust
// Sample of generated code (if applicable)
```

## Screenshots (if applicable)
Add screenshots to help explain your changes.
