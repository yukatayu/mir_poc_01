# Report 2153 - G1 ELAB-07 set-insertion exact-locus guard hardening

- Date: 2026-07-04 05:43 JST
- Author / agent: Codex
- Scope: LAB-only exact-locus guard hardening around the current `ELAB-07`
  set-insertion repair payload prototype.
- Decision levels touched: `L3` LAB evidence only.

## Objective

Keep the non-final `ELAB-07` `set_insertion` repair limited to the exact
current `ELAB-07` source locus, and prevent current Surface-expressible
omitted-row / retargeting proxies from receiving that set repair.

## Scope and assumptions

The scope is limited to Surface-to-Core elaboration LAB repair evidence.
Expected sample JSON and public serialized LAB output shape should remain
unchanged.

Working assumptions:

- canon in `mirrorea_canon/` remains normative;
- `plan/102` really means the exact current `ELAB-07` fact pattern, not a
  generalized set-insertion rule;
- current Surface syntax cannot separately represent missing target row versus
  row creation, so omitted `fails` is the current proxy;
- current Surface diagnostics do not have first-class row movement or
  cross-row retargeting carriers;
- event / role / owner / state / field retargeting fixtures are current
  sample-identity proxies, not final row movement semantics.

This package does not claim general set-insertion support, a final source-locus
identity model, a final row identity model, bundle semantics, partial guidance,
repair ranking, visibility ranking, multi-edit support, final Diagnostic /
repair ABI, OBL-024/025 proof, conformance, canon movement, or G1 exit.

## Start state / dirty state

Package start:

- `HEAD = origin/main = 7bfd7d15e8584533515b5ad2a5e37b49062e9783`
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
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `mirrorea_canon/plan/00-gates.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- `crates/mir-ast/src/surface_alpha.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/main/src/write-failure-row-negative.mir`

## Actions taken

- Added helper `assert_no_set_insertion_repair` in the Rust elaboration test
  file.
- Added omitted failure row / row-creation proxy fixture.
- Added event retargeting proxy fixture.
- Added role retargeting proxy fixture.
- Added state-field retargeting proxy fixture.
- Added reviewer-follow-up owner-locus retargeting proxy fixture.
- Added reviewer-follow-up state-name retargeting proxy fixture.
- Verified RED: event, role, and field retargeting proxies incorrectly
  received `set_insertion` repairs before the guard.
- Added internal exact-locus constants for the current `ELAB-07` source locus.
- Required target ref, owner locus, state name, and field name to match the
  current `ELAB-07` locus before emitting the set payload.
- Added `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`.
- Updated `plan/101`, `plan/102`, `plan/104`, `plan/00-index.md`, and
  `plan/90-source-traceability.md`.
- Updated `README.md`, `Documentation.md`,
  `docs/research_abstract/surface_mir_alpha_01.md`, `progress.md`,
  `tasks.md`, and `samples_progress.md`.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
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
- `docs/reports/2153-g1-erow07-set-insertion-exact-locus-guard-hardening.md`

## Commands run

Already run before this report was first written:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M %Z'
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows -- --nocapture
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

Fresh final validation:

```bash
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test --workspace
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2153.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2153.json
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
(git diff --name-only; git ls-files --others --exclude-standard) | sort -u | xargs -r rg -n --pcre2 '<endpoint-form pattern>' || true
```

Reviewer follow-up validation:

```bash
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test --workspace
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2153-review.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2153-review.json
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
(git diff --name-only; git ls-files --others --exclude-standard) | sort -u | xargs -r rg -n --pcre2 '<endpoint-form pattern>' || true
```

## Evidence / outputs / test results

Pre-report evidence:

- RED: the focused `elab07_set_insertion_is_not_emitted` target failed for
  event, role, and state-field retargeting proxies with unexpected
  `set_insertion` repair output.
- GREEN: the focused target passed after adding the exact-locus guard:
  8 passed / 0 failed.
- Exact `ELAB-07` positive path still passed: 1 passed / 0 failed.
- `plan/104` same-event positive path still passed: 1 passed / 0 failed.
- `cargo fmt --check`: exit 0.
- Full `surface_to_core_elaboration` integration test file passed:
  29 passed / 0 failed. The printed panic is from an expected `should_panic`
  test and the final test result is pass.

Fresh final validation:

- `cargo fmt --check`: exit 0.
- Focused negative guard target: 8 passed / 0 failed.
- Exact `ELAB-07` positive path: 1 passed / 0 failed.
- `plan/104` same-event row association positive path: 1 passed / 0
  failed.
- Full `surface_to_core_elaboration` integration test file: 29 passed / 0
  failed. The printed panic is from an expected `should_panic` test and the
  final test result is pass.
- `cargo test --workspace`: exit 0.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 tests,
  OK.
- `python3 scripts/surface_mir_samples.py --format json check-all`: summary
  extracted from `/tmp/mirrorea-surface-check-all-2153.json`:

```json
{
  "sample_count": 52,
  "failed_count": 0,
  "validation_error_count": 0,
  "elab07_repair_shape": "set_insertion",
  "elab04_has_repair": false,
  "elab10_repair_count": 1,
  "elab13_repair_count": 1
}
```

- `python3 scripts/validate_docs.py`: scaffold complete; 1305 numbered
  reports found.
- `python3 scripts/check_source_hierarchy.py`: required 602 / present 602 /
  missing 0.
- `git diff --check`: exit 0.
- Changed-file endpoint-form secret scan: no matches.

Reviewer follow-up validation after owner-locus and state-name retargeting
fixtures:

- `cargo fmt --check`: exit 0.
- Focused negative guard target: 10 passed / 0 failed.
- Full `surface_to_core_elaboration` integration test file: 31 passed / 0
  failed. The printed panic is from an expected `should_panic` test and the
  final test result is pass.
- `cargo test --workspace`: exit 0.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 tests,
  OK.
- `python3 scripts/surface_mir_samples.py --format json check-all`: summary
  extracted from `/tmp/mirrorea-surface-check-all-2153-review.json`:

```json
{
  "sample_count": 52,
  "failed_count": 0,
  "validation_error_count": 0,
  "elab07_repair_shape": "set_insertion",
  "elab04_has_repair": false,
  "elab10_repair_count": 1,
  "elab13_repair_count": 1
}
```

- `python3 scripts/validate_docs.py`: scaffold complete; 1305 numbered
  reports found.
- `python3 scripts/check_source_hierarchy.py`: required 602 / present 602 /
  missing 0.
- `git diff --check`: exit 0.
- Changed-file endpoint-form secret scan: no matches.

## What changed in understanding

`plan/104` fixed row association, but the set-payload predicate was still
broader than the documented exact `ELAB-07` fact pattern. Matching only failure
set arithmetic was not enough: nearby role / event / owner / state / field
shapes could look arithmetically identical while requiring source-locus
retargeting.

The narrowest current executable fix is to guard the set path by the current
`ELAB-07` target ref plus generated request locus fields. This is intentionally
less general and should be treated as LAB guard evidence, not final source
identity design.

## Open questions

- What final source-locus identity, if any, should replace these LAB constants?
- How should true row movement and cross-row retargeting be represented?
- Should future generalized set insertion key off explicit sample / rule IDs,
  AST row IDs, or proof-facing local witness objects?
- Which partial-guidance / child-repair / bundle exclusion fixtures should be
  promoted next?

## Suggested next prompt

「`plan/105` の後続として、ELAB-07 set insertion の child singleton /
bundle / partial-guidance exclusion fixtures を Rust-only で追加してください。」

## Plan update status

更新済み:

- Added `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`.
- Updated `plan/101`, `plan/102`, and `plan/104`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み:

- Added `plan/105` exact-locus guard hardening to the concise Surface Mir
  current summary.

## progress.md update status

更新済み:

- Updated current E-ROW / ELAB-07 status with exact-locus guard hardening.
- Added a 2026-07-04 05:43 JST recent-log row.

## tasks.md update status

更新済み:

- Moved row-creation / retargeting rejection proxy work into current evidence.
- Replaced the candidate with child / bundle / partial-guidance exclusion
  fixtures.

## samples_progress.md update status

更新済み:

- Updated dashboard wording for `plan/105`.
- Added a recent validation log row.
- Kept sample row count at 52.

## Reviewer findings and follow-up

Reviewer sub-agent `019f29c0-aa12-75a0-8e1f-a23773eb0e21` found:

- Medium: report omitted relevant canon files from Documents consulted.
  Follow-up: added the canon files consulted for this package.
- Low: owner-locus and state-name exact-locus predicate dimensions lacked
  direct tests. Follow-up: added both Rust-only retargeting fixtures.
- Low: report embedded the endpoint-form scan pattern literally, making the
  report self-match. Follow-up: redacted the literal pattern in this report
  command block while keeping the actual scan in validation.

## Skipped validations and reasons

No required validation was intentionally skipped for this package. External
browser / Oracle consultation was not used because the package was a bounded
Rust guard hardening task with local executable evidence and existing `plan/`
context.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f29b2-e521-7ac3-96c7-2ac581ec60fe` mapped current set emission
predicates, Surface representability limits, already-covered cases, and
over-spec risks. It was closed after its findings were incorporated.

Reviewer sub-agent `019f29c0-aa12-75a0-8e1f-a23773eb0e21` completed review and
was closed after follow-up fixes were applied.
