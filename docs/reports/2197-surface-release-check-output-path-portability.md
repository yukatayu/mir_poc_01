# Report 2197 — Surface release-check output path portability

- Date: 2026-07-04 15:38 JST
- Author / agent: Codex
- Scope: Surface release-check plan/report/bundle/viewer output path serialization
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Harden `scripts/surface_mir_release_check.py` so release-owned output paths and
repo-owned path text are serialized portably in plan JSON, per-command report
JSON, bundle JSON, and generated viewer HTML.

## Scope and assumptions

- Scope is limited to Surface release-check output serialization and tests.
- The release-check still writes files under the user-specified `--out`
  directory.
- Displayed paths under `--out` should be relative to that release root.
- Repo-owned free-text paths should lose the checkout-specific prefix.
- External absolute paths outside both the release root and repo root should
  remain unchanged.
- This is path-portability maintenance only. It does not change Surface sample
  status, workflow status, semantics, ABI, canon status, or product/public
  claims.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `08bc8e38ec89ec5f224a774f24c7ee210688c1ff`
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
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `scripts/product_alpha1_release_check.py`
- `scripts/full_system_v1_release_check.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_release_check.py`

## Actions taken

- Added release-display helpers to Surface release-check:
  `release_relative_path()`, `release_display_text()`, and
  `release_display_value()`.
- Applied display normalization recursively to `command_plan_payload()`,
  `write_report()`, and `run_check_all()`.
- Changed `write_report()` to return the exact display-normalized record that
  it writes to disk, so the top-level bundle and per-command report files stay
  aligned.
- Added tests that cover plan path fields, recursive display rewriting,
  external absolute path preservation, fake check-all report generation, and
  generated file scans.
- Ran the real Surface release-check and scanned both stdout payload and
  generated files for host path leakage.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.
- Used a read-only sub-agent review for the release-check serialization blast
  radius and incorporated its extra test recommendation.

## Files changed

- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2197-surface-release-check-output-path-portability.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 -m unittest scripts.tests.test_surface_mir_release_check
python3 -m unittest scripts.tests.test_surface_mir_release_check.SurfaceMirReleaseCheckTests.test_release_display_value_rewrites_only_release_and_repo_owned_paths scripts.tests.test_surface_mir_release_check.SurfaceMirReleaseCheckTests.test_plan_payload_serializes_release_owned_paths_without_host_prefixes scripts.tests.test_surface_mir_release_check.SurfaceMirReleaseCheckTests.test_check_all_serializes_release_owned_paths_without_host_prefixes
python3 scripts/surface_mir_release_check.py --format json plan --out /tmp/mirrorea-surface-release-path-portability-plan/home/codex/surface
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-path-portability
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_surface_mir_release_check`: 12 tests
  passed.
- Focused serialization tests: 3 tests passed.
- Plan smoke output:
  - `out_dir="."`
  - `reports_dir="reports"`
  - `bundle_path="bundle.json"`
  - `html_path="index.html"`
  - `/home/` matches `0`
  - `/Users/` matches `0`
  - repo absolute matches `0`
- `python3 scripts/surface_mir_samples.py check-all --format json`: 52/52
  passed, repo absolute matches `0`.
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`:
  accepted `True`, source count `53`, repo absolute matches `0`.
- Real Surface release-check:
  - ready `True`
  - failed commands `[]`
  - result count `18`
  - generated report count `18`
  - stdout payload `/home/`, `/Users/`, repo absolute matches all `0`
  - generated `bundle.json`, `index.html`, and reports `/home/`, `/Users/`,
    repo absolute matches all `0`

## What changed in understanding

Surface release-check differed from Full System V1 release-check because it
serializes free-text `stdout` / `stderr`, not only path-valued fields. The safer
pattern is therefore closer to Product Alpha release-check: recursively rewrite
string values against both release root and repo root, while leaving unrelated
external absolute paths intact.

## Open questions

- No semantic or user-choice question blocks the next maintenance package.
- Remaining broader candidates are installed-binary generated path
  serialization / argv, Full System V1 nested source argv, alpha network Docker
  success/failure path serialization, current-L2 pipeline / detached-loop
  repo-owned helper argv, source-hierarchy status JSON, and shared practical
  failure-path redaction.

## Suggested next prompt

Continue the path-portability broader sweep with Product Alpha
installed-binary generated path serialization / argv, unless another candidate
is more urgent.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 15:38 JST Surface release-check
output serialization hardening log.

## tasks.md update status

`tasks.md` 更新済み: added Surface release-check output serialization hardening
and removed that item from the remaining broader candidate list.

## samples_progress.md update status

`samples_progress.md` 更新済み: updated the Surface row and recent validation
log.

## Reviewer findings and follow-up

Sub-agent `Nash` completed a read-only audit. It confirmed the patch strategy
and identified one useful extra test: directly exercise `release_display_value()`
with release-owned, repo-owned, and external absolute paths. That test was
added before final validation.

## Skipped validations and reasons

No Surface release-check validation was skipped. The real release-check was
run through its 18-command floor. Broader path-portability candidates were not
fixed in this package to keep scope narrow.

## Commit / push status

- Implementation / snapshot / initial report commit:
  `b194555e Use relative surface release output paths`
- Push status: pushed to `origin/main`.
- Follow-up report-status metadata update is committed and pushed separately.

## Sub-agent session close status

Sub-agent `019f2bd4-9c19-7bd2-825e-657be4286ac8` completed its read-only audit
and was closed.
