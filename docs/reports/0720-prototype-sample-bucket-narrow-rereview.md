# Report 0720 — prototype sample bucket narrow rereview

- Date: 2026-04-17T02:00:42.310200Z
- Author / agent: Codex
- Scope: narrow re-review of the current uncommitted diff, limited to the two prior findings about comment-block boundary and CLI host-plan omission wording
- Decision levels touched: none; review-only confirmation of helper-local boundary wording and tests

## 1. Objective

Re-inspect the current uncommitted diff after fixes and determine whether the two previously reported issues are resolved or whether any actionable drift remains.

### Scope and assumptions

- This task is review-only except for this report.
- Only the two prior findings are in scope:
  1. leading contiguous `#` comment block boundary in `crates/mir-runtime/src/current_l2.rs` plus tests/docs
  2. operational CLI `--host-plan` omission wording vs docs
- `plan/` 更新不要。
- `progress.md` 更新不要。
- `tasks.md` 更新不要。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `docs/reports/0718-prototype-sample-actualization-first-tranche.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `samples/prototype/README.md`

## 3. Actions taken

1. Re-read the repo entry docs required by `AGENTS.md`.
2. Inspected the current diff only for the previously flagged code/docs/tests areas.
3. Compared the current implementation behavior against the current spec/report wording for both issues.
4. Re-ran the relevant `mir-runtime` test targets.

## 4. Files changed

- `docs/reports/0720-prototype-sample-bucket-narrow-rereview.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 5. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-17 11:00:47 JST`
- `git diff -- crates/mir-runtime/src/current_l2.rs crates/mir-runtime/src/current_l2_cli.rs crates/mir-runtime/tests/current_l2_source_lowering.rs crates/mir-runtime/tests/current_l2_operational_cli.rs specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md .docs/current-l2-source-sample-authoring-policy.md docs/reports/0718-prototype-sample-actualization-first-tranche.md plan/09-helper-stack-and-responsibility-map.md samples/prototype/README.md`
  - confirmed:
    - `collect_current_l2_source_lines` now tracks a leading comment block only
    - a new rejection test exists for non-leading `#` comments
    - report/spec wording now says `leading contiguous` block
    - CLI/report/helper docs now describe adjacent sidecar omission in terms of explicit sample paths, not prototype-only wording
- `cargo test -p mir-runtime --test current_l2_source_lowering`
  - `running 16 tests`
  - `test current_l2_source_lowering_ignores_leading_hash_comment_lines ... ok`
  - `test current_l2_source_lowering_rejects_non_leading_hash_comment_lines ... ok`
  - `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - `running 5 tests`
  - `test operational_cli_rejects_missing_host_plan_flag ... ok`
  - `test operational_cli_uses_adjacent_host_plan_for_prototype_sample_when_omitted ... ok`
  - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

## 6. Evidence / findings

### Issue 1: leading contiguous `#` comment block boundary

- Resolved.
- `crates/mir-runtime/src/current_l2.rs` now ignores only the initial contiguous `#` block before the first non-comment, nonblank content.
- `crates/mir-runtime/tests/current_l2_source_lowering.rs` now covers both:
  - accepted leading comment block
  - rejected non-leading `#` comment
- The narrowed boundary is mirrored in:
  - `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`
  - `.docs/current-l2-source-sample-authoring-policy.md`
  - `docs/reports/0718-prototype-sample-actualization-first-tranche.md`
- No actionable drift remains on this point.

### Issue 2: operational CLI `--host-plan` omission wording vs docs

- Resolved.
- The previous prototype-only wording drift is gone. Current docs now match the implementation more closely:
  - implementation: omission succeeds when the resolved sample path has an adjacent `.host-plan.json`
  - docs/report: omission is described as adjacent-sidecar convenience for explicit sample paths that provide one
- The relevant alignment is visible in:
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `docs/reports/0718-prototype-sample-actualization-first-tranche.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `samples/prototype/README.md`
- The CLI usage string remains intentionally concise rather than exhaustive, but given the matching error message and surrounding docs, it is not an actionable inconsistency in the current diff.

## 7. Changes in understanding

- The fixes did not just patch code; they also closed the test and docs gaps that made the earlier findings actionable.
- The remaining wording is now within acceptable summary-level abstraction rather than spec/implementation drift.

## 8. Open questions

- None for this narrow re-review.

## 9. Suggested next prompt

No follow-up is required for these two issues; if needed, the next review can return to broader semantic or docs-snapshot consistency outside this narrow scope.
