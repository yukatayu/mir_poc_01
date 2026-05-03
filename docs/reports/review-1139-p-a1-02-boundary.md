# Report 1139 — Review P-A1-02 Checker Boundary

- Date: 2026-05-03
- Author / agent: Codex
- Scope: boundary review for the proposed `P-A1-02` practical checker direction
- Decision levels touched: none

## Objective

Review the proposed `P-A1-02` direction from a runtime/toolchain-boundary perspective and identify what must stay present or absent so the package remains checker-only and does not pre-claim `PA1-3` runtime plan execution, runtime/Docker freshness, or public CLI freeze.

## Scope and assumptions

- Read only the user-requested practical alpha-1 sources:
  - `specs/18-practical-alpha1-scope.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `sub-agent-pro/alpha-1/03-decisions.md`
  - `sub-agent-pro/alpha-1/04-stage-roadmap.md`
  - `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
  - `sub-agent-pro/alpha-1/09-validation-plan.md`
  - `crates/mir-ast/src/practical_alpha1.rs`
  - relevant `scripts/` references for cargo example wrappers
- This is a review-only task; no implementation changes are made.
- The branch is `main` at commit `d942c95` as requested.

## Start state / dirty state

- start state: `main` at `d942c953698eb1bc75b051bc99418cef2f287aca`
- worktree status: clean before this review report

## Documents consulted

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/03-decisions.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/tests/practical_alpha1_front_door.rs`
- `scripts/README.md`
- `scripts/alpha_lifetime_fallback_checker.py`
- `scripts/alpha_local_runtime_samples.py`

## Actions taken

1. Read the practical alpha-1 spec/roadmap sections that define `PA1-2` checker scope, `PA1-3` runtime-plan scope, and CLI/toolchain intent.
2. Reviewed the current practical front-door implementation and tests to confirm the present boundary ends at package loading/schema checks.
3. Compared existing checker-helper and runtime-helper script/report surfaces to identify which output fields keep or blur the runtime handoff boundary.

## Files changed

- `docs/reports/review-1139-p-a1-02-boundary.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git show --stat --oneline --no-patch d942c95
rg -n "practical_alpha1|cargo example|cargo run --example|mir-ast.*example|practical alpha1" scripts crates -g '*.py' -g '*.rs'
rg -n "cargo run .*--example|subprocess.*cargo|\\bcargo\\b.*--example|mir-ast --example|cargo test -p mir-ast" scripts -g '*.py'
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- `main` resolves to `d942c953698eb1bc75b051bc99418cef2f287aca`, matching the requested review base.
- Current practical front-door still stops at load/shape validation in `crates/mir-ast/src/practical_alpha1.rs`; no checker/runtime bridge exists yet.
- Existing runtime helper precedent in `scripts/alpha_local_runtime_samples.py` shows the pattern of explicit negative flags and stop-lines needed to avoid over-claiming later surfaces.

## What changed in understanding

- The main risk is not the existence of `scripts/practical_alpha1_check.py` itself; the risk is allowing its implementation or JSON/report surface to mention runtime-plan, local runtime, Docker, or stable CLI behavior too early.
- The repo already has a proven pattern for boundary-preserving closeout surfaces: explicit `stop_lines`, `limitations`, and negative claim flags on helper outputs.

## Open questions

- None that block the review conclusions below.

## Suggested next prompt

Implement `P-A1-02` as a checker-only Rust/library surface plus `scripts/practical_alpha1_check.py`, but keep the output schema limited to checker artifacts and add explicit negative-claim fields for runtime-plan, run-local, run-docker, and public-CLI freeze.

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory は変更していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり snapshot wording は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current status snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり sample dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  `P-A1-02` must not call into `mir-runtime` examples or emit any `runtime plan` artifact, because the normative split is still `front-door -> alpha IR -> checker -> runtime plan -> local runtime/Docker runtime`, with `PA1-2` owning only the checker stage and `PA1-3` starting at checked-plan execution. The current front-door code ends at package loading/validation in `crates/mir-ast/src/practical_alpha1.rs`, while `specs/18-practical-alpha1-scope.md` reserves reusable runtime for the next section and `plan/44-practical-alpha1-roadmap.md` reserves `checked plan から local world 実行` for `PA1-3`. Follow-up: if a Rust example is used, keep it in the checker lane and have it return only checker diagnostics/artifacts, not runtime-plan or execution handles.

- Finding 2:
  The proposed `scripts/practical_alpha1_check.py` should expose an alpha-local checker surface, but the output/report schema needs explicit checker-only presence fields and explicit runtime/transport absence fields. Recommended present fields are: `front_door_scope`, `checker_scope`, `package_path`, `package_id`, `package_kind`, `verdict`, `diagnostics`, `accepted_obligations`, `rejected_rows`, `canonical_fallback_chains`, `contract_comparison_report`, `cut_validity_report`, `package_admission_report`, `retained_later_refs`, `stop_lines`, and `limitations` as described by the checker/toolchain docs. Recommended absent fields are: `runtime_plan`, `run_id`, `runtime_scope`, `dispatch_records`, `event_dag`, `route_trace`, `membership_timeline`, `transport_surface`, `transport_medium`, `docker_stdout`, `savepoint`, and any local/Docker freshness marker. Existing runtime helper precedent shows why these fields are boundary-significant: `scripts/alpha_local_runtime_samples.py` carries runtime-only fields plus explicit negatives such as `parser_runtime_bridge_claimed: False`; `P-A1-02` should mirror the negative-claim style but keep runtime fields entirely absent.

- Finding 3:
  `P-A1-02` report/validation wording must not imply public CLI freeze or runtime/Docker freshness. `sub-agent-pro/alpha-1/03-decisions.md` and `06-toolchain-architecture.md` allow alpha-local command names, but only if they are not treated as final public API; `sub-agent-pro/alpha-1/09-validation-plan.md` shows `practical_alpha1_run_local.py` and `practical_alpha1_run_docker.py` as later-stage commands, not checker-package requirements. Follow-up: in docs/report fields, include flags such as `public_cli_frozen: false`, `runtime_plan_emitted: false`, `run_local_claimed: false`, `run_docker_claimed: false`, and limit the `P-A1-02` validation floor to `practical_alpha1_check.py`, focused Rust checker tests, and scaffold checks. Do not list Docker availability, runtime examples, or runtime closeout commands as `P-A1-02` freshness evidence.

## Skipped validations and reasons

- `python3 scripts/validate_docs.py`: not run in this review task because an unrelated draft report in `docs/reports/` is already known to violate the latest-report template check; rerunning would not add signal about the `P-A1-02` boundary question
- Cargo/tests: not run because this is a design review over an implementation direction; no `P-A1-02` checker code exists yet to validate

## Commit / push status

No commit; review-only task.

## Sub-agent session close status

- No sub-agents used.
