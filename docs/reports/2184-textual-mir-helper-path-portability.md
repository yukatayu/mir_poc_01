# Report 2184 — Textual Mir helper path portability

- Date: 2026-07-04 14:00 JST
- Author / agent: Codex
- Scope: Full System V1 textual Mir helper raw payload portability
- Decision levels touched: none; maintenance hardening only

## Objective

Normalize repo-owned paths embedded in `textual_mir_samples.py` raw parser
payloads so Full System V1 parser-helper evidence is portable across checkout
locations.

## Scope and assumptions

Scope:

- reproduce active JSON output drift in `scripts/textual_mir_samples.py`
- add regression tests for raw positive source path and negative diagnostic
  message path portability
- normalize repo-root prefixes in raw parser payload strings
- rerun textual helper and Full System V1 release-check validation
- update current snapshots and report

Assumptions:

- Raw parser payloads are evidence embedded by the helper, so repo-owned
  `source_path` and diagnostic path text should use repo-relative `samples/...`
  strings.
- Semantic module names such as `Computational.AddOne` and import module names
  such as `Missing.Module` are not filesystem paths and should not be changed.
- This package does not change parser semantics, sample status, workflow
  readiness, final grammar/API status, or ABI claims.

## Start state / dirty state

Package 46 started from clean `HEAD == origin/main == 013b4b8e`.

## Documents consulted

- `AGENTS.md`
- `scripts/textual_mir_samples.py`
- `scripts/tests/test_textual_mir_samples.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2183-installed-binary-helper-path-portability.md`

## Actions taken

- Ran a single textual Mir sample helper command and scanned its JSON output.
- Confirmed `raw_parse_report.source_path` still contained a repo-root absolute
  path.
- Added a RED test for positive raw parser `source_path` portability.
- Added a RED test for negative unresolved-import diagnostic message path
  portability after the first fix exposed the second path source.
- Added a recursive `_repo_relative_payload()` normalizer for raw parser
  payloads before embedding them in helper JSON.
- Re-ran textual helper tests, textual `check-all`, docs validators, source
  hierarchy check, and Full System V1 release-check.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/textual_mir_samples.py`
- `scripts/tests/test_textual_mir_samples.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2184-textual-mir-helper-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `sed -n '1,220p' scripts/textual_mir_samples.py`
- `sed -n '220,430p' scripts/textual_mir_samples.py`
- `sed -n '1,260p' scripts/tests/test_textual_mir_samples.py`
- `python3 scripts/textual_mir_samples.py run mir-01-add-one-positive --format json > /tmp/mirrorea-textual-mir-p46-before.json`
- JSON scan of `/tmp/mirrorea-textual-mir-p46-before.json`
- `python3 -m unittest scripts.tests.test_textual_mir_samples.TextualMirSamplesTests.test_raw_parse_report_source_path_is_repo_relative`
- `python3 scripts/textual_mir_samples.py check-all --format json > /tmp/mirrorea-textual-mir-p46-fixed.json`
- JSON scan of `/tmp/mirrorea-textual-mir-p46-fixed.json`
- `python3 -m unittest scripts.tests.test_textual_mir_samples.TextualMirSamplesTests.test_negative_raw_parse_report_diagnostics_are_repo_relative`
- `python3 -m unittest scripts.tests.test_textual_mir_samples.TextualMirSamplesTests.test_raw_parse_report_source_path_is_repo_relative scripts.tests.test_textual_mir_samples.TextualMirSamplesTests.test_negative_raw_parse_report_diagnostics_are_repo_relative`
- `python3 -m unittest scripts.tests.test_textual_mir_samples`
- `python3 scripts/textual_mir_samples.py check-all --format json > /tmp/mirrorea-textual-mir-p46-fixed2.json`
- `git diff --check`
- JSON scan of `/tmp/mirrorea-textual-mir-p46-fixed2.json`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-p46 > /tmp/mirrorea-full-v1-release-p46.json`
- JSON summary scan of `/tmp/mirrorea-full-v1-release-p46.json`
- `date '+%Y-%m-%d %H:%M JST'`
- post-report `python3 scripts/validate_docs.py`
- post-report `python3 scripts/check_source_hierarchy.py`
- post-report `git diff --check`

## Evidence / outputs / test results

- Initial single-sample JSON scan:
  - sample accepted `true`
  - repo-root absolute matches 1
  - match: `raw_parse_report.source_path`
- First RED test failed as expected because `raw_parse_report.source_path` was
  `/home/.../samples/full-system-v1/computational/add-one-positive/src/add-one.mir`.
- After the first source-path-only fix, `textual_mir_samples.py check-all`
  passed but still had one repo-root absolute match in the unresolved-import
  diagnostic message.
- Second RED test failed as expected because the negative diagnostic message
  still contained `/home/.../samples/full-system-v1/...`.
- Focused GREEN tests passed: 2 tests.
- `python3 -m unittest scripts.tests.test_textual_mir_samples` passed: 7 tests.
- `python3 scripts/textual_mir_samples.py check-all --format json` passed:
  - failed `[]`
  - passed rows 10
  - repo-root absolute matches 0
  - negative diagnostic message uses
    `samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir`
- `git diff --check` passed before report creation.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1335 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- Full System V1 release-check passed:
  - status `accepted`
  - passed commands 29
  - failed commands 0
  - planned commands 29
  - command results 29
  - repo-root absolute matches 0
- Post-report `python3 scripts/validate_docs.py` passed and found 1336
  numbered reports.
- Post-report `python3 scripts/check_source_hierarchy.py` passed:
  required/present 659/659.
- Post-report `git diff --check` passed.

## What changed in understanding

`textual_mir_samples.py` already exposed its primary `source` field
repo-relatively, but it embedded the parser example's raw payload unchanged.
That meant raw `source_path` and diagnostic messages could still leak the host
checkout path even though the release-check wrapper itself was portable.

## Open questions

No blocking questions for this package.

Remaining follow-up candidates:

- `scripts/full_system_v1_samples.py` typed/runtime payload path fields.
- `scripts/surface_mir_samples.py` Surface raw payload path fields.

## Suggested next prompt

Continue autonomous maintenance by fixing the next source-first helper
portability candidate, likely `full_system_v1_samples.py` or
`surface_mir_samples.py`.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, open-question, source-traceability, or repository
  memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- No reader-facing document entry point was added or removed.

## progress.md update status

Updated:

- advanced the top `最終更新` timestamp
- updated the textual Mir source feature row
- added a recent-log entry for textual Mir helper raw payload portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- added current holding-state text for textual Mir helper portability and the
  remaining source-first helper candidates

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the Full System V1 roadmap row
- updated the Full System V1 computational sample line row
- added a Recent Validation Log row for textual Mir helper portability

## Reviewer findings and follow-up

Focused self-review:

- Confirmed the new normalizer only changes repo-root string prefixes in raw
  parser payload values.
- Confirmed semantic module/import names are preserved.
- Confirmed textual helper and Full System V1 release-check generated JSON now
  have zero repo-root absolute matches.

No new sub-agent was opened for this narrow package; it follows the prior
code-mapper recommendation from Package 45.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper and its tests. The relevant helper
  test suite, textual helper `check-all`, docs validators, source hierarchy
  check, and Full System V1 release-check were rerun.

## Commit / push status

Pending at initial report creation.

## Sub-agent session close status

No new sub-agent session was opened for this package.
