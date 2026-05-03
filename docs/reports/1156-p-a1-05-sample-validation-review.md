# Report 1156 — P-A1-05 sample / validation review

- Date: 2026-05-03 19:51:56 JST
- Author / agent: Codex
- Scope: `P-A1-05` review only. Determine the minimal practical sample set and validation floor for transport practical E2E after `P-A1-04c`, including Docker-closeout policy and Docker-unavailable handling.
- Decision levels touched: `L1` / `L2` review only; no normative edits.

## Objective

Review the current `P-A1-05` inputs and adjacent practical-alpha1 transport evidence, then write down the narrowest honest closeout condition for the next transport package without editing specs, plans, dashboards, or code.

## Scope and assumptions

- Scope is limited to review/reporting over the current repository state after `P-A1-04c`.
- No implementation, spec, roadmap, or dashboard edits were made outside this new report.
- `specs/18-practical-alpha1-scope.md` remains the normative authority for practical transport completion.
- `plan/44-practical-alpha1-roadmap.md` remains repository-memory authority for package order and current promoted next package.
- I treated silent scope reduction as disallowed: if Docker is required by the named package, lack of Docker blocks honest closeout rather than downgrading the package silently.

## Start state / dirty state

- `git status --short` was empty at task start.
- The practical alpha-1 tree already contained `SRC-*`, `CHK-*`, `RUN-01/02`, and `HP-A1-01..07` expected reports, but no practical transport implementation files or transport expected reports.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/91-maintenance-rules.md`
- `sub-agent-pro/alpha-1/02-practical-alpha1-definition.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/practical-alpha1/docker/README.md`
- `samples/practical-alpha1/source/README.md`
- `scripts/README.md`
- `scripts/alpha_network_docker_e2e.py`
- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_run_local.py`
- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_alpha_network_docker_e2e.py`
- `scripts/tests/test_practical_alpha1_check.py`
- `scripts/tests/test_practical_alpha1_run_local.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- representative expected reports under `samples/practical-alpha1/expected/`
- prior reports used as transport-floor precedent:
  `docs/reports/1104-alpha0-network-docker-first-cut.md`,
  `docs/reports/1115-p-a0-15-stage-e-widening-closeout.md`,
  `docs/reports/1131-p-a0-24-stage-c-transport-closeout.md`,
  `docs/reports/1155-p-a1-04c-practical-detach-minimal-contract.md`

## Actions taken

1. Read the user-specified `sub-agent-pro/alpha-1/08-sample-matrix.md` and `09-validation-plan.md` first, then followed the repository reading order through the baseline docs/specs/plan files.
2. Inspected the current practical sample root, package README files, expected-report inventory, and practical runner scripts to see what is actually implemented today.
3. Compared the practical transport requirements in `specs/18` and `plan/44` against the current practical sample matrix and validation-plan wording.
4. Inspected the existing alpha Docker transport runner and its tests to identify the smallest already-proven transport contract that practical `P-A1-05` can reuse without overclaim.
5. Ran focused repo checks, Python unit tests, practical closeout commands, Docker availability checks, and the alpha Stage-C Docker closeout command to collect evidence.
6. Derived the minimal honest `P-A1-05` sample obligations, the focused validation floor, and the Docker-unavailable policy.

## Files changed

- `docs/reports/1156-p-a1-05-sample-validation-review.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- `sed -n '1,240p' docs/reports/TEMPLATE.md`
- `sed -n '1,240p' plan/91-maintenance-rules.md`
- `sed -n ...` over the consulted `README.md`, `Documentation.md`, `specs/*.md`, `plan/*.md`, `sub-agent-pro/alpha-1/*.md`, sample README files, scripts, tests, and expected reports
- `rg --files plan sub-agent-pro specs samples/practical-alpha1 scripts/tests`
- `rg -n "P-A1-05|run_docker|Docker|missing capability|missing witness|stale membership|observer-safe route trace|auth lane" ...`
- `find samples/practical-alpha1/expected -maxdepth 1 -type f -name '*.expected.json' | sort`
- `python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_alpha_network_docker_e2e`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `python3 scripts/practical_alpha1_check.py closeout --format json`
- `python3 scripts/practical_alpha1_run_local.py closeout --format json`
- `python3 scripts/practical_alpha1_attach.py closeout --format json`
- `docker --version`
- `docker compose version`
- `python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json`
- `cargo fmt --check`
- `if [ -e scripts/practical_alpha1_run_docker.py ]; then echo present; else echo missing; fi`
- `if [ -e samples/practical-alpha1/packages/sugoroku-space ]; then echo present; else echo missing; fi`
- `date '+%Y-%m-%d %H:%M:%S %Z'`

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - passed; `required: 73`, `present: 73`, `missing: 0`
- `python3 scripts/validate_docs.py`
  - passed; documentation scaffold complete, 1155 numbered reports before this new report
- `git diff --check`
  - passed
- `cargo fmt --check`
  - passed
- `python3 -m unittest ...`
  - passed; 33 tests in `0.009s`
- `python3 scripts/practical_alpha1_check.py closeout --format json`
  - passed
  - current implemented rows are only `CHK-LIF-01..04`, `CHK-VAR-01..03`, `CHK-CUT-01`, `CHK-PKG-01/02`
  - explicit non-claim: `run_docker_claimed = false`
- `python3 scripts/practical_alpha1_run_local.py closeout --format json`
  - passed
  - current implemented rows are only `RUN-01`, `RUN-02`
  - explicit non-claim: `run_docker_claimed = false`
- `python3 scripts/practical_alpha1_attach.py closeout --format json`
  - passed
  - current implemented rows are only `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, `HP-A1-07`
  - explicit non-claim: `run_docker_claimed = false`
- expected-report inventory
  - 26 practical expected reports exist today
  - all current files are `src-*`, `chk-*`, `run-*`, or `hp-a1-*`
  - there are no practical `tr-*` / Docker expected reports yet
- file existence checks
  - `scripts/practical_alpha1_run_docker.py` is missing
  - `samples/practical-alpha1/packages/sugoroku-space` is missing
  - `samples/practical-alpha1/docker/README.md` still says Docker fixtures are reserved for later work
- Docker environment checks
  - `docker --version` -> `Docker version 29.3.0, build 5927d80`
  - `docker compose version` -> `Docker Compose version v5.1.0`
- `python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json`
  - passed
  - Stage C current-scope closeout rows are `NET-02`, `NET-03`, `NET-04`, `NET-05`, `NET-07`, `NET-09`
  - explicit non-claims remain false for WAN federation, partition completion, medium substitution, active-root promotion, and final public transport ABI

## What changed in understanding

- The practical transport package is blocked less by theory than by stale/stubbed transport-facing scaffolding. The normative and roadmap texts are already clear that `P-A1-05` must prove practical `local TCP + Docker` execution plus explicit negative/security transport evidence.
- The current practical matrix under-specifies transport negatives. `specs/18` and `plan/44` require `stale membership`, `missing capability`, and `missing witness` negatives for the transport stage, but the current `TR-A1-*` rows name only stale membership plus route/auth visibility rows.
- The current validation-plan command examples are not executable as written. They still reference a nonexistent runner and a nonexistent package path, so they cannot be used as the closeout source of truth.
- Docker is a real stage gate for the named package, not an optional extra. The spec already encodes the honest behavior: if Docker is unavailable, record the skip and do not claim `P-A1-05` success.

## Open questions

- Should the practical transport matrix stay compact by combining `observer-safe route trace` and `auth lane separate from transport` into one accepted security row, or should it mirror Alpha Stage-C and split them into two rows for easier failure localization?
- Should `P-A1-05` expose one transport runner with multiple submodes (`local-tcp`, `docker`) or two entrypoints, as long as both remain package-input driven and exact-report checked?
- Does the practical transport stage need a distinct transport-plan carrier between `PracticalAlpha1RuntimePlan` and the final transport report, or is direct lowering from the current runtime-plan carrier still narrow enough?

## Suggested next prompt

Implement `P-A1-05` as the narrowest honest practical transport package: add package-input-driven local-TCP and Docker transport runner(s), create exact expected reports for the required transport rows, keep explicit non-claims for save/load/devtools/public ABI, run the focused validation floor sequentially for Docker-backed rows, and close only if both local-TCP and Docker evidence pass.

## Plan update status

`plan/` 更新不要:
This task was review-only and intentionally made no roadmap-memory edits.

## Documentation.md update status

`Documentation.md` 更新不要:
This task recorded review conclusions only.

## progress.md update status

`progress.md` 更新不要:
No package was implemented or closed here.

## tasks.md update status

`tasks.md` 更新不要:
No queue state was changed by this review-only task.

## samples_progress.md update status

`samples_progress.md` 更新不要:
No runnable sample status changed in this task.

## Reviewer findings and follow-up

1. `sub-agent-pro/alpha-1/09-validation-plan.md` is stale for the practical transport lane and cannot be used verbatim for `P-A1-05` closeout. It still points at `python3 scripts/practical_alpha1_run_docker.py samples/practical-alpha1/packages/sugoroku-space --format json`, but both the runner and that package path are absent in the current tree, while `samples/practical-alpha1/docker/README.md` and `scripts/README.md` still mark practical Docker work as later. Follow-up: before or within implementation, replace the stale runner/path references with the real practical transport command surface and actual transport fixture paths.

2. The current `TR-A1-*` matrix is semantically under-complete for `P-A1-05`. `specs/18-practical-alpha1-scope.md` and `plan/44-practical-alpha1-roadmap.md` require transport-stage evidence for `stale membership`, `missing capability`, and `missing witness` negatives, but `sub-agent-pro/alpha-1/08-sample-matrix.md` names only `TR-A1-03 stale membership over TCP reject` plus visibility rows. Reusing `RUN-02` or `HP-A1-04B2` is not sufficient because those prove local-runtime or hot-plug rejection, not transport-lane rejection. Follow-up: add explicit practical transport rows for missing capability and missing witness, or rename/expand the current matrix so those obligations are unambiguously present.

3. Docker must be mandatory for honest `P-A1-05` closeout. `specs/18-practical-alpha1-scope.md` requires `Docker/local TCP E2E` for the transport stage and says that if Docker is unavailable, success must not be claimed. Follow-up: treat Docker availability as a package gate. If Docker/Compose is missing or unusable, keep `P-A1-05` open, record the skipped Docker rows and reason in the report, and leave explicit non-claim fields such as `run_docker_claimed = false` and stage-complete flags false.

4. The minimal honest sample set for `P-A1-05` is transport-obligation based, not “whatever practical rows already exist.” Recommended minimum:

- one positive local-TCP accepted row proving package-input-driven transport without falling back to in-process-only `RUN-01`
- one positive Docker two-node accepted row proving package-input-driven Docker E2E
- one stale-membership transport reject row with exact rejection refs
- one missing-capability transport reject row with exact `missing_capability:<capability>` evidence
- one missing-witness transport reject row with exact `missing_witness:<witness_ref>` evidence
- one accepted security row that exact-checks `observer_route_trace` redaction and `auth_lane.preserved_separately = true`

If maintainers want failure localization aligned with the Alpha Stage-C precedent, split the final security row into two separate rows (`route trace` and `auth lane`) instead of keeping it merged. I do not recommend going below the six logical obligations above.

5. The focused validation floor for `P-A1-05` should mirror the current practical exact-report pattern and the Alpha Stage-C transport contract, not the helper-local canaries. Recommended minimum closeout floor:

- basic floor: `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, `cargo fmt --check`, `git diff --check`
- Docker gate: `docker --version`, `docker compose version`
- focused tests for the new practical transport runner/runtime surface plus its Python unit tests
- exact practical checker/runtime prerequisites remain green via `python3 scripts/practical_alpha1_check.py check-all --format json` and the relevant focused Rust tests
- exact practical transport sidecar parity via the new practical transport `check-all --format json`
- sequential Docker-backed execution only, unless the new runner introduces explicit Compose-project isolation

The report shape should also keep today’s explicit non-claims: no WAN federation, no final public transport ABI, no save/load, no devtools completion, no active-root promotion.

## Skipped validations and reasons

- No new practical transport runner or practical transport Cargo test was executed because this task was review-only and those surfaces do not exist yet.
- No practical Docker fixture under `samples/practical-alpha1/docker/` was executed because the directory is still a reserved placeholder and no practical transport implementation is present there.

## Commit / push status

Not committed or pushed in this review-only task.

## Sub-agent session close status

No sub-agent was dispatched. This review session can close after the report is recorded.
