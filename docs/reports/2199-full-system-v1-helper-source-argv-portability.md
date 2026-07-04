# Report 2199 — Full System V1 helper source argv portability

- Date: 2026-07-04 15:51 JST
- Author / agent: Codex
- Scope: Full System V1 helper nested source argv path portability
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Harden `scripts/full_system_v1_samples.py` so repo-owned `.mir` source files are
passed to nested checker/runtime Cargo examples as repo-relative `samples/...`
argv, while preserving external absolute paths.

## Scope and assumptions

- Scope is limited to `scripts/full_system_v1_samples.py`, its focused unit tests,
  and status/report snapshots.
- Public helper output was already repo-root clean for the `check-all` path; the
  remaining issue was nested subprocess argv for source files.
- Actual subprocess working directory remains `REPO_ROOT`, so repo-relative argv
  keep existing runtime behavior.
- External absolute paths outside the checkout should remain unchanged.
- This is maintenance hardening only. It does not change sample status, workflow
  status, semantics, ABI, final product/API claims, or canon status.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `fc59140dd34c60951aed0b49b17b26cbc26429f9`
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
- `docs/reports/2198-product-installed-binary-generated-path-portability.md`
- `scripts/full_system_v1_samples.py`
- `scripts/tests/test_full_system_v1_samples.py`

## Actions taken

- Added `repo_cli_arg(path: Path) -> str` to `scripts/full_system_v1_samples.py`.
- Updated `_check_source()` and `_run_runtime_source()` to pass repo-owned
  source paths through `repo_cli_arg()` before invoking nested Cargo examples.
- Added focused tests for repo-relative source argv, external absolute path
  preservation, checker subprocess argv, and runtime subprocess argv.
- Re-ran direct checker/runtime/operational sample commands, `check-all`, and the
  Full System V1 release check.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.

## Files changed

- `scripts/full_system_v1_samples.py`
- `scripts/tests/test_full_system_v1_samples.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2199-full-system-v1-helper-source-argv-portability.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 scripts/full_system_v1_samples.py check-all --format json
python3 -m unittest scripts.tests.test_full_system_v1_samples
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-system-v1-release-argv-portability
python3 scripts/full_system_v1_samples.py run mir-02-record-field-positive --format json
python3 scripts/full_system_v1_samples.py run-runtime mir-03-add-one-positive --format json
python3 scripts/full_system_v1_samples.py run-operational fsv1-ops-world-core-positive --format json
```

## Evidence / outputs / test results

- Pre-change `full_system_v1_samples.py check-all`: failed `0`, repo absolute
  matches `0`.
- `python3 -m unittest scripts.tests.test_full_system_v1_samples`: 21 tests
  passed.
- Post-change `full_system_v1_samples.py check-all`:
  - failed `0`
  - validation errors `[]`
  - checker passed `12`
  - runtime passed `17`
  - operational passed `12`
  - repo absolute matches `0`
  - `/home/` matches `0`
- Real Full System V1 release check:
  - status `accepted`
  - passed commands `29`
  - failed commands `0`
  - `release_bundle_built=True`
  - `viewer_ready=True`
  - `full_system_v1_release_check_ready=True`
  - repo absolute matches `0`
  - `/home/` matches `0`
- Direct sample commands:
  - `run mir-02-record-field-positive`: accepted `True`, passed `True`, repo
    absolute matches `0`
  - `run-runtime mir-03-add-one-positive`: accepted `True`, passed `True`,
    output summary `Int64(42)`, repo absolute matches `0`
  - `run-operational fsv1-ops-world-core-positive`: passed `True`, manifest
    passed `True`, runtime passed `True`, repo absolute matches `0`

## What changed in understanding

The earlier Full System V1 path audit correctly showed that public helper output
was already repo-root clean. The remaining portability gap was narrower:
repo-owned source paths were still passed as host-absolute argv to nested Cargo
examples. That boundary can be fixed without changing helper output shape,
sample status, or runtime semantics.

## Open questions

- No semantic or user-choice question blocks the next maintenance package.
- Remaining broader path-portability candidates are alpha network Docker
  success/failure path serialization, current-L2 pipeline / detached-loop
  repo-owned helper argv, source-hierarchy status JSON, and shared practical
  failure-path redaction.

## Suggested next prompt

Continue the broader path-portability sweep with alpha network Docker
success/failure path serialization or current-L2 pipeline / detached-loop
repo-owned helper argv.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 15:51 JST Full System V1 nested
source argv path portability log.

## tasks.md update status

`tasks.md` 更新済み: updated the Full System V1 helper portability reading and
removed Full System V1 nested source argv from the remaining broader candidate
list.

## samples_progress.md update status

`samples_progress.md` 更新済み: updated the Full System V1 roadmap/computational
rows and recent validation log.

## Reviewer findings and follow-up

Sub-agent reviewer `019f2be3-ddb4-72d2-9691-d5b8838809db` reported no findings
in the current diff. The reviewer checked `repo_cli_arg()`, both nested Cargo
argv surfaces, the operational path that reuses the wrappers, and the Rust
examples' path handling. The reviewer also recommended the direct checker /
runtime / operational sample commands that were run above.

## Skipped validations and reasons

- Full workspace Cargo validation was not rerun because the code change is a
  Python helper argv transformation and the real helper / release-check
  validation already exercised the nested Cargo examples.
- Broader path-portability candidates were not fixed in this package to keep
  scope narrow.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent reviewer `019f2be3-ddb4-72d2-9691-d5b8838809db` completed and was
closed.
