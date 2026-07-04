# Report 2202 — Source-hierarchy status root-display portability

- Date: 2026-07-04 16:37 JST
- Author / agent: Codex
- Scope: Source-hierarchy status `repo_root` display portability
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Remove the checkout-specific absolute path from `scripts/check_source_hierarchy.py`
pretty and JSON status output while preserving absolute `REPO_ROOT` internally
for filesystem existence checks.

## Scope and assumptions

- Scope is limited to the source-hierarchy status payload/display, focused
  validator tests, and status/report snapshots.
- `repo_root` remains part of the status payload for compatibility, but its
  display value should be portable.
- Required path inventory and missing/present path lists remain repo-relative
  strings from `REQUIRED_PATHS`.
- `validate_docs.py` host-path lint patterns intentionally contain host-shaped
  regexes and are not a status-output leak.
- This is maintenance hardening only. It does not change source hierarchy
  inventory, sample status, workflow status, semantics, ABI, public API, or
  canon status.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `7666a96eddbd41b883e0b5bf1eff2244a3af1302`
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
- `docs/reports/2201-current-l2-pipeline-path-portability.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- Release-check wrappers that invoke source-hierarchy validation:
  `scripts/surface_mir_release_check.py`,
  `scripts/product_alpha1_release_check.py`,
  `scripts/full_system_v1_release_check.py`,
  `scripts/product_alpha1_installed_binary_check.py`

## Actions taken

- Changed `check_source_hierarchy.build_status()["repo_root"]` from the host
  checkout path to `"."`.
- Kept internal path existence checks based on absolute `REPO_ROOT`.
- Added a validator unit test asserting source-hierarchy status and pretty output
  do not include the host checkout root.
- Ran direct pretty/JSON source-hierarchy output checks.
- Ran release-check wrapper unit tests to guard wrapper interaction.
- Ran a representative Surface release-check because it preserves non-JSON
  validation stdout in generated reports before display sanitization.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.

## Files changed

- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2202-source-hierarchy-status-root-display-portability.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 scripts/check_source_hierarchy.py --help
python3 scripts/check_source_hierarchy.py
python3 scripts/check_source_hierarchy.py --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_product_alpha1_release_check scripts.tests.test_full_system_v1_release_check scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_surface_mir_release_check
python3 scripts/validate_docs.py
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-source-hierarchy-status
rg -n --pcre2 '/home/codex/dev/mir_poc_01|/home/' /tmp/mirrorea-surface-source-hierarchy-status || true
```

## Evidence / outputs / test results

- Before change:
  - `python3 scripts/check_source_hierarchy.py`: repo absolute matches `1`,
    `/home/` matches `1`; pretty output printed
    `repo_root: /home/codex/dev/mir_poc_01`.
  - `python3 scripts/check_source_hierarchy.py --format json`: repo absolute
    matches `1`, `/home/` matches `1`; JSON field `repo_root` held the checkout
    path.
- After change:
  - Pretty output prints `repo_root: .`, required `659`, present `659`,
    missing `0`, repo absolute matches `0`, `/home/` matches `0`.
  - JSON output has `"repo_root": "."`, required count `659`, missing count `0`,
    repo absolute matches `0`, `/home/` matches `0`.
- Focused tests:
  - `python3 -m unittest scripts.tests.test_validate_docs`: 37 tests passed.
  - Release-check wrapper unit coverage: 39 tests passed.
- Docs/source validation:
  - `python3 scripts/validate_docs.py`: documentation scaffold complete, 1354
    numbered reports.
  - `python3 scripts/check_source_hierarchy.py`: passed with required `659`,
    present `659`, missing `0`.
- Representative wrapper validation:
  - `python3 scripts/surface_mir_release_check.py --format json check-all --out
    /tmp/mirrorea-surface-source-hierarchy-status`: output JSON had repo
    absolute matches `0`, `/home/` matches `0`, and generated output scan found
    no repo-root or `/home/` matches.

## What changed in understanding

The source-hierarchy status leak was narrower than the broader helper sweep:
`check_source_hierarchy.py` is the direct status producer, and higher-level
release wrappers either do not embed source-hierarchy JSON or already sanitize
repo-owned paths. The right fix is therefore the stable status display value,
not a wrapper-wide rewrite.

## Open questions

- No semantic or user-choice question blocks the next maintenance package.
- Remaining broader path-portability candidate: shared practical failure-path
  redaction.

## Suggested next prompt

Continue the broader path-portability sweep with shared practical failure-path
redaction.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 16:37 JST source-hierarchy status
root-display portability log.

## tasks.md update status

`tasks.md` 更新済み: recorded source-hierarchy status hardening and narrowed the
remaining broader path-portability candidate list to shared practical
failure-path redaction.

## samples_progress.md update status

`samples_progress.md` 更新済み: added a recent validation log row for
source-hierarchy status root-display portability.

## Reviewer findings and follow-up

Sub-agent code mapper `019f2c09-8f06-74e3-a90a-332c96ba184b` confirmed
`scripts/check_source_hierarchy.py` is the direct source-hierarchy status JSON
producer, `validate_docs.py` has no JSON mode, release wrappers do not require
their own source-hierarchy status rewrite, and host-shaped lint regexes in
`validate_docs.py` are intentional detection boundaries. The implemented patch
matches that scope.

Reviewer sub-agent `019f2c10-f205-7573-9b3a-47a0b8ce70d4` found no
implementation issues. It flagged the report's numbered-report count as stale
after this new report was added; this report now records the final `1354`
numbered-report count.

## Skipped validations and reasons

- Full Product Alpha / Full System V1 / installed-binary release-checks were not
  rerun because their wrapper unit tests cover source-hierarchy command
  serialization and they do not embed source-hierarchy JSON directly.
- Broader practical failure-path redaction was not fixed in this package to keep
  scope narrow.

## Commit / push status

- Implementation / snapshot / initial report commit: pending.
- Push status: pending.

## Sub-agent session close status

Sub-agent code mapper `019f2c09-8f06-74e3-a90a-332c96ba184b` completed and was
closed.
