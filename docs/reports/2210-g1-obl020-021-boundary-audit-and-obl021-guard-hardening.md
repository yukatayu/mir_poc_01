# Report 2210 - G1 OBL-020/021 boundary audit and OBL-021 guard hardening

- Date: 2026-07-04 19:26 JST
- Author / agent: Codex
- Scope: LAB repository memory, Lean statement sync tests, snapshot docs, validators, sub-agent review, and report
- Decision levels touched: L0/L1 canon references only; no canon decision changed

## Objective

Audit whether OBL-020 and OBL-021 statement boundaries need refinement after the
SCN-02 blocker review, then harden the OBL-021 sync guard only if a concrete
maintenance weakness is found.

## Scope and assumptions

The source hierarchy remains unchanged: `mirrorea_canon/` is normative, legacy
`specs/` are LAB-facing specification evidence, `plan/` is repository memory,
and samples / helpers / tests are executable evidence.

This package is a G1 bridge support package. It does not edit canon, claim G1
exit, move OBL status, prove OBL-020/021, select final equality, claim runtime
dispatch, or change sample status.

## Start state / dirty state

Start state was clean on `main` with `main...origin/main` at
`1a0f5983633e0c1871bf42fe3a9130e6fbefb73f`
(`Record SCN-02 blocker review commit`).

The Discord report skill task baseline for P72 was recorded before inspection
and edits with `python3 .agents/skills/discord-report/scripts/discord_notify.py
begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- `plan/125-g1-scn02-direct-local-write-blocker-review.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `scripts/README.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Added `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`.
- Recorded that no Lean predicate refinement is needed for OBL-020 or OBL-021
  at this checkpoint.
- Preserved the non-claims: no OBL completion, no proof skeleton, no proof
  discharge, no G1 exit, no conformance claim, no runtime dispatch, and no
  final ABI freeze.
- Added TDD red tests showing that the old guard accepted bare `:= True` and
  comment-only required body links.
- Hardened `scripts/tests/test_current_l2_lean_sample_sync.py` so body-link
  checks strip Lean comments and vacuity checks reject bare `:= True`.
- Routed OBL-001 / OBL-020 / OBL-021 required body-link and regex checks through
  the uncommented-body helpers.
- Updated `plan/00-index.md`, `plan/90-source-traceability.md`, `README.md`,
  `Documentation.md`, `progress.md`, `tasks.md`, `scripts/README.md`,
  `scripts/validate_docs.py`, `scripts/check_source_hierarchy.py`, and
  `scripts/tests/test_validate_docs.py`.
- Used two read-only sidecar reviewers for OBL-020 and OBL-021 challenge review
  and closed both sessions after collecting the results.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2210-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`

## Commands run

- `sed -n ...` / `rg -n ...` / `rg --files ...` for consulted repo, canon,
  specs, plan, progress, tasks, scripts, and Lean files
- `git status --short --branch`
- `git rev-parse HEAD origin/main`
- `date '+%Y-%m-%d %H:%M %Z'`
- `lake env lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `lake env lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` after
  adding red tests, expected failure
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` after
  guard hardening
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py --format json`
- `python3 scripts/check_source_hierarchy.py --format json | jq '{status, required_count, present_count, missing_count}'`
- `git diff --check`
- endpoint scan over changed and untracked files for Discord webhook URL
  patterns

## Evidence / outputs / test results

- OBL-020 read-only reviewer verdict: no new docs-only or Lean/test guard is
  needed for OBL-020 now. The existing `StepWFStatementDraft.lean` boundary is
  sufficient while the bridge stays static/elaboration level; future refinement
  should wait for concrete `Config`, `StepLabel`, `StepFamily`, `WellFormed`,
  and per-step proof obligations.
- OBL-021 read-only reviewer verdict: no semantic or Lean statement-boundary
  refinement is needed for the next G1 bridge. It found one medium guard
  weakness: raw required-link assertions could be satisfied by comments, and
  bare `:= True` was not rejected.
- `lake env lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
  and `lake env lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
  passed with no output before edits.
- Baseline `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  passed before edits: 19 tests OK.
- Red TDD run passed as a red signal: 21 tests ran, with two expected failures:
  `test_no_vacuous_weakening_rejects_bare_true_definition` and
  `test_required_lean_body_link_helper_ignores_comments`.
- Green TDD run after implementation passed: 21 tests OK.
- Final Lean checks passed:
  `lake env lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
  and
  `lake env lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
  returned exit code 0 with no output.
- Final `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  passed: 21 tests OK.
- Final `python3 -m unittest scripts.tests.test_validate_docs` passed:
  37 tests OK.
- First `python3 scripts/validate_docs.py` after report creation failed because
  the last four report headings were lowercase while the template requires
  `Reviewer findings and follow-up`, `Skipped validations and reasons`,
  `Commit / push status`, and `Sub-agent session close status`.
- After fixing the report headings, `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete`; found 1362 numbered reports.
- `python3 scripts/check_source_hierarchy.py --format json` passed:
  status `ok`, required `666`, present `666`, missing `0`.
- `git diff --check` passed with no whitespace errors.
- Endpoint scan over changed and untracked files found no Discord webhook URL
  pattern.

## What changed in understanding

The OBL-020 and OBL-021 statement drafts are not the next semantic blocker for
the current G1 bridge. Their current abstract predicate boundaries remain the
right level until a later proof package chooses concrete step families,
well-formedness clauses, final equality, projection-totality, and diagnostic
equivalence details.

The actual actionable issue was narrower: the sync guard could be fooled by
comment-only required predicates or by a bare `:= True` body. That is now a
test-only maintenance hardening, not a semantic change.

## Open questions

- When should OBL-020 move from statement-shape draft to concrete `WellFormed`
  clauses and per-step preservation lemmas?
- When should OBL-021 choose the final result equality and diagnostic
  equivalence relation?
- Should future proof packages move these guards from LAB tests into a
  dedicated Lean-side lint or statement-shape checker?

## Suggested next prompt

Continue the G1 ordinary-assignment bridge from `plan/121` / `plan/122` using
`plan/126` as the closed OBL-020/021 boundary check. Keep OBL-020/021
refinement reserve-only unless a concrete proof package or bridge blocker
reopens it.

## Plan update status

`plan/` 更新済み:

- Added `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Added the OBL-020/021 boundary audit and OBL-021 guard hardening to the
  Surface/G1 LAB memory summary without changing canon, proof, conformance,
  runtime, ABI, or sample status.

## progress.md update status

`progress.md` 更新済み:

- Updated timestamp to `2026-07-04 19:26 JST`.
- Added the `plan/126` current note.
- Updated the Macro 5 and LAB Lean statement draft rows.
- Added a recent log entry for this package.

## tasks.md update status

`tasks.md` 更新済み:

- Updated timestamp to `2026-07-04 19:26 JST`.
- Added the `plan/126` holding-state note.
- Updated validator/scaffold range wording to `plan/00..126` /
  `plan/39..126` / `plan/118..126`.
- Kept OBL-020 and OBL-021 statement refinement as reserve-only after this
  audit.

## samples_progress.md update status

`samples_progress.md` 更新不要:

- No runnable sample path, sample row, validation command, or sample dashboard
  status changed.

## Reviewer findings and follow-up

- OBL-020 sidecar reviewer found no immediate boundary or guard blocker.
  Follow-up is deferred until concrete runtime step families and proof
  obligations are introduced.
- OBL-021 sidecar reviewer found no semantic boundary blocker, but did find the
  comment-only required-link / bare-`True` sync-guard weakness. This package
  closed that weakness in `scripts/tests/test_current_l2_lean_sample_sync.py`.

## Skipped validations and reasons

Skipped broader Cargo / sample runner suites. Reason: this package only changes
LAB docs, validator registration, and the Lean statement sync unit test; the
affected Lean drafts and focused unit/docs validators were run directly.

## Commit / push status

Pending.

## Sub-agent session close status

Closed:

- OBL-020 boundary reviewer `019f2ca1-619f-7630-b468-8b7298c856b8`
- OBL-021 boundary reviewer `019f2ca1-8007-72f1-8d14-323eab05844f`
