# ADR 0001: YAML Product Manifest

## Context

The first repository needs a format that is readable by data owners, reviewable in Git, and directly deserializable into typed Rust structures.

## Decision

Use YAML as the authored manifest format and expose JSON for machine-readable validation reports.

## Consequences

YAML is approachable but whitespace-sensitive. TokMesh reports parse failures separately from contract-validation failures. A future repository may publish a formal JSON Schema without changing the Rust domain vocabulary.

