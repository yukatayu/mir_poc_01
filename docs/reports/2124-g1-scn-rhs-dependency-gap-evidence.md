# Report 2124 — G1 SCN RHS Dependency-Gap Evidence

- Date: 2026-07-03 21:10 JST
- Author / agent: Codex
- Scope: LAB-only Surface-to-Core dependency evidence for G1 SCN-01 / SCN-02 RHS reads
- Decision levels touched: L0 canon consulted, no canon edit; LAB evidence and repository memory updated

## Objective

Add narrow LAB evidence for the immediate G1 dependency gap identified in
`plan/72`: SCN-01 requires a same-field RHS dependency row and SCN-02 requires
target/self RHS dependency rows. Keep the package below C-static conformance,
runtime read materialization, proof discharge, G1 exit, and final Core IR
schema/API freeze.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This package edits only LAB
implementation, samples, helper projections, snapshot docs, and repository
memory.

Working assumption: the helper may expose a static dependency carrier if it is
clearly LAB-only and source-span backed. It must not turn RHS dependencies into
observer-safe remote reads, runtime occurrences, or OPEN-014 materialization
policy.

## Start state / dirty state

The package started from a clean `main...origin/main` worktree after commit
`057cabed Add G1 OBL-001 Lean statement draft` was pushed. The Discord
task baseline was recorded with `discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `mirrorea_canon/spec/04-core-ir.md`
- `mirrorea_canon/spec/06-conformance.md`
- `mirrorea_canon/plan/00-gates.md`
- `plan/71-g1-ordinary-assignment-target.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- Oracle advisory output for the SCN dependency-gap package

## Actions taken

- Added `SurfaceCoreIr.dependencies` and a `SurfaceCoreDependency` row type.
- Changed cross-locus write elaboration so RHS indexed reads on the same
  assignment are preserved as `rhs_indexed_read` dependency rows linked to the
  generated owner-directed write request.
- Kept ordinary RHS read lowering unchanged when no remote write request is
  generated.
- Added dependency rows to `source_spans` as `entity_kind = dependency`.
- Added `dependency_summaries` to the Surface sample projection.
- Added `ELAB-11` for SCN-01-shaped
  `World { player[self].position = player[self].position + draw }`.
- Added `ELAB-12` for SCN-02-shaped
  `S { player[target].hp = player[target].hp - player[self].atk }`.
- Updated `ELAB-02` and `ELAB-09` expected outputs to include dependency
  summaries and dependency source-span entity kinds.
- Updated repository memory, snapshot docs, validators, and sample dashboard.
- Ran a read-only sub-agent review and an Oracle advisory consult, then checked
  both against canon and local evidence.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/surface_mir_samples.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`
- `samples/full-system-v1-surface/README.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/elaboration/elab-02-cross-place-write-positive/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-09-visible-write-auto-communication-positive/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-11-scn01-rhs-dependency-positive/`
- `samples/full-system-v1-surface/elaboration/elab-12-scn02-two-read-dependency-positive/`
- `plan/00-index.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `plan/90-source-traceability.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2124-g1-scn-rhs-dependency-gap-evidence.md`

No file under `mirrorea_canon/` was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
rg -n "SCN-01|SCN02|same-field|position|rhs" mirrorea_canon plan/72-g1-scn01-scn02-static-consequence-drilldown.md plan/75-g1-scn-rhs-dependency-gap-evidence.md
ask-chatgpt-pro-followup follow-up-for-mirrorea-package -p "<SCN dependency-gap package prompt>"
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 scripts/surface_mir_samples.py run ELAB-02 --format json
python3 scripts/surface_mir_samples.py run ELAB-09 --format json
python3 scripts/surface_mir_samples.py run ELAB-11 --format json
python3 scripts/surface_mir_samples.py run ELAB-12 --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 -m unittest scripts.tests.test_surface_mir_samples
cargo fmt --check
cargo fmt
cargo fmt --check
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_validate_docs
python3 scripts/surface_mir_samples.py check-all --format json | jq '{sample_count, passed_count:(.passed|length), failed_count:(.failed|length), validation_errors:(.validation_errors|length), workflow_ready}'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- TDD red check: the new dependency exposure test initially failed because
  Core IR had no dependency rows.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
  passed with 15 tests.
- `python3 -m unittest scripts.tests.test_surface_mir_samples` passed with
  42 tests.
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_validate_docs`
  passed with 62 tests.
- `python3 scripts/surface_mir_samples.py check-all --format json` reported
  `sample_count = 48`, `passed = 48`, `failed = 0`, `validation_errors = 0`,
  `workflow_ready = false`.
- `python3 scripts/check_source_hierarchy.py` reported 571 required paths,
  571 present, 0 missing.
- `python3 scripts/validate_docs.py` reported the documentation scaffold
  complete after this report was added, with 1276 numbered reports.
- `cargo fmt --check` initially failed only on Rust test formatting; `cargo fmt`
  was run and the next `cargo fmt --check` passed.
- `git diff --check` passed.

## What changed in understanding

The SCN-01 canon shape is not an owner-local example. It is explicitly
`BrowserClient[self]` issuing a nested `World { ... }` ordinary assignment, so
the right LAB evidence row is an owner-directed write request plus a same-field
RHS dependency row and visible publish/observe rows.

The previous elaboration path created the write request before RHS dependency
evidence existed. The safe correction is a static dependency row linked to the
generated write request, not an automatic remote read/observe row.

## Open questions

- OBL-020 / OBL-021 still need a separate dependency inventory.
- OBL-001 Lean statement wording may later mention dependency preservation
  abstractly, but should not import this LAB carrier as canon.
- Canon E-ROW-001 / E-ROW-002 vocabulary still needs alignment with current
  LAB `generated_failure_not_declared` diagnostics.
- OPEN-014 read materialization remains open.

## Suggested next prompt

自走で OBL-020/021 dependency inventory を進め、OBL-001 とは別枠で WF
preservation / elaboration determinism の依存関係を整理してください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface line now records 48 helper rows / 49 `.mir` source files and
keeps the result as alpha/LAB evidence, not final runtime/transport/API.

## progress.md update status

更新済み: added the current SCN dependency-gap evidence note and recent log.

## tasks.md update status

更新済み: removed SCN dependency-gap evidence from the candidate next package
set and promoted OBL-020/021 inventory, OBL-001 refinement, and E-ROW alignment
as next candidates.

## samples_progress.md update status

更新済み: Surface helper row count and elaboration dashboard now include
`ELAB-11/12`.

## Reviewer findings and follow-up

Read-only sub-agent review found the core bug: write lowering returned before
RHS dependency analysis, and helper projection had no dependency carrier. The
implementation follows that recommendation by adding a static `rhs_indexed_read`
carrier and avoiding remote read / observe materialization for SCN-02.

Oracle advice agreed with the boundary: add only ELAB-11/12 if implementation
can stay helper-local; do not claim C-static, runtime, proof, G1 exit,
OPEN-014 resolution, or final Core schema/API. Oracle's owner-local example was
treated as advisory; canon SCN-01 was used for the actual ELAB-11 shape.

## Skipped validations and reasons

- Full workspace Cargo test/build/clippy: skipped because this package touches
  only `mir-semantics` Surface-to-Core elaboration plus helper/docs.
- Runtime tests and release checks: skipped because no runtime request serving,
  MessageEnvelope dispatch, store mutation, product release, or operational
  workflow claim is made.
- Lean validation: skipped because no Lean files or proof-status files changed.
- Canon validators: skipped because `mirrorea_canon/` was not edited.

## Commit / push status

Pending at report write. This package should be committed with
`git commit --no-gpg-sign` and pushed after final post-report validation.

## Sub-agent session close status

Read-only review sub-agent `019f27d3-ade4-7990-ae57-110e2d211b33` was closed
after its findings were integrated. Oracle session completed and is advisory
only; its useful points were mirrored into this report and `plan/75`.
