# Report 1142 — Review P-A1-03 Runtime/Transport Boundary

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-03` review from the runtime/transport implementation boundary, focused on how a checked practical package should feed a reusable local runtime without implying Docker/transport/package hot-plug completion
- Decision levels touched: L1, L2
- 日本語要約: `P-A1-03` は `P-A1-02` の checker report をそのまま runtime surface に昇格させず、distinct runtime-plan carrier を介して current `mir-runtime` の local queue / `MessageEnvelope` / membership / event-DAG floor へ流すべきである。keep すべきなのは `Place` / membership / envelope lane / event edge の reusable data であり、avoid すべきなのは `sample_id`、`current_owner`、`visible_history`、Docker/hot-plug/package completion を示唆する report-only fields や claim である。

## Objective

`P-A1-03` に入る前に、checked practical package から reusable local runtime へ渡す boundary を runtime/transport implementation の観点で絞り込み、keep/avoid fields、validation anchors、integration seams を明示する。

## Scope and assumptions

- この review は user 指定の source set のみを読む。
- `P-A1-02` の checker floor は runtime plan を emit していないという前提を保つ。
- current `mir-runtime` alpha local runtime surface は non-public Stage-B/F bridge であり、そのまま practical alpha-1 public/runtime-complete surface に昇格させない。
- Docker/local TCP、package hot-plug、runtime package admission、final public ABI はこの review の closeout 対象外である。

## Start state / dirty state

- start state: `main` branch
- worktree state before this review: clean (`git status --short` produced no output)
- no heavy build or generated artifact work was needed for this review-only task

## Documents consulted

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/alpha_local_runtime.rs`
- `crates/mir-runtime/tests/alpha_local_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_local_runtime.rs`
- `crates/mirrorea-core/src/fabric.rs`
- `crates/mirrorea-core/src/runtime.rs`
- `samples/alpha/local-runtime/README.md`
- `samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.expected.json`
- `samples/alpha/local-runtime/lr-02-stale_membership_rejected.expected.json`

## Actions taken

1. Read the practical alpha-1 scope/roadmap documents and the `P-A1-02` closeout report to restate the non-claim boundary around checker output vs. runtime plan.
2. Read the current `mir-runtime` alpha local runtime surface and its `mirrorea-core` substrate to identify which data is reusable execution input versus sample/report-specific output.
3. Compared the practical alpha-1 required path (`checker -> runtime plan -> local runtime or Docker runtime`) against the current Stage-B/F local runtime floor.
4. Wrote concrete findings for:
   - fields to keep in a new runtime-plan carrier
   - fields to avoid copying from checker reports or sample runtime reports
   - focused validation anchors for `P-A1-03`
   - likely integration seams between `mir-ast` practical package checking and `mir-runtime`

## Files changed

- `docs/reports/1142-review-p-a1-03-runtime-transport-boundary.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,240p' specs/18-*.md
sed -n '1,260p' plan/44-*.md
sed -n '1,260p' sub-agent-pro/alpha-1/06*.md
sed -n '1,260p' sub-agent-pro/alpha-1/09*.md
sed -n '1,260p' docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md
rg --files | rg 'mir-runtime|runtime'
sed -n '1,260p' crates/mir-runtime/src/lib.rs
sed -n '1,320p' crates/mir-runtime/src/alpha_local_runtime.rs
sed -n '321,980p' crates/mir-runtime/src/alpha_local_runtime.rs
sed -n '1,220p' crates/mir-runtime/examples/mirrorea_alpha_local_runtime.rs
sed -n '1,260p' crates/mir-runtime/tests/alpha_local_runtime.rs
sed -n '1,280p' crates/mirrorea-core/src/runtime.rs
sed -n '280,520p' crates/mirrorea-core/src/runtime.rs
sed -n '1,280p' crates/mirrorea-core/src/fabric.rs
sed -n '1,220p' samples/alpha/local-runtime/README.md
sed -n '1,220p' samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.expected.json
sed -n '1,220p' samples/alpha/local-runtime/lr-02-stale_membership_rejected.expected.json
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-runtime --test alpha_local_runtime
python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json
```

## Evidence / outputs / test results

- `specs/18` requires `checker -> runtime plan -> local runtime or Docker runtime` and explicitly says the current checker floor does not emit a runtime plan or claim local/Docker run, save/load, or devtools export.
- `plan/44` promotes `P-A1-03` specifically as `checked plan から local world 実行` and reopens the line only if checked package output is consumed through a distinct runtime-plan boundary.
- current `mir-runtime` alpha local runtime is still a report-builder surface over a narrow in-process queue floor:
  - reusable substrate exists in `LogicalPlaceRuntimeShell`, `LogicalPlaceRuntimeSnapshot`, and `MessageEnvelope`
  - positive/negative reports remain sample-shaped (`sample_id`, `current_owner`, `visible_history`, generated event ids)
  - retained-later refs explicitly keep `network_docker_runtime`, `runtime_package_avatar_admission`, `distributed_save_load`, and `final_public_abi` out of the current claim surface
- current dispatch evaluation checks membership presence/activity/place match, membership epoch/incarnation freshness, and destination registration, but does not yet execute capability, auth-evidence, witness, route-trace, or hot-plug semantics.
- current alpha local runtime README still says this family is not an active parser/runtime sample root and does not claim hot-plug lifecycle completion, runtime package/avatar admission, network/Docker runtime, distributed save/load completion, or final public ABI.
- validator evidence:
  - `python3 scripts/check_source_hierarchy.py`: required/present/missing = `73/73/0`
  - `python3 scripts/validate_docs.py`: `Documentation scaffold looks complete.` and `Found 1144 numbered report(s).`
  - `cargo fmt --check`: passed
  - `git diff --check`: passed
- focused runtime evidence:
  - `cargo test -p mir-runtime --test alpha_local_runtime`: passed `3` tests
  - `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json`: `stage_b_complete = true`, `passed = [LR-01, LR-02]`, supporting local save/load subset `passed = [CUT-04, CUT-17]`, `parser_runtime_bridge_claimed = false`, `distributed_save_load_claimed = false`

## What changed in understanding

- The practical runtime seam is already partially present, but it is below the current sample-facing report layer.
- The main risk for `P-A1-03` is not missing runtime primitives; it is accidentally reusing sample/report fields as if they were the reusable execution boundary.
- Capability/auth/witness data should survive into the runtime-plan and local envelope surfaces even before full enforcement exists, because those lanes are part of the boundary discipline and later transport/package work depends on them staying separate.

## Open questions

- Whether `P-A1-03` should lower directly from a checked practical package carrier into a runtime plan, or insert one more explicit `CheckedRuntimePlanInput` adapter type, remains open.
- Whether the first practical local runtime package should stay Sugoroku-shaped or immediately admit a more generic world/object package family remains open.
- Whether `P-A1-03` should emit a practical alpha-local JSON report in one shot or separate `run result` from `devtools export seed` remains open.

## Suggested next prompt

Implement `P-A1-03` by introducing a distinct runtime-plan carrier from checked practical packages into `mir-runtime`, wiring that carrier into `LogicalPlaceRuntimeShell` + local queue + `MessageEnvelope` execution, and keep Docker/hot-plug/save-load/package-admission claims explicitly deferred.

## Plan update status

`plan/` 更新不要: this task is a review report only and does not change repository memory or roadmap status.

## Documentation.md update status

`Documentation.md` 更新不要: this task records review findings only.

## progress.md update status

`progress.md` 更新不要: this task does not change current progress state.

## tasks.md update status

`tasks.md` 更新不要: this task does not reorder or close task packages.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample scope or validation state changed.

## Reviewer findings and follow-up

- Finding 1: `P-A1-03` should not consume the checker report directly as runtime input.
  - Basis:
    - `specs/18` says the current checker floor is a checker-only non-final surface and does not emit runtime plan or run-local/run-docker claims.
    - `plan/44` says `P-A1-03` reopens only if checked package output is consumed through a distinct runtime-plan boundary.
    - report `1139` already treats “checker report stays distinct from runtime plan” as the main honesty boundary.
  - Follow-up recommendation:
    - add a separate runtime-plan carrier between `mir-ast` checked package output and `mir-runtime`
    - keep the checker report as evidence/debug output, not as executable input

- Finding 2: keep `Place` registration, membership seeds/frontier, and envelope lanes; avoid sample/report-only fields.
  - Keep:
    - place ids and kinds
    - bootstrap participants / membership frontier data
    - queue kind or runtime transport selector for local in-process execution
    - `MessageEnvelope` lanes needed by the current substrate: principal claim, auth evidence slot, membership epoch, member incarnation, capability requirements, authorization checks, witness refs, payload ref/kind
    - event-DAG edge/node identities as execution output or export seed
  - Avoid:
    - `sample_id`
    - `current_owner`
    - `visible_history`
    - pre-baked `generated_event_refs` lists
    - sample-narrative `notes`
    - `retained_later_refs` as runtime input
    - `saved_state_format`, `serialized_state_bytes`, and other save/load report fields in the initial `P-A1-03` runtime-plan
  - Reason:
    - current local runtime reports are still Sugoroku/sample-shaped and are not the reusable execution API.

- Finding 3: keep capability/auth/witness lanes explicit, but do not imply their runtime enforcement is complete in `P-A1-03`.
  - Basis:
    - the fabric layer already exposes separate lanes for auth evidence, membership epoch/incarnation, capability requirements, authorization checks, and witness refs
    - the current local runtime dispatch evaluator checks membership and destination only, not full capability/auth/witness semantics
    - `specs/18` separates reusable runtime, package/hot-plug, and transport completion into later requirements
  - Follow-up recommendation:
    - preserve these lanes in the runtime-plan and local runtime input/output surfaces
    - describe them as carried/observable data for `P-A1-03`
    - avoid claiming typed capability reject, auth enforcement, witness consumption, or hot-plug verdict completion until later packages widen the runtime

- Finding 4: the best integration seam is `CheckedPracticalPackage -> RuntimePlan -> LogicalPlaceRuntimeShell + local queue<MessageEnvelope> -> execution report/event DAG`.
  - Basis:
    - `LogicalPlaceRuntimeShell` already supports place registration, participant registration, typed snapshots, and restore
    - `MessageEnvelope` already carries the lane separation expected by the runtime/transport boundary
    - current `build_local_sugoroku_runtime_report()` is a sample-specific builder over that thinner substrate
  - Follow-up recommendation:
    - do not teach `P-A1-03` to instantiate `LocalRuntimeReport` directly from checker artifacts
    - instead, lower checked package data into:
      - shell bootstrap plan
      - initial envelope queue
      - local runtime transport selector
      - devtools/event export seed

- Finding 5: `P-A1-03` validation should prove the new practical path while keeping the alpha local runtime floor green; it should not require Docker/hot-plug/save-load closure.
  - Minimum anchors for the `P-A1-03` package:
    - mandatory package floor:
      - `python3 scripts/check_source_hierarchy.py`
      - `python3 scripts/validate_docs.py`
      - `cargo fmt --check`
      - `git diff --check`
    - focused practical path:
      - `python3 scripts/practical_alpha1_check.py <practical-package> --format json`
      - `python3 scripts/practical_alpha1_run_local.py <practical-package> --format json`
    - focused runtime regression:
      - `cargo test -p mir-runtime --test alpha_local_runtime`
      - practical `mir-runtime` tests for the new runtime-plan lowering seam
      - `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json`
  - Avoid as closeout requirements for `P-A1-03`:
    - `run_docker`
    - route-trace over TCP/Docker
    - package attach/detach API
    - save/load command closure

## Skipped validations and reasons

- practical `run-local` / `run-docker` scripts from the `P-A1-03` target line were not run because this task is a boundary review and those practical commands are not the current implemented surface under review.
- no Docker/local TCP transport validation was run because this review explicitly avoids implying transport completion and the current checked evidence line under review is the in-process local runtime floor.
- no package hot-plug attach/detach validation was run because `P-A1-03` is being reviewed as the pre-hot-plug runtime-plan execution package.

## Commit / push status

Pending at report update time. A docs-only commit/push is expected after validators pass.

## Sub-agent session close status

- No sub-agent was used for this review.
