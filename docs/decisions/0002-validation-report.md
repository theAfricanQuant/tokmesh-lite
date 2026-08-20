# ADR 0002: Collect Validation Issues

## Context

Stopping at the first invalid field forces users through repeated edit-run cycles.

## Decision

Return a validation report containing every issue that can be found safely in one pass. Each issue has a code, location, and message.

## Consequences

Human and JSON output share the same report. Tests assert on observable issue codes rather than private validation functions.

