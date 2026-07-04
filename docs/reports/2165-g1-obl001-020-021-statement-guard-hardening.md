# 2165 - G1 OBL-001 / OBL-020 / OBL-021 statement guard hardening

## Title and identifier

2165 - G1 OBL-001 / OBL-020 / OBL-021 statement guard hardening

## Objective

Harden the LAB-only Lean sync-unit guards for OBL-001, OBL-020, and OBL-021 so
the older compile-check-only statement drafts cannot silently drop guarded
body-level links or use obvious vacuous weakeners while still compiling and
remaining registered in `samples/lean/manifest.json`.

## Scope and assumptions

- Scope is limited to sync-unit guard tests, LAB statement explanations,
  per-directory statement README files, and repository memory/status docs.
- `mirrorea_canon/` remains the normative source. This package did not edit
  canon.
- The Lean statement drafts remain compile-check-only LAB `Prop` shapes.
- This package does not claim OBL-001, OBL-002, OBL-020, or OBL-021 completion,
  proof discharge, proof skeleton completion, G1/T1/T2 exit, conformance,
  runtime dispatch, runtime scheduling determinism, final equality selection,
  final diagnostic ABI, final runtime API, or final step-family taxonomy.

## Start state / dirty state

- Started from clean pushed `a43f1141` (`origin/main` matched `HEAD`).
- Discord task baseline was recorded before package work with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- No unrelated user changes were observed in the working tree.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl001/README.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.md`
- `samples/lean/lab-statements/obl020/README.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.md`
- `samples/lean/lab-statements/obl021/README.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- Sub-agent mapping findings from the OBL-001/020/021 guard-risk package.

## Actions taken

- Added `plan/117-g1-obl001-020-021-statement-guard-hardening.md`.
- Added sync-unit body-level guards for OBL-001:
  `RequestEvidenceSound`, `GeneratedWriteSound`,
  `AssignmentElabSoundnessPost`, and `THM001StatementDraft`.
- Added sync-unit body-level guards for OBL-020:
  `PreservesWF`, `FamilyStepPreservesWF`, and `OBL020StatementDraft`.
- Added sync-unit body-level guards for OBL-021:
  `SameElabResult`, `SameDiagnostic`, `ElabDeterministicPost`, and
  `OBL021StatementDraft`.
- Added negative guard checks against proof/final namespace drift such as
  `theorem`, `axiom`, `constant`, `sorry`, final Lean target names, final
  equality names, runtime scheduling determinism, final step API names, and
  scheduler/step-family completion wording.
- Added comment-stripping vacuity checks for guarded bodies against obvious
  weakeners such as `True \/ ...`, `... \/ True`, `False -> ...`, and trivial
  proof-shaped bodies.
- Updated explanation and README files to state that these are drift guards
  only, not proof skeletons, runtime dispatch, runtime scheduling determinism,
  final equality selection, per-step proof decomposition, or canon ledger
  movement.
- Updated `README.md`, `Documentation.md`, `plan/00-index.md`,
  `plan/74-g1-obl001-lean-statement-draft.md`,
  `plan/77-g1-obl021-lean-statement-draft.md`,
  `plan/78-g1-obl020-lean-statement-draft.md`,
  `plan/90-source-traceability.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl001/README.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.md`
- `samples/lean/lab-statements/obl020/README.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.md`
- `samples/lean/lab-statements/obl021/README.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `docs/reports/2165-g1-obl001-020-021-statement-guard-hardening.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl001_draft_body_keeps_assignment_soundness_boundary scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl020_draft_body_keeps_wf_preservation_boundary scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl021_draft_body_keeps_determinism_boundary`
  - First run intentionally failed on missing explanatory guard phrases.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl001_draft_body_keeps_assignment_soundness_boundary scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl020_draft_body_keeps_wf_preservation_boundary scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl021_draft_body_keeps_determinism_boundary`
  - Passed after explanation boundary phrases were added.
- `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `git status --short -- samples/lean/manifest.json samples/lean/clean-near-end`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- Changed-file endpoint leak scan with `rg --pcre2`.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl001_draft_body_keeps_assignment_soundness_boundary scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl020_draft_body_keeps_wf_preservation_boundary scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl021_draft_body_keeps_determinism_boundary`
  - Passed after final reviewer follow-up added comment-stripping vacuity
    checks.
- Re-ran the full validation set after final reviewer follow-up:
  - the three Lean compile commands;
  - `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`;
  - `python3 scripts/current_l2_lean_sample_sync.py`;
  - manifest / clean-near-end diff check;
  - `python3 scripts/validate_docs.py`;
  - `python3 -m unittest scripts.tests.test_validate_docs`;
  - `python3 scripts/check_source_hierarchy.py`;
  - `git diff --check`;
  - changed-file endpoint leak scan.

## Evidence / outputs / test results

- Targeted RED failed on missing explanation phrases:
  `not a proof skeleton`, `WF clauses stay behind WellFormed`, and
  `not final equality selection`.
- Targeted GREEN passed after explanation and README boundary wording was added:
  3 tests.
- Lean compile passed for OBL-001, OBL-020, and OBL-021 drafts with no output
  after final reviewer follow-up.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed:
  16 tests after final reviewer follow-up.
- `python3 scripts/current_l2_lean_sample_sync.py` printed
  `samples/lean/manifest.json`; no tracked manifest / clean-near-end diff was
  left afterward after final reviewer follow-up.
- `python3 scripts/validate_docs.py` passed and reported 1316 numbered reports
  before this report was added.
- After this report was added, `python3 scripts/validate_docs.py` passed and
  reported 1317 numbered reports. It also passed after final reviewer
  follow-up.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests
  after final reviewer follow-up.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required, 602
  present, 0 missing after final reviewer follow-up.
- `git diff --check` passed with no output after final reviewer follow-up.
- Changed-file endpoint leak scan reported no endpoint matches in changed files
  after final reviewer follow-up.
- The final reviewer follow-up targeted test passed: 3 tests.

## What changed in understanding

OBL-001, OBL-020, and OBL-021 were already useful compile-check-only statement
drafts, but their test coverage was materially weaker than the newer
OBL-024/025 draft guards. The important risk was not Lean compilation failure;
it was semantic weakening that preserved names and registration. The
appropriate package is therefore a body-level sync guard, not a statement
refinement or proof-interface expansion.

## Open questions

- OBL-001, OBL-002, OBL-020, and OBL-021 proof obligations remain OPEN.
- Future proof interface shape for OBL-020 per-step lemmas versus aggregate
  theorem remains OPEN.
- Future OBL-021 equality/equivalence relation remains OPEN.
- Final diagnostic ABI, Core IR JSON/API, runtime dispatch, scheduler
  semantics, and conformance claims remain OPEN.

## Suggested next prompt

Continue the G1 ratchet by choosing the next narrow package from `tasks.md`,
preferably one that either closes another silent-drift risk or prepares a real
proof-interface decision without moving canon ledger status.

## Plan update status

Updated. Added `plan/117-g1-obl001-020-021-statement-guard-hardening.md` and
linked it from `plan/00-index.md`, `plan/74`, `plan/77`, `plan/78`, and
`plan/90-source-traceability.md`.

## Documentation.md update status

Updated. `Documentation.md` now names OBL-001/020/021 statement guard
hardening and states that it is body-level drift checking, not proof skeleton,
completion, final equality, or runtime dispatch.

## progress.md update status

Updated. `progress.md` now records `plan/117` in the OBL-001/020/021 current
statement notes, macro phase row, Lean statement feature row, and recent log.

## tasks.md update status

Updated. `tasks.md` now records `plan/117` in current holding state while
leaving future statement refinements as candidates only if a real abstraction
gap is later found.

## samples_progress.md update status

Updated. `samples_progress.md` now records OBL-001/020/021 statement guard
hardening in current focus, Lean mechanization evidence, and recent validation
log.

## Reviewer findings and follow-up

- Mapping sub-agent found that OBL-001/020/021 had shallow registration-only
  sync checks and recommended exact body-level guards for OBL-001 request /
  postcondition links, OBL-020 WF preservation links, and OBL-021 component /
  diagnostic / exclusion links. Follow-up: implemented those guards.
- Final reviewer reported no blocking semantic findings and no claim-boundary
  drift. Medium follow-up: raw substring checks inside definition bodies could
  be satisfied by dead branches or comments, which was weaker than the package
  wording. Follow-up: added Lean comment stripping plus simple vacuity guards
  against `True \/ ...`, `... \/ True`, `False -> ...`, and trivial
  proof-shaped bodies, and narrowed the report objective to guarded-link /
  vacuous-drift hardening.
- Final reviewer noted report placeholders. Follow-up: replaced pending
  reviewer / sub-agent lines with this completed status.

## Skipped validations and reasons

- Rust runtime / JSON fixture validation was not run because this package did
  not change Rust runtime code, expected JSON, executable repair output,
  executable diagnostic output, or sample source rows.
- Oracle consult was not run because the package was a narrow guard-hardening
  task aligned with existing plan boundaries and focused sub-agent mapping.
- No canon ledger validation beyond local source-hierarchy/docs checks was run
  because canon files were not edited and no OBL status movement is claimed.

## Commit / push status

Pending at first report write.

## Sub-agent session close status

- Mapping sub-agent `019f2a9e-5067-7232-9935-1fc25acc70e6` completed and was
  closed.
- Final reviewer sub-agent `019f2aa8-fb7f-76c3-9881-26718e4d5235` completed
  and was closed.
