# Report 1138 — P-A1-01 sample/validation integrity review

- Date: 2026-05-03 15:52:58 JST
- Author / agent: Codex
- Scope: Review `P-A1-01` practical alpha-1 sample/validation integrity with focus on `samples/practical-alpha1/`, `crates/mir-ast/tests/practical_alpha1_front_door.rs`, `crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs`, `scripts/check_source_hierarchy.py`, `scripts/validate_docs.py`, and `scripts/tests/test_validate_docs.py`
- Decision levels touched: None

## Objective

Review whether the current `P-A1-01` practical front-door sample root, expected summaries, focused tests, and validator updates provide sufficient and honest coverage for the current non-final `package.mir.json` cut.

## Scope and assumptions

- This task is review-only. No production code or spec edits were requested.
- The review is against the current dirty worktree state.
- Findings are limited to concrete integrity, coverage, and honesty gaps in the targeted files and the sample root they exercise.

## Start state / dirty state

- The repository was already dirty at start. Existing modifications included `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `scripts/README.md`, `scripts/check_source_hierarchy.py`, `scripts/validate_docs.py`, `scripts/tests/test_validate_docs.py`, `specs/18-practical-alpha1-scope.md`, `plan/44-practical-alpha1-roadmap.md`, `crates/mir-ast/Cargo.toml`, `crates/mir-ast/src/lib.rs`, plus untracked practical alpha-1 files under `crates/mir-ast/src/`, `crates/mir-ast/tests/`, and `samples/practical-alpha1/`.
- I did not revert or modify any of those existing task files during the review.

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
- `scripts/README.md`

## Actions taken

- Read the required repository front-door docs and practical alpha-1 scope/roadmap.
- Inspected the targeted sample fixtures, expected summaries, Rust tests, and validation scripts.
- Ran the focused cargo and Python validation commands requested by the current docs.
- Compared the implemented test anchors with the practical alpha-1 scope wording and roadmap claims.

## Files changed

- `docs/reports/1138-review-p-a1-01-sample-validation-integrity.md` (new report)

## Commands run

```bash
git status --short
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,240p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,240p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,240p' specs/09-invariants-and-constraints.md
sed -n '1,240p' specs/18-practical-alpha1-scope.md
sed -n '1,240p' plan/00-index.md
sed -n '1,240p' plan/44-practical-alpha1-roadmap.md
sed -n '1,240p' samples/README.md
sed -n '1,240p' scripts/README.md
sed -n '1,240p' crates/mir-ast/tests/practical_alpha1_front_door.rs
sed -n '1,260p' crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs
sed -n '1,220p' scripts/check_source_hierarchy.py
sed -n '1,260p' scripts/validate_docs.py
sed -n '1,260p' scripts/tests/test_validate_docs.py
sed -n '1,260p' crates/mir-ast/src/practical_alpha1.rs
find samples/practical-alpha1 -maxdepth 3 -type f | sort
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture`
  - Passed: 6/6 tests in `tests/practical_alpha1_front_door.rs`
- `python3 scripts/check_source_hierarchy.py`
  - Passed: `required: 65`, `missing: 0`
- `python3 scripts/validate_docs.py`
  - Passed: `Documentation scaffold looks complete.`
- `python3 -m unittest scripts.tests.test_validate_docs`
  - Passed: 11 tests

## What changed in understanding

- The current `P-A1-01` line is honest about being a non-final `package.mir.json` loader cut.
- The main risk is not overclaim in the prose, but under-anchored coverage in the focused tests and validators: several practical root and package-shape details are present in fixtures and implementation, yet not protected by the current expected summaries or scaffold validators.

## Open questions

- Should the practical root reserve directories (`packages/`, `expected/`, `source/`, `docker/`) be treated as required scaffold now, or only once more than placeholder README content lands in `source/` / `docker/`?
- Is the intended expectation format for negative `SRC-*` rows a sidecar/JSON artifact, or is error-kind assertion in Rust tests the long-term plan?

## Suggested next prompt

Audit `P-A1-01` front-door coverage gaps and patch the tests/validators so that practical fixture summaries cover authority/capability/manifest detail, schema-error branches are exercised, and practical root reserved subdirectories are structurally anchored.

## Plan update status

`plan/` 更新不要 / review-only task; no roadmap-memory correction applied in this task.

## Documentation.md update status

`Documentation.md` 更新不要 / review-only task; no wording change applied in this task.

## progress.md update status

`progress.md` 更新不要 / review-only task; current progress snapshot was not edited.

## tasks.md update status

`tasks.md` 更新不要 / review-only task; current task map was not edited.

## samples_progress.md update status

`samples_progress.md` 更新不要 / review-only task; dashboard was not edited.

## Reviewer findings and follow-up

- Finding 1: positive fixture summaries currently drop meaningful front-door fields (`authority`, fallback `capability`/`options`, layer `kind`/`attach_mode`, manifest `requires_capabilities`/`effect_row`/`failure_row`), so the current `SRC-01..04` expectations do not actually protect several parts of the accepted `package.mir.json` surface.
- Finding 2: negative coverage only exercises malformed JSON, while implemented reject paths for missing package file and schema validation are untested.
- Finding 3: validator updates only require the practical root README/root directory, not the advertised `packages/`, `expected/`, `source/`, and `docker/` scaffold, so practical root shape can drift without either validator failing.

## Skipped validations and reasons

- `cargo fmt --check` and `git diff --check` were not run because this task did not modify production code and the requested scope was focused review, not implementation. Their absence is not treated as success evidence.

## Commit / push status

Not committed. Not pushed.

## Sub-agent session close status

No sub-agent sessions were opened.
