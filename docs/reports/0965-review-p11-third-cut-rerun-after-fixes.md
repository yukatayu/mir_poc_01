# 0965 — review P11 third-cut rerun after fixes

## Objective

Re-review the current uncommitted `P11` third-cut work after the parity-helper fixes, with maintainer focus on semantic correctness, regression risk, and any remaining doc overclaim around shell-backed bootstrap / join / leave parity.

## Scope and assumptions

- Review only; no implementation changes were requested.
- Primary code scope:
  - `crates/mirrorea-core/src/runtime.rs`
  - `crates/mirrorea-core/tests/runtime_substrate.rs`
- Secondary wording scope:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/16-shared-space-membership-and-example-boundary.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/19-repository-map-and-taxonomy.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `docs/hands_on/current_phase_closeout_01.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
- Helper-side Sugoroku join/leave behavior remains the parity anchor.
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要
- `plan/` 更新不要

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0963-p11-shell-bootstrap-join-leave-parity.md`
- `docs/reports/0964-review-p11-third-cut-uncommitted.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`

## Actions taken

1. Re-read the required front-door docs and normative spec anchors before judging the updated implementation.
2. Inspected the current runtime/test bodies and the helper-side Sugoroku changes that now serve as the parity anchor.
3. Verified that the shell helper methods now derive `ParticipantPlace[{principal}]` internally and use rollback on newly registered places when membership insertion fails.
4. Re-checked the wording sweep for overclaiming against the narrowed shell frontier.
5. Ran crate tests, helper tests, closeout output, and lightweight repo validation commands.

## Evidence / outputs / test results

- `cargo test -p mirrorea-core`
  - passed
  - runtime substrate tests: `11 passed`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - passed
  - `42 tests`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - passed
  - helper closeout still reports `runtime_components` including helper-local `PlaceRuntime` / `MessageQueue` / `SugorokuState`
  - helper closeout still reports `membership_model.source_of_truth = MembershipRegistry`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 962 numbered report(s).`
- `python3 scripts/check_source_hierarchy.py`
  - `required: 23`
  - `present: 23`
  - `missing: 0`
  - `all required paths present`
- `git diff --check`
  - no output

## What changed in understanding

- The previous semantic mismatch is addressed: the shell-backed participant helpers now derive the participant place from the principal rather than accepting an arbitrary caller-supplied place string.
- The previous partial-update risk is also addressed for the new-place path: the shell rolls back newly introduced place registration when membership insertion fails.
- The reader-facing docs now better separate shell-backed membership/place parity from helper-owned game-domain aftermath.

## Open questions

- No blocking semantic question remains in the reviewed scope.
- Residual non-blocking test gap: there is still no shell-level regression that exercises duplicate-principal failure through `add_participant`, although the underlying invariants and current implementation shape appear correct.

## Suggested next prompt

Continue `P11` by tightening the next kept-later boundary without moving `WorldState`, `PlaceRuntime`, `MessageQueue`, or `SugorokuState` into Rust, and add any shell-level regression tests you want before widening the runtime substrate further.
