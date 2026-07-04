# 2164 - G1 OBL-025 repair completeness guard hardening

## Title and identifier

2164 - G1 OBL-025 repair completeness guard hardening

## Objective

Harden the LAB-only OBL-025 Lean sync guard so the current repair-completeness
draft cannot silently drift into a placeholder non-empty repair-list reading,
repair ranking, all-repairs/minimality wording, final repair ABI naming, or
branch-local guidance as whole-gap coverage.

## Scope and assumptions

- Scope is limited to repository memory, LAB Lean statement-draft explanation,
  and sync-unit guard tests around
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`.
- `mirrorea_canon/` remains the normative source. This task did not edit canon.
- The current draft remains compile-check-only LAB evidence. This task does not
  prove OBL-025, move canon ledger status, claim G1 exit, freeze Diagnostic /
  repair ABI, change runtime JSON, or change executable repair output.
- Branch-local guidance remains outside current whole rejected-gap coverage
  unless a later whole-gap relation covers every missing failure.

## Start state / dirty state

- Started from clean pushed `ad22d968` (`origin/main` matched `HEAD`).
- Discord task baseline was recorded before package work with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Local package changes were present when this report was first written; no
  unrelated user changes were observed in the working tree.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-overview.md`
- `specs/01-core-invariants.md`
- `specs/02-atomic-cut.md`
- `specs/03-place-and-authority.md`
- `specs/09-runtime-and-adapters.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`
- `samples/lean/lab-statements/obl025/README.md`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- Sub-agent mapping findings from the OBL-025 guard-hardening package.

## Actions taken

- Added `plan/116-g1-obl025-repair-completeness-guard-hardening.md`.
- Added a sync-unit guard for the OBL-025 draft that checks:
  - positive vocabulary for `EligibleSingleEditRepair`,
    `SuggestionCoversWitness`, whole-gap repair/suggestion coverage, grouped
    multi-edit, partial-guidance non-coverage, and branch-local non-coverage;
  - negative vocabulary for ranking, all-repairs/minimality, final repair ABI,
    and placeholder repair-list names;
  - explanatory text that the draft is not a placeholder non-empty repair list,
    not repair ranking, not all possible repairs, and not branch-local
    whole-gap coverage;
  - the body of `RepairCompletenessForRejection` still requires an eligible
    single-edit witness and concludes through `SuggestedRepairOf` plus
    `SuggestionCoversWitness`;
  - grouped multi-edit, partial guidance, and branch-local repair/suggestion
    helpers remain explicit non-coverage helpers.
- Updated the OBL-025 LAB explanation and local README to state the narrowed
  boundary.
- Updated `README.md`, `Documentation.md`, `plan/00-index.md`,
  `plan/87-g1-obl025-lean-statement-draft.md`,
  `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`,
  `plan/90-source-traceability.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md` so repository memory matches the new guard package.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`
- `plan/116-g1-obl025-repair-completeness-guard-hardening.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/lean/lab-statements/obl025/README.md`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `docs/reports/2164-g1-obl025-repair-completeness-guard-hardening.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl025_draft_names_repair_completeness_boundary`
  - First run intentionally failed before the explanatory boundary phrases were
    added.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl025_draft_names_repair_completeness_boundary`
  - Passed after adding and line-splitting the explanatory boundary phrases.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl025_draft_names_repair_completeness_boundary`
  - Passed after adding body-level assertions.
- `lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `git status --short -- samples/lean/manifest.json samples/lean/clean-near-end`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- Changed-file endpoint leak scan with `rg --pcre2`.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl025_draft_names_repair_completeness_boundary`
  - Passed after final reviewer follow-up added body-level guards for
    `EligibleSingleEditRepair`, `SuggestionCoversWitness`, and
    `AssociatedEmittedDiagnostic`, and replaced exact indentation checks with
    whitespace-tolerant regex checks.

## Evidence / outputs / test results

- Targeted RED confirmed the new OBL-025 boundary test failed before the
  explanatory phrase `not a placeholder non-empty repair list` existed.
- Targeted GREEN passed after the explanatory boundary phrases were added.
- Targeted GREEN passed after body-level assertions were added.
- `lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
  passed with no output after final reviewer follow-up.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed:
  13 tests after final reviewer follow-up.
- `python3 scripts/current_l2_lean_sample_sync.py` printed
  `samples/lean/manifest.json`; no tracked manifest / clean-near-end diff was
  left afterward after final reviewer follow-up.
- `python3 scripts/validate_docs.py` passed and reported 1315 numbered reports
  before this report was added.
- After this report was added, `python3 scripts/validate_docs.py` passed and
  reported 1316 numbered reports. It also passed after final reviewer
  follow-up.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests
  after final reviewer follow-up.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required, 602
  present, 0 missing after final reviewer follow-up.
- `git diff --check` passed with no output after final reviewer follow-up.
- Changed-file endpoint leak scan reported no endpoint matches in changed files.
- The final reviewer follow-up targeted test passed: 1 test.

## What changed in understanding

The current OBL-025 LAB statement draft is better understood as a very narrow
single-edit whole-gap coverage shape. The useful guard is not just that the
right vocabulary exists, but that the central definition still goes through an
eligible single-edit witness and a diagnostic suggestion that covers that
witness. Grouped multi-edit, partial guidance, and branch-local guidance should
remain named because they are important future vocabulary, but current coverage
must continue to treat them as non-coverage helpers.

## Open questions

- OBL-025 proof obligations remain OPEN.
- Final Diagnostic / repair ABI, repair JSON shape, branch identifiers, and
  replay/association proof-level relations remain OPEN.
- Repair ranking, all-repairs/minimality, bundle semantics, and general
  set-insertion support remain OPEN.
- `ELAB-04` mixed visibility branch repair output remains deferred.

## Suggested next prompt

Continue the G1 ratchet by either hardening the remaining LAB statement drafts
where drift risk is high, or by opening the next narrow executable evidence
package only when the relevant statement boundary is already explicit.

## Plan update status

Updated. Added `plan/116-g1-obl025-repair-completeness-guard-hardening.md` and
linked it from the plan index, OBL-025 statement memory, branch-local
non-coverage memory, and source traceability table.

## Documentation.md update status

Updated. `Documentation.md` now names OBL-025 repair completeness guard
hardening and records the placeholder/ranking/all-repairs/branch-local
whole-gap drift guard without changing final claims.

## progress.md update status

Updated. `progress.md` now records the OBL-025 repair completeness guard
hardening in the current statement-draft note, feature status table, and recent
log.

## tasks.md update status

Updated. `tasks.md` now records `plan/116` in the current holding state and
keeps the candidate next strategy packages as candidates only.

## samples_progress.md update status

Updated. `samples_progress.md` now records OBL-025 repair completeness guard
hardening in the current focus and recent validation log.

## Reviewer findings and follow-up

- Mapping sub-agent found that the narrow useful hardening was in
  `scripts/tests/test_current_l2_lean_sample_sync.py`, with body-level guards
  for `RepairCompletenessForRejection` and the non-coverage helper definitions.
  Follow-up: implemented those body-level guards.
- Final reviewer reported no blocking findings and no claim-boundary drift.
  Medium follow-up: the sync guard was still semantically under-scoped because
  it did not body-check `EligibleSingleEditRepair`,
  `SuggestionCoversWitness`, or `AssociatedEmittedDiagnostic`. Follow-up:
  added body-level assertions that eligible single-edit repair still includes
  declared-fragment and rejected-gap coverage, that suggestion coverage still
  realizes a compatible witness and covers the rejected gap, and that
  `RepairCompletenessForRejection` still keeps
  `AssociatedEmittedDiagnostic`.
- Final reviewer also found low-severity formatting brittleness in exact
  `forall witness` indentation checks. Follow-up: replaced those checks with
  whitespace-tolerant regexes.
- Final reviewer noted report placeholders. Follow-up: replaced pending
  reviewer / sub-agent lines with this completed status.

## Skipped validations and reasons

- Rust runtime / JSON fixture validation was not run because this package did
  not change Rust runtime code, expected JSON, executable repair output, or
  sample source rows.
- Oracle consult was not run because the task was a narrow guard-hardening
  package following existing OBL-025 direction and a focused sub-agent mapping
  review.
- No canon ledger validation beyond local source-hierarchy/docs checks was run
  because canon files were not edited and no OBL status movement is claimed.

## Commit / push status

- Body commit `e8e83ec1` (`Harden OBL-025 repair completeness guards`) was
  pushed to `origin/main`.
- This report status update is committed separately so the pushed body commit
  can be named accurately.

## Sub-agent session close status

- Mapping sub-agent `019f2a8f-e346-7702-b6ce-afd34a32b681` completed and was
  closed.
- Final reviewer sub-agent `019f2a98-18d2-7a13-af23-574fab990079` completed
  and was closed.
