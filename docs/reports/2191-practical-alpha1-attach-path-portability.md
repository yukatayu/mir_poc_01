# Report 2191 — Practical alpha-1 attach path portability

- Date: 2026-07-04 14:52 JST
- Author / agent: Codex
- Scope: Practical alpha-1 attach helper subprocess path portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha1_attach.py` pass repo-owned practical package
roots to its nested hotplug Cargo example through repo-relative `samples/...`
arguments.

## Scope and assumptions

Scope:

- audit `practical_alpha1_attach.py check-all` output for repo-root absolute
  path drift
- add regression coverage for repo-relative hotplug subprocess package argv
- preserve absolute arguments for paths outside the repository
- rerun attach helper, focused tests, and relevant practical hotplug Cargo
  tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested hotplug Cargo example runs with `cwd=REPO_ROOT`, so repo-relative
  package-root arguments are portable and executable.
- This package does not change hotplug semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 53 started from clean `HEAD == origin/main == 4e75b9cf` after the
practical alpha-1 run-local helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_transport.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2190-practical-alpha1-run-local-path-portability.md`

## Actions taken

- Ran `practical_alpha1_attach.py check-all` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - hotplug Cargo example package argv
- Added `repo_cli_arg()` to `scripts/practical_alpha1_attach.py`.
- Converted the hotplug Cargo example package argv and JSON decode failure
  message to use `repo_cli_arg()`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Spawned a read-only code-mapper sub-agent for remaining helper-candidate
  auditing; it returned before commit and was used to rank the next practical
  helper audits.

## Files changed

- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2191-practical-alpha1-attach-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha1_attach.py check-all --format json > /tmp/mirrorea-practical-alpha1-attach-p53-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-attach-p53-before.json`
- `sed -n '1,320p' scripts/practical_alpha1_attach.py`
- `sed -n '1,260p' scripts/tests/test_practical_alpha1_attach.py`
- `rg -n "subprocess\\.run|str\\(|Path\\(|REPO_ROOT|repo_cli_arg|package_path|package_dir|closeout|cargo|sample_root|source" scripts/practical_alpha1_attach.py scripts/tests/test_practical_alpha1_attach.py`
- `python3 -m unittest scripts.tests.test_practical_alpha1_attach` (RED)
- same unit command after implementation
- `python3 scripts/practical_alpha1_attach.py check-all --format json > /tmp/mirrorea-practical-alpha1-attach-p53-after.json`
- `python3 scripts/practical_alpha1_attach.py closeout --format json > /tmp/mirrorea-practical-alpha1-attach-p53-closeout.json`
- `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
- `date '+%Y-%m-%d %H:%M %Z'`
- spawned code-mapper sub-agent `Halley` for read-only remaining-helper audit
- waited 10 seconds for `Halley`; result was still pending
- received `Halley`'s completed audit result and closed the sub-agent

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 9
  - passed `["HP-A1-01", "HP-A1-02", "HP-A1-03", "HP-A1-04", "HP-A1-05", "HP-A1-04B1", "HP-A1-04B2", "HP-A1-06", "HP-A1-07"]`
  - failed `[]`
  - repo-root absolute matches 0
- RED unit run failed as expected:
  - `repo_cli_arg` did not exist
  - hotplug Cargo example argv contained a host absolute package path
- `python3 -m unittest scripts.tests.test_practical_alpha1_attach` passed
  after implementation: 11 tests.
- Final `check-all` scan:
  - sample_count 9
  - passed `["HP-A1-01", "HP-A1-02", "HP-A1-03", "HP-A1-04", "HP-A1-05", "HP-A1-04B1", "HP-A1-04B2", "HP-A1-06", "HP-A1-07"]`
  - failed `[]`
  - stage_pa1_4_complete `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - implemented_rows `["HP-A1-01", "HP-A1-02", "HP-A1-03", "HP-A1-04", "HP-A1-05", "HP-A1-04B1", "HP-A1-04B2", "HP-A1-06", "HP-A1-07"]`
  - package_hotplug_first_floor_complete `True`
  - hotplug_plan_boundary_present `True`
  - stage_pa1_4_complete `True`
  - repo-root absolute matches 0
- `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
  passed: 11 tests.
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
  passed: 10 tests.
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
  passed: 8 tests.
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  passed: 17 tests.
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime` passed:
  6 tests.

## What changed in understanding

The attach helper emitted clean public JSON already. The non-portable piece was
the internal hotplug Cargo example invocation, which passed repo-owned package
roots as host absolute paths.

The read-only code-mapper audit found that the remaining practical alpha helper
happy paths are repo-root clean, but their nested subprocess argv and
failure-path `failed[].error` serialization remain plausible portability risks.
The recommended next order is `practical_alpha09_devtools.py`,
`practical_alpha08_session_hotplug.py`, `practical_alpha1_avatar.py`, then
`practical_alpha1_save_load.py`.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audits remain for
  `practical_alpha09_devtools.py`, `practical_alpha08_session_hotplug.py`,
  `practical_alpha1_avatar.py`, and `practical_alpha1_save_load.py`, in that
  recommended order unless local evidence changes it.

## Suggested next prompt

Continue autonomous maintenance with the next practical alpha helper
path-portability audit.

## Plan update status

`plan/` 更新不要:

- This package did not change roadmap, semantics, source-traceability,
  open-question, or repository-memory decisions.

## Documentation.md update status

`Documentation.md` 更新不要:

- No top-level reader-facing status or source hierarchy changed.

## progress.md update status

Updated:

- advanced the top `最終更新` timestamp
- added a recent-log entry for practical alpha-1 attach helper path portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-1 attach helper path portability is now
  hardened

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the `HP-A1-*` row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all` and `closeout` JSON have zero repo-root
  absolute matches after the change.
- Confirmed tests cover path helper behavior and the hotplug Cargo example
  argv.

Sub-agent review:

- `Halley` completed a read-only candidate mapping of the remaining helper
  families. It ranked `practical_alpha09_devtools.py` and
  `practical_alpha08_session_hotplug.py` as the widest remaining nested-argv
  risks, followed by `practical_alpha1_avatar.py` and
  `practical_alpha1_save_load.py`.
- The same audit confirmed happy-path `check-all` / `closeout` JSON was
  repo-root clean for the audited remaining helpers; the likely residual risk
  is failure-path serialization when nested subprocess argv contains repo-root
  absolute paths.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical alpha-1 attach helper and hotplug validation
  floor were rerun.
- Oracle was not used because the package was a narrow mechanical portability
  hardening step with direct local evidence.

## Commit / push status

Pending at report creation time.

## Sub-agent session close status

`Halley` (`019f2bae-60a9-75c1-ab6d-70cbee476f54`) completed and was closed
before this package was committed.
