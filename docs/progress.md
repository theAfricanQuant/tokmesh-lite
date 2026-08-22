# Learning Progress

This log records completed sessions. It captures evidence and resumption state;
the ordered plan remains in `learning.md`.

## 2026-08-22 — Session 2: A trustworthy Nigeria learning product

Full workbook: [Session 2 HTML report](sessions/2026-08-22-session-02.html)

### Goal

Turn the abstract validation engine into a runnable, clearly synthetic Nigeria
lithium learning product, then learn how CSV fixtures, integration tests, shell
commands, Git checkpoints, and GitHub Pages fit together.

### Delivered

- Added `examples/nigeria-lithium/product.yaml` with 12 required columns and
  executable rules for uniqueness, accepted values, and numeric ranges.
- Added an eight-record `valid.csv` fixture that passes product and data
  validation.
- Added an eight-record `invalid.csv` fixture containing four independent,
  intentional defects.
- Added two CLI integration tests covering successful validation and exit code
  `1` for a contract-valid but data-invalid CSV.
- Added three report-level integration tests that identify an unknown
  jurisdiction, an out-of-range grade, and an empty required site name by
  stable issue code and exact location.
- Published prominent rendered-HTML links and new-tab instructions in the
  repository README.

### Failure-and-repair evidence

- The first CLI test failed with exit code `2` because the manifest did not
  exist. Creating the fixture changed the failure boundary from file access to
  parsing.
- The first YAML draft failed at line 2 because indentation made the mapping
  invalid. Repairing the YAML allowed the validator to reach the CSV.
- The first CSV draft contained physical line breaks inside records. The CSV
  reader reported four fields where the 12-column contract expected a complete
  record. Field-count inspection with `awk` exposed the damage; after repair,
  every physical line had 12 fields.
- The initially copied invalid fixture still contained only valid records, so
  the CLI exited `0`. Each deliberate mutation was then introduced separately
  and confirmed through a focused test.

### TDD and fixture evidence

The public seams remained the CLI process boundary and the library's
`validate_data` function. The invalid fixture now demonstrates:

- duplicate `site_id` at `row[3].site_id` →
  `quality.unique.duplicate`;
- `Unknown` jurisdiction at `row[4].jurisdiction` →
  `quality.accepted_values.rejected`;
- grade `12.50` at `row[5].li2o_grade_percent` →
  `quality.range.outside`;
- empty `site_name` at `row[6].site_name` →
  `data.value.required`.

The proposed invalid-date cycle was intentionally paused before its test was
added. It remains the next RED→GREEN slice.

### What we learned

Rust and Cargo:

- `cargo fmt` formats Rust; `cargo fmt --check` verifies formatting without
  modifying files.
- `cargo clippy --all-targets --all-features -- -D warnings` checks every
  target and enabled feature and rejects warnings.
- `cargo test --test validation NAME -- --exact` selects one integration-test
  binary and one exact test name.
- `env!`, `format!`, `Path::new`, `.join`, `.iter().any(...)`, closures,
  `assert!`, and `assert_eq!` create readable behavior-level tests.

Shell and data inspection:

- `sed` is a Unix command run by the Zsh or Bash shell; it is not part of the
  Bash language itself.
- A trailing backslash continues one shell command onto the following line.
- Pipes pass one command's output into the next command.
- `awk -F','` counts comma-separated fields; `cut -d',' -f...` selects fields;
  `sed -n` prints selected lines without printing every line automatically.
- A structurally valid CSV row can still contain contract-invalid values; both
  structure and semantics need tests.

Git and publishing:

- `git add` selects the exact snapshot for the next commit; `git commit`
  records it locally; `git push` publishes commits to the tracked remote.
- `git push -u origin main` both pushes and sets `origin/main` as the upstream;
  later pushes can simply use `git push`.
- GitHub's repository viewer shows HTML source. GitHub Pages renders
  `docs/index.html` as the learning website.
- GitHub README links cannot reliably force a new tab. Ctrl+click,
  Command+click, or middle-click gives the reader that choice.

### Verification and repository state

- `cargo fmt --check` passed.
- strict Clippy passed with zero warnings.
- the full suite passed: 4 CLI tests plus 7 validation tests, 11 total.
- feature commit:
  `d3883f9 feat: add Nigeria lithium validation example`.
- README publishing commit:
  `2e00acd docs: link rendered learning library`.
- the working tree was clean after both commits were pushed to `origin/main`.

### Exact resumption point

Before writing another test, explain and reconfirm the public seam: observable
behavior at `validate_data` and at the CLI exit-code boundary, rather than
private implementation details. Then add one focused RED test requiring
`data.value.type` at `row[7].sample_date`, mutate only physical CSV line 7 to
an unambiguously invalid month such as `2026-13-20`, and confirm GREEN. The
current simple date checker validates shape and broad ranges; replacing it with
calendar-valid date handling remains a separate roadmap feature.

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
