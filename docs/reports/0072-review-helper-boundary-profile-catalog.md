# Report 0072 — review helper boundary profile catalog

- Date: 2026-04-02T09:27:29.111889Z
- Author / agent: Codex (GPT-5)
- Scope: Narrow-scope review of the current uncommitted diff for the named profile behavior table helper-boundary task in `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`, with spec alignment check against `specs/examples/13-current-l2-profile-catalog.md`.
- Decision levels touched: L2 companion-spec clarification only; no L0/L1 invariant change reviewed or proposed.

## 1. Objective

Determine whether helperizing selected bundle counts and concrete fixture suffix expectations:

1. weakens public behavior coverage in the named profile behavior tests,
2. accidentally derives test expectations from production catalog code, or
3. conflicts with `specs/examples/13-current-l2-profile-catalog.md`.

## 2. Scope and assumptions

- Scope was limited to the uncommitted diff in the test file and the concurrently edited profile-catalog spec companion.
- Review focus was limited to behavior coverage, oracle independence, and spec alignment.
- No unrelated implementation changes were proposed.
- No runtime test execution was required for this review; the assessment is diff- and spec-based.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/src/harness.rs`

## 4. Actions taken

- Inspected repository-required documents in order before reviewing the diff.
- Confirmed the working tree only changed the target test file and the relevant spec companion.
- Compared the uncommitted test diff against the surrounding named-profile test table and assertion flow.
- Checked `crates/mir-semantics/src/harness.rs` to ensure the new test-local helper does not derive expectations from `ProfileCatalog::resolve()` or other production catalog internals.
- Created this required review report.

Files changed:

- `docs/reports/0072-review-helper-boundary-profile-catalog.md`

Commands run and exact outputs:

- `git status --short`
  - ` M crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - ` M specs/examples/13-current-l2-profile-catalog.md`
- `python scripts/new_report.py --slug review-helper-boundary-profile-catalog`
  - `/usr/bin/bash: line 1: python: command not found`
- `python3 scripts/new_report.py --slug review-helper-boundary-profile-catalog`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0072-review-helper-boundary-profile-catalog.md`
- Additional read-only inspection commands:
  - `git diff -- crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `git diff -- specs/examples/13-current-l2-profile-catalog.md`
  - `sed -n ...` / `nl -ba ...` against the consulted docs and source files

## 5. Evidence / outputs / test results

No findings.

Evidence:

- The new `ExpectedSelectedBundles` carrier in `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` keeps selected-bundle counts and single-fixture suffixes as literals in the test-local case table; it does not consult `ProfileCatalog`, `named_profile_specs()`, or any other production catalog source.
- `assert_named_profile_selected_bundles()` still checks the same public summary fields as before:
  - `total_selected_bundles`
  - `runtime_selected_bundles`
  - `static_selected_bundles`
  - `bundle_reports.len()`
  - single-fixture `fixture_path` suffix for the focused aliases
- `resolved_request` remains independently asserted via the literal builder functions stored in `expected_request`, so the helperization did not collapse request expectations into the production resolver path.
- Unknown-alias behavior remains a literal error expectation in `run_directory_named_profile_rejects_unknown_alias()`.
- The companion spec change in `specs/examples/13-current-l2-profile-catalog.md` explicitly permits exactly this boundary:
  - tests may bundle selected bundle counts and concrete fixture suffix checks into a test-local assertion helper,
  - but must not derive expectations from `ProfileCatalog::resolve()` or other catalog implementation paths.
- The updated test code matches that rule: helperization is local to tests and still uses literal expectations.

Test execution:

- Not run. This review was limited to diff inspection and spec conformance.

## 6. What changed in understanding

- The helper-boundary intent is now explicit in both the test code and the companion spec: `resolved_request` must remain an implementation-independent oracle, while selected-count and suffix checks may be factored into test-local helpers.
- The uncommitted diff preserves that separation rather than weakening it.

## 7. Open questions

- None within the requested narrow scope.

## 8. Suggested next prompt

Broaden the review to the production named-profile catalog implementation in `crates/mir-semantics/src/harness.rs` and check whether its internal tests are still appropriately separated from the public-behavior tests.
