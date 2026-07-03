# Report 2158 - G1 OBL-024 executable projection carrier

Date: 2026-07-04 07:57 JST
Author: Codex
Scope: Package 20 / G1 LAB OBL-024 executable projection carrier for Surface E-ROW diagnostics
Decision levels: L2/L3 LAB evidence only; no canon status movement

## Objective

Add an executable, non-final LAB projection carrier for the current OBL-024
diagnostic-soundness pressure case in Surface-to-Core elaboration E-ROW
diagnostics. The carrier should make the current diagnostic report local rule,
failed premise, bindings, association key, and trace-local replay anchor explicit
without claiming a final Diagnostic ABI, replay ABI, OBL-024 proof, conformance,
or G1 exit.

## Scope and assumptions

- Scope is limited to current LAB `lab_diagnostic_details` emitted for
  underdeclared generated failure rows.
- Current executable cases are `ELAB-04/07/10/13..16`.
- The projection is an implementation-side evidence carrier, not a normative
  `mirrorea_canon/` update.
- OBL-024 remains separate from OBL-025. This package does not widen repair
  output and does not add an `ELAB-04` repair payload.
- The serialized helper-local association key is separate from the internal
  span-based duplicate-repair suppression key, which remains non-serialized and
  module-private in Rust.
- Working assumption: the current report-local replay anchor is useful evidence
  for the next statement/refinement step, but it is not final replay semantics.

## Start state / dirty state

- Started from pushed clean `main` at `f1e08ad4`.
- `origin/main` also pointed to `f1e08ad4`.
- Discord task baseline was recorded with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  before Package 20 file edits.
- Worktree became dirty only through this package's implementation, expected
  sample output updates, and documentation/report updates.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-mir-core.md`
- `specs/01-runtime-places.md`
- `specs/02-communication-and-effects.md`
- `specs/03-elaboration.md`
- `specs/09-architecture.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/07-examples-and-format.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- `plan/85-g1-erow-carrier-precondition-hardening.md`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- Read-only sub-agent mapper notes from Hume on the current E-ROW diagnostic
  ownership and overclaim boundaries.

## Actions taken

1. Added a RED Rust test assertion requiring
   `diagnostic_soundness_projection` inside the `ELAB-04` LAB diagnostic detail.
2. Added Rust carrier types and emission logic:
   - `SurfaceLabDiagnosticSoundnessProjection`
   - `SurfaceLabDiagnosticReportedBindings`
   - `SurfaceLabDiagnosticTraceLocalReplay`
   - `erow_diagnostic_soundness_projection(...)`
3. Kept the internal span-based `failure_row_context.association_key`
   non-serialized and module-private with `serde(skip)`, preserving
   duplicate-repair suppression behavior.
4. Emitted a helper-local serialized `lab_association_key` with
   `target_ref|request=req-*` shape for current LAB projection evidence.
5. Updated expected elaboration JSON for `ELAB-04/07/10/13..16`.
6. Added Python sample assertions that the projection fields match the enclosing
   LAB diagnostic detail and current expected cases.
7. Added `plan/110-g1-obl024-executable-projection-carrier.md`.
8. Synchronized `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
   `samples_progress.md`, `scripts/README.md`, `plan/00-index.md`,
   `plan/81`, `plan/90`, `plan/109`, and OBL-024 LAB statement notes.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/lean/lab-statements/obl024/README.md`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `scripts/README.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/110-g1-obl024-executable-projection-carrier.md`
- `docs/reports/2158-g1-obl024-executable-projection-carrier.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture
cargo fmt
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_reports_repair_payload scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_erow_suggested_repair_payloads_are_not_placeholders scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2200.json
cargo fmt --check
python3 scripts/validate_docs.py
jq '{sample_count, failed_count: (.failed | length), validation_error_count: (.validation_errors | length), elab04_has_repair: ((.results[] | select(.sample_id == "ELAB-04") | .actual.lab_diagnostic_details[0].suggested_repair // []) | length > 0), projection_count: ([.results[] | .actual.lab_diagnostic_details[]? | select(.diagnostic_soundness_projection != null)] | length), elab07_repair_shapes: ([.results[] | select(.sample_id == "ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[]?.repair_shape] | unique)}' /tmp/mirrorea-surface-check-all-2200.json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
git diff --check
bash -lc 'ENDPOINT_SEGMENT=webhooks; ENDPOINT_PATTERN="discord[.]com/api|api/${ENDPOINT_SEGMENT}"; mapfile -t files < <(git ls-files --modified --others --exclude-standard); if ((${#files[@]} == 0)); then printf "no changed files\n"; exit 0; fi; if rg -n --pcre2 "$ENDPOINT_PATTERN" "${files[@]}"; then exit 2; else rc=$?; if [ "$rc" -eq 1 ]; then printf "no endpoint matches in changed files\n"; exit 0; fi; exit "$rc"; fi'
```

Final validation commands after documentation sync passed.

## Evidence / outputs / test results

- RED Rust run failed as expected because the LAB detail did not yet emit
  `diagnostic_soundness_projection`.
- Focused GREEN Rust run passed after adding the carrier.
- Focused Python sample tests passed: 5 tests.
- Full Rust elaboration test passed after final documentation sync: 32 tests.
- Full Python sample test passed after final documentation sync: 45 tests.
- Surface sample check-all passed after final documentation sync with:

```json
{
  "sample_count": 52,
  "failed_count": 0,
  "validation_error_count": 0,
  "elab04_has_repair": false,
  "projection_count": 7,
  "elab07_repair_shapes": ["set_insertion"]
}
```

- `cargo fmt --check` passed.
- `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete. Found 1310 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required paths, 602
  present, 0 missing.
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  passed.
- `python3 scripts/current_l2_lean_sample_sync.py` passed and reported
  `samples/lean/manifest.json`.
- `git diff --check` passed.
- Endpoint scan over changed files passed: no endpoint matches in changed files.

## What changed in understanding

The current OBL-024 pressure case can be ratcheted from a purely abstract Lean
statement-shape draft into executable LAB evidence without freezing the final
Diagnostic ABI. The important split is between:

- serialized helper-local projection association, used for report readability;
- internal span-based association, used only for duplicate-repair suppression
  and kept out of serialized LAB JSON;
- future formal association / replay semantics, still unresolved.

This keeps the current E-ROW evidence useful for the next formalization step
while preserving the canon boundary and OBL-025 separation.

## Open questions

- What final Diagnostic ABI, if any, should preserve this projection shape?
- How should request id, association key, and branch id semantics be defined for
  proof-level replay rather than helper-local reporting?
- Should `ELAB-04` mixed visibility branches eventually receive branch-local
  repair noncoverage witnesses, or remain diagnostic-only until OBL-025 is
  stronger?
- When should OBL-024 move from LAB statement/projection evidence into a canon
  proof obligation refinement?

## Suggested next prompt

Continue with the next G1 diagnostic-proof ratchet package: either refine the
OBL-024 association/replay statement around the executable projection carrier,
or run a narrow executable guard pass ensuring no non-final projection fields are
mistaken for final Diagnostic ABI or repair coverage.

## Plan update status

Updated. Added `plan/110-g1-obl024-executable-projection-carrier.md` and linked
it from `plan/00-index.md`, `plan/90-source-traceability.md`, `plan/81`, and
`plan/109`.

## Documentation.md update status

Updated. `Documentation.md` now records the current LAB-only executable
projection carrier and its non-final ABI boundary.

## progress.md update status

Updated. `progress.md` now records the Package 20 carrier in the current G1
notes, feature maturity row, and recent log.

## tasks.md update status

Updated. `tasks.md` now records that `plan/110` adds executable projection
evidence while OBL-024 proof/final ABI remain open.

## samples_progress.md update status

Updated. `samples_progress.md` now records the `ELAB-04/07/10/13..16`
projection evidence and current validation hook.

## Reviewer findings and follow-up

Reviewed by Beauvoir. Findings and follow-up:

- Medium: report wording said the internal association key was "private to
  Rust" while the field was `pub` but `serde(skip)`. Follow-up: made
  `association_key` and the adjacent skipped `associated_request_count`
  module-private, and clarified report wording as non-serialized /
  module-private.
- Low: report listed stale consulted paths for canon diagnostics and older
  plan file names. Follow-up: corrected consulted paths.
- Low: reviewer / sub-agent status sections were still pending. Follow-up:
  resolved the sections before commit.

Reviewer found no semantic/code regression in the E-ROW carrier path.
`ELAB-04` remains no-repair, `ELAB-07` still has only the exact
`set_insertion` payload, and serialized `lab_association_key` remains distinct
from the skipped span-based implementation key.

## Skipped validations and reasons

- No canon proof validation was run because this package intentionally does not
  edit canon or discharge OBL-024.
- No product/runtime release check was run because this package is scoped to
  Surface elaboration diagnostics and sample expected output.
- No Oracle consult was started; the package is a narrow executable follow-up to
  already reviewed local statement/projection work, and a read-only sub-agent
  mapper plus focused local reviewer is sufficient unless reviewer evidence
  surfaces a higher-level design conflict.

## Commit / push status

Implemented, committed, and pushed.

- Package commit:
  `80816365e5f4e5a70035cf75ae167bd9b486e5fc`
  (`Add OBL-024 executable projection carrier`)
- Push status: `main` was pushed to `origin/main`, and `HEAD` matched
  `origin/main` immediately after the package commit.
- This commit-status note is recorded in the follow-up report-status commit.

## Sub-agent session close status

Hume provided a read-only implementation map and overclaim-risk summary.
Beauvoir provided a focused read-only closeout review. Both sessions were
closed before final package commit.
