# Report 2182 — Operational product helper path portability

- Date: 2026-07-04 13:42 JST
- Author / agent: Codex
- Scope: Product Alpha / operational helper compatibility and path portability
- Decision levels touched: none; maintenance hardening only

## Objective

Audit Product Alpha compatibility anchors after docs-validator maintenance and
fix the operational product helper so repo-owned nested command argv do not
depend on the host checkout path.

## Scope and assumptions

Scope:

- run Product Alpha release / installed-binary / operational / minimal-pattern
  compatibility checks
- fix `scripts/operational_product_samples.py` nested `mirrorea-alpha` argv for
  repo-owned operational roots and layer package inputs
- add regression coverage for repo-relative operational release-check argv
- update current snapshots and report

Assumptions:

- Repo-owned sample inputs should be represented as repo-relative `samples/...`
  arguments in transient helper reports.
- Temporary output directories under `/tmp` are disposable execution outputs and
  are not changed by this package.
- This does not change sample status, workflow readiness, semantics, ABI, final
  product status, or broader distribution scope.

## Start state / dirty state

Package 44 started from clean `HEAD == origin/main == 988dc373`.

During the initial validation audit, `operational_product_samples.py check-all`
passed but its nested `release_check.commands[].argv` still contained
repo-root absolute paths such as the operational sample roots. That drift became
the implementation target for this package.

## Documents consulted

- `AGENTS.md`
- `tasks.md`
- `progress.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/operational_product_samples.py`
- `scripts/tests/test_operational_product_samples.py`

## Actions taken

- Checked disk / memory state before running compatibility validation.
- Ran Product Alpha release-check and installed-binary probes.
- Ran operational product sample `check-all` and found repo-root absolute paths
  in nested helper command argv.
- Added a RED regression test that calls `release_check()` with mocked command
  execution and rejects repo-root absolute argv for repo-owned operational
  inputs.
- Added `repo_cli_arg()` and used it for operational roots and layer package
  inputs passed into nested `mirrorea-alpha` commands.
- Re-ran operational helper unit tests and the real operational `check-all`.
- Re-ran Product Alpha release-check, installed-binary probe, and minimal
  alpha-1 pattern verifier.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Used one sub-agent for an independent minimal alpha-1 pattern verifier check;
  the main agent also reran that command after the fix.

## Files changed

- `scripts/operational_product_samples.py`
- `scripts/tests/test_operational_product_samples.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2182-operational-product-helper-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/validate_docs.py --help`
- `df -h .`
- `free -h`
- `du -sh . target .git .cargo .lake`
- `du -sh target`
- `du -sh .git`
- `du -sh .`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p44 > /tmp/mirrorea-alpha1-release-p44.json`
- `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-p44 > /tmp/mirrorea-alpha1-installed-binary-p44.json`
- `python3 scripts/operational_product_samples.py check-all --format json > /tmp/mirrorea-operational-product-p44.json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json` (sub-agent)
- `rg -n "ROOT|resolve\\(|cargo|samples/product-alpha1/operational|release_check|argv|Path\\(" scripts/operational_product_samples.py scripts/tests/test_operational_product_samples.py`
- `python3 -m unittest scripts.tests.test_operational_product_samples.OperationalProductSamplesTests.test_release_check_records_repo_relative_operational_argv`
- `python3 -m unittest scripts.tests.test_operational_product_samples`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`
- `python3 scripts/operational_product_samples.py check-all --format json > /tmp/mirrorea-operational-product-p44-fixed.json`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p44-fixed > /tmp/mirrorea-alpha1-release-p44-fixed.json`
- `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-p44-fixed > /tmp/mirrorea-alpha1-installed-binary-p44-fixed.json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json > /tmp/mirrorea-minimal-alpha1-patterns-p44-fixed.json`
- `date '+%Y-%m-%d %H:%M JST'`
- post-report `python3 scripts/validate_docs.py`
- post-report `python3 scripts/check_source_hierarchy.py`
- post-report `git diff --check`

## Evidence / outputs / test results

- Resource state before validation:
  - `df -h .`: `/dev/sda2` 188G size, 139G used, 40G available, 78% use.
  - `free -h`: 15Gi total memory, 9.6Gi available; 15Gi swap, 661Mi used.
  - `du -sh .`: 7.0G.
  - `du -sh target`: 6.9G.
  - `du -sh .git`: 36M.
  - `.cargo` and `.lake` are not present under the repo root.
- Initial Product Alpha release-check passed:
  - status `accepted`
  - passed commands 29
  - failed commands 0
  - `product_alpha1_release_candidate_ready: true`
  - `final_public_api_frozen: false`
  - `final_product_claimed: false`
- Initial installed-binary probe passed:
  - status `accepted`
  - passed commands 11
  - failed commands 0
  - compatibility scope stayed on versioned `package.mir.json`, documented
    `mirrorea-alpha` command family, and native host launch bundle replay
    surface
- Initial operational `check-all` passed, but nested `release_check` command
  argv still contained repo-root absolute paths. That was treated as drift.
- RED test failed as expected: the new test could not find
  `samples/product-alpha1/operational/sugoroku-world` in command argv because
  the payload contained `/home/.../samples/product-alpha1/operational/...`.
- Focused GREEN test passed after `repo_cli_arg()`.
- `python3 -m unittest scripts.tests.test_operational_product_samples` passed:
  30 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `git diff --check` passed before report creation.
- Final operational `check-all` passed:
  - status `accepted`
  - top-level failed commands 0
  - nested release-check status `accepted`
  - nested release commands 32
  - nested release failed commands 0
  - Docker included
  - repo-root absolute matches in generated JSON: 0
  - repo-owned argv include relative paths such as
    `samples/product-alpha1/operational/sugoroku-world`
- Final Product Alpha release-check passed:
  - status `accepted`
  - passed commands 29
  - failed commands 0
  - `product_alpha1_release_candidate_ready: true`
  - `final_public_api_frozen: false`
  - `final_product_claimed: false`
- Final installed-binary probe passed:
  - status `accepted`
  - passed commands 11
  - failed commands 0
  - `installed_binary_candidate_ready: true`
  - `public_packaging_candidate: installed_binary_plus_native_host_launch_bundle`
  - `final_public_api_frozen: false`
- Final minimal alpha-1 pattern verifier passed:
  - status `accepted`
  - failed `[]`
  - failures `[]`
  - strict family count 4
  - workflow anchors not checked in that command
  - final public product claimed `false`
- Post-report `python3 scripts/validate_docs.py` passed and found 1334
  numbered reports.
- Post-report `python3 scripts/check_source_hierarchy.py` passed:
  required/present 659/659.
- Post-report `git diff --check` passed.

## What changed in understanding

The operational product suite itself remained compatible, but the helper still
had one portability gap: transient nested command evidence used host-absolute
repo paths even though Product Alpha and Full System V1 release-check paths had
already been hardened. The fix keeps execution behavior the same while making
the recorded repo-owned inputs portable.

## Open questions

None for this package.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`; good next candidates are
focused source-hierarchy drift scans or another bounded compatibility audit.

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
- updated the operational suite row to mention repo-relative nested helper argv
- added a recent-log entry for the portability fix and compatibility audit

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- added a current holding-state bullet for operational helper nested argv
  portability and compatibility validation status

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the operational product sample suite evidence row
- added a Recent Validation Log row for the operational helper portability fix
  and compatibility audit

## Reviewer findings and follow-up

Focused self-review:

- Confirmed the helper only converts repo-owned `Path` arguments to
  repo-relative strings before passing them to nested CLI commands.
- Confirmed temporary output directories are left as external paths.
- Confirmed final generated operational JSON contains zero repo-root absolute
  path matches.

Sub-agent verification:

- Eval runner independently ran `python3 scripts/minimal_alpha1_patterns.py
  check-all --format json` before the implementation change and reported
  status `accepted`, strict family count 4, no failures, and clean workspace.
  The main agent reran the same command after the fix.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  the change is isolated to the operational Python helper and the package ran
  the Product Alpha release-check, installed-binary probe, operational
  `check-all`, helper unit tests, docs validator unit tests, and minimal
  pattern verifier.

## Commit / push status

Pending at initial report creation.

## Sub-agent session close status

Sub-agent `019f2b6a-d680-7792-b88e-6b5f6423c0d9` was closed after completing
the independent minimal alpha-1 pattern verifier check.
