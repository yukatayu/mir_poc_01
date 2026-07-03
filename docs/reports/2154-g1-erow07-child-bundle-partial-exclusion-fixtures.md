# Report 2154 - G1 ELAB-07 child / bundle / partial exclusion fixtures

- Date: 2026-07-04 06:07 JST
- Author / agent: Codex
- Scope: Rust-only characterization guard for the current exact `ELAB-07`
  `set_insertion` repair payload.
- Decision levels touched: `L3` LAB evidence only.

## Objective

Close the current executable fixture gap for `ELAB-07` child singleton
alternatives, conjunctive bundle fields, partial guidance, and textual-only
guidance by asserting that the exact current `ELAB-07` payload remains one
complete top-level `set_insertion` item.

## Scope and assumptions

The scope is limited to test-only Surface-to-Core elaboration LAB evidence.
Production repair emission logic, expected JSON, and sample row count should
remain unchanged.

Working assumptions:

- canon in `mirrorea_canon/` remains normative;
- `plan/96` bundle / partial vocabulary is candidate LAB vocabulary only;
- `plan/101` N13/N14/N15 can be closed for the current executable payload by
  shape assertions, because current Surface syntax has no child-repair,
  bundle, partial-guidance, or textual-guidance syntax;
- future explicit bundle semantics or partial-guidance output may be designed
  later, but should require an intentional package and docs update.

This package does not claim general set-insertion support, bundle semantics,
partial-guidance output, repair ranking, visibility ranking, multi-edit
support, final Diagnostic / repair ABI, OBL-024/025 proof, conformance, canon
movement, or G1 exit.

## Start state / dirty state

Package start:

- `HEAD = origin/main = 026ba8ce91dec8c19bd5a6a9124582fd43ea335c`
- notifier task baseline was recorded before package work;
- working tree was clean.

## Documents consulted

- `AGENTS.md`
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
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `mirrorea_canon/plan/00-gates.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/README.md`

## Actions taken

- Added `assert_complete_set_insertion_not_bundle_or_partial` in the Rust
  elaboration test file.
- Added
  `elab07_set_insertion_is_not_child_bundle_or_partial_guidance`.
- Reused the same helper in the exact positive `ELAB-07` test.
- Added `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`.
- Updated `plan/101`, `plan/102`, `plan/00-index.md`, and
  `plan/90-source-traceability.md`.
- Updated `README.md`, `Documentation.md`,
  `docs/research_abstract/surface_mir_alpha_01.md`, `progress.md`,
  `tasks.md`, `samples_progress.md`, and the Surface elaboration sample
  README.

## Files changed

- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `docs/reports/2154-g1-erow07-child-bundle-partial-exclusion-fixtures.md`

## Commands run

Commands run:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M %Z'
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_child_bundle_or_partial_guidance -- --nocapture
cargo fmt --check
cargo fmt
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_child_bundle_or_partial_guidance -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test --workspace
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2154.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2154.json
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
(git diff --name-only; git ls-files --others --exclude-standard) | sort -u | xargs -r rg -n --pcre2 '<endpoint-form pattern>' || true
```

The endpoint-form scan above records the redacted command shape. The local
command used the real endpoint-form regular expression without writing it into
this report.

## Evidence / outputs / test results

Evidence:

- Focused Rust test passed:
  `elab07_set_insertion_is_not_child_bundle_or_partial_guidance` 1 passed / 0
  failed.
- Sub-agent mapping found that child / bundle / partial / textual guidance
  cannot be produced from current Surface syntax and can only appear through
  Rust-side repair emission or future payload-field widening.
- First `cargo fmt --check` detected a formatting diff in the new helper;
  `cargo fmt` was run, and the subsequent `cargo fmt --check` exited 0.
- Fresh focused new guard test:
  `elab07_set_insertion_is_not_child_bundle_or_partial_guidance` 1 passed / 0
  failed.
- Fresh existing exact positive `ELAB-07` test:
  `rejects_generated_write_request_when_failure_row_is_underdeclared` 1 passed
  / 0 failed.
- Fresh `surface_to_core_elaboration` full test: 32 passed / 0 failed. The
  expected `should_panic` test printed its panic message and was reported as
  `ok`.
- `cargo test --workspace` exited 0.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 tests OK.
- Surface helper `check-all` JSON summary:
  `sample_count = 52`, `failed_count = 0`, `validation_error_count = 0`,
  `elab07_repair_shape = "set_insertion"`, `elab04_has_repair = false`,
  `elab10_repair_count = 1`, `elab13_repair_count = 1`.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1306
  numbered reports found.
- `python3 scripts/check_source_hierarchy.py`: required 602 / present 602 /
  missing 0.
- `git diff --check` exited 0.
- Endpoint-form scan over changed and untracked files returned no matches.

## What changed in understanding

The remaining `plan/101` N13/N14/N15 gap is not a new source-level fixture
family in the current parser. It is a payload-shape regression risk. The
minimal current guard is therefore to assert that the exact `ELAB-07` positive
payload is one complete set item and lacks child / bundle / partial /
textual-guidance markers.

## Open questions

- Should future explicit bundle semantics use `suggested_repair[]` or a
  separate repair-plan field?
- Should partial guidance live in `suggested_repair[]` with explicit
  `partiality`, or outside executable repair suggestions?
- What final source-locus / edit-script representation should replace current
  LAB-local payload fields if this becomes public?
- Should future OBL-025 remain single-edit only or gain a separate grouped
  multi-edit relation?

## Suggested next prompt

「`ELAB-04` mixed visibility branch payload-model preflight を docs-only で進め、
base / visibility branch ownership と ranking の未決を整理してください。」

## Plan update status

更新済み:

- Added `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`.
- Updated `plan/101` and `plan/102`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み:

- Added `plan/106` child / bundle / partial exclusion fixture status to the
  concise Surface Mir current summary.

## progress.md update status

更新済み:

- Updated current E-ROW / ELAB-07 status with child / bundle / partial
  exclusion fixture evidence.
- Added a 2026-07-04 06:07 JST recent-log row.

## tasks.md update status

更新済み:

- Moved child / bundle / partial exclusion fixtures into current evidence.
- Removed that candidate from the remaining candidate table.

## samples_progress.md update status

更新済み:

- Updated dashboard wording for `plan/106`.
- Added a recent validation log row.
- Kept sample row count at 52.

## Reviewer findings and follow-up

- Spec-compliance reviewer `019f29d6-c603-7083-9191-95143bdfbb24`
  reported the package as spec compliant, with no blocking overclaims, stale
  report sections, fixture drift, or future-ABI over-freeze.
- Code-quality reviewer `019f29d9-ffb2-7ee3-81bd-a3b6da03a5bd`
  found no Rust/test issue and no critical issue. It flagged two bookkeeping
  fixes: remove this stale reviewer placeholder and add
  `mirrorea_canon/MAP.md` to the consulted-documents list. Both fixes are
  applied in this report revision.

## Skipped validations and reasons

No required local validation was skipped for this package. Oracle consultation
was not used because this was a bounded test-only characterization package and
the sub-agent mapper covered the representability / over-spec risk.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f29ca-10b3-7c43-9433-0d66b72954d8` mapped current ELAB-07
repair JSON shape, singleton/set differences, Surface representability limits,
minimal safe assertions, and over-spec risks. It was closed after its findings
were incorporated.
