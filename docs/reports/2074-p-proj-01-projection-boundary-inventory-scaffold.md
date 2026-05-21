# Report 2074 — P-PROJ-01 projection boundary inventory scaffold

- Date: 2026-05-21
- Author / agent: Codex
- Scope: `P-PROJ-01` planned-only projection boundary inventory scaffold actualization under `samples/product-alpha1/projection/**`, `scripts/projection_boundary_samples.py`, `scripts/tests/test_projection_boundary_samples.py`, and `plan/55-projection-backend-roadmap.md`
- Decision levels touched: no new normative decision; existing `L1` / `L2` projection boundary wording was preserved and repository memory in `plan/55` was synchronized

## Objective

Close `P-PROJ-01` by actualizing the planned-only projection boundary inventory scaffold, helper, tests, and `plan/55` wording without touching global snapshot docs or validators.

## Scope and assumptions

- `P-PROJ-01` is inventory/scaffold work only. It must not claim code generation, server/client binary split, or backend execution.
- `ManifestProviderCompatibility` must be surfaced as machine-readable planned evidence with at least one accepted row and one rejected row.
- `workflow_ready` must remain `false` across the projection scaffold.
- The user-owned file set is limited; unrelated concurrent edits must be left intact.

## Start state / dirty state

- Initial inspection showed no existing `samples/product-alpha1/projection/` root or `scripts/projection_boundary_samples.py` helper.
- Later in the task, unrelated concurrent changes appeared elsewhere in the shared worktree (notably posegraph and doc/validator paths). Those files were not modified by this task.
- This task intentionally avoided touching `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `scripts/check_source_hierarchy.py`, `scripts/validate_docs.py`, and related global snapshot/validator surfaces.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/30-projection-and-backend-boundary.md`
- `specs/31-engine-wasm-ffi-adapter-boundary.md`
- `plan/00-index.md`
- `plan/55-projection-backend-roadmap.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `samples/product-alpha1/computational/README.md`
- `samples/product-alpha1/computational/matrix.json`
- `samples/product-alpha1/operational/deployments/projection/projection.profile.json`
- `scripts/mir_computational_samples.py`
- `scripts/tests/test_mir_computational_samples.py`
- `scripts/projection_codegen_samples.py`
- `scripts/tests/test_projection_codegen_samples.py`
- `docs/reports/TEMPLATE.md`
- `docs/reports/2073-p-comp-01-computational-scaffold-actualization.md`

## Actions taken

- Read the required repo sequence and the projection/backend boundary sources before editing.
- Added `samples/product-alpha1/projection/README.md` and `matrix.json` as the planned-only front door for the new inventory root.
- Added planned roots for `server-client-target-manifest`, `packet-boundary-schema`, `ffi-boundary-schema`, and `manifest-provider-compatibility`, each with representative JSON inventory artifacts.
- Added accepted and rejected `ManifestProviderCompatibility` rows as machine-readable planned evidence, with `workflow_ready = false`.
- Wrote `scripts/tests/test_projection_boundary_samples.py` first and confirmed the red state while the helper/root were missing.
- Implemented `scripts/projection_boundary_samples.py` with `list`, `matrix`, `run`, `check-all`, and `closeout`, plus missing-root / missing-artifact validation and planned-only rejection behavior.
- Updated `plan/55-projection-backend-roadmap.md` to remove stale “not yet present” wording and record the closed `P-PROJ-01` scaffold state.
- Performed a narrow diff review over the owned paths and did not modify unrelated concurrent worktree changes.

## Files changed

- `samples/product-alpha1/projection/README.md`
- `samples/product-alpha1/projection/matrix.json`
- `samples/product-alpha1/projection/server-client-target-manifest/server-client-target-manifest.json`
- `samples/product-alpha1/projection/packet-boundary-schema/packet-boundary-schema.json`
- `samples/product-alpha1/projection/ffi-boundary-schema/ffi-boundary-schema.json`
- `samples/product-alpha1/projection/manifest-provider-compatibility/manifest-provider-compatibility.json`
- `scripts/projection_boundary_samples.py`
- `scripts/tests/test_projection_boundary_samples.py`
- `plan/55-projection-backend-roadmap.md`
- `docs/reports/2074-p-proj-01-projection-boundary-inventory-scaffold.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 scripts/projection_boundary_samples.py matrix --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py run proj-01-server-client-target-manifest --format json
python3 scripts/projection_boundary_samples.py closeout --format json
git diff --check
git status --short --untracked-files=all
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_projection_boundary_samples`
  - red pass before implementation: `FAILED (failures=11)` while the helper/root were missing
  - final pass after implementation: `Ran 11 tests`, `OK`
- `python3 scripts/projection_boundary_samples.py matrix --format json`
  - `sample_count = 4`
  - `planned_count = 4`
  - `executable_count = 0`
  - `workflow_ready = false`
  - accepted compatibility rows: `compat-accepted-renderer-view`
  - rejected compatibility rows: `compat-rejected-missing-capability`
  - `validation_errors = []`
- `python3 scripts/projection_boundary_samples.py check-all --format json`
  - `planned` contains all four projection sample IDs
  - `passed = []`
  - `failed = []`
  - `workflow_ready = false`
- `python3 scripts/projection_boundary_samples.py run proj-01-server-client-target-manifest --format json`
  - `terminal_outcome = planned_only`
  - rejection reason explicitly says the later projection realization package is not implemented yet
  - stop lines include `no generated server/client binary` and `no LLVM/backend execution`
- `python3 scripts/projection_boundary_samples.py closeout --format json`
  - `validation_floor` records the focused unittest/matrix/check-all/run commands
  - `accepted_compatibility_rows` and `rejected_compatibility_rows` are preserved in closeout output
- `git diff --check`
  - passed
- `date '+%Y-%m-%d %H:%M %Z'`
  - `2026-05-21 19:40 JST`

## What changed in understanding

- `P-PROJ-01` closes cleanly as a thin matrix/helper scaffold; it does not need any runtime placeholder or codegen simulation.
- The compatibility evidence belongs in its own machine-readable inventory file, but the helper still needs to mirror accepted/rejected row IDs from the top-level matrix so missing or mislabeled compatibility rows can be detected.
- Reusing the computational scaffold helper shape kept the command surface consistent while still allowing projection-specific stop lines and compatibility reporting.

## Open questions

- `UNRESOLVED`: the later package name and exact reopen sequence for actual projection realization beyond inventory scaffolding is not fixed in `plan/55` / `plan/57`; this task intentionally did not invent one.
- `OPEN QUESTION`: if a later package wants stronger validation than root/artifact presence plus compatibility-row classification, should it validate internal JSON field completeness in the helper or defer that to a dedicated schema checker package?

## Suggested next prompt

Proceed with the next boundary package in your chosen queue, or explicitly define the later projection-realization reopen package if you want runtime/codegen work to follow the `P-PROJ-01` scaffold without ambiguity.

## Plan update status

`plan/` 更新済み:
`plan/55-projection-backend-roadmap.md` now records the actualized planned-only projection root/helper and current validation anchors.

## Documentation.md update status

`Documentation.md` 更新不要:
the user explicitly excluded global snapshot docs from this task, and the required repository-memory change was confined to `plan/55`.

## progress.md update status

`progress.md` 更新不要:
the user explicitly excluded global snapshot docs from this task.

## tasks.md update status

`tasks.md` 更新不要:
the user explicitly excluded global snapshot docs from this task.

## samples_progress.md update status

`samples_progress.md` 更新不要:
the user explicitly excluded global snapshot docs from this task.

## Reviewer findings and follow-up

- The `requesting-code-review` workflow could not be executed literally because the required review-dispatch tool/template path is not available in this Codex session.
- Fallback used: a local narrow diff review over the owned projection paths plus fresh verification commands.
- No additional blocking issue was found during the local review.

## Skipped validations and reasons

- `python3 scripts/check_source_hierarchy.py` was not run because the user explicitly said not to touch global validators, and this task did not widen the validator registration surface.
- `python3 scripts/validate_docs.py` and `python3 -m unittest scripts.tests.test_validate_docs` were not run for the same reason.
- `cargo fmt --check` and Rust test suites were not run because this task added only Python/sample/plan/report changes and did not modify Rust code.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent session was opened for this task.
