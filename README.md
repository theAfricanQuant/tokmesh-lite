# TokMesh Lite

TokMesh Lite is a small Rust command-line application for validating a data-product manifest and its CSV dataset. It is the first standalone TokMesh learning repository.

## What you can do

```bash
cargo run -- product validate examples/copper-mines/product.yaml
cargo run -- product inspect examples/copper-mines/product.yaml
cargo run -- data validate \
  examples/copper-mines/product.yaml \
  examples/copper-mines/data.csv
```

Add `--json` before the command for machine-readable output:

```bash
cargo run -- --json data validate \
  examples/copper-mines/product.yaml \
  examples/copper-mines/data.csv
```

The process exits with code `0` when validation passes, `1` when validation fails, and `2` when the command cannot read or parse its input.

## Prerequisites

- A stable Rust toolchain installed with [rustup](https://rustup.rs/)
- No database, container runtime, network service, or cloud account

## Five-minute demo

```bash
cargo test
cargo run -- product validate examples/copper-mines/product.yaml
cargo run -- data validate examples/copper-mines/product.yaml examples/copper-mines/data.csv
cargo run -- product validate examples/invalid-products/duplicate-columns.yaml
```

The first two validations pass. The deliberately invalid manifest fails and explains why.

## Repository status

**Learning release.** TokMesh Lite is intentionally narrow. It is not an identity system, catalog server, workflow engine, or production data platform.

## Concepts

- [Rust learning guide](docs/learning.md)
- [Data-mesh concepts](docs/data-mesh.md)
- [Architecture decisions](docs/decisions/)

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

## License

MIT

