# Report 2152 — G1 ELAB-07 set-insertion row-identity guard hardening

- Date: 2026-07-04 05:22 JST
- Author / agent: Codex
- Scope: LAB-only internal row association hardening around the exact
  `ELAB-07` set-insertion repair payload prototype.
- Decision levels touched: `L3` LAB evidence only.

## Objective

Prevent the `plan/103` multi-request suppression guard from under-suggesting
the exact `set_insertion` repair across two distinct same-event `when` rows in
one role locus.

## Scope and assumptions

The scope is limited to the Surface-to-Core elaboration LAB repair evidence
path. Public LAB `target_ref` strings and sample expected JSON remain stable.

Working assumptions:

- canon in `mirrorea_canon/` remains normative;
- source spans already exist on `SurfaceWhenBlock` and can serve as an internal
  LAB association discriminator;
- no AST field should be added for this package;
- span identity is adequate for current LAB under-suggestion prevention but is
  not a final row identity model.

This package does not claim general set-insertion support, a final row identity
model, bundle semantics, partial guidance, repair ranking, visibility ranking,
multi-edit support, final Diagnostic / repair ABI, OBL-024/025 proof,
conformance, canon movement, or G1 exit.

## Start state / dirty state

Package start:

- `HEAD = origin/main = 685a21b6b4327cec0cf361d77b3fb60aa73479fe`
- notifier task baseline was recorded before continuing package work;
- working tree was clean.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- `crates/mir-ast/src/surface_alpha.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`

## Actions taken

- Added Rust RED test
  `elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows`.
- Verified the RED failure: two distinct same-event rows collapsed to one public
  `target_ref`, causing one detail to have `suggested_repair = null`.
- Kept public `failure_row_context.target_ref` unchanged.
- Added serialization-skipped internal `association_key` to
  `SurfaceLabDiagnosticFailureRowContext`.
- Derived `association_key` from public `target_ref` plus
  `when.span.start..when.span.end`.
- Changed `failure_row_request_counts` and retrospective set-repair suppression
  to use `association_key`.
- Kept the previous same-row multi-request suppression behavior.
- Added `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`.
- Updated `plan/101`, `plan/102`, `plan/103`, `plan/00-index.md`, and
  `plan/90-source-traceability.md`.
- Updated `README.md`, `Documentation.md`,
  `docs/research_abstract/surface_mir_alpha_01.md`, `progress.md`,
  `tasks.md`, and `samples_progress.md`.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
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
- `docs/reports/2152-g1-erow07-set-insertion-row-identity-guard-hardening.md`

## Commands run

Already run before this report was first written:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows -- --nocapture
cargo fmt
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

Fresh validation commands run before review:

```bash
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test --workspace
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2152.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2152.json
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
```

Changed-file secret-pattern scans were also run. The broad long-token scan only
matched long test / diagnostic identifiers; the endpoint-form scan returned no
matches.

After applying reviewer follow-up wording fixes, these commands were rerun:

```bash
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test --workspace
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2152.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2152.json
```

## Evidence / outputs / test results

Pre-report evidence:

- RED: the new same-event distinct-row test failed with
  `left: Null`, `right: "set_insertion"`.
- GREEN: the same test passed after switching internal association to include
  the existing `when` source span.
- `plan/103` focused negative tests still passed: 4 passed / 0 failed.
- Full `surface_to_core_elaboration` integration test file passed: 25 passed /
  0 failed.

Fresh final validation:

- `cargo fmt --check`: exit 0.
- New same-event distinct-row Rust target: 1 passed / 0 failed.
- Previous `plan/103` focused negative targets: 4 passed / 0 failed.
- Full `surface_to_core_elaboration` integration file: 25 passed / 0 failed.
  The printed panic is from an expected `should_panic` test and the final test
  result is pass.
- `cargo test --workspace`: exit 0.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 tests OK.
- `surface_mir_samples.py check-all`: exit 0.
- `jq` summary for `/tmp/mirrorea-surface-check-all-2152.json`:
  `sample_count = 52`, `failed_count = 0`, `validation_error_count = 0`,
  `elab07_repair_shape = "set_insertion"`, `elab04_has_repair = false`,
  `elab10_repair_count = 1`, `elab13_repair_count = 1`.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1304
  numbered reports found.
- `python3 scripts/check_source_hierarchy.py`: required 602 / present 602 /
  missing 0.
- `git diff --check`: exit 0.
- Changed-file endpoint-form secret scan: no matches.
- Reviewer follow-up reruns after wording fixes:
  `validate_docs.py`, `check_source_hierarchy.py`, `git diff --check`,
  `cargo fmt --check`, full `surface_to_core_elaboration`, `cargo test
  --workspace`, Surface helper unit tests, and Surface sample check-all all
  returned exit 0. The final Surface sample summary remained
  `sample_count = 52`, `failed_count = 0`, `validation_error_count = 0`,
  `elab07_repair_shape = "set_insertion"`, `elab04_has_repair = false`,
  `elab10_repair_count = 1`, `elab13_repair_count = 1`.

## What changed in understanding

The `plan/103` guard was correct for multiple requests inside one `when` row,
but public `target_ref = locus + event` was too broad for request association.
The public string remains useful as a human-facing LAB target, but internal
repair association needs a row-level discriminator.

The existing `when` source span is the narrowest current discriminator that
does not widen AST serialization or expected sample JSON. It is still not a
durable final row identity.

## Open questions

- Should a future final row identity be AST-assigned rather than span-derived?
- How should row identity survive source-preserving edits or formatting?
- When should row creation, row splitting, row movement, and retargeting
  rejection fixtures be added?
- What is the future policy for true multi-target-row diagnostics?

## Suggested next prompt

「`plan/104` の残りとして、ELAB-07 set insertion の row creation / splitting /
movement / retargeting rejection fixtures を Rust-only で追加してください。」

## Plan update status

更新済み:

- Added `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`.
- Updated `plan/103`, `plan/102`, and `plan/101`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み:

- Added the `plan/104` row-identity guard hardening status to the concise
  Surface Mir current summary.

## progress.md update status

更新済み:

- Updated current E-ROW / ELAB-07 status with span-based internal association
  hardening.
- Added a 2026-07-04 05:22 JST recent-log row.

## tasks.md update status

更新済み:

- Moved row-identity guard hardening from candidate work into current evidence.
- Replaced that candidate with row creation / splitting / movement / retargeting
  rejection fixtures.

## samples_progress.md update status

更新済み:

- Updated dashboard wording for `plan/104`.
- Added a recent validation log row.
- Kept sample row count at 52.

## Reviewer findings and follow-up

Read-only reviewer `019f29ab-ae7a-7960-8803-a50abb6f5d74` reported no
code-level semantic findings.

Low-severity follow-up applied:

- Reworded the `progress.md` and `tasks.md` `plan/103` bullets so the
  public-target-ref guard is described as the `plan/103` close state, with
  `plan/104` explicitly recorded as the later internal span-based association
  narrowing.
- Replaced this report's reviewer placeholder with the actual review outcome.

Residual risk recorded by the reviewer and retained here: span identity is
LAB-only and not durable across formatting / source-preserving edits. The
serialization-skipped internal fields also remain part of Rust `Debug` and
`PartialEq`, which is acceptable for the current LAB-only report type but not a
final public ABI claim.

## Skipped validations and reasons

None for this package. The validation scope includes focused Rust targets,
full Surface-to-Core elaboration tests, full workspace Cargo tests, Surface
helper unit tests, Surface sample check-all, docs/source validators, whitespace
diff check, and changed-file secret-pattern scans.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f299f-f601-71b1-883f-5d8c773cbd12` mapped the relevant parser and
elaboration paths and was closed before report finalization.

Key findings incorporated:

- `SurfaceWhenBlock` already carries `span`; no explicit row/block ID exists.
- `parse_when_block` is centralized, but adding an AST field would widen parse
  report shape and equality surface.
- Public `target_ref` should remain unchanged to avoid expected JSON and exact
  test churn.
- The minimal boundary is semantics-only: derive an internal key from
  `when.span.start/end`.

Reviewer sub-agent `019f29ab-ae7a-7960-8803-a50abb6f5d74` was closed after its
low-severity documentation findings were applied and revalidated.
