# Report 0074 — current l2 profile catalog test responsibility boundary

- Date: 2026-04-02T09:52:02.854844Z
- Author / agent: Codex
- Scope: Narrow-scope review of the current uncommitted diff for the named profile catalog internal-vs-integration test responsibility task.
- Decision levels touched: L2 companion-example test-boundary clarification only.

## 1. Objective

Review the current uncommitted diff for the named profile catalog internal-vs-integration test responsibility change, focusing on whether the new boundary:

- weakens public behavior coverage,
- derives integration expectations from production catalog code, or
- conflicts with `specs/examples/13-current-l2-profile-catalog.md`.

## 2. Scope and assumptions

- Review only the current uncommitted diff in `crates/mir-semantics/` and `specs/examples/13-current-l2-profile-catalog.md`.
- Treat `specs/examples/13-current-l2-profile-catalog.md` as the subsystem-specific companion spec for this review.
- No implementation edits were made as part of the review; only this report was completed to satisfy repository reporting policy.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/13-current-l2-profile-catalog.md`

## 4. Actions taken

- Inspected `git status --short` to identify the current uncommitted scope.
- Read the uncommitted diffs for:
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `specs/examples/13-current-l2-profile-catalog.md`
- Read surrounding implementation and test context in `harness.rs` and `current_l2_minimal_interpreter.rs`.
- Compared the revised spec text against the actual remaining `run_directory_profiled` coverage.

## 5. Files changed

- `docs/reports/0074-current-l2-profile-catalog-test-responsibility-boundary.md`
  - Completed this report.

## 6. Commands run

```bash
git status --short
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,320p' specs/examples/13-current-l2-profile-catalog.md
git diff -- crates/mir-semantics/src/harness.rs
git diff -- crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
git diff -- specs/examples/13-current-l2-profile-catalog.md
rg -n "named_profile|ProfileCatalog|run_directory_profiled|run_directory_named_profile|NamedProfileRunSummary|SelectionProfile::new|named_profile_specs" crates/mir-semantics/src/harness.rs crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
nl -ba crates/mir-semantics/tests/current_l2_minimal_interpreter.rs | sed -n '630,870p'
nl -ba specs/examples/13-current-l2-profile-catalog.md | sed -n '84,152p'
rg -n "run_directory_profiled\\(|SelectionProfile::new\\(" crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
rg -n "e3-option-admit-chain\\.json|e3-option-admit-chain\\)|StaticOnly\\)|total_selected_bundles.*2|runtime_selected_bundles.*1|static_selected_bundles.*2|smoke-static|runtime-e3" crates/mir-semantics/tests crates/mir-semantics/src
```

## 7. Evidence / outputs / test results

- No test suite was executed; this was a diff review only.
- Current diff touches:
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `specs/examples/13-current-l2-profile-catalog.md`
  - this report file

## 8. What changed in understanding

- The implementation change in `harness.rs` is only a comment update; the behavioral change is entirely in test responsibility and companion-spec wording.
- The revised named-profile integration test still keeps literal alias and `resolved_request` expectations independent from `ProfileCatalog::resolve()`.
- The actual gap is narrower: the revised spec says selected-bundle counts and concrete single-fixture suffix checks now mainly live in `run_directory_profiled` integration tests, but two of those concrete expectations are not covered there today.

## 9. Open questions

- Should `smoke-static` broad static-only summary coverage be added under `run_directory_profiled`, or should the spec text say that selection-helper tests, not only profiled tests, still carry part of that responsibility?
- Should `runtime-e3` keep a direct concrete suffix assertion in named-profile tests until a matching `run_directory_profiled` test exists?

## 10. Suggested next prompt

Add or justify missing `run_directory_profiled` coverage for `smoke-static` and `runtime-e3`, then re-check that `specs/examples/13-current-l2-profile-catalog.md` matches the actual test boundary.
