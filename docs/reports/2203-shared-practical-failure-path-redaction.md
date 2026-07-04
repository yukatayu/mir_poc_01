# Report 2203 — Shared practical failure-path redaction

- Date: 2026-07-04 17:12 JST
- Author / agent: Codex
- Scope: Practical helper failure-surface path portability
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Remove repo-checkout-specific absolute path leaks from practical alpha helper
failure surfaces by sharing one display helper, while preserving external and
temporary absolute paths that represent true external boundaries.

## Scope and assumptions

- Scope is limited to practical alpha Python helper failure display surfaces,
  focused tests, `scripts/README.md`, and status/report snapshots.
- Repo-owned absolute path prefixes should be displayed relatively.
- External paths such as `/tmp/...` should remain absolute.
- Success-path argv and JSON portability was already handled in prior packages;
  this package targets failure text and `failed[].error` serialization.
- Docker bind mount environment values remain host absolute where Docker
  Compose needs them.
- This is maintenance hardening only. It does not change sample status,
  workflow status, semantics, ABI, public API, proof status, or canon status.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `aba65cac1193e5a22914163005c8266d90891d69`
- Start state: clean and matched `origin/main`.

## Documents consulted

- `AGENTS.md` instructions supplied in the task context
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `docs/reports/TEMPLATE.md`
- `docs/reports/2202-source-hierarchy-status-root-display-portability.md`
- `scripts/README.md`
- Practical alpha helpers and focused tests under `scripts/` and
  `scripts/tests/`

## Actions taken

- Added `scripts/practical_alpha_error_display.py` with repo-owned path display
  redaction helpers and a boundary guard for repo-root lookalike prefixes.
- Hardened the helper so external paths containing the checkout path as a later
  path component are preserved rather than partially rewritten.
- Routed practical helper `check_all()` `failed[].error` fields through the
  shared redaction helper for checker, run-local, attach, avatar, save/load,
  transport, alpha-0.5, alpha-0.8, alpha-0.9, export-devtools,
  product-preview, and integrated-workflow helpers.
- Routed direct non-JSON subprocess stdout failure messages through the shared
  redaction helper for checker, run-local, attach, avatar, and save/load.
- Routed save/load invalid distributed cut guard errors and product-preview
  viewer HTML errors through the same helper.
- Hardened practical alpha transport direct failure text for missing transport
  surface, non-JSON local transport stdout, Docker availability stderr/stdout,
  and Docker Compose run stderr/stdout.
- Added focused unit tests for shared redaction behavior, transport direct
  failure paths, and representative `check_all()` failure serialization.
- Updated `scripts/README.md` to record the helper responsibility.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.

## Files changed

- `scripts/practical_alpha_error_display.py`
- `scripts/practical_alpha05_session.py`
- `scripts/practical_alpha08_session_hotplug.py`
- `scripts/practical_alpha09_devtools.py`
- `scripts/practical_alpha1_attach.py`
- `scripts/practical_alpha1_avatar.py`
- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_export_devtools.py`
- `scripts/practical_alpha1_integrated_workflow.py`
- `scripts/practical_alpha1_product_preview.py`
- `scripts/practical_alpha1_run_local.py`
- `scripts/practical_alpha1_save_load.py`
- `scripts/practical_alpha1_transport.py`
- `scripts/tests/test_practical_alpha_error_display.py`
- `scripts/tests/test_practical_alpha_failure_redaction.py`
- `scripts/tests/test_practical_alpha1_export_devtools.py`
- `scripts/tests/test_practical_alpha1_integrated_workflow.py`
- `scripts/tests/test_practical_alpha1_product_preview.py`
- `scripts/tests/test_practical_alpha1_transport.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2203-shared-practical-failure-path-redaction.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 -m unittest scripts.tests.test_practical_alpha_error_display scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_practical_alpha1_integrated_workflow
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_avatar scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha05_session scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_practical_alpha1_integrated_workflow scripts.tests.test_practical_alpha_error_display
python3 -m unittest scripts.tests.test_practical_alpha_error_display scripts.tests.test_practical_alpha_failure_redaction scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_practical_alpha1_integrated_workflow
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_avatar scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha05_session scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_practical_alpha1_integrated_workflow scripts.tests.test_practical_alpha_error_display scripts.tests.test_practical_alpha_failure_redaction
python3 -m unittest scripts.tests.test_practical_alpha_error_display scripts.tests.test_practical_alpha_failure_redaction
rg -n "failed\.append\(\{\"sample_id\": sample_id, \"error\": str\(error\)\}\)|guard_error = str\(error\)|html_error = str\(error\)|practical transport command did not return JSON for \{package_path\}|missing alpha_local_transport_input\.transport_surface in \{package_path\}" scripts/practical_alpha*.py || true
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_avatar.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
# reran the 12 helper check-all commands into /tmp/mirrorea-practical-failure-redaction.aohbeu and scanned for the checkout path
python3 - <<'PY'
from scripts import practical_alpha1_check, practical_alpha1_run_local, practical_alpha1_attach, practical_alpha1_avatar, practical_alpha1_transport
from scripts import practical_alpha1_save_load, practical_alpha05_session, practical_alpha08_session_hotplug, practical_alpha09_devtools
from scripts import practical_alpha1_export_devtools, practical_alpha1_product_preview, practical_alpha1_integrated_workflow
print('package imports ok')
PY
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py --format json
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_check_source_hierarchy
python3 -m unittest scripts.tests.test_validate_docs
```

## Evidence / outputs / test results

- Focused new/regression tests:
  - 51 tests passed for shared redaction, direct failure redaction, transport,
    export-devtools, product-preview, and integrated-workflow focused coverage.
  - 122 tests passed for the touched practical helper suite.
- Direct non-JSON failure tests now cover checker, run-local, attach, avatar,
  and save/load helpers.
- Package-style import smoke passed for the 12 changed practical helper modules.
- Pattern scan for old raw failure serialization found no remaining matches for
  `failed[].error = str(error)`, save/load `guard_error = str(error)`,
  product-preview `html_error = str(error)`, or old raw transport error text.
- Real helper validation:
  - `practical_alpha1_transport.py check-all`: 7 passed, 0 failed, Docker row
    complete, `stage_pa1_5_complete: true`.
  - 12 practical helper `check-all` outputs together passed 86 rows, failed 0.
  - The captured helper outputs under
    `/tmp/mirrorea-practical-failure-redaction.aohbeu` had repo-root absolute
    matches `0`.
- Docs/source validation:
  - `python3 scripts/validate_docs.py`: documentation scaffold complete.
  - `python3 scripts/check_source_hierarchy.py --format json`: required 659,
    present 659, missing 0.
  - `python3 -m unittest scripts.tests.test_validate_docs`: 37 tests passed.
- One attempted validation command failed because
  `scripts.tests.test_check_source_hierarchy` does not exist. The standalone
  `check_source_hierarchy.py --format json` command is the relevant validation
  for that surface and passed; the valid docs unit was rerun separately.

## What changed in understanding

The remaining practical path leak was not a success-path argv issue. It was a
shared failure-display issue: several helper `check_all()` implementations
serialized child exceptions verbatim, and transport had direct JSON/Docker
failure text that could carry repo-owned absolute paths. A shared display helper
keeps the policy consistent without hiding external `/tmp` boundaries.

## Open questions

- No semantic or user-choice question blocks this maintenance package.
- The known high-confidence broader path-portability candidate set is closed
  for now. Future path portability work should reopen from new scan evidence or
  a touched surface, not from this package's closed candidate list.

## Suggested next prompt

Continue autonomous maintenance by selecting the next candidate from `tasks.md`,
or reopen path portability only if a fresh scan finds a concrete leak.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 17:12 JST shared practical
failure-surface path redaction log.

## tasks.md update status

`tasks.md` 更新済み: marked the known high-confidence broader path-portability
candidate set closed for now.

## samples_progress.md update status

`samples_progress.md` 更新済み: updated practical alpha rows to mention
repo-relative failure display and added a recent validation log row.

## Reviewer findings and follow-up

Code-mapper sub-agent `019f2c15-2977-7391-a56c-07cef61cd9ee` identified the
primary direct transport failure leaks and the broader `check_all()`
`str(error)` failure serialization pattern across practical helpers. The patch
follows that map.

Reviewer sub-agent `019f2c27-5910-7520-af3a-86d3a35b354c` found four issues:
direct checker/run-local/attach/avatar/save-load non-JSON stdout failures still
used raw stdout, the first redaction helper draft could rewrite external paths
that merely contained the checkout path as a later component, status docs
claimed closure before those gaps were fixed, and package-style imports could
regress for simple helpers. Follow-up fixed all four: direct failure messages
now use `repo_display_text`, the redaction helper is token-boundary guarded,
focused tests cover external lookalikes and direct non-JSON failures, package
import smoke covers all 12 changed helpers, and the closure/status claim is now
backed by that evidence.

## Skipped validations and reasons

- Full workspace Cargo checks were not rerun because this package changes
  Python helper failure serialization only; practical helper unit tests, real
  helper `check-all` commands, and docs/source validators cover the touched
  surfaces.
- Product Alpha / Full System V1 / Surface release-checks were not rerun
  because this package does not touch those helpers.

## Commit / push status

- Implementation / snapshot / report commit:
  `5641a1bf Redact practical helper failure paths`
- Push status: pushed to `origin/main`.
- Follow-up report-status metadata update is pending at this line before its
  own commit.

## Sub-agent session close status

Code-mapper sub-agent `019f2c15-2977-7391-a56c-07cef61cd9ee` completed and was
closed after its result was mirrored into this report.

Reviewer sub-agent `019f2c27-5910-7520-af3a-86d3a35b354c` completed and was
closed; follow-up fixes were implemented and verified.
