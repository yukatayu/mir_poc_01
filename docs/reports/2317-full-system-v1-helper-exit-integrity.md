# Report 2317 — Full System V1 helper exit integrity and row-count correction

- Date: 2026-07-21 23:08 JST
- Author / agent: Codex with read-only reviewers Banach and Erdos
- Scope: exit-code/evidence agreement for active Full System V1 parser, PoseGraph, and projection helpers; forward correction of the active 41-row snapshot
- Decision levels touched: none; LAB evidence-maintenance only

## Objective

Make the remaining active Full System V1 helper rows reject a contradictory
nested command exit code even when the JSON projection matches committed
expected evidence, and correct the active snapshot's mistaken 42-row claim.

## Scope and assumptions

The existing nested examples use exit code `0` for an accepted payload and `2`
for a rejection or violation. Helper-level expected-negative rows still succeed
only when that inner result, the projected JSON, and any committed generated
evidence agree. This package does not add language syntax or alter bounded
runtime semantics.

## Start state / dirty state

Started from pushed commit `e450d09e` with a clean worktree. The immediately
preceding package had aligned the Full System V1 runtime-matrix readiness field
with the existing evidence-closed dashboard classification.

## Documents consulted

- `AGENTS.md`
- `CANON.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `tasks.md`
- `progress.md`
- `samples_progress.md`
- `docs/project-status.md`
- `scripts/README.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`
- `plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`
- active helper scripts, matrices, expected JSON, and focused unit tests

## Actions taken

1. Mapped writes and expected-evidence handling across all active Full System
   V1 helpers. The projection, parser, and PoseGraph helpers already compared
   fresh results without rewriting committed artifacts, but did not bind their
   pass result to the nested exit code.
2. Added three regression tests first. Each supplies an expected rejected
   payload with a fabricated success exit code and requires helper failure.
3. Confirmed all three tests failed before implementation.
4. Added `returncode_expected` and `returncode_passed` to the parser,
   PoseGraph, and projection/local-split result surfaces; their pass result now
   includes that agreement.
5. Ran direct accepted/rejected helper paths and each helper's complete matrix.
6. Updated dashboard, task map, status view, script guidance, and LAB memory.
7. Incorporated the independent planner's evidence-count audit: the aggregate
   partition is 12 checker + 17 runtime + 12 operational = 41. Historical
   Report 2316 remains unchanged; this report and `progress.md` correct its
   active-snapshot mirror.

## Files changed

- `scripts/textual_mir_samples.py`
- `scripts/posegraph_runtime_samples.py`
- `scripts/projection_v1_samples.py`
- `scripts/tests/test_textual_mir_samples.py`
- `scripts/tests/test_posegraph_runtime_samples.py`
- `scripts/tests/test_projection_v1_samples.py`
- `scripts/README.md`
- `plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2317-full-system-v1-helper-exit-integrity.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- focused `rg` / `sed` source, matrix, expected-evidence, and Canon checks
- three focused unit tests before the implementation change
- three focused unit tests after the implementation change
- `python3 -m unittest scripts.tests.test_textual_mir_samples scripts.tests.test_posegraph_runtime_samples scripts.tests.test_projection_v1_samples`
- direct parser, PoseGraph, and projection negative helper runs
- the three helpers' `check-all --format json` commands
- `python3 -m unittest scripts.tests.test_full_system_v1_samples scripts.tests.test_full_system_v1_release_check`
- final combined focused helper/aggregate/release unit suite
- `python3 scripts/full_system_v1_samples.py check-all --format json`
- active-source row-count search with `rg`
- `make check && git diff --check`

## Evidence / outputs / test results

Before implementation, all three new regression tests failed because a nested
exit code `0` was ignored when expected rejection JSON matched. After the
change:

- the textual unresolved-import negative reported raw `returncode: 2`,
  `returncode_expected: 2`, and `returncode_passed: true` while the helper row
  correctly passed its expected-negative evidence;
- the PoseGraph split-frame violation reported the same `2/2/true` agreement;
- the projection client-write rejection reported the same `2/2/true` agreement;
- all 29 focused Python tests passed;
- parser `check-all` passed 10 rows, PoseGraph `check-all` passed 9 rows, and
  projection/local-split `check-all` passed 6 rows, with no validation errors.
- the aggregate helper's existing contract fixes the executable partition at
  12 + 17 + 12 = 41; the former current-snapshot wording of 42 was a
  documentation error, not executable drift.
- the focused aggregate/release suite passed 34 tests, and the aggregate
  `check-all` reran all 41 rows with no failures or validation errors.
- the final combined helper/aggregate/release unit suite passed 63 tests.
- `make check` passed Canon index validation (84 files), source hierarchy
  validation (711/711 paths), documentation validation (1,471 reports), and
  `cargo check`; `git diff --check` was empty.

## What changed in understanding

Expected rejection JSON is not sufficient evidence on its own. The nested
command's process result is a separate contract surface and must agree with the
payload's accepted/rejected state. The bounded Full System V1 helpers now apply
that rule consistently across parser, checker/runtime, PoseGraph, projection,
provider, and renderer families.

The source-first aggregate contains 41 executable rows, not 42. The error was
limited to Report 2316 and its then-current progress mirror; no historical
report was rewritten.

## Open questions

No new theoretical question was introduced. A separate reviewer identified
three existing-Canon checker gaps outside this helper package: operation-specific
capability binding, duplicate record field rejection, and composite equality
rejection. They are explicitly scheduled in `tasks.md` and are not claimed
fixed here.

## Suggested next prompt

Continue with the scheduled Full System V1 semantic invariant repair: reproduce
the three reviewer findings as adversarial source tests, repair the bounded
checker, and verify that no privileged external operation reaches runtime under
a mismatched capability.

## Plan update status

`plan/` 更新済み: `plan/161` now records the full active helper exit-code
integrity floor. It does not change the roadmap, candidate triage, or Canon
authority boundary.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing project entry, command family,
or scoped capability claim changed.

## docs/project-status.md update status

更新済み: the concise human view now separates the strengthened helper
integrity floor from the three reviewer-found bounded checker gaps awaiting
repair.

## progress.md update status

更新済み: the recent log records the helper integrity closeout and the next
maintenance package.

## tasks.md update status

更新済み: the current task map now lists the three reviewer-found invariant
repairs as the next self-driven Macro 2/3 maintenance package.

## samples_progress.md update status

更新済み: active computational, PoseGraph, projection, and local-role-split
rows now state their inner `0/2` exit-code validation and the pending bounded
computational guards.

## Reviewer findings and follow-up

Banach's independent read-only computational review found three untested
checker invariants: a `write_int` effect can be redeclared with `HostRead` and
still reach `host_output`; duplicate record fields are set-compared then
silently overwritten at runtime; and record/array equality is accepted despite
the current static-semantics restriction. Local source inspection confirmed the
reported checker/interpreter shapes and their Canon relevance. The repairs are
out of scope for this helper-only package and are recorded as the next task.

Erdos's read-only planning review confirmed the 12/17/12 = 41 partition and
identified the Report 2316/progress mirror error. It found no qualifying new
L3 candidate and advised against unrelated Full System V1 widening. Its
recommendation is incorporated as this forward correction; the scheduled
semantic invariant repair remains narrow maintenance for a concrete
reviewer-found defect, not a new runtime line.

## Skipped validations and reasons

The full 29-command release-check and broad Cargo suite were not repeated in
this package: the changed behavior is covered by three focused regression
tests, 29 focused helper tests, and all 10/9/6 active helper rows. The prior
release-check was accepted immediately before these maintenance packages.
Focused aggregate-count and release-count tests, repository documentation
validation, source-hierarchy validation, and the aggregate check were run after
this report was finalized; only the full 29-command release-check and broad
Cargo suite remain intentionally skipped for the stated scope reason.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Banach and Erdos completed their read-only reviews and were closed. Neither
sub-agent made workspace edits.
