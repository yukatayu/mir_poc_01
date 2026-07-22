# Report 2358 — Theory core correspondence audit

- Date: 2026-07-22 16:21 JST
- Author / agent: Codex
- Scope: Canon-to-LAB theory correspondence, existing Lean replay, and next-action triage
- Decision levels touched: L3 research interpretation only; no L0/L1 decision

## Objective

Reconcile the canonical theory core with current Lean and runtime evidence,
then identify whether a non-duplicative autonomous research record is available.

## Scope and assumptions

`mirrorea_canon/` was treated as normative. Existing Lean drafts,
countermodels, Rust/runtime samples, Oracle opinions, and sub-agent reviews were
treated as LAB/advisory evidence only. No Canon file was edited.

## Start state / dirty state

`main...origin/main` was clean at `d1e586af`. The configured external workdir
was unavailable; the fresh Lean output was kept under `/tmp` and not committed.

## Documents consulted

Read the Canon entry points, theory/01, theory/03, theory/11, ADR-0014,
working annex, PROPOSAL-003, PROPOSAL-008, relevant WRK-0004/0006/0007 records,
and the current LAB status, plan, report, and sample dashboards. Consulted
plan/156, plan/163--170, and the post-WRK-0013 triage records as LAB memory.

## Actions taken

Replayed the current Lean statements and import-bearing countermodels in a clean
detached worktree. Compared the actual claims against Canon THM-001, OBL-020,
and OBL-021. Obtained two independent sub-agent reviews and a temporary Oracle
review. Recorded the resulting correspondence matrix and non-duplicative
research disposition in plan/171.

## Files changed

- `plan/171-theory-core-correspondence-and-disposition-checkpoint.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- this report

## Commands run

- `python3 scripts/current_l2_lean_sample_sync.py`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- direct Lean checks for the OBL-001, OBL-020, and OBL-021 statement drafts
- external import-relative `.olean` replay with `LEAN_PATH` for WRK-0007,
  WRK-0006, and WRK-0004
- `make docs`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 -m py_compile scripts/validate_docs.py scripts/check_source_hierarchy.py`
- `git diff --check`
- source inspections with `rg` and `sed`
- `df -h .`, `free -h`, and detached-worktree status checks

## Evidence / outputs / test results

The synchronizer passed; its focused suite passed 21 tests. All three
standalone drafts and all three listed import-bearing countermodels compiled.
Fresh external Lean output was 588 KiB. `make docs` passed the Canon index check
(93 files), source hierarchy (721 required/present), and document validator
(1512 numbered reports). The documentation unit suite passed 87 tests; Python
compilation and `git diff --check` passed. Root storage had about 8.6 GiB free
(96% used), so no broad rebuild was started. Oracle and sub-agent reviews agreed
that the current evidence establishes LAB statement boundaries, not a Canon
proof or a new selected formal interface.

## What changed in understanding

The current blocker is not a missing small counterexample. The three existing
findings delimit how a later proof-facing package must be formed: direct Core
write coverage for OBL-001, demonstrated coverage before using familywise
OBL-020 reasoning as a global result, and an explicit totality treatment for
any OBL-021 result that needs an outcome. The BND-001 interpretation and
placement remains an open owner-decision request through PROPOSAL-008; it has no
owner answer or automatic Canon effect.

## Open questions

- PROPOSAL-008: does BND-001 require total outcome production, and where is it
  tracked?
- What direct Canon-aligned Core/write representation should a future
  proof-facing OBL-001 package use?
- When such a package exists, is a shared OBL-020 review checklist desired
  under PROPOSAL-003?

## Suggested next prompt

Record an owner response to PROPOSAL-008 and complete the applicable Canon
process, or authorize a specific proof-facing Core formalization boundary after
reviewing plan/171. Until then, continue only with new, source-grounded L3
candidates that have distinct live outcomes.

## Plan update status

`plan/` 更新済み: plan/171 records the compact correspondence and disposition
checkpoint; plan/00-index registers it.

## Documentation.md update status

`Documentation.md` 更新済み: the theory checkpoint is linked from the reading
map.

## docs/project-status.md update status

更新済み: the concise status now distinguishes the three proof-hygiene
boundaries from an OBL or workflow completion claim.

## progress.md update status

`progress.md` 更新済み: the logical-specification and recent-log snapshots now
record the correspondence cut and its remaining decision boundary.

## tasks.md update status

`tasks.md` 更新済み: the closed checkpoint and current proof-facing reopen
conditions are reflected in the task map.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or workflow classification changed.

## Reviewer findings and follow-up

Two sub-agents independently confirmed that the Lean countermodels are limited
to their abstract LAB statements and that LAB runtime behavior is not Canon
proof. The temporary Oracle review recommends a narrow owner-decision request
rather than another L3 experiment. The final reviewer found an incorrect
owner-disposition implication, two LAB/Canon wording conflations, and missing
validation evidence; all four were corrected before final validation. Follow-up
is limited to PROPOSAL-008 or a genuinely new source-grounded candidate; no
WRK-0014 is opened.

## Skipped validations and reasons

No broad Cargo, Docker, or full release sweep was run. This task changed only
documentation and source-registration metadata; root storage was already at
96% use, and the focused Lean and documentation validation are the relevant
evidence. No implementation behavior changed.

## Commit / push status

Pending at report write; this task package will be committed with
`--no-gpg-sign` and pushed after validation.

## Sub-agent session close status

Three completed review sub-agents were closed after their findings were
incorporated. The Oracle consultation completed successfully.
