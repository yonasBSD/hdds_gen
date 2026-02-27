---
name: Bug Report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

## Bug Description
A clear and concise description of what the bug is.

## IDL Input
```idl
// Minimal IDL that reproduces the issue
module Example {
    struct MyType {
        long value;
    };
};
```

## Command Used
```bash
hddsgen gen rust input.idl -o output.rs
```

## Expected Behavior
What you expected to happen.

## Actual Behavior
What actually happened.

## Error Output
```
# Paste error messages here
```

## Environment
- **OS**: [e.g., Ubuntu 22.04, macOS 14, Windows 11]
- **Rust Version**: [e.g., 1.75.0]
- **hdds_gen Version**: [e.g., 0.1.0]
- **Backend**: [e.g., rust, c, python]

## Additional Context
Add any other context about the problem here.
