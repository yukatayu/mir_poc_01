# Report 2173 — Snapshot last-updated validator guard

- Date: 2026-07-04 12:38 JST
- Author / agent: Codex
- Scope: docs validator maintenance after active path-portability package
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Add a narrow validator guard that prevents `progress.md` / `tasks.md` top
`最終更新` headers from going missing or lagging behind newer timestamped
entries in the same snapshot document.

## Scope and assumptions

Scope is limited to current snapshot freshness checks in `scripts/validate_docs.py`.

Working assumptions:

- `progress.md` and `tasks.md` are current snapshots, not append-only logs.
- A missing top header or a header older than a body timestamp is almost always
  stale maintenance evidence, not a semantic project decision.
- The guard should not parse arbitrary dates; it only compares existing
  `YYYY-MM-DD HH:MM JST` strings, whose format sorts lexicographically.

## Start state / dirty state

Package 35 started after Package 34 was committed and pushed.

Start state:

- `HEAD == origin/main == f81b2c23`
- working tree clean

Package 34 had exposed a concrete maintenance miss: `progress.md` body text was
updated before its top `最終更新` header was corrected.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Added RED/GREEN tests for stale and current snapshot `最終更新` headers, plus
  `tasks.md` and missing-top-header regression cases for the same guard.
- Added `SNAPSHOT_LAST_UPDATED_FILES`, timestamp regexes, and
  `stale_snapshot_last_updated_headers()` to `scripts/validate_docs.py`.
- After review, narrowed the header extractor to the top snapshot header
  position: either the first non-empty line, or the first non-empty line after
  an H1 title.
- Added a validator failure branch that reports stale snapshot headers before
  report-template validation.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` to record this
  maintenance guard.

## Files changed

- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2173-snapshot-last-updated-validator-guard.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_stale_snapshot_last_updated_header scripts.tests.test_validate_docs.ValidateDocsTests.test_main_allows_current_snapshot_last_updated_header` (RED)
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_stale_snapshot_last_updated_header scripts.tests.test_validate_docs.ValidateDocsTests.test_main_allows_current_snapshot_last_updated_header` (GREEN)
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_missing_top_snapshot_last_updated_header` (RED after review)
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_stale_snapshot_last_updated_header scripts.tests.test_validate_docs.ValidateDocsTests.test_main_allows_current_snapshot_last_updated_header scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_stale_tasks_last_updated_header scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_missing_top_snapshot_last_updated_header` (GREEN after review)
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 -m py_compile scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `python3 -m unittest discover -s scripts/tests`
- `date '+%Y-%m-%d %H:%M %Z'`

## Evidence / outputs / test results

- RED: stale snapshot header test failed before implementation because
  `validate_docs.main()` returned `0` instead of rejecting the stale header.
- GREEN: the targeted timestamp tests passed after implementation.
- Reviewer found that the first implementation could accept a body-level
  `最終更新:` line as the header. A missing-top-header regression test failed
  before the fix and passed after top-header extraction was narrowed.
- `python3 scripts/validate_docs.py` passed and found 1325 numbered reports.
- `python3 -m unittest scripts.tests.test_validate_docs` passed 31 tests.
- `python3 scripts/check_source_hierarchy.py` passed with required/present
  602/602.
- `python3 -m unittest discover -s scripts/tests` passed 680 tests.
- `git diff --check` passed.

## What changed in understanding

Snapshot freshness can drift even in a well-maintained package when the body is
updated before the top header. Treating that as validator-backed maintenance is
lower risk than relying on manual report review.

## Open questions

- Whether `samples_progress.md` should get a similar top-level freshness header
  remains open. It currently uses a table-first dashboard shape and has no
  matching `最終更新` header to validate.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`, preferring narrow guards that
protect active snapshot / reader-facing docs without rewriting historical
evidence.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, or repository-memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for the snapshot timestamp guard and updated the
  top `最終更新` timestamp.

## tasks.md update status

`tasks.md` 更新済み:

- Updated the docs freshness audit row to include `progress.md` / `tasks.md`
  `最終更新` header freshness.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a maintenance validation log row. Sample workflow status did not change.

## Reviewer findings and follow-up

Reviewer sub-agent findings and follow-up:

- Medium: `LAST_UPDATED_PATTERN.search(text)` could treat a body-level
  `最終更新:` line as the header. Fixed by adding a missing-top-header
  regression test and extracting only the first non-empty line after an H1
  title, or the first non-empty line if no H1 title exists.
- Low: the report validation evidence was stale after adding this report and
  another test. Updated the report to `1325` reports and `31` tests.

## Skipped validations and reasons

No skipped validations at this checkpoint. Broader Cargo tests are not relevant
because this package only changes Python docs validation and snapshot docs.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Reviewer sub-agent completed, findings were processed, and the session was
closed after the final local validation pass.
