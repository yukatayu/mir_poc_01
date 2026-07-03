# Report 2133 - G1 E-ROW Carrier-Only Diagnostic Detail Prototype

- Date: 2026-07-03 22:37 JST
- Author / agent: Codex
- Scope: LAB-only E-ROW diagnostic detail carrier implementation
- Decision levels touched: L1/L2 canon consulted, no canon edit; LAB code, sample evidence, tests, and repository memory updated

## Objective

Implement a carrier-only LAB E-ROW diagnostic detail for Surface-to-Core
elaboration failure-row containment, preserving legacy
`generated_failure_not_declared` diagnostics and helper `diagnostic_codes`.

The prototype must not emit `suggested_repair[]`, freeze a diagnostic or repair
ABI, prove OBL-024/025, claim explanation soundness/completeness, claim
conformance, claim G1 exit, or edit canon.

## Scope and assumptions

The normative source remains `mirrorea_canon/`. This package is a LAB
implementation/evidence step following `plan/83`.

Working assumptions:

- E-ROW-001 covers general or mixed generated failure omissions.
- E-ROW-002 is limited to the clean case where the only missing generated
  failure is `VisibilityDenied`.
- `ELAB-04` is mixed E-ROW-shaped evidence, `ELAB-07` is clean E-ROW-001
  evidence, and `ELAB-10` is clean E-ROW-002 evidence.
- Repair-bearing rows are out of scope.

## Start state / dirty state

The package started from clean `main...origin/main` at commit
`f6b664e4 Add G1 EROW repair payload inventory`. A Discord task baseline had
already been recorded with `discord_notify.py begin --cwd .` before file
edits. Red tests were added before production code.

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
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/90-source-traceability.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/main/src/undeclared-generated-failure-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/main/src/write-failure-row-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/main/src/visibility-failure-row-negative.mir`

## Actions taken

- Added failing Rust and Python tests for LAB diagnostic details.
- Corrected the initial test expectation split before implementation:
  `ELAB-04` and `ELAB-07` remain E-ROW-001; `ELAB-10` is E-ROW-002.
- Added `SurfaceLabDiagnosticDetail` and report-level
  `lab_diagnostic_details` in the Surface-to-Core elaboration report.
- Computed missing generated failures from required and declared failure rows.
- Added E-ROW-001/E-ROW-002 selection:
  `VisibilityDenied` alone maps to E-ROW-002; all other missing sets map to
  E-ROW-001.
- Kept the legacy diagnostic code `generated_failure_not_declared`.
- Ensured the `surface_to_core_elaborate` example serializes the LAB carrier.
- Updated the Surface sample helper projection to include
  `lab_diagnostic_details` only when non-empty.
- Added expected JSON carrier evidence for `ELAB-04`, `ELAB-07`, and
  `ELAB-10`.
- Added `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`.
- Updated source hierarchy / docs validators for `plan/84`.
- Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `scripts/README.md`, `plan/00-index.md`,
  `plan/90-source-traceability.md`, and the Surface elaboration README.
- Used one read-only sub-agent to check the E-ROW split, implementation blast
  radius, expected JSON impact, and repo-policy pitfalls.

## Files changed

- `README.md`
- `Documentation.md`
- `crates/mir-semantics/examples/surface_to_core_elaborate.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `plan/00-index.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/surface_mir_samples.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2133-g1-erow-carrier-only-diagnostic-detail-prototype.md`

No `mirrorea_canon/` file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
cargo test -p mir-semantics --test surface_to_core_elaboration failure_row -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic
nl -ba crates/mir-semantics/tests/surface_to_core_elaboration.rs | sed -n '340,490p'
rg -n "VisibilityDenied|visibility_failure|failure_row|generated_failure_not_declared|lab_diagnostic_details|required_failures" crates/mir-semantics/src/surface_to_core_elaboration.rs crates/mir-semantics/tests/surface_to_core_elaboration.rs scripts/tests/test_surface_mir_samples.py scripts/surface_mir_samples.py samples/full-system-v1-surface -g '*.rs' -g '*.py' -g '*.mir' -g '*.json' -g '*.md'
nl -ba scripts/tests/test_surface_mir_samples.py | sed -n '240,420p'
nl -ba scripts/surface_mir_samples.py | sed -n '560,735p'
for p in samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/main/src/*.mir samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/main/src/*.mir samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/main/src/*.mir; do printf '\n== %s ==\n' "$p"; nl -ba "$p"; done
cargo test -p mir-semantics --test surface_to_core_elaboration failure_row -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic
python3 scripts/surface_mir_samples.py run ELAB-04 --format json
python3 scripts/surface_mir_samples.py run ELAB-07 --format json
python3 scripts/surface_mir_samples.py run ELAB-10 --format json
python3 scripts/surface_mir_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
cargo fmt --check
cargo fmt
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
```

## Evidence / outputs / test results

TDD red evidence before production code:

```text
cargo test -p mir-semantics --test surface_to_core_elaboration failure_row -- --nocapture
running 3 tests
0 passed; 3 failed
failure reason: LAB diagnostic details are emitted

python3 -m unittest ...ELAB-04 ...ELAB-07 ...ELAB-10
FAILED (errors=3)
failure reason: KeyError: 'lab_diagnostic_details'
```

Post-implementation targeted evidence:

```text
cargo test -p mir-semantics --test surface_to_core_elaboration failure_row -- --nocapture
3 passed; 0 failed

python3 -m unittest ...ELAB-04 ...ELAB-07 ...ELAB-10
Ran 3 tests
OK
```

Fresh final validation after formatting:

```text
cargo fmt --check
exit 0

cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
16 passed; 0 failed

python3 -m unittest scripts.tests.test_surface_mir_samples
Ran 42 tests
OK

python3 scripts/surface_mir_samples.py check-all --format json
sample_count: 48
passed: 48
failed: []
workflow_ready: false

python3 scripts/check_source_hierarchy.py
required: 580
present: 580
missing: 0

python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1285 numbered report(s).

python3 -m unittest scripts.tests.test_validate_docs
Ran 20 tests
OK

git diff --check
exit 0
```

`cargo fmt --check` initially found only a formatting diff in the new Rust test;
`cargo fmt` was run, and the fresh `cargo fmt --check` passed.

## What changed in understanding

The clean E-ROW-002 case is narrower than the initial red test shape implied.
Only missing `VisibilityDenied` alone should be classified as E-ROW-002.
Mixed missing sets, including a visible read that declares only
`MissingCapability`, remain E-ROW-001.

The Python helper did not initially see the Rust carrier because
`surface_to_core_elaborate` manually builds its JSON payload. The root cause was
the example boundary dropping the new field; adding it there made the helper
projection see the LAB carrier.

## Open questions

- Whether a later repair-bearing prototype should first add request id,
  target-row context, and declared/required failure projections to the LAB
  detail before emitting `suggested_repair[]`.
- How to recover declaration-site spans for the relevant `when ... fails` row.
- Whether adding a set of missing failures is one edit or multiple edits.
- Whether E-ROW-002 should eventually include alternatives besides adding
  `VisibilityDenied` to the failure row.
- When OBL-024/025 Lean statement drafts become stable enough to avoid freezing
  the wrong diagnostic or repair interface.

## Suggested next prompt

自走で E-ROW repair precondition hardening を進めてください。まだ
`suggested_repair[]` は出さず、target row / request id / required failures /
declared failures / missing failures / local premise の non-final carrier を
整え、placeholder repair を拒否できる状態を作ってください。final ABI、
OBL-024/025 proof、conformance、G1 exit は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface Mir line now mentions the E-ROW carrier-only diagnostic detail
prototype as LAB evidence, not diagnostic/repair ABI freeze, OBL-024/025
discharge, proof discharge, or G1 exit.

## progress.md update status

更新済み: Added the current E-ROW carrier-only diagnostic detail note, updated
next-gap wording, updated Surface elaboration status, and added a recent log
entry.

## tasks.md update status

更新済み: Moved carrier-only E-ROW detail from next candidate to implemented
holding state, and narrowed the next front candidate to repair precondition
hardening without repair rows.

## samples_progress.md update status

更新済み: Updated the Surface Mir dashboard row and elaboration root row to note
LAB-only `lab_diagnostic_details` evidence for `ELAB-04/07/10`, and added a
recent validation log entry.

## Reviewer findings and follow-up

Read-only sub-agent review found:

- `ELAB-04` is mixed E-ROW-shaped evidence and should remain E-ROW-001.
- `ELAB-07` is clean non-visibility E-ROW-001 evidence.
- `ELAB-10` is clean E-ROW-002 evidence.
- The implementation blast radius is limited to the elaboration report/context,
  `push_remote_request`, missing-failure/detail helpers, the example JSON
  boundary, the Surface helper projection, expected JSON, and tests.
- Expected JSON updates for `ELAB-04`, `ELAB-07`, and `ELAB-10` are useful
  committed evidence even though the helper comparison would not require them.
- Repo-policy pitfalls are to keep this LAB-only, preserve legacy diagnostic
  output, keep `suggested_repair` absent, and avoid widening E-ROW-002.

Follow-up from review was integrated into tests, expected JSON, `plan/84`,
`progress.md`, `tasks.md`, and `samples_progress.md`.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets`: skipped because this
  package is narrowly scoped to `mir-semantics` Surface elaboration plus the
  Surface sample helper; the targeted Rust test and Surface helper suite cover
  the changed executable path.
- Release checks (`surface_mir_release_check.py`, Product Alpha release check,
  operational product helper, minimal alpha-1 pattern verifier): skipped
  because this package does not change release orchestration, product alpha
  compatibility, operational product samples, or minimal alpha-1 pattern rows.
- Lean validation: skipped because no Lean files or Lean manifest changed.
- Canon validators: skipped because `mirrorea_canon/` was not edited.
- Oracle advisory review: skipped because the immediate issue was a bounded
  implementation/evidence step, and a read-only sub-agent review covered the
  semantic split and policy risks.

## Commit / push status

Pending at report write. This package should be committed with
`git commit --no-gpg-sign` and pushed after report validation.

## Sub-agent session close status

Sub-agent `019f282d-db0b-7762-abc9-ce7ad1415480` completed read-only review and
was closed before this report was written.
