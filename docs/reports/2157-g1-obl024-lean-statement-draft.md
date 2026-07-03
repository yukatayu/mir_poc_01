# Report 2157 - G1 OBL-024 LAB Lean statement draft

- Date: 2026-07-04
- Author / agent: Codex
- Scope: LAB-only OBL-024 Lean statement draft and repository-memory sync
- Decision levels touched: L2 / L3 LAB statement-shape memory only

## Objective

Add a LAB-only, compile-check-only Lean statement-shape draft for OBL-024
explanation soundness, and keep repository memory / progress snapshots aligned.

The package must not edit canon, move the OBL ledger, prove OBL-024, claim
OBL-024 completion, freeze final Diagnostic ABI / JSON fields / request IDs /
branch IDs / association-key ABI / replay semantics, claim root-cause
uniqueness, alter OBL-025 repair completeness, claim conformance, or claim G1
exit.

## Scope and assumptions

- Scope is restricted to LAB statement shape and documentation synchronization.
- `mirrorea_canon/` remains the normative source.
- `specs/` and `plan/` remain LAB / repository memory unless explicitly stated
  otherwise.
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  is evidence that the statement vocabulary typechecks, not proof discharge.
- Mixed-row branch vocabulary is classification / partition evidence only; it is
  not an OBL-025 repair witness and not an independent failed premise claim.

## Start state / dirty state

- Started after Package 18 on `main` at
  `c6554541b57a5126267ce3037ea26e2b67de3409`.
- `origin/main` matched `HEAD` at package start.
- Working tree was clean before the package began.
- Discord task baseline was recorded with:
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00...03`
- `specs/09`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- `plan/85-g1-erow-carrier-precondition-hardening.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`
- `samples/lean/lab-statements/README.md`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Added a RED unit test requiring a manifest entry for
  `obl024-diagnostic-soundness-statement-draft`.
- Confirmed the RED test failed before implementation with a missing draft id.
- Added `DiagnosticSoundnessStatementDraft.lean` under
  `samples/lean/lab-statements/obl024/`.
- Added the OBL-024 lab-statement explanation and local README.
- Registered the OBL-024 statement draft in
  `scripts/current_l2_lean_sample_sync.py`.
- Ran the Lean sample sync script to update `samples/lean/manifest.json`.
- Added `plan/109-g1-obl024-lean-statement-draft.md`.
- Updated snapshot / index docs to point from the previous OBL-024 inventory to
  the new LAB statement draft.
- Consulted Oracle through the repository-local operating process and folded the
  useful advisory points into local source files after checking them against the
  repository evidence.
- Started a final read-only reviewer sub-agent before commit.
- Followed reviewer feedback by strengthening `MixedDiagnosticBranchBoundary`
  so every branch of a mixed diagnostic gap must classify some missing evidence,
  remain partition evidence, and stay non-independent from the whole failed
  premise.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `progress.md`
- `samples/lean/README.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `samples/lean/lab-statements/obl024/README.md`
- `samples/lean/manifest.json`
- `samples_progress.md`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `tasks.md`
- `docs/reports/2157-g1-obl024-lean-statement-draft.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_statement_drafts_include_obl024_draft`
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2157.json`
- `python3 -m unittest scripts.tests.test_surface_mir_samples`
- `cargo fmt --check`
- `git diff --check`
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | (.expected.lab_diagnostic_details[]?.suggested_repair // [])[]] | length > 0), elab07_repair_shapes: ([.results[] | select(.sample_id=="ELAB-07") | (.expected.lab_diagnostic_details[]?.suggested_repair // [])[]?.repair_shape] | unique), elab07_repair_count: ([.results[] | select(.sample_id=="ELAB-07") | (.expected.lab_diagnostic_details[]?.suggested_repair // [])[]] | length), elab10_repair_count: ([.results[] | select(.sample_id=="ELAB-10") | (.expected.lab_diagnostic_details[]?.suggested_repair // [])[]] | length), elab13_repair_count: ([.results[] | select(.sample_id=="ELAB-13") | (.expected.lab_diagnostic_details[]?.suggested_repair // [])[]] | length)}' /tmp/mirrorea-surface-check-all-2157.json`
- endpoint scan over changed files with split endpoint variables:
  `ENDPOINT_SEGMENT=...; ENDPOINT_PATTERN="discord[.]com/api|api/${ENDPOINT_SEGMENT}"; ... rg -n --pcre2 "$ENDPOINT_PATTERN" ...`

## Evidence / outputs / test results

- Initial RED unit test failed before implementation with missing
  `obl024-diagnostic-soundness-statement-draft`.
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  passed.
- `python3 scripts/current_l2_lean_sample_sync.py` updated
  `samples/lean/manifest.json`; the OBL-024 manifest entry verifies with
  return code `0` and status `lab-compile-check-only`.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed:
  10 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests.
- `python3 scripts/validate_docs.py` passed after report addition and reported
  1309 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required 602, present 602,
  missing 0.
- Surface check JSON summary:
  - `sample_count`: 52
  - `failed_count`: 0
  - `validation_error_count`: 0
  - `elab04_has_repair`: false
  - `elab07_repair_shapes`: `["set_insertion"]`
  - `elab07_repair_count`: 1
  - `elab10_repair_count`: 1
  - `elab13_repair_count`: 1
- `python3 -m unittest scripts.tests.test_surface_mir_samples` passed: 45 tests.
- `cargo fmt --check` passed.
- `git diff --check` passed.
- Endpoint scan over changed files reported no endpoint matches in changed
  files.

## What changed in understanding

- `plan/81` was sufficient as a relation inventory; `plan/109` now provides the
  first Lean-checked LAB vocabulary for the same OBL-024 shape.
- A sound OBL-024 statement draft needs an explicit abstract association layer
  between a rejection and an emitted diagnostic; relying on list position or
  diagnostic order would be premature.
- Mixed E-ROW rows need a branch-classification boundary over every diagnostic
  branch, but those branches must not become independent failed-premise witnesses
  and must not import OBL-025 repair vocabulary.
- The current safe result is a compile-check-only statement shape. It improves
  proof-boundary clarity but does not move proof status.

## Open questions

- Should the first later proof target quantify over every diagnostic family, or
  over a narrower E-ROW fragment plus later generalization?
- What is the final carrier shape for diagnostic field projection?
- Should replay be whole-judgment replay, rule-local replay witness, or both?
- How should diagnostic equality / ordering interact with OBL-021 elaboration
  determinism?
- What is the final multi-span declaration-site / use-site blame model?
- How should mixed-row associated diagnostics avoid double-counting one
  generated request?

## Suggested next prompt

Continue autonomously with the next G1 package: harden executable OBL-024
diagnostic association / projection evidence without freezing final Diagnostic
JSON or replay ABI.

## Plan update status

Updated.

- Added `plan/109-g1-obl024-lean-statement-draft.md`.
- Updated `plan/00-index.md`.
- Updated `plan/81-g1-obl024-statement-shape-inventory.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

Updated to list OBL-024 as an active LAB Lean statement draft and to separate
the OBL-024 draft from proof discharge / canon movement.

## progress.md update status

Updated with the OBL-024 statement-draft note, macro-phase row adjustment, LAB
Lean draft row adjustment, and a timestamped recent log entry.

## tasks.md update status

Updated to remove the OBL-024 Lean statement draft from future candidate work
and to record `plan/109` as current LAB memory.

## samples_progress.md update status

Updated to include the OBL-024 statement draft in Lean mechanization evidence
and the current repo-local focus / validation log.

## Reviewer findings and follow-up

- Medium reviewer finding: `MixedDiagnosticBranchBoundary` constrained only
  branches classifying the same existential `missing` value, leaving other
  mixed diagnostic branches unconstrained.
  - Follow-up: strengthened the Lean helper so every branch of the diagnostic
    gap must classify some missing evidence, remain in the diagnostic partition,
    and stay non-independent from the top-level failed premise.
- Low reviewer finding: report closeout was not final because front matter and
  pending reviewer / sub-agent statuses remained.
  - Follow-up: added report front matter and replaced pending reviewer status
    with this closeout note. Sub-agent close status is updated below.

## Skipped validations and reasons

- No full public product release validation was run; this package is a LAB Lean
  statement-shape draft and documentation synchronization package.
- No canon proof attempt was run; this package intentionally does not prove
  OBL-024 or move the proof-status ledger.
- No Oracle follow-up was requested after the first advisory result because the
  local artifact could be aligned with existing repo evidence without a further
  theoretical blocker.

## Commit / push status

- Content/report commit:
  `18fac09bcc61acba55dfa43572e4f0427f6b4167`
  (`Add OBL-024 diagnostic soundness draft`)
- Pushed to `origin/main`; immediately after push, `HEAD` and `origin/main`
  matched at `18fac09bcc61acba55dfa43572e4f0427f6b4167`.
- This report status update is pending its own follow-up commit at the time of
  this line.

## Sub-agent session close status

- Oracle runner sub-agent: completed and closed.
- Mapper sub-agent: completed and closed.
- Final reviewer sub-agent: completed and closed.
