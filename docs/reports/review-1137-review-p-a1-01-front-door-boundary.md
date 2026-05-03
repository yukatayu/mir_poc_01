# Report 1137 — Review P-A1-01 Front-Door Boundary

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review `P-A1-01` practical alpha-1 front-door changes for runtime/transport boundary correctness and loader-boundary honesty
- Decision levels touched: none

## Objective

Review the `P-A1-01` practical alpha-1 front-door changes with focus on whether the new loader/doc surface stays narrow, honest, and non-claiming with respect to runtime, transport, and package execution.

## Scope and assumptions

- Review focus is limited to:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `samples/practical-alpha1/`
  - `samples/README.md`
  - `scripts/README.md`
- Practical alpha-1 current cut is expected to be a non-final `package.mir.json` loader floor only.
- This review reports concrete findings only; no implementation changes are made in this task.

## Start state / dirty state

- start state: `main` branch with pre-existing dirty worktree
- notable pre-existing changes included practical alpha-1 docs/spec/dashboard updates plus new `mir-ast` front-door files under review

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/practical-alpha1/source/README.md`
- `samples/practical-alpha1/docker/README.md`
- `scripts/README.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Read the required repo front-door/spec/plan documents in repository order through practical alpha-1 scope and roadmap.
2. Reviewed the target implementation and docs for boundary wording, especially whether they imply runtime execution, transport realization, or broader package execution than actually implemented.
3. Reviewed the practical alpha-1 tests and fixture summaries for evidence of accepted surface and missing negative coverage.
4. Ran the targeted `mir-ast` front-door test suite.

## Files changed

- `docs/reports/1137-review-p-a1-01-front-door-boundary.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
git diff -- crates/mir-ast/src/practical_alpha1.rs samples/practical-alpha1 samples/README.md scripts/README.md
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
python3 scripts/validate_docs.py
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` passed with `6` tests.
- `python3 scripts/validate_docs.py` failed because another existing draft report, `docs/reports/2026-05-03-pa1-01-front-door-overclaim-review.md`, is currently the latest report by validator ordering and is missing the required template sections. This failure is unrelated to the reviewed `P-A1-01` front-door files.
- Concrete finding confirmed by source inspection:
  - `crates/mir-ast/src/practical_alpha1.rs` accepts any existing file path as a load target without requiring the basename `package.mir.json`.
  - The docs/spec repeatedly describe the current cut as a limited `package.mir.json` loader floor rooted in `samples/practical-alpha1/`.

## What changed in understanding

- The docs are otherwise careful not to claim runtime execution, transport realization, or practical CLI completion.
- The main honesty gap is narrower and more mechanical: the implemented file-path entrypoint is broader than the documented `package.mir.json` contract.

## Open questions

- None beyond the concrete finding below.

## Suggested next prompt

Tighten `crates/mir-ast::practical_alpha1` so file-path loading also requires the basename `package.mir.json`, add a negative test for an arbitrary JSON filename, and keep the docs wording unchanged.

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory は変化していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり snapshot wording は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current status snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり sample dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  `crates/mir-ast/src/practical_alpha1.rs` documents and exports a narrow `package.mir.json` front-door, but `resolve_package_path()` accepts any existing file path verbatim when the caller passes a file instead of a directory. This silently widens the accepted carrier beyond the stated `package.mir.json` contract in `specs/18`, `plan/44`, `README.md`, and `samples/practical-alpha1/packages/README.md`. The result is a dishonest loader boundary: a file such as `foo.json` can be treated as a valid practical front-door input even though the current line is explicitly described as limited `package.mir.json` loading. Follow-up: reject non-`package.mir.json` basenames for file-path input and add a negative regression test.

- No additional runtime/transport/package-execution claim issues were found in the reviewed docs/files.

## Skipped validations and reasons

- broader `cargo test -p mir-ast`: skipped because this was a focused review of the practical alpha-1 front-door boundary, and the targeted front-door suite already covers the changed lane under review

## Commit / push status

No commit; review-only task.

## Sub-agent session close status

- No sub-agents used.
