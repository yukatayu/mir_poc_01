# Report 2201 — Current-L2 pipeline path portability

- Date: 2026-07-04 16:19 JST
- Author / agent: Codex
- Scope: Current-L2 detached-loop, theorem/model pipeline, source-regression,
  and Lean sync path portability
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Harden current-L2 helper and pipeline surfaces so repo-owned helper argv,
artifact displays, plan/result JSON, source-regression command plans, and Lean
verification argv use repo-relative paths while preserving external artifact
roots and temp paths where they are true external boundaries.

## Scope and assumptions

- Scope is limited to current-L2 Python helper/pipeline scripts, their focused
  tests, and status/report snapshots.
- Repo-owned paths should be displayed or passed to repo-root subprocesses as
  repo-relative paths.
- External artifact roots such as `/tmp/...` should remain absolute.
- Cargo itself may print checkout paths in compiler/test harness lines; this
  package targets helper/pipeline stable surfaces, not Cargo diagnostics.
- This is maintenance hardening only. It does not change sample status,
  workflow status, semantics, proof status, ABI, public API, or canon status.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `54a389710792e62426b0e8aa3cae525517e071c9`
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
- `docs/reports/2200-alpha-network-docker-path-serialization.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_model_check_carrier_pipeline.py`
- `scripts/current_l2_theorem_lean_stub_pipeline.py`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_lean_sample_sync.py`
- Focused current-L2 Python test files under `scripts/tests/`

## Actions taken

- Added `repo_cli_arg()` path serialization helpers to current-L2 detached-loop,
  model-check pipeline, theorem Lean-stub pipeline, and source-regression
  planner surfaces.
- Changed detached-loop subprocess argv and user-facing artifact/status text to
  use repo-relative paths for repo-owned fixtures, helpers, and artifacts.
- Added an in-process helper wrapper that temporarily runs checker/assist
  helpers from `REPO_ROOT`, allowing their displayed fixture/artifact paths to
  be repo-relative even when the top-level script is invoked from `/tmp`.
- Changed model-check and theorem pipeline plan/result JSON to serialize
  repo-owned output artifacts relatively, and changed nested detached-loop /
  Cargo example argv to receive repo-relative paths.
- Changed pipeline `run_command()` calls to capture nested helper stdout/stderr
  so final pipeline stdout remains parseable JSON.
- Changed source-regression command planning to use repo-relative nested helper
  script paths and repo-relative default artifact roots.
- Changed Lean sample sync verification to invoke repo-owned Lean files through
  repo-relative argv while preserving external absolute paths.
- Changed `scan-reason-code-readiness` to delegate through the same repo-root
  wrapper so its fixture/artifact directory output is repo-relative for
  repo-owned paths.
- Added focused unit tests for repo-relative argv/display boundaries, external
  absolute preservation, captured nested output, source-regression planning,
  scan-readiness delegation, and Lean verification argv.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.

## Files changed

- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_model_check_carrier_pipeline.py`
- `scripts/current_l2_theorem_lean_stub_pipeline.py`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `scripts/tests/test_current_l2_model_check_carrier_pipeline.py`
- `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2201-current-l2-pipeline-path-portability.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
du -sh . target .git .cargo .lake 2>/dev/null || true
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression --help
python3 -m unittest scripts.tests.test_current_l2_detached_loop scripts.tests.test_current_l2_model_check_carrier_pipeline scripts.tests.test_current_l2_theorem_lean_stub_pipeline
python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop scripts.tests.test_current_l2_model_check_carrier_pipeline scripts.tests.test_current_l2_theorem_lean_stub_pipeline scripts.tests.test_current_l2_lean_sample_sync scripts.tests.test_current_l2_source_sample_regression
python3 scripts/current_l2_model_check_carrier_pipeline.py e2-try-fallback --plan-only
python3 scripts/current_l2_theorem_lean_stub_pipeline.py e2-try-fallback --plan-only
python3 scripts/current_l2_model_check_carrier_pipeline.py e2-try-fallback
python3 scripts/current_l2_theorem_lean_stub_pipeline.py e2-try-fallback
python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root target/current-l2-detached-path-portability --run-label path-portability --overwrite
python3 scripts/current_l2_lean_sample_sync.py
lean samples/lean/foundations/CurrentL2LabelModel.lean
rg -n --pcre2 '/home/codex/dev/mir_poc_01|/home/' target/current-l2-source-regression-path-portability || true
python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --artifact-root target/current-l2-detached-path-portability --run-label scan-readiness --overwrite
cd /tmp && python3 /home/codex/dev/mir_poc_01/scripts/current_l2_detached_loop.py smoke-same-lineage-checker e5-underdeclared-lineage --artifact-root /home/codex/dev/mir_poc_01/target/current-l2-detached-path-portability --run-label wrapper-cwd --overwrite
python3 scripts/current_l2_source_sample_regression.py regression --artifact-root target/current-l2-source-regression-path-portability --run-label path-portability
```

## Evidence / outputs / test results

- Resource check before generating artifacts:
  - `df -h .`: root filesystem 188G size, 146G used, 33G available.
  - `free -h`: 15GiB memory total, 10GiB available, 14GiB swap free.
- Inventory after source-regression planning hardening:
  - `current_l2_source_sample_regression.py inventory`: 19 lines, repo absolute
    matches `0`, `/home/` matches `0`.
- Focused unit coverage:
  - Initial detached/model/theorem path tests: 32 tests passed.
  - Final current-L2 focused suite, including static-gate loop scan-readiness
    coverage: 82 tests passed.
- Plan/result path scans:
  - model-check repo plan JSON: repo absolute matches `0`, `/home/` matches `0`.
  - theorem Lean-stub repo plan JSON: repo absolute matches `0`, `/home/`
    matches `0`.
  - external `/tmp` plan-only roots preserved external absolute paths.
- Real pipeline runs:
  - model-check carrier for `e2-try-fallback`: matched pairs `1`, formal hook
    pair count `1`, model-check carrier count `1`, repo absolute matches `0`.
  - theorem Lean-stub for `e2-try-fallback`: matched pairs `1`, review unit
    count `1`, Lean stub count `1`, repo absolute matches `0`.
- Detached-loop formal hook smoke:
  - `bundle artifact` and `formal hook artifact` printed `target/...` paths;
    repo absolute matches `0`.
- Detached-loop scan-readiness:
  - `fixture directory` printed `crates/mir-ast/tests/fixtures/current-l2`.
  - `artifact directory` printed
    `target/current-l2-detached-path-portability/static-gates/scan-readiness`.
  - Repo absolute matches `0`, `/home/` matches `0`.
- Lean verification:
  - `lean samples/lean/foundations/CurrentL2LabelModel.lean` passed.
  - `python3 scripts/current_l2_lean_sample_sync.py` printed
    `samples/lean/manifest.json`; repo absolute matches `0`.
- Generated artifact scan:
  - `rg` over `target/current-l2-source-regression-path-portability` found no
    repo-root or `/home/` path matches.
- External cwd checker invocation:
  - `/tmp` invocation of `smoke-same-lineage-checker` passed with status
    `matched` and printed repo-relative fixture/artifact paths.
- Full current-L2 source regression after wrapper hardening:
  - 23/23 commands passed.
  - Included Rust runtime lowering 18 tests, source sample runner 2 tests,
    verification ladder 16 tests, formal hook support 5 tests, runtime/static
    formal-hook smokes, theorem Lean-stub conformance, and model-check carrier
    conformance.

## What changed in understanding

The current-L2 stack had two distinct path surfaces. Subprocesses already run
from `REPO_ROOT`, so repo-owned argv can safely be relative there. In-process
checker helpers needed an explicit repo-root wrapper before their argv could be
made relative without breaking invocation from a non-repo cwd. Pipeline stdout
also needed nested helper output capture; otherwise JSON payload parsing stayed
fragile even after path serialization was fixed.

## Open questions

- No semantic or user-choice question blocks the next maintenance package.
- Remaining broader path-portability candidates are source-hierarchy status JSON
  and shared practical failure-path redaction.

## Suggested next prompt

Continue the broader path-portability sweep with source-hierarchy status JSON or
shared practical failure-path redaction.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 16:19 JST current-L2 pipeline /
detached-loop path portability log.

## tasks.md update status

`tasks.md` 更新済み: recorded the current-L2 helper hardening and narrowed the
remaining broader path-portability candidate list.

## samples_progress.md update status

`samples_progress.md` 更新済み: added a recent validation log row for the
current-L2 pipeline / detached-loop path portability pass.

## Reviewer findings and follow-up

Sub-agent code mapper `019f2bf1-d58d-76d1-bfc0-5a8b1ae149dd` identified the
remaining upstream leaks in source-regression command planning and Lean sync
verification, and confirmed that direct detached/model/theorem surfaces needed
coverage rather than more structural redesign. The implemented patch addresses
those surfaces and adds regression tests.

Reviewer sub-agent `019f2c02-cf7f-74d3-922b-3e15960611ee` found one remaining
detached-loop output gap: `scan-reason-code-readiness` still called
`reason_code_readiness.main()` with raw absolute paths. The follow-up patch routes
that subcommand through `run_repo_helper(..., repo_cli_arg(...))`, adds
static-gate loop regression coverage, and verifies the real scan-readiness
command has repo absolute matches `0`.

## Skipped validations and reasons

- Full workspace Cargo validation was not rerun because this package changes
  Python helper display/argv boundaries. The full current-L2 source regression
  reran the relevant Rust tests and current-L2 command chain.
- Broader path-portability candidates were not fixed in this package to keep
  scope narrow.

## Commit / push status

- Implementation / snapshot / initial report commit:
  `454de834 Use portable current-l2 pipeline paths`
- Push status: pushed to `origin/main`.
- Follow-up report-status metadata update is committed and pushed separately.

## Sub-agent session close status

Sub-agent code mapper `019f2bf1-d58d-76d1-bfc0-5a8b1ae149dd` completed and was
closed.
