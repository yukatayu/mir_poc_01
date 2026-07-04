# Report 2200 — Alpha network Docker path serialization

- Date: 2026-07-04 16:01 JST
- Author / agent: Codex
- Scope: Alpha network Docker success / failure path serialization
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Harden `scripts/alpha_network_docker_e2e.py` so user-facing JSON/status payloads
and failure reasons do not expose checkout-specific repo paths or temporary
Compose output paths, while preserving host absolute paths where Docker Compose
bind mounts require them.

## Scope and assumptions

- Scope is limited to the Alpha-0 network Docker helper, its focused tests, and
  status/report snapshots.
- Repo-owned paths in `list`, `closeout`, `_run_compose()` payloads, and
  `docker compose -f` argv should be repo-relative.
- Docker bind mount environment variables still need host absolute paths:
  `MIRROREA_ALPHA_NETWORK_BINARY` and `MIRROREA_ALPHA_NETWORK_OUTPUT_DIR`.
- Temporary Compose output paths should not leak into stable success/failure
  payloads; output-file failures can name `world.json` / `participant.json`.
- This is maintenance hardening only. It does not change sample status, workflow
  status, transport semantics, WAN/federation scope, ABI, final product claims,
  or canon status.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `46dc406ef7b3d3973bb820f854978708f0a929c3`
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
- `docs/reports/2199-full-system-v1-helper-source-argv-portability.md`
- `scripts/alpha_network_docker_e2e.py`
- `scripts/tests/test_alpha_network_docker_e2e.py`
- `samples/alpha/network-docker/docker-compose.alpha-net.yml`

## Actions taken

- Added `repo_cli_arg()`, `repo_display_text()`, and `compose_display_text()`.
- Changed `list_samples()` `source_root` and `closeout()` `sample_root`,
  `compose_file`, and `binary_path` to display repo-relative paths.
- Changed nested Docker Compose `-f` argv to use the repo-relative compose file
  while keeping subprocess `cwd=REPO_ROOT`.
- Kept Docker bind mount env values absolute for runtime correctness.
- Sanitized Docker stdout and Compose failure stderr before returning or
  surfacing them in `check_all()` failures.
- Changed missing compose-output JSON failures to report `world.json` or
  `participant.json` rather than a temporary output directory.
- Added focused unit tests for list/closeout display, compose argv/env boundary,
  Docker stdout sanitization, failure stderr sanitization, missing JSON output
  display, and missing repo binary failure text.
- Ran real Docker Compose validation for `NET-02`, full `check-all`, and
  `stage-c-closeout`.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.

## Files changed

- `scripts/alpha_network_docker_e2e.py`
- `scripts/tests/test_alpha_network_docker_e2e.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2200-alpha-network-docker-path-serialization.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 scripts/alpha_network_docker_e2e.py list --format json
python3 scripts/alpha_network_docker_e2e.py closeout --format json
python3 -m unittest scripts.tests.test_alpha_network_docker_e2e
docker --version
docker compose version
cargo build -p mir-runtime --example mirrorea_alpha_network_runtime
cargo test -p mir-runtime --test alpha_network_runtime
python3 scripts/alpha_network_docker_e2e.py run NET-02 --format json
python3 scripts/alpha_network_docker_e2e.py check-all --format json
python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json
cd /tmp && python3 /home/codex/dev/mir_poc_01/scripts/alpha_network_docker_e2e.py closeout --format json
```

## Evidence / outputs / test results

- Before change:
  - `list`: repo absolute matches `6`, `/home/` matches `6`
  - `closeout`: repo absolute matches `3`, `/home/` matches `3`
- Unit coverage:
  - `python3 -m unittest scripts.tests.test_alpha_network_docker_e2e`: 16 tests
    passed.
- Static helper payloads:
  - `list`: repo absolute matches `0`, `/home/` matches `0`
  - `closeout`: repo absolute matches `0`, `/home/` matches `0`
  - `cd /tmp && python3 .../alpha_network_docker_e2e.py closeout`: repo absolute
    matches `0`, `/home/` matches `0`
- Docker availability:
  - Docker version `29.6.0`
  - Docker Compose version `v5.1.4`
- Runtime validation:
  - `cargo build -p mir-runtime --example mirrorea_alpha_network_runtime` passed.
  - `cargo test -p mir-runtime --test alpha_network_runtime`: 7 tests passed.
- Real Docker helper validation:
  - `run NET-02`: sample `NET-02`, outcome `accepted`, repo absolute matches `0`,
    `/home/` matches `0`
  - `check-all`: sample count `6`, passed
    `NET-02/NET-03/NET-04/NET-05/NET-07/NET-09`, failed `[]`, repo absolute
    matches `0`, `/home/` matches `0`
  - `stage-c-closeout`: `stage_c_complete=True`, failed `[]`, repo absolute
    matches `0`, `/home/` matches `0`

## What changed in understanding

The helper mixed two path categories. Docker Compose execution really does need
host absolute bind mount sources, but the stable helper JSON/status surface does
not. Keeping that split explicit lets the Docker workflow remain runnable while
making the evidence portable across checkout roots.

## Open questions

- No semantic or user-choice question blocks the next maintenance package.
- Remaining broader path-portability candidates are current-L2 pipeline /
  detached-loop repo-owned helper argv, source-hierarchy status JSON, and shared
  practical failure-path redaction.

## Suggested next prompt

Continue the broader path-portability sweep with current-L2 pipeline /
detached-loop repo-owned helper argv or source-hierarchy status JSON.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 16:01 JST alpha network Docker path
serialization log.

## tasks.md update status

`tasks.md` 更新済み: updated the broader path-portability candidate list and
recorded the Alpha network Docker helper hardening.

## samples_progress.md update status

`samples_progress.md` 更新済み: updated the Alpha-0 evidence row and recent
validation log.

## Reviewer findings and follow-up

Sub-agent code mapper `019f2bea-7e70-78e0-90e8-d55388d363b7` identified the
same high-risk surfaces: list/closeout absolute paths, Docker Compose argv/env
boundary, raw Docker stdout, raw failure text, missing JSON output paths, and
thin test coverage around `_run_compose()`. The implemented patch addresses
those surfaces while preserving absolute host paths only for Compose bind mount
environment variables.

## Skipped validations and reasons

- Full workspace Cargo validation was not rerun because this package changes a
  Python helper display/argv boundary and the focused runtime test plus real
  Docker helper validation exercised the affected runtime path.
- Broader path-portability candidates were not fixed in this package to keep
  scope narrow.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent code mapper `019f2bea-7e70-78e0-90e8-d55388d363b7` completed and was
closed.
