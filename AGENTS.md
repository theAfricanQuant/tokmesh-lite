# TokMesh Lite Working Agreement

## Mission

Build TokMesh Lite as an independent, greenfield, Rust-leaning data-product
contract validator. Teach Rust and data-mesh ideas through small, working
vertical slices.

## Start each session

1. Read `docs/learning.md` for the ordered journey and current milestone.
2. Read the latest entry in `docs/progress.md` for evidence and the next step.
3. Check `git status` and the latest commit before proposing changes.

The session is ready when the next behavior, its public testing seam, and its
completion check are explicit.

## Hands-on learning

The learner types code and runs commands unless they explicitly ask the agent
to edit or execute them. Work one step at a time:

1. Explain one behavior in plain language.
2. Agree on the public testing seam.
3. Give the learner the smallest test to type.
4. Interpret the expected RED output together.
5. Give the learner the smallest implementation to type.
6. Interpret GREEN and run repository-wide verification.
7. Review the diff before the learner commits and pushes.

Explain each new Rust construct where it first appears. Connect every feature
to the data-product behavior it enables.

## Scope and evidence

- Use synthetic Nigeria-focused learning data and label it as synthetic.
- Verify factual domain claims against authoritative, openly available sources
  before publishing them as facts.
- Generate large performance datasets locally; keep them out of Git.
- Preserve the small public validation interfaces and add abstraction only
  when a second implementation demonstrates a real seam.
- Keep infrastructure, identity, web interfaces, and federation outside this
  repository.

## Quality gates

Before committing code, run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
git diff --check
```

## Close each session

Update `docs/progress.md` with the goal, evidence, Rust concepts, data-mesh
concepts, commit state, unresolved questions, and exact next step. Update
`docs/learning.md` when milestone status changes and `CHANGELOG.md` when
user-visible behavior changes. Create a self-contained HTML workbook under
`docs/sessions/` containing the lesson, code, commands, captured outputs,
command explanations, and resumption point; link it from the progress entry.
Add the workbook to `docs/index.html` newest-first and update the landing page's
session count and active learning milestone.

The session is closed when the progress entry matches the repository state and
a fresh clone has enough context to resume safely. The HTML workbook must open
offline and print cleanly without external assets. Render the workbook and
landing page at desktop and mobile widths before publishing, then verify their
GitHub Pages URLs after pushing.
