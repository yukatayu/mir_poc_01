# Report 2183 — Installed-binary helper path portability

- Date: 2026-07-04 13:49 JST
- Author / agent: Codex
- Scope: Product Alpha installed-binary helper path portability
- Decision levels touched: none; maintenance hardening only

## Objective

Fix the remaining repo-root absolute path drift in
`product_alpha1_installed_binary_check.py` output for repo-owned binary and
package inputs.

## Scope and assumptions

Scope:

- inspect active helper output for remaining repo-root absolute path matches
- add regression coverage for default installed-binary binary/package argv
- convert default installed-binary helper argv and top `binary_path` output to
  repo-relative display/execute strings
- rerun the installed-binary probe and update current snapshots

Assumptions:

- The default built binary `target/debug/mirrorea-alpha` and default package
  `samples/product-alpha1/demo` are repo-owned inputs and should be represented
  repo-relatively.
- External output paths such as `/tmp/.../native-bundle/run.sh` remain external
  execution artifacts and are not changed by this package.
- This does not change final CLI/API/ABI, packaging, distribution, semantics,
  or product-readiness claims.

## Start state / dirty state

Package 45 started from clean `HEAD == origin/main == 95c1a247`.

The local output scan found that
`/tmp/mirrorea-alpha1-installed-binary-p44-fixed.json` still contained 7
repo-root absolute matches:

- top-level `binary_path`
- `command_results[*].argv[0]` for the built binary
- `command_results[*].argv[2]` for `samples/product-alpha1/demo`

## Documents consulted

- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/product_alpha1_installed_binary_check.py`
- `scripts/tests/test_product_alpha1_installed_binary_check.py`

## Actions taken

- Scanned Package 44 generated JSON outputs for repo-root absolute matches.
- Selected the installed-binary helper as the smallest evidence-backed fix
  because release-check, operational helper, and minimal pattern outputs already
  had zero repo-root absolute matches.
- Added RED tests for `plan_check_all()` and `check_all()` requiring default
  repo-owned binary/package arguments to be repo-relative.
- Added `repo_cli_arg()` to the installed-binary helper.
- Changed `binary_alpha_args()`, the default package argument, and top-level
  `binary_path` reporting to use repo-relative strings for repo-owned paths.
- Re-ran installed-binary unit tests, docs validator unit tests, and the real
  installed-binary probe.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Spawned a read-only code-mapper sub-agent to look for additional active
  helper path portability candidates.

## Files changed

- `scripts/product_alpha1_installed_binary_check.py`
- `scripts/tests/test_product_alpha1_installed_binary_check.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2183-installed-binary-helper-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `rg -n "str\\([A-Z0-9_]*(ROOT|DIR|WORLD|SAMPLE|DEMO|PATH|PACKAGE|BUNDLE|VIEWER|SOURCE|TARGET)[A-Z0-9_]*\\)|str\\([^)]*\\.resolve\\(\\)|Path\\(__file__\\).*parents|cwd=REPO_ROOT|command_payload|argv" scripts/*.py`
- `rg -n "/home/codex/dev/mir_poc_01|/home/[^/]+/dev/mir_poc_01" README.md Documentation.md progress.md tasks.md samples_progress.md scripts docs/hands_on docs/research_abstract samples/current-l2 samples/clean-near-end samples/product-alpha1 samples/full-system-v1 samples/full-system-v1-surface -g '*.md' -g '*.json'`
- JSON scan of `/tmp/mirrorea-alpha1-release-p44-fixed.json`, `/tmp/mirrorea-alpha1-installed-binary-p44-fixed.json`, `/tmp/mirrorea-operational-product-p44-fixed.json`, and `/tmp/mirrorea-minimal-alpha1-patterns-p44-fixed.json`
- field walk over `/tmp/mirrorea-alpha1-installed-binary-p44-fixed.json`
- `sed -n '1,220p' scripts/product_alpha1_installed_binary_check.py`
- `sed -n '220,380p' scripts/product_alpha1_installed_binary_check.py`
- `sed -n '1,260p' scripts/tests/test_product_alpha1_installed_binary_check.py`
- `rg -n "DEFAULT_PACKAGE|BINARY_PATH|binary_path|argv|str\\(" scripts/product_alpha1_installed_binary_check.py scripts/tests/test_product_alpha1_installed_binary_check.py`
- `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check.ProductAlpha1InstalledBinaryCheckTests.test_plan_commands_record_repo_relative_binary_and_package_args scripts.tests.test_product_alpha1_installed_binary_check.ProductAlpha1InstalledBinaryCheckTests.test_check_all_reports_repo_relative_binary_path_and_argv`
- `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`
- `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-p45-fixed > /tmp/mirrorea-alpha1-installed-binary-p45-fixed.json`
- summary scan of `/tmp/mirrorea-alpha1-installed-binary-p45-fixed.json`
- `date '+%Y-%m-%d %H:%M JST'`
- `df -h .`
- `free -h`
- `du -sh . target .git 2>/dev/null`
- `du -sh target`
- `du -sh .git`
- `test -d .cargo && du -sh .cargo || printf '.cargo missing\n'`
- `test -d .lake && du -sh .lake || printf '.lake missing\n'`
- post-report `python3 scripts/validate_docs.py`
- post-report `python3 scripts/check_source_hierarchy.py`
- post-report `git diff --check`

## Evidence / outputs / test results

- Active docs host-path scan only found historical archived
  `docs/research_abstract/old/...` matches, which are excluded from active
  reader-facing host-path lint.
- Package 44 generated output scan:
  - Product Alpha release-check JSON: repo-root absolute matches 0
  - installed-binary JSON: repo-root absolute matches 7
  - operational product JSON: repo-root absolute matches 0
  - minimal pattern JSON: repo-root absolute matches 0
- RED tests failed as expected:
  - `target/debug/mirrorea-alpha` was absent because argv used
    `/home/.../target/debug/mirrorea-alpha`
  - top-level `binary_path` was `/home/.../target/debug/mirrorea-alpha`
- Focused GREEN tests passed after `repo_cli_arg()`: 2 tests.
- `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
  passed: 7 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `git diff --check` passed before report creation.
- Real installed-binary probe passed:
  - status `accepted`
  - passed commands 11
  - failed commands 0
  - `installed_binary_candidate_ready: true`
  - `public_packaging_candidate: installed_binary_plus_native_host_launch_bundle`
  - top `binary_path: target/debug/mirrorea-alpha`
  - generated JSON repo-root absolute matches 0
  - command argv includes `target/debug/mirrorea-alpha` and
    `samples/product-alpha1/demo`
- Resource state after the real probe:
  - `df -h .`: `/dev/sda2` 188G size, 141G used, 39G available, 79% use.
  - `free -h`: 15Gi total memory, 9.7Gi available; 15Gi swap, 731Mi used.
  - `du -sh .`: 7.0G.
  - `du -sh target`: 6.9G.
  - `du -sh .git`: 36M.
  - `.cargo` and `.lake` are not present under the repo root.
- Post-report `python3 scripts/validate_docs.py` passed and found 1335
  numbered reports.
- Post-report `python3 scripts/check_source_hierarchy.py` passed:
  required/present 659/659.
- Post-report `git diff --check` passed.

## What changed in understanding

The Product Alpha release-check and operational helper portability fixes did
not cover the installed-binary adoption probe. That helper records the built
repo binary and default sample package directly, so it needed the same
repo-relative path treatment.

## Open questions

No blocking questions for this package.

Follow-up candidate from read-only code mapping:

- The next smallest high-value portability package is likely the
  release-check-facing source-first helper trio:
  `scripts/textual_mir_samples.py`, `scripts/full_system_v1_samples.py`, and
  `scripts/surface_mir_samples.py`.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`; if the code-mapper finds a
small active-output portability candidate, promote that as the next package.

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
- updated the Product Alpha status row
- added a recent-log entry for installed-binary helper path portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- added a current holding-state bullet for installed-binary helper
  binary/package argv portability

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the installed-binary adoption probe row
- added a Recent Validation Log row for installed-binary helper path
  portability

## Reviewer findings and follow-up

Focused self-review:

- Confirmed the helper only converts repo-owned binary/package paths.
- Confirmed external output directories and bundle `run.sh` paths remain
  external paths.
- Confirmed the final installed-binary JSON has zero repo-root absolute matches.

Sub-agent review:

- Code-mapper sub-agent `019f2b73-8e91-76f1-96c6-f9302edf9d62` found the
  release/report wrappers mostly clean for repo-owned sample argv.
- It identified likely remaining active-output portability candidates in
  `textual_mir_samples.py`, `surface_mir_samples.py`,
  `full_system_v1_samples.py`, and lower-priority practical alpha /
  computational helpers.
- It recommended the source-first helper trio as the next smallest high-value
  package because those helpers are directly exercised by Full System V1 and
  Surface release checks.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package only changes the Product Alpha installed-binary helper. The
  installed-binary probe itself runs the relevant build and command family, and
  focused Python unit tests cover the changed path behavior.

## Commit / push status

Committed and pushed:

- `015f9d3f Use relative installed binary helper inputs`

This report section was updated after the first push and will be captured by a
report-only follow-up commit.

## Sub-agent session close status

Closed:

- `019f2b73-8e91-76f1-96c6-f9302edf9d62`
