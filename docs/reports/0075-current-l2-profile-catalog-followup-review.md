# Report 0075 — current l2 profile catalog followup review

- Date: 2026-04-02T09:57:14.149937Z
- Author / agent: Codex
- Scope: Narrow-scope follow-up review of the current uncommitted diff after the named profile test responsibility fix.
- Decision levels touched: L2 companion-example test-boundary clarification only.

## 1. Objective

Review whether the current uncommitted diff resolves the earlier coverage-gap finding for the named profile catalog boundary, while keeping public behavior coverage adequate without deriving integration expectations from production catalog code, and whether `specs/examples/13-current-l2-profile-catalog.md` now matches the actual test split.

## 2. Scope and assumptions

- Review only the current uncommitted diff in:
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `specs/examples/13-current-l2-profile-catalog.md`
- Treat `specs/examples/13-current-l2-profile-catalog.md` as the subsystem-specific companion spec for this review.
- Do not broaden into unrelated catalog design or implementation changes.
- No implementation edits were made as part of the review; only this report was added to satisfy repository reporting policy.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `docs/reports/0074-current-l2-profile-catalog-test-responsibility-boundary.md`

## 4. Actions taken

- Inspected `git status --short`, `git diff --stat`, and per-file diffs to isolate the uncommitted scope.
- Read the earlier follow-up review report `0074` to confirm the exact prior finding.
- Read the updated profile-layer and named-profile integration test sections in `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`.
- Read the relevant catalog implementation and internal-test boundary comment in `crates/mir-semantics/src/harness.rs`.
- Compared the revised spec wording in `specs/examples/13-current-l2-profile-catalog.md` against the actual test split.
- Ran the `mir-semantics` current-L2 integration test target to verify the affected tests execute and pass.

## 5. Files changed

- `docs/reports/0075-current-l2-profile-catalog-followup-review.md`
  - Added this follow-up review report.

## 6. Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/examples/12-current-l2-selection-profiles.md
sed -n '1,260p' specs/examples/13-current-l2-profile-catalog.md
git status --short
git diff --stat
git diff --name-only
git diff -- crates/mir-semantics/src/harness.rs
git diff -- crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
git diff -- specs/examples/13-current-l2-profile-catalog.md
sed -n '1,220p' docs/reports/0074-current-l2-profile-catalog-test-responsibility-boundary.md
nl -ba crates/mir-semantics/tests/current_l2_minimal_interpreter.rs | sed -n '620,920p'
nl -ba crates/mir-semantics/src/harness.rs | sed -n '700,780p'
nl -ba crates/mir-semantics/src/harness.rs | sed -n '1158,1185p'
nl -ba specs/examples/13-current-l2-profile-catalog.md | sed -n '76,152p'
cargo test -p mir-semantics current_l2_minimal_interpreter -- --nocapture
cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --nocapture
```

## 7. Evidence / outputs / test results

- `git diff --stat` shows only three behavioral review targets in the current diff:
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `specs/examples/13-current-l2-profile-catalog.md`
- The earlier finding in `0074` was that the spec moved selected-count and suffix responsibility to `run_directory_profiled` tests, but `runtime-e3` and broad static-only coverage were still missing there.
- That gap is now addressed in the integration test file:
  - `run_directory_profiled_runtime_e3_runs_one_runtime_bundle` asserts the `runtime-e3` count split and concrete `e3-option-admit-chain.json` suffix.
  - `run_directory_profiled_static_only_includes_profile_name_in_summary` asserts the broad static-only selected-count split (`2 / 0 / 2`) and pass/fail totals.
- The named-profile integration test still keeps literal public behavior oracles independent from production catalog resolution:
  - alias list exact compare remains literal,
  - `resolved_request` expectations still come from test-local literal builders,
  - unknown alias failure remains a literal error expectation.
- The named-profile delegation helper now compares `run_directory_named_profile(...)` against `run_directory_profiled(...)` built from the literal expected request, not from `ProfileCatalog::resolve()`. This preserves the “no production catalog as test oracle” boundary while allowing profile-layer behavior to be owned by profile-layer tests.
- `specs/examples/13-current-l2-profile-catalog.md` now matches the actual split:
  - tests keep alias list, literal `resolved_request`, delegation-to-profiled execution, and unknown alias failure,
  - detailed selected counts and concrete single-fixture suffix checks are mainly owned by `run_directory_profiled` integration tests.
- Verification:
  - `cargo test -p mir-semantics current_l2_minimal_interpreter -- --nocapture` matched zero tests because the string was interpreted as a name filter.
  - `cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --nocapture` ran 32 tests and all 32 passed, including:
    - `run_directory_profiled_runtime_e3_runs_one_runtime_bundle`
    - `run_directory_profiled_static_only_includes_profile_name_in_summary`
    - `run_directory_named_profiles_match_catalog_resolution_and_expected_selection`

## 8. What changed in understanding

- The earlier follow-up finding was specifically about test-placement mismatch, not about the delegation strategy itself.
- With the added `run_directory_profiled` cases, the named-profile test can legitimately stop restating selected-count and suffix facts while still preserving public behavior coverage.
- The companion spec text now accurately describes the implemented split between profile-layer integration coverage and named-profile integration coverage.

## 9. Open questions

- None within this narrow review scope.

## 10. Suggested next prompt

No follow-up is required for this narrow scope. If you want a broader pass next, review whether the same “literal public oracle vs delegated lower-layer behavior” boundary is applied consistently across other current L2 helper layers.
