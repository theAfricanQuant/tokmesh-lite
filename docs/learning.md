# Rust and Data-Mesh Learning Roadmap

TokMesh Lite teaches Rust by building one complete, useful data-product
contract validator. Each lesson produces observable behavior, an automated
test, and a small commit.

## Learning method

Use one vertical slice at a time:

```text
data-product need
    -> public behavior
    -> failing test (RED)
    -> minimal Rust implementation
    -> passing test (GREEN)
    -> full verification
    -> review and commit
```

Read compiler and test failures as feedback. Introduce an abstraction only
after a real variation demonstrates the seam it serves.

## Journey

### 1. Establish the local contract engine — complete

- Parse a typed YAML product manifest.
- Validate manifest structure and semantic versioning.
- Validate CSV headers, required values, types, uniqueness, and ranges.
- Produce human and JSON reports with automation-friendly exit codes.

Rust concepts: Cargo packages, modules, structs, enums, Serde, `Option`,
`Result`, pattern matching, borrowing, iterators, collections, and integration
tests.

### 2. Grow quality rules safely — in progress

- [x] Declare and enforce `accepted_values`.
- [x] Reject an empty accepted-values declaration.
- [ ] Replace the deliberately simple date checker with calendar-valid dates.
- [ ] Add the first dataset-level measure, such as null percentage.

Rust concepts: exhaustive matching, vectors, iterator predicates, dependency
selection, counters, and state accumulated across rows.

### 3. Publish the Nigeria lithium learning product — in progress

- [x] Add a clearly labelled synthetic product manifest.
- [x] Add small valid and invalid CSV examples.
- [x] Model jurisdictions, minerals, activity, grade, coordinates, and dates.
- Document the contract without presenting synthetic records as real mines.

Data-mesh concepts: product ownership, schema, executable quality expectations,
sovereignty metadata, and publication readiness.

### 4. Learn scale through measurement — planned

- Build a deterministic Rust generator for 10K, 100K, 1M, and 5M rows.
- Keep generated files outside Git.
- Measure release-build duration, throughput, file size, findings, and memory.
- Explain why uniqueness needs more memory than range checks.

Rust concepts: buffered writing, deterministic generation, release builds,
allocation, hashing, and performance measurement.

### 5. Strengthen the contract interface — planned

- Publish a machine-readable manifest schema.
- Improve inspection and error ergonomics.
- Add stable examples for human and JSON output.
- Document compatibility expectations for rule evolution.

Rust concepts: serialization contracts, error design, compatibility, and
documentation tests.

### 6. Release TokMesh Lite — planned

- Verify a fresh clone can build, test, and run without external services.
- Complete user, contributor, security, and architecture documentation.
- Create a tagged learning release with reproducible examples.

## References

Use [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) as
a reference. Chapters 4, 6, 8, 9, 11, and 12 are especially relevant; the
journey does not require reading the book front to back before coding.

See `progress.md` for completed-session evidence and the exact resumption point.
