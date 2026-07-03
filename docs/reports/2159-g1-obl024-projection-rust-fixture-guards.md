# Report 2159 - G1 OBL-024 projection Rust fixture guards

Date: 2026-07-04 08:12 JST
Author: Codex
Scope: Package 21 / test-only Rust fixture guard hardening for OBL-024 projection carrier
Decision levels: L2/L3 LAB evidence only; no canon status movement

## Objective

Harden Rust-side test coverage for the LAB-only OBL-024
`diagnostic_soundness_projection` carrier added in `plan/110`, especially around
the real Surface E-ROW sample fixtures that carry projection evidence.

## Scope and assumptions

- Scope is test-only plus repository-memory/status documentation.
- Current projection-bearing fixtures remain `ELAB-04/07/10/13..16`.
- Production emission logic, expected JSON, Python helper behavior, Lean
  statement files, and repair semantics are intentionally unchanged.
- The package does not claim final Diagnostic ABI, replay ABI, OBL-024 proof,
  conformance, or G1 exit.
- OBL-024 diagnostic projection remains separate from OBL-025 repair
  completeness.

## Start state / dirty state

- Started from pushed clean `main` at `df42a53a`.
- `origin/main` also pointed to `df42a53a`.
- Discord task baseline was recorded with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  before Package 21 file edits.
- Worktree became dirty only through this package's Rust test hardening,
  repository-memory/status docs, and this report.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/110-g1-obl024-executable-projection-carrier.md`
- `docs/reports/2158-g1-obl024-executable-projection-carrier.md`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/README.md`
- Read-only sub-agent mapper notes from Erdos on current fixture parity gaps.

## Actions taken

1. Strengthened `assert_obl024_diagnostic_soundness_projection` so Rust tests now
   check:
   - skipped internal association fields are not serialized;
   - reported bindings match the enclosing request/failure-row contexts;
   - trace-local replay fields match the local failed premise and missing
     evidence;
   - `projection_non_final`, `lab_non_final`, and `replay_non_final` remain true.
2. Added `rejected_lab_details_for_sample(...)` for direct Rust fixture reads.
3. Added fixture-backed Rust guards for:
   - `ELAB-04` projection present and no `suggested_repair`;
   - `ELAB-07` projection present and exact `set_insertion` repair still present;
   - `ELAB-10` projection present and singleton `VisibilityDenied` row-addition
     repair still present.
4. Extended the existing `ELAB-13..16` fixture loop to use the strengthened
   projection guard.
5. Added `plan/111-g1-obl024-projection-rust-fixture-guards.md`.
6. Synchronized `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
   `samples_progress.md`, `plan/00-index.md`, and
   `plan/90-source-traceability.md`.

## Files changed

- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/111-g1-obl024-projection-rust-fixture-guards.md`
- `docs/reports/2159-g1-obl024-projection-rust-fixture-guards.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_remote_request_when_failure_row_is_underdeclared rejects_generated_write_request_when_failure_row_is_underdeclared emits_non_visibility_singleton_erow001_repair_payload sample_fixtures_cover_each_non_visibility_singleton_with_repair_payload rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_remote_request_when_failure_row_is_underdeclared -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration emits_non_visibility_singleton_erow001_repair_payload -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration sample_fixtures_cover_each_non_visibility_singleton_with_repair_payload -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab04_sample_fixture_carries_obl024_projection_without_repair -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_sample_fixture_carries_obl024_projection_with_exact_set_repair -- --exact --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab10_sample_fixture_carries_obl024_projection_with_visibility_repair -- --exact --nocapture
cargo fmt --check
cargo fmt
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2159.json
jq '{sample_count, failed_count: (.failed | length), validation_error_count: (.validation_errors | length), elab04_has_repair: ((.results[] | select(.sample_id == "ELAB-04") | .actual.lab_diagnostic_details[0].suggested_repair // []) | length > 0), projection_count: ([.results[] | .actual.lab_diagnostic_details[]? | select(.diagnostic_soundness_projection != null)] | length), elab07_repair_shapes: ([.results[] | select(.sample_id == "ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[]?.repair_shape] | unique)}' /tmp/mirrorea-surface-check-all-2159.json
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
git diff --check
bash -lc 'ENDPOINT_SEGMENT=webhooks; ENDPOINT_PATTERN="discord[.]com/api|api/${ENDPOINT_SEGMENT}"; mapfile -t files < <(git ls-files --modified --others --exclude-standard); if ((${#files[@]} == 0)); then printf "no changed files\n"; exit 0; fi; if rg -n --pcre2 "$ENDPOINT_PATTERN" "${files[@]}"; then exit 2; else rc=$?; if [ "$rc" -eq 1 ]; then printf "no endpoint matches in changed files\n"; exit 0; fi; exit "$rc"; fi'
```

Final docs/source/leak validation passed.

## Evidence / outputs / test results

- The first attempted multi-filter `cargo test` command failed because
  `cargo test` accepts one test-name filter in that position. The focused tests
  were rerun individually.
- Focused Rust tests for the existing projection-bearing rows passed
  individually.
- New fixture-backed Rust tests for `ELAB-04`, `ELAB-07`, and `ELAB-10` passed.
- `cargo fmt --check` initially found one formatting-only diff; `cargo fmt` was
  run, then `cargo fmt --check` passed.
- Full Rust elaboration test passed: 35 tests.
- Full Python sample test passed: 45 tests.
- Surface sample check-all passed with:

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

- `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete. Found 1311 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required paths, 602
  present, 0 missing.
- `git diff --check` passed.
- Endpoint scan over changed files passed: no endpoint matches in changed files.

## What changed in understanding

`plan/110` already centralized projection emission, so no production-source gap
was found. The remaining weakness was fixture parity: Rust had inline-source
projection guards for `ELAB-04/07/10`, while Python guarded the real sample
fixtures. Package 21 closes that Rust-side fixture parity gap without changing
serialized output.

## Open questions

- What final Diagnostic / replay ABI, if any, should preserve this projection
  shape?
- When OBL-024 proof work resumes, how should these fixture guards map to a
  proof-level replay witness rather than report-local replay evidence?
- Should future `ELAB-04` mixed branch modeling remain diagnostic-only, or gain
  branch-local repair noncoverage witnesses under OBL-025?

## Suggested next prompt

Continue with a narrow OBL-024 replay-vocabulary planning package, or switch to
the next G1 proof-obligation ratchet while keeping this projection guard as
LAB-only executable evidence.

## Plan update status

Updated. Added `plan/111-g1-obl024-projection-rust-fixture-guards.md` and linked
it from `plan/00-index.md` and `plan/90-source-traceability.md`.

## Documentation.md update status

Updated. `Documentation.md` now mentions the OBL-024 projection Rust fixture
guard hardening as LAB evidence and keeps the non-claim boundary.

## progress.md update status

Updated. `progress.md` now records the Package 21 guard in current G1 notes,
feature maturity row, and recent log.

## tasks.md update status

Updated. `tasks.md` now records that `plan/111` is test-only Rust fixture guard
hardening, not production behavior or proof status movement.

## samples_progress.md update status

Updated. `samples_progress.md` now records the projection Rust fixture guards in
the Surface line and recent validation log.

## Reviewer findings and follow-up

Reviewed by Hypatia. Findings and follow-up:

- Low: report evidence mentioned `python3 scripts/validate_docs.py`, but the
  command block omitted the direct command. Follow-up: added it to Commands run.
- Low: `plan/90` traceability for `plan/111` listed `ELAB-04/07/10` sources but
  not `ELAB-13..16`, despite the package covering them through the existing
  fixture loop. Follow-up: added the singleton fixture source paths to
  traceability.

Hypatia found no semantic issue in the Rust change itself and confirmed that
the docs keep non-claims around final ABI, OBL-024 proof/discharge, conformance,
G1 exit, `ELAB-04` repair output, and repair widening. Residual risk: full
repair payload exactness for `ELAB-07` / `ELAB-10` still relies on existing
inline Rust checks and sample expected-JSON validation, while the new fixture
tests focus on projection presence/consistency and basic repair shape.

## Skipped validations and reasons

- No production release check was run because this package is test-only and
  scoped to Surface elaboration fixture guards.
- No canon proof validation was run because the package does not edit canon or
  discharge OBL-024.
- No Oracle consult was started; the package is narrow and was covered by local
  evidence plus a read-only sub-agent map.

## Commit / push status

Implemented, committed, and pushed.

- Package commit:
  `84dba6951b4744644c702c391d5e0dc140435831`
  (`Harden OBL-024 projection fixture guards`)
- Push status: `main` was pushed to `origin/main`, and `HEAD` matched
  `origin/main` immediately after the package commit.
- This commit-status note is recorded in the follow-up report-status commit.

## Sub-agent session close status

Erdos provided a read-only fixture parity map. Hypatia provided a focused
read-only closeout review. Both sessions were closed before final package
commit.
