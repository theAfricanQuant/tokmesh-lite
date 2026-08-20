# Learning Progress

This log records completed sessions. It captures evidence and resumption state;
the ordered plan remains in `learning.md`.

## 2026-08-20 — Session 1: From contracts to accepted values

Full workbook: [Session 1 HTML report](sessions/2026-08-20-session-01.html)

### Goal

Establish a working TokMesh Lite baseline, learn the RED-to-GREEN development
cycle, and add a useful categorical quality rule.

### Delivered

- Confirmed valid product manifests and CSV datasets exit successfully.
- Confirmed invalid manifests report multiple stable issue codes and exit `1`.
- Added `accepted_values` to the YAML quality-rule vocabulary.
- Rejected CSV values outside the declared list with
  `quality.accepted_values.rejected` and a precise row-and-column location.
- Rejected empty accepted-values declarations with
  `quality.accepted_values.empty` before reading a dataset.
- Converted the new learning fixture to a synthetic Nigeria lithium context.

### TDD evidence

First RED: the YAML parser reported that `accepted_values` was unknown.

First GREEN: a dataset containing `operating` and `unknown` produced exactly
one finding at `row[3].status`.

Second RED: an accepted-values declaration with `values: []` produced zero
issues where the specification expected one.

Second GREEN: product validation produced one
`quality.accepted_values.empty` finding at `quality[0].values`.

### What we learned

Rust:

- An enum defines the supported quality-rule vocabulary.
- Exhaustive `match` expressions make new variants visible to implementations.
- `Vec<String>` owns a variable-length list of accepted values.
- `.iter().any(...)` checks borrowed values without consuming the vector.
- `&&` combines conditions, while `.is_empty()` states intent directly.
- `expect(...)` makes fixture setup failures explain themselves.

Data mesh:

- A contract can make categorical expectations executable.
- Contract validity and dataset validity are separate questions.
- Stable issue codes and locations support both people and automation.
- A validation failure uses exit code `1`; an execution or input failure uses
  exit code `2`.

Testing:

- RED must fail for the expected missing behavior.
- GREEN uses the smallest implementation that satisfies that behavior.
- Tests cross public seams: `load_manifest`, `validate_product`, and
  `validate_data`.

### Verification and repository state

- `cargo fmt` completed.
- Strict Clippy completed with zero warnings.
- The complete suite passed: two CLI tests and four validation tests.
- Feature commit: `da81912 feat: add accepted-values quality validation`.
- Session documentation commit:
  `54f8e2e docs: add guided learning roadmap and session workbook`.
- GitHub Pages learning library:
  <https://theafricanquant.github.io/tokmesh-lite/>.

### Workflow adjustment

The learner will type feature code and run commands unless they explicitly ask
the agent to edit. The agent will explain one step at a time and interpret the
result before moving forward. The agent owns the end-of-session documentation
update.

### Next session

Create the small synthetic Nigeria lithium product manifest plus valid and
invalid CSV examples. After the small example is trustworthy, begin the
deterministic large-dataset generator.
