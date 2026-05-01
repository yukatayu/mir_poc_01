# Report 1078 — latest report heading guardrail

- Date: 2026-05-01 11:21 JST
- Author / agent: Codex
- Scope: docs scaffold validation / report discipline
- Decision levels touched: none; process guardrail only

## Objective

Extend `validate_docs.py` narrowly so it checks the latest numbered report for the required report headings, without retroactively linting every historical report.

## Scope and assumptions

- Scope is limited to `validate_docs.py`, its focused unit tests, `scripts/README.md`, snapshot mirrors, and this report.
- Historical reports are not rewritten or mass-linted.
- The validator still performs scaffold checks, not semantic validation of report content.
- Stop line: this package does not change normative specs, sample semantics, public API, packaging, host integration, or product-surface decisions.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `AGENTS.md`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`
- `docs/reports/TEMPLATE.md`
- `docs/reports/1077-report-template-commands-run-guardrail.md`

## Actions taken

- Added a failing unit test showing that `validate_docs.main()` did not reject a latest numbered report missing `## Commands run`.
- Implemented a narrow latest-report heading check using the existing required heading list.
- Added regression coverage showing a historical report can miss a required heading while a valid latest report still passes, preserving the latest-only scope.
- Updated `scripts/README.md` to describe the new latest-report scaffold check and its non-semantic scope.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` to mirror the maintenance checkpoint.
- Added this report.

## Files changed

- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1078-latest-report-heading-guardrail.md`

## Commands run

```bash
git status --short
git log -1 --oneline
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git log -1 --oneline
67a8f1c Guard report template commands section
```

TDD RED:

```text
$ python3 -m unittest scripts.tests.test_validate_docs
F..
...
FAIL: test_main_rejects_latest_report_missing_commands_run_section
AssertionError: 0 != 1
FAILED (failures=1)
```

Implementation summary:

```text
validate_docs.py now selects reports[-1], reads that latest numbered report, and applies the same required heading list used for the report template.
If the latest report is missing required sections, it prints:
Latest report is missing required sections: <report-name>
```

Focused validation:

```text
$ python3 -m unittest scripts.tests.test_validate_docs
...
----------------------------------------------------------------------
Ran 3 tests in 0.006s

OK

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1075 numbered report(s).
```

Reviewer follow-up latest-only coverage:

```text
$ python3 -m unittest scripts.tests.test_validate_docs
....
----------------------------------------------------------------------
Ran 4 tests in 0.007s

OK
```

Post-report validation:

```text
$ python3 scripts/check_source_hierarchy.py
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 35
  present: 35
  missing: 0
  all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1076 numbered report(s).

$ git diff --check
<no output>
```

## What changed in understanding

The previous package protected the canonical report template but did not protect the latest report itself. A narrow latest-report check is a better ratchet than linting every historical report, because it catches current-package omissions without forcing broad historical cleanup.
Reviewer follow-up clarified that the tests also need to protect the latest-only scope, not just latest-report failure. The focused unit suite now covers both latest missing failure and historical-only missing pass.

## Open questions

- Whether to add opt-in semantic checks for the latest report content remains open. This package checks headings only.
- Actual `U1` commitment remains open and user-facing.

## Suggested next prompt

Continue autonomous maintenance: run post-report docs-focused validation, review, commit/push, then reassess whether another safe maintenance package remains.

## Reviewer findings and follow-up

- Reviewer found that the first latest-report test did not prove latest-only scope because the historical fixture was also invalid.
  Follow-up: added `test_main_allows_historical_report_missing_heading_when_latest_is_valid`, which keeps an older invalid report but a valid latest report and expects `validate_docs.main()` to pass.
- No stale report count, dashboard mismatch, or final-public overclaim was found in the first review pass.

## Plan update status

`plan/` 更新不要: this is a scaffold validation guardrail, not a roadmap or repository-memory boundary change.

## progress.md update status

`progress.md` 更新済み: recent log records the latest-report guardrail package.

## tasks.md update status

`tasks.md` 更新済み: current task-level status records the latest-report scaffold guardrail.

## samples_progress.md update status

`samples_progress.md` 更新済み: docs/traceability validation row records the focused unit and docs validation.

## Skipped validations and reasons

- Full sample/Cargo floor was skipped because this package changed only docs scaffold validation, script docs, a focused Python unit test, and snapshot docs. Package `1076` recorded a fresh corrected full validation checkpoint immediately before this report-discipline sequence.
- Post-report docs-focused validation passed after this report was added.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

No implementation sub-agent was spawned because the write set was narrow and local. A reviewer will be used before commit if the diff is non-trivial after post-report validation.
