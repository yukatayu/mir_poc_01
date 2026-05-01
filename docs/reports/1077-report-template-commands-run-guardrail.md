# Report 1077 — report template Commands run guardrail

- Date: 2026-05-01 11:13 JST
- Author / agent: Codex
- Scope: report discipline / docs scaffold validation
- Decision levels touched: none; process guardrail only

## Objective

Reduce template drift around the required `Commands run` section by adding the section to the report template and to the `validate_docs.py` required template heading list.

## Scope and assumptions

- Scope is limited to the report template, docs scaffold validator, focused unit coverage, snapshot mirrors, and this report.
- This does not retroactively rewrite historical reports.
- `validate_docs.py` still validates the documentation scaffold and report template headings; it does not become a semantic linter for every historical report.
- This guardrail prevents the canonical template from losing `Commands run`; it does not prove every numbered report contains complete command history.
- Stop line: this package does not change normative specs, sample semantics, final public API, packaging, or product shape.

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
- `docs/reports/TEMPLATE.md`
- `scripts/tests/`
- reviewer findings from package `1076`

## Actions taken

- Used `superpowers:test-driven-development` because this package changes validator/template behavior.
- Added `scripts/tests/test_validate_docs.py` asserting that `## Commands run` is required by `validate_docs.py` and present in `docs/reports/TEMPLATE.md`.
- Ran the focused unit test before implementation and confirmed it failed for the expected reason.
- Added `## Commands run` to `scripts/validate_docs.py` `REQUIRED_TEMPLATE_HEADINGS`.
- Added `## Commands run` to `docs/reports/TEMPLATE.md`.
- Added behavior-level coverage that runs `validate_docs.main()` against a temporary scaffold whose template omits `## Commands run` and confirms validator failure.
- Re-ran the focused unit tests and docs scaffold validation successfully.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` with the maintenance checkpoint.

## Files changed

- `docs/reports/TEMPLATE.md`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1077-report-template-commands-run-guardrail.md`

## Commands run

```bash
git status --short
git log -1 --oneline
python3 -m pytest --version
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
git diff --stat
```

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git log -1 --oneline
6bd4ac1 Record corrected full validation checkpoint
```

Test runner probe:

```text
$ python3 -m pytest --version
/usr/bin/python3: No module named pytest
```

The repo-local script tests are standard `unittest` tests, so focused validation used `python3 -m unittest`.

TDD RED:

```text
$ python3 -m unittest scripts.tests.test_validate_docs
F
...
AssertionError: '## Commands run' not found in ['## Objective', ...]
FAILED (failures=1)
```

Implementation:

```text
docs/reports/TEMPLATE.md now contains:
## Commands run

scripts/validate_docs.py REQUIRED_TEMPLATE_HEADINGS now contains:
"## Commands run",
```

TDD GREEN and focused docs validation:

```text
$ python3 -m unittest scripts.tests.test_validate_docs
.
----------------------------------------------------------------------
Ran 1 test in 0.000s

OK

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1074 numbered report(s).
```

Behavior-level test added after reviewer follow-up:

```text
$ python3 -m unittest scripts.tests.test_validate_docs
..
----------------------------------------------------------------------
Ran 2 tests in 0.004s

OK
```

Post-report docs-focused validation:

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
Found 1075 numbered report(s).

$ git diff --check
<no output>
```

## What changed in understanding

The report template itself lacked a `Commands run` section even though `AGENTS.md` requires commands in non-trivial reports. The appropriate narrow fix is to guard the template and validator heading list, not to reinterpret historical reports or turn `validate_docs.py` into a semantic report linter.
Reviewer follow-up clarified that this should be described as template-drift prevention, not proof that all actual numbered reports contain complete command histories.

## Open questions

- Whether future maintenance should add opt-in validation for the newest report's mandatory sections remains open; doing so across all historical reports would be too broad for this package.
- Actual `U1` commitment remains open and user-facing.

## Suggested next prompt

Continue autonomous maintenance: run docs-focused validation, review the diff for overclaim, commit/push, and then reassess the next safe maintenance package.

## Reviewer findings and follow-up

- `test_validate_docs.py` initially checked only literal synchronization, not validator behavior.
  Follow-up: added a temporary-scaffold test that calls `validate_docs.main()` and confirms a template missing `## Commands run` fails.
- The first draft overstated the scope as preventing all reports from missing `Commands run`.
  Follow-up: wording now says template drift guardrail and explicitly excludes semantic linting of all numbered reports.
- The report `Commands run` list omitted `git status --short` and `git log -1 --oneline`.
  Follow-up: both commands were added.
- The report needed clearer user-decision blocker coverage.
  Follow-up: the open questions section and the checklist below explicitly retain actual `U1` as the only product-shaping blocker.

## Remaining user decision blockers

- Actual `U1` commitment remains open: packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.
- This package does not change or resolve final public parser/API/ABI, production transport, rollback, durable migration, distributed ordering, final viewer/verifier, packaging, or host integration blockers.

## Plan update status

`plan/` 更新不要: process guardrail changed only; no roadmap or repository-memory boundary changed.

## progress.md update status

`progress.md` 更新済み: recent log records the guardrail package.

## tasks.md update status

`tasks.md` 更新済み: current task-level status records the report template compliance guardrail.

## samples_progress.md update status

`samples_progress.md` 更新済み: docs/traceability validation row records the focused unit and docs validation.

## Skipped validations and reasons

- Full sample/Cargo floor was skipped because this package changed only docs scaffold validation, report template structure, and a focused Python unit test. Package `1076` recorded a fresh corrected full validation checkpoint immediately before this package.
- `pytest` was not used because it is not installed in this environment; the repo-local script tests are `unittest` based and the focused `unittest` command passed.
- Post-report docs-focused validation passed after this report was added.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

No sub-agent was spawned for implementation because the write set and validation target were narrow and local. Package `1076` reviewer findings were used as input and that reviewer session was already closed.
