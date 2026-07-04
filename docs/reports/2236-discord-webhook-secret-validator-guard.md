# Report 2236 - Discord webhook secret validator guard

- Date: 2026-07-05 02:18 JST
- Author / agent: Codex
- Scope: Macro 0 docs-validator secret leak guard hardening
- Decision levels touched: LAB repository memory only

## Objective

Prevent concrete Discord webhook URLs from being committed into tracked
repository docs/source, while ensuring validation output reports only
path / line and never prints the secret URL body.

## Scope and assumptions

This package is limited to a concrete Discord webhook URL-shape guard in
`scripts/validate_docs.py`. It does not change Discord notification behavior,
local webhook configuration storage, or general secret scanning policy.

## Start state / dirty state

Start state was clean and synced on `main` at
`602208c8844b09928105faf208644da9f401b5ec`.

At report creation, the worktree contains the intentional P98 edits to
validator tests, validator code, repository memory, snapshot docs, and this
report.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Added a RED test with a fake concrete Discord webhook URL assembled from
  string pieces so the test source itself does not contain a URL-shaped
  credential.
- Confirmed the RED failure: `validate_docs.main()` returned `0` before the
  guard existed.
- Added a concrete Discord webhook URL-shape detector to `scripts/validate_docs.py`.
- Made the failure output secret-safe by reporting only relative path, line
  number, and the label `concrete Discord webhook URL`.
- Fixed reviewer-found ordering and candidate coverage gaps: the secret guard
  now runs before full-line echoing lints, and candidate files include Git
  cached/other files plus required docs and numbered reports.
- Added `plan/151-discord-webhook-secret-validator-guard.md`.
- Registered `plan/151` in docs/source hierarchy validators and test fixtures.
- Updated `README.md`, `Documentation.md`, `scripts/README.md`,
  `plan/00-index.md`, `plan/90-source-traceability.md`, `progress.md`, and
  `tasks.md`.

## Files changed

- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/README.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/151-discord-webhook-secret-validator-guard.md`
- `docs/reports/2236-discord-webhook-secret-validator-guard.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_concrete_discord_webhook_without_printing_secret`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_webhook_before_line_echoing_lints scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_untracked_report_webhook_when_git_scan_succeeds`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_webhook_before_line_echoing_lints scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_untracked_report_webhook_when_git_scan_succeeds scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_concrete_discord_webhook_without_printing_secret`
- `python3 scripts/validate_docs.py`
- Workspace concrete webhook scan with the local denylist pattern omitted from
  this report.
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `python3 -m unittest discover -s scripts/tests`
- `make check`
- `cargo fmt --check`
- `cargo test --workspace --all-targets --no-fail-fast`

## Evidence / outputs / test results

RED result:

```text
FAIL: test_main_rejects_concrete_discord_webhook_without_printing_secret
AssertionError: 0 != 1
```

GREEN result:

```text
Ran 1 test in 0.137s
OK
```

Reviewer regression RED result:

```text
FAIL: test_main_rejects_webhook_before_line_echoing_lints
AssertionError: 'Tracked files contain concrete Discord webhook URLs' not found

FAIL: test_main_rejects_untracked_report_webhook_when_git_scan_succeeds
AssertionError: 0 != 1
```

Reviewer regression GREEN result:

```text
Ran 3 tests in 0.287s
OK
```

Initial real-repo check after the implementation:

- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1387
  numbered reports.
- Workspace concrete webhook scan: no concrete webhook URL or configured token
  fragments found.

Focused and full validation after reviewer fixes:

- `python3 -m unittest scripts.tests.test_validate_docs`: 42 tests, OK.
- `python3 scripts/check_source_hierarchy.py`: required 691, present 691,
  missing 0.
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1388
  numbered reports.
- `git diff --check`: exit 0.
- Workspace concrete webhook scan: no concrete webhook URL or configured token
  fragments found.
- `python3 -m unittest discover -s scripts/tests`: 790 tests, OK.
- `make check`: source hierarchy check, docs validation, and `cargo check`
  passed.
- `cargo fmt --check`: exit 0.
- `cargo test --workspace --all-targets --no-fail-fast`: exit 0.

Review completed. Commit and push are still pending before the primary package
commit.

## What changed in understanding

The repository already keeps the live webhook configuration local, but a report
or docs edit can still accidentally paste a concrete webhook URL. The new guard
is intentionally narrow and secret-safe: it catches concrete URL-shaped
Discord webhook credentials and does not print the matched URL.

## Open questions

Whether to add broader generic secret scanning is still separate. This package
only covers concrete Discord webhook URL shape.

## Suggested next prompt

Continue the autonomous run with the next coherent Macro 0 or G1 maintenance
package after this guard is validated, reviewed, committed, and pushed.

## Plan update status

`plan/` updated: added `plan/151-discord-webhook-secret-validator-guard.md`,
updated `plan/00-index.md`, and updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` updated: added the `plan/151` guard note.

## progress.md update status

`progress.md` updated: added the Discord webhook secret validator guard note,
Macro 0 row update, and 2026-07-05 02:18 JST recent log entry.

## tasks.md update status

`tasks.md` updated: added the `plan/151` guard note, maintenance refresh row,
and Macro 0 row update.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample, validation
command, debug surface, or sample blocker changed.

## Reviewer findings and follow-up

Read-only reviewer sub-agent `019f2e26-fc2a-7fd2-bf75-6530943e4dda` reported
one Critical issue, one Important issue, and draft-status Minor issues.

Critical finding: the new secret guard originally ran after source-hierarchy
and active host-path lints that print full offending lines. If a line contained
both a concrete webhook URL and an earlier lint hit, the validator could print
the URL. Follow-up: added a RED test for this case, then moved the secret guard
before line-echoing lints.

Important finding: the candidate scan used tracked files only when Git
succeeded, so an untracked new report could be missed before staging.
Follow-up: added a RED test with a mocked Git scan result, then widened the
candidate set to Git cached/other files plus required docs and numbered
reports.

Minor finding: the report start hash was stale and draft review/commit status
needed refresh. Follow-up: corrected the start hash and updated this section.

## Skipped validations and reasons

No relevant local validations were skipped for this docs-validator package.

## Commit / push status

Pending before the primary package commit.

## Sub-agent session close status

Reviewer sub-agent `019f2e26-fc2a-7fd2-bf75-6530943e4dda` completed. A fresh
close attempt after the context transition reported that the agent ID was not
found, so there is no open reviewer session visible in the current tool
context.
