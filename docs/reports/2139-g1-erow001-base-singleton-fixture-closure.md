# Report 2139 — G1 E-ROW-001 base singleton fixture closure

- Date: 2026-07-04 00:18 JST
- Author / agent: Codex
- Scope: LAB-only Surface Mir elaboration fixtures, tests, repository memory, and snapshot docs
- Decision levels touched: L2/L3 LAB evidence only; no canon edit

## Objective

Close the no-repair fixture set for non-visibility singleton `E-ROW-001`
generated failure-row omissions before any future repair output widening.

`ELAB-13` already covered `MissingWitness`. This package adds `ELAB-14..16`
for `MissingCapability`, `RouteUnavailable`, and `StaleMembership`, with all
four rows still omitting `suggested_repair`.

## Scope and assumptions

- Normative source remains `mirrorea_canon/`.
- This task is LAB-only repository memory and executable evidence.
- Existing `ELAB-10` remains the only repair-bearing `suggested_repair[]`
  evidence.
- `ELAB-04` and `ELAB-07` remain no-repair mixed / multi-missing evidence.
- The package follows the stricter Oracle recommendation: one representative
  is enough for inventory, but class-wide future widening should have one
  no-repair singleton fixture per base failure atom first.

## Start state / dirty state

Start state was a clean worktree at pushed `main`:

- `HEAD == origin/main == 14d047e4e8ae064ad2877db5c51996ad06f2ecc3`

Dirty state at report write includes:

- modified Rust elaboration tests
- modified Surface helper Python tests
- modified Surface elaboration matrix
- new `ELAB-14..16` sample roots
- modified plan / snapshot / validator docs
- new `plan/92`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `tasks.md`
- `samples_progress.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- ChatGPT Pro Oracle consult `we-are-working-in-the-2`
- sub-agent code-mapping findings from `019f2888-474d-7892-babd-1b30b781f1df`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Started a new Discord baseline for this package.
- Consulted ChatGPT Pro Oracle on whether one no-repair singleton representative
  is sufficient before class-wide repair widening.
- Asked a code-mapping sub-agent to check implementation/helper symmetry across
  `MissingCapability`, `MissingWitness`, `RouteUnavailable`, and
  `StaleMembership`.
- Chose the stricter Oracle path for specification/testing risk: add all
  remaining singleton no-repair fixtures before any class-wide widening.
- Wrote RED tests first:
  - Python helper test expected `ELAB-14..16` and failed with unknown samples.
  - Rust sample-path regression expected `ELAB-14..16` and failed with missing
    sample files.
- Added `ELAB-14..16` sample roots and expected JSON.
- Registered `ELAB-14..16` in the Surface elaboration matrix.
- Updated Python helper expected counts from 49 to 52.
- Added a Rust sample-path regression over `ELAB-13..16`.
- Added `plan/92-g1-erow001-base-singleton-fixture-closure.md`.
- Updated `plan/88`, `plan/89`, `plan/00-index.md`, and
  `plan/90-source-traceability.md`.
- Updated validator required-path lists for `plan/92` and the new sample files.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`,
  and the Surface elaboration README.

## Files changed

- `Documentation.md`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `docs/reports/2139-g1-erow001-base-singleton-fixture-closure.md`
- `plan/00-index.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- `plan/90-source-traceability.md`
- `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- `progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/main/src/missing-capability-singleton-failure-row-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/main/src/route-unavailable-singleton-failure-row-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/main/src/stale-membership-singleton-failure-row-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/expected/elaboration.json`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,240p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
sed -n '1,520p' /home/codex/.codex/superpowers/skills/test-driven-development/SKILL.md
ask-chatgpt-pro -p "<E-ROW singleton fixture gate question>" --file mirrorea_canon/theory/03-elaboration.md --file mirrorea_canon/theory/10-diagnostics.md --file mirrorea_canon/spec/07-diagnostics-format.md --file plan/88-g1-erow-repair-shape-inventory.md --file plan/89-g1-erow001-non-visibility-singleton-fixture.md --file samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/expected/elaboration.json
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_stays_no_repair
cargo test -p mir-semantics --test surface_to_core_elaboration sample_fixtures_cover_each_non_visibility_singleton_without_repair -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration sample_fixtures_cover_each_non_visibility_singleton_without_repair -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_stays_no_repair
cargo test -p mir-semantics --test surface_to_core_elaboration sample_fixtures_cover_each_non_visibility_singleton_without_repair -- --nocapture
python3 scripts/surface_mir_samples.py run ELAB-14 --format json
python3 scripts/surface_mir_samples.py run ELAB-15 --format json
python3 scripts/surface_mir_samples.py run ELAB-16 --format json
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
cargo fmt
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 scripts/validate_docs.py
git diff --check
date '+%Y-%m-%d %H:%M %Z'
git status --short --branch
git rev-parse HEAD origin/main
sed -n '1,220p' /home/codex/dev/mir_poc_01/.agents/skills/discord-report/SKILL.md
sed -n '1,220p' /home/codex/.codex/superpowers/skills/verification-before-completion/SKILL.md
rg -n 'Result pending|still running|49 sample|49 helper|validate 49|ELAB-01\\.\\.13|1290 reports|TBD|TODO' docs/reports/2139-g1-erow001-base-singleton-fixture-closure.md Documentation.md progress.md tasks.md samples_progress.md plan/92-g1-erow001-base-singleton-fixture-closure.md
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- RED Python: targeted helper test failed for `ELAB-14`, `ELAB-15`, and
  `ELAB-16` with `unknown Surface Mir sample`.
- RED Rust setup correction: first Rust sample-path RED failed on `ELAB-13`
  due crate-relative path lookup; after fixing test setup to repo-root relative,
  the RED failure moved to missing `ELAB-14`, as intended.
- GREEN Python: targeted helper test passed for `ELAB-13..16`.
- GREEN Rust: sample-path singleton fixture test passed for `ELAB-13..16`.
- `python3 scripts/surface_mir_samples.py run ELAB-14 --format json`: passed
  with `accepted: true`, `mismatches: []`, `missing_evidence:
  ["MissingCapability"]`, and no `suggested_repair`.
- `python3 scripts/surface_mir_samples.py run ELAB-15 --format json`: passed
  with `accepted: true`, `mismatches: []`, `missing_evidence:
  ["RouteUnavailable"]`, and no `suggested_repair`.
- `python3 scripts/surface_mir_samples.py run ELAB-16 --format json`: passed
  with `accepted: true`, `mismatches: []`, `missing_evidence:
  ["StaleMembership"]`, and no `suggested_repair`.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  18 passed, 0 failed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 43 tests
  passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: 52 rows
  passed, `failed: []`.
- `python3 scripts/check_source_hierarchy.py`: required 598, present 598,
  missing 0.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- First `cargo fmt --check`: failed on Rust test formatting only.
- `cargo fmt`: applied formatting.
- Second `cargo fmt --check`: passed.
- Final pre-commit `cargo fmt --check`: passed.
- `python3 scripts/validate_docs.py`: complete, 1290 numbered reports before
  adding this report; final pre-commit rerun found 1291 numbered reports
  including this report.
- `git diff --check`: passed.
- Read-only reviewer found no blocker. The only low traceability follow-up was
  addressed by adding ELAB-14..16 README/source `.mir` paths to the `plan/92`
  row in `plan/90-source-traceability.md`.

## What changed in understanding

The implementation and helper path treat the four base remote-request failures
symmetrically, but a future class-wide repair-widening claim would expose the
missing atom in diagnostic and repair payloads. Therefore the safer LAB gate is
one no-repair singleton fixture per base failure atom before any repair output
widening.

The current state is now:

- `ELAB-10`: only repair-bearing singleton, `E-ROW-002` /
  `VisibilityDenied`.
- `ELAB-13..16`: no-repair singleton fixture set for all four
  non-visibility base remote-request failures under `E-ROW-001`.
- `ELAB-04/07`: no-repair mixed / multi-missing fences.

## Open questions

- Whether the later repair-bearing prototype should flip all four
  non-visibility singleton rows at once or stage one row first while preserving
  the other singleton rows as no-repair fences.
- What exact single-edit assumption should govern non-visibility singleton
  repair output.
- Whether mixed / multi-missing rows should eventually decompose into several
  singleton repairs or remain no-repair until ranking and atomicity are settled.
- Whether target-span / declaration-span should become more precise than the
  current LAB-local `target_ref`.

## Suggested next prompt

Continue self-driven G1 E-ROW work by defining the LAB-only single-edit
assumption and no-placeholder payload tests for a possible non-visibility
singleton repair prototype, while preserving `ELAB-04/07` as no-repair fences.

## Plan update status

更新済み:

- Added `plan/92-g1-erow001-base-singleton-fixture-closure.md`.
- Updated `plan/88-g1-erow-repair-shape-inventory.md`.
- Updated `plan/89-g1-erow001-non-visibility-singleton-fixture.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み: Surface Mir line count and G1 E-ROW summary now include
`ELAB-13..16`, 52 sample rows, 53 `.mir` source files, and no repair output
beyond `ELAB-10`.

## progress.md update status

更新済み: current E-ROW note, helper/source counts, feature row, and recent log
now include the full `E-ROW-001` base singleton no-repair fixture set.

## tasks.md update status

更新済み: current holding state now includes `plan/92`; candidate packages were
reframed around the singleton repair assumption and a later singleton repair
prototype.

## samples_progress.md update status

更新済み: Surface dashboard now validates 52 rows / 53 `.mir` sources and
records `ELAB-13..16` as the no-repair singleton fixture set.

## Reviewer findings and follow-up

Sub-agent code-mapper `019f2888-474d-7892-babd-1b30b781f1df` found the current
implementation and helper paths symmetric across all four base failures and
recommended that one representative would be enough for the current no-repair
boundary.

ChatGPT Pro Oracle `we-are-working-in-the-2` recommended adding all three
remaining singleton no-repair fixtures before any class-wide widening. I chose
the Oracle recommendation because this package is setting a future
specification/testing gate rather than only documenting current implementation
symmetry.

Read-only reviewer `019f2893-6710-7f42-a6c3-922dc8fb4073` found no blockers.
It confirmed that stale 49/50 counts remain only in timestamped historical
logs, current status uses 52 rows / 53 `.mir` sources, ELAB-14..16 omit exactly
one intended base failure, and `suggested_repair` is absent from singleton
no-repair fixtures. Its only low finding was weaker `plan/90` traceability for
ELAB-14..16; follow-up added the README and source `.mir` paths.

## Skipped validations and reasons

- Full workspace Cargo test/build/clippy was not rerun in this package because
  the change is isolated to Surface elaboration tests, sample fixtures, docs,
  and validators.
- Surface release check was not rerun because this package only adds LAB G1
  elaboration rows and `surface_mir_samples.py check-all` covers the touched
  sample family.

## Commit / push status

Pending at initial report write.

## Sub-agent session close status

- Code-mapping sub-agent completed and was closed.
- Reviewer sub-agent completed and was closed before package close.
