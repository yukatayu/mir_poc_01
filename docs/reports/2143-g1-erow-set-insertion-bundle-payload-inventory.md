# Report 2143 - G1 E-ROW set-insertion / bundle payload inventory

- Date: 2026-07-04 01:27 JST
- Author / agent: Codex
- Scope: G1 LAB Surface-to-Core E-ROW repair payload vocabulary
- Decision levels touched: L3 LAB repository memory only

## Objective

Create a docs-only inventory for future E-ROW set-insertion, conjunctive
bundle, and partial-guidance repair payload vocabulary before any executable
`ELAB-04/07` repair widening.

The package must keep `ELAB-04` and `ELAB-07` as no-repair rows. It must not
claim set-insertion support, bundle semantics support, final diagnostic/repair
ABI, OBL-025 proof or completion, repair ranking, multi-edit support,
conformance, or G1 exit.

## Scope and assumptions

Scope included:

- Candidate vocabulary for future set-insertion repair items.
- Candidate vocabulary for conjunctive bundles with all-required semantics.
- Candidate vocabulary for partial guidance that does not discharge the local
  row-containment premise.
- The distinction between `ELAB-07` non-visibility multi-missing rows and
  `ELAB-04` mixed visibility / non-visibility rows.
- Repository memory, snapshot docs, validators, and this report.

Assumptions:

- Current singleton repair output for `ELAB-10` and `ELAB-13..16` remains the
  only executable repair-bearing Surface elaboration evidence.
- `ELAB-04` and `ELAB-07` should continue to omit `suggested_repair`.
- A future payload can only claim local-premise discharge if applying the item
  or group makes the associated generated failure set a subset of the declared
  failure row.
- Partial guidance is not OBL-025-shaped coverage unless a later formal
  relation explicitly says so.

## Start state / dirty state

Start state for this package:

- Branch: `main`
- Upstream: `origin/main`
- Starting HEAD: `4f2976711468e7bbb8a73fe672b648992c190efd`
- Start dirty state: clean at package start.
- Discord baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/00-index.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/93-g1-erow001-singleton-repair-assumption.md`
- `plan/94-g1-erow001-singleton-repair-prototype.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `docs/reports/TEMPLATE.md`
- `docs/reports/2142-g1-erow-mixed-multi-repair-decomposition-inventory.md`

## Actions taken

- Added `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`.
- Defined candidate LAB vocabulary for `set_insertion`, `conjunctive_bundle`,
  `partial_guidance`, and `deferred` repair shapes.
- Recorded the future `ELAB-07` safe path as one grouped / set item, not
  independent singleton alternatives.
- Recorded `ELAB-04` as still blocked on visibility/base branch separation,
  alternative visibility repairs, and ordering / ranking.
- Kept no-repair rows as omission of `suggested_repair`, not empty-array ABI.
- Updated `plan/00-index.md`, `plan/83`, `plan/88`, `plan/90`,
  `plan/94`, and `plan/95`.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Updated validator required-path lists for `plan/96`.

## Files changed

- `Documentation.md`
- `docs/reports/2143-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/00-index.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/94-g1-erow001-singleton-repair-prototype.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `progress.md`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
git rev-parse HEAD origin/main
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,240p' README.md
sed -n '241,520p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '261,620p' progress.md
sed -n '1,260p' tasks.md
sed -n '261,620p' tasks.md
sed -n '1,240p' .docs/progress-task-axes.md
sed -n '1,240p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,220p' specs/39-surface-mir-placement-elaboration.md
sed -n '221,520p' specs/39-surface-mir-placement-elaboration.md
sed -n '1,220p' specs/40-indexed-state-semantics.md
sed -n '221,520p' specs/40-indexed-state-semantics.md
sed -n '1,220p' specs/41-role-admission-and-capability-grant.md
sed -n '1,260p' specs/42-source-patch-hotplug-semantics.md
sed -n '1,260p' specs/43-surface-mir-v1-alpha-scope.md
sed -n '1,220p' mirrorea_canon/README.md
sed -n '1,260p' mirrorea_canon/MAP.md
sed -n '1,260p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '1,260p' mirrorea_canon/theory/02-types-effects-failures.md
sed -n '1,260p' mirrorea_canon/theory/03-elaboration.md
sed -n '1,260p' mirrorea_canon/theory/10-diagnostics.md
sed -n '1,320p' mirrorea_canon/theory/11-metatheory-ledger.md
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' plan/82-g1-obl025-statement-shape-inventory.md
sed -n '1,260p' plan/83-g1-erow-repair-payload-inventory.md
sed -n '1,280p' plan/88-g1-erow-repair-shape-inventory.md
sed -n '1,280p' plan/93-g1-erow001-singleton-repair-assumption.md
sed -n '1,300p' plan/94-g1-erow001-singleton-repair-prototype.md
sed -n '1,320p' plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md
jq '.lab_diagnostic_details[0]' samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json
jq '.lab_diagnostic_details[0]' samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json
jq 'has("suggested_repair"), (.suggested_repair // null)' samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json
jq 'has("suggested_repair"), (.suggested_repair // null)' samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json
python3 scripts/surface_mir_samples.py run ELAB-04 --format json | jq '{accepted: .accepted, mismatches: .mismatches, missing_failures: .actual.lab_diagnostic_details[0].failure_row_context.missing_failures, has_suggested_repair: (.actual | has("suggested_repair"))}'
python3 scripts/surface_mir_samples.py run ELAB-07 --format json | jq '{accepted: .accepted, mismatches: .mismatches, missing_failures: .actual.lab_diagnostic_details[0].failure_row_context.missing_failures, has_suggested_repair: (.actual | has("suggested_repair"))}'
python3 scripts/surface_mir_samples.py check-all --format json | jq '{sample_count, failed: [.samples[] | select(.accepted != true) | .sample_id], workflow_ready}'
sed -n '1,220p' /home/codex/.codex/superpowers/skills/systematic-debugging/SKILL.md
python3 scripts/surface_mir_samples.py check-all --format json | jq 'keys'
python3 scripts/surface_mir_samples.py check-all --format json | jq '{sample_count: .sample_count, failed: .failed, accepted: .accepted, workflow_ready: .workflow_ready}'
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
cargo fmt --check
```

The first `check-all` jq summary used a stale `.samples[]` assumption and
failed in jq. The helper output schema was then inspected (`results` /
`failed` keys) and the summary command was corrected.

## Evidence / outputs / test results

Evidence observed before validation:

- `ELAB-04` expected JSON records missing failures:
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, and
  `VisibilityDenied`.
- `ELAB-07` expected JSON records missing failures:
  `MissingWitness`, `RouteUnavailable`, and `StaleMembership`.
- Both `ELAB-04` and `ELAB-07` expected JSON omit `suggested_repair`.
- Current repository snapshot still has 52 Surface helper rows and 53 `.mir`
  Surface source files.

Validation results:

- `ELAB-04` helper run: accepted, no mismatches, missing failures
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`,
  `VisibilityDenied`, and no `suggested_repair`.
- `ELAB-07` helper run: accepted, no mismatches, missing failures
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, and no
  `suggested_repair`.
- Corrected `surface_mir_samples.py check-all` summary:
  `sample_count = 52`, `failed = []`, `workflow_ready = false`.
- `python3 scripts/check_source_hierarchy.py`: required 602, present 602,
  missing 0.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 passed.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 passed.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1295
  numbered reports found.
- `git diff --check`: passed.
- `cargo fmt --check`: passed.

## What changed in understanding

The useful split is not just "single edit versus multi edit". The payload must
also say which object discharges the local premise:

- a set-insertion item can discharge the premise only if it covers all missing
  failures and is accepted as one source edit;
- a conjunctive bundle can discharge the premise only at the group level, not
  at each child singleton item;
- partial guidance must not be counted as a local repair witness.

`ELAB-04` requires a further visibility/base branch split because
`VisibilityDenied` should not be collapsed into the base
capability/witness/route/membership failure family.

## Open questions

- Is adding several failures to one `fails` row one source edit in the
  declared fragment?
- If a bundle has several child edits, is the bundle itself a repair witness
  or only a repair plan?
- Should partial guidance live in `suggested_repair[]` with explicit
  partiality, or in a separate field?
- How should mixed rows associate `E-ROW-001` and `E-ROW-002` branches without
  duplicate or conflicting diagnostics?
- Does OBL-025 stay single-edit only for G1, or should a later obligation
  cover grouped multi-edit repair completeness?

## Suggested next prompt

Continue autonomously with either an OBL-025 statement refinement around
single-edit / set-insertion / bundle boundaries, or a docs-only `ELAB-07`
set-insertion gate review. Do not widen executable repair output unless the
local-premise discharge and single-edit status are explicit.

## Plan update status

`plan/` 更新済み:

- Added `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`.
- Updated `plan/00-index.md`, `plan/83`, `plan/88`, `plan/90`, `plan/94`,
  and `plan/95`.

## Documentation.md update status

`Documentation.md` 更新済み:

- The Surface Mir LAB summary now includes the set-insertion / bundle payload
  inventory and keeps `ELAB-04/07` no-repair.

## progress.md update status

`progress.md` 更新済み:

- Current E-ROW notes, next gap, feature row, and recent log now mention
  `plan/96`.

## tasks.md update status

`tasks.md` 更新済み:

- The set-insertion / bundle payload inventory is recorded as current memory.
- Candidate next packages now point to OBL-025 refinement, `ELAB-07`
  set-insertion gate review, or `ELAB-04` mixed visibility branch inventory.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Dashboard rows and recent validation log now mention `plan/96`. Sample row
  count is unchanged.

## Reviewer findings and follow-up

Reviewer sub-agent `019f28d7-910c-7851-a417-aa2a76f07bd8` reported two
findings:

- Medium: `plan/83` still contained stale pre-prototype wording saying current
  LAB did not emit repair payloads. Follow-up: updated `plan/83` to record the
  current singleton repair-bearing boundary (`ELAB-10`, `ELAB-13..16`) while
  keeping `ELAB-04/07` no-repair.
- Low: this report still had placeholder jq commands and pending reviewer /
  sub-agent status. Follow-up: replaced placeholders with the actual jq
  commands and updated reviewer status.

The reviewer found no executable widening: no Rust or sample expected-output
files were changed, and validator edits only add `plan/96` to required lists.

## Skipped validations and reasons

No intended validation skips.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Reviewer sub-agent `019f28d7-910c-7851-a417-aa2a76f07bd8` completed and was
closed after findings were addressed.
