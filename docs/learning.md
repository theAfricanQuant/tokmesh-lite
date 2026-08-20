# Rust Learning Guide

TokMesh Lite introduces Rust by building one complete, useful program.

## Concepts used

- Cargo packages and modules
- Structs and enums for domain vocabulary
- Ownership and borrowing while reading rows
- `Option` for optional manifest fields
- `Result` for recoverable failures
- Pattern matching for data and rule types
- Serde for YAML and JSON
- Iterators and collections for validation
- Unit, integration, and CLI tests

Use [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) as a reference. Chapters 4, 6, 8, 9, 11, and 12 are especially relevant, but you do not need to finish the book before changing the code.

## Learning rule

Make one behavior work, read the compiler message when it does not, and add a test that explains the behavior. Avoid abstractions whose need cannot yet be demonstrated.

