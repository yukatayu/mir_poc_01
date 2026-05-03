# Report 1157 — Review P-A1-05 Runtime/Transport Carrier Split

- Date: 2026-05-03 19:50:52 JST
- Author / agent: Codex
- Scope: Review the current runtime/transport code after `P-A1-04c` and determine the narrowest honest practical-transport carrier split for `P-A1-05`, including how to reuse the existing Stage-C runtime without reintroducing sample-ID keyed semantics.
- Decision levels touched: Review only over existing `L1`/`L2` material in `specs/18`, `plan/22`, and `plan/44`. No normative or roadmap text changed in this task.

## Objective

Inspect the current practical alpha-1 local-runtime line and the existing Stage-C network runtime, then answer two review questions:

1. what is the narrowest honest carrier split for practical transport after `P-A1-04c`, and
2. which parts of the existing Stage-C runtime can be reused for `P-A1-05` without falling back to `NET-*` sample-ID theater.

## Scope and assumptions

- Scope is limited to the requested target files plus the required repo/spec context and directly adjacent prior reports.
- This task is review-only and writes one new report under `docs/reports/`.
- `P-A1-03` remains the current practical local-runtime floor:
  `checked package -> runtime plan -> local runtime report`.
- `P-A1-04c` remains the current practical hot-plug floor:
  `checked package -> hotplug plan -> non-final hot-plug report`.
- `P-A1-05` must not overclaim production WAN/federation, final public transport ABI, save/load, devtools completion, or final product/runtime completion.
- If a shape is not already settled in the repo, this report marks it as recommendation or `UNRESOLVED`, rather than silently promoting it to fact.

## Start state / dirty state

- start state: `main`
- worktree state before this review: clean (`git status --short` produced no output)
- no pre-existing dirty changes were present in the reviewed target files at review start

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
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/22-network-transport-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/reports/1142-review-p-a1-03-runtime-transport-boundary.md`
- `docs/reports/1154-review-p-a1-04c-runtime-transport-boundary.md`
- `docs/reports/1155-p-a1-04c-practical-detach-minimal-contract.md`
- `crates/mir-runtime/src/alpha_network_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_runtime_plan.rs`
- `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`
- `scripts/alpha_network_docker_e2e.py`
- `scripts/practical_alpha1_run_local.py`

## Actions taken

1. Read the required repo entrypoint docs and the practical alpha-1 / transport roadmap documents.
2. Read the adjacent `P-A1-03`, `P-A1-04c`, and transport-boundary review reports to establish the current non-claim boundary.
3. Inspected the practical package schema and runtime-plan carrier to identify which lanes already exist for transport reuse.
4. Inspected the practical local-runtime implementation to separate reusable execution input from sample-shaped runtime-report output.
5. Inspected the Stage-C alpha network runtime, example CLI, and Docker E2E script to determine which parts are generic transport logic versus `NET-*` sample wrappers.
6. Re-ran one focused practical local-runtime test target, the Stage-C runtime closeout example, and the practical run-local closeout script for evidence.
7. Wrote this report with concrete findings and a recommended narrow carrier split for `P-A1-05`.

## Files changed

- `docs/reports/1156-review-p-a1-05-runtime-transport-carrier-split.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,240p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,240p' plan/00-index.md
sed -n '1,260p' plan/09-helper-stack-and-responsibility-map.md
sed -n '1,260p' plan/22-network-transport-roadmap.md
sed -n '1,300p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' docs/reports/1142-review-p-a1-03-runtime-transport-boundary.md
sed -n '1,260p' docs/reports/1154-review-p-a1-04c-runtime-transport-boundary.md
sed -n '1,220p' docs/reports/1155-p-a1-04c-practical-detach-minimal-contract.md
nl -ba crates/mir-runtime/src/alpha_network_runtime.rs | sed -n '1,1040p'
nl -ba crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs | sed -n '1,260p'
nl -ba crates/mir-ast/src/practical_alpha1.rs | sed -n '1,360p'
nl -ba crates/mir-ast/src/practical_alpha1_runtime_plan.rs | sed -n '1,520p'
nl -ba crates/mir-runtime/src/practical_alpha1_local_runtime.rs | sed -n '1,560p'
nl -ba scripts/alpha_network_docker_e2e.py | sed -n '1,320p'
nl -ba scripts/practical_alpha1_run_local.py | sed -n '1,320p'
rg -n "pub fn |fn build_|sample_id|transport_surface|required_witness_refs|observer_route_trace|what_it_proves|run_participant_client|run_world_server|NetworkScenario" crates/mir-runtime/src/alpha_network_runtime.rs
rg -n "current_owner|visible_history|build_runtime_observations|evaluate_dispatch|run_local_claimed|runtime_plan_emitted|requested_layers|message_envelopes" crates/mir-runtime/src/practical_alpha1_local_runtime.rs
rg -n "sample_id|run_docker_claimed|check-all|runtime_plan_boundary_present|implemented_rows|closeout" scripts/practical_alpha1_run_local.py
rg -n "sample_id|compose_transport_surface|expected_sidecar|check-all|closeout|stage-c-closeout|IMPLEMENTED_ROWS|PLANNED_ONLY_ROWS" scripts/alpha_network_docker_e2e.py
date '+%Y-%m-%d %H:%M:%S %Z'
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout
python3 scripts/practical_alpha1_run_local.py closeout --format json
```

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
  - passed `4/4`
- `cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout`
  - re-emitted all current Stage-C loopback reports for `NET-02/03/04/05/07/09`
  - confirmed that the reusable Stage-C payload is still carried as `MessageEnvelope` + explicit admission checks + route trace + optional auth lane
  - confirmed that the same file also still derives semantics from `NetworkScenario`
- `python3 scripts/practical_alpha1_run_local.py closeout --format json`
  - `implemented_rows = ["RUN-01", "RUN-02"]`
  - `local_runtime_first_floor_complete = true`
  - `runtime_plan_boundary_present = true`
  - `run_docker_claimed = false`
  - `package_hotplug_claimed = false`
  - `save_load_claimed = false`

Key code evidence used in the review:

- the practical package front door currently exposes checker/runtime/hotplug sections, but no distinct transport section:
  - `crates/mir-ast/src/practical_alpha1.rs:86-108`
- the practical runtime input already carries most transport-relevant lanes:
  - `crates/mir-ast/src/practical_alpha1.rs:253-356`
- the practical runtime-plan carrier stays explicitly below local runtime / Docker / hot-plug / save-load:
  - `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:18-41`
  - `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:174-195`
  - `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:198-403`
- the practical local-runtime report includes sample/product-shaped fields and Sugoroku-specific event/report shaping:
  - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:77-121`
  - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:166-234`
  - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:366-534`
- the Stage-C runtime has reusable transport/admission logic, but it is wrapped by `NetworkScenario` and `NET-*` sample mapping:
  - `crates/mir-runtime/src/alpha_network_runtime.rs:44-245`
  - `crates/mir-runtime/src/alpha_network_runtime.rs:596-703`
  - `crates/mir-runtime/src/alpha_network_runtime.rs:706-1031`
- the Stage-C example CLI is also sample-ID keyed:
  - `crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs:21-31`
  - `crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs:40-117`
- both Python runners are still exact-report regression wrappers keyed by finite sample rows:
  - `scripts/practical_alpha1_run_local.py:16-29`
  - `scripts/practical_alpha1_run_local.py:98-163`
  - `scripts/alpha_network_docker_e2e.py:23-66`
  - `scripts/alpha_network_docker_e2e.py:137-219`
  - `scripts/alpha_network_docker_e2e.py:228-345`

## What changed in understanding

- The practical line already has enough runtime-plan data to seed a transport floor without routing through the local-runtime report.
- The Stage-C runtime is split into two layers:
  - a genuinely reusable lower layer that evaluates a typed wire request against a shell snapshot and emits typed admission / trace output
  - a sample-specific upper layer that picks semantics by `NET-*` identifier
- The overclaim risk for `P-A1-05` is therefore not “reusing Stage-C code” by itself.
  The risk is reusing the `NetworkScenario` wrapper and pretending that mapped sample IDs are a practical transport carrier.

## Open questions

- `UNRESOLVED`: should the practical transport carrier be named a new `PracticalAlpha1TransportPlan`, or a thinner `transport input` overlay derived from `PracticalAlpha1RuntimePlan` plus CLI execution options?
- `UNRESOLVED`: should the first practical transport cut admit only `local_tcp`, or both `local_tcp` and Docker in the same package?
- `UNRESOLVED`: should bind/listen address and Docker-compose-specific details live outside the semantic carrier as execution options, or be partially reflected in a transport-plan/report surface?

## Suggested next prompt

`Implement P-A1-05 by introducing a distinct practical transport carrier below the local-runtime report and above the Stage-C socket bridge, extract the generic request/admission/route-trace engine from alpha_network_runtime, keep NET-* wrappers as alpha-0 evidence-only adapters, and validate the new path from package path rather than sample ID.`

## Plan update status

`plan/` 更新不要:
review only; no repository-memory decision or roadmap state changed in this task.

## Documentation.md update status

`Documentation.md` 更新不要:
review only; no snapshot wording changed in this task.

## progress.md update status

`progress.md` 更新不要:
review only; no progress state changed in this task.

## tasks.md update status

`tasks.md` 更新不要:
review only; no task-map state changed in this task.

## samples_progress.md update status

`samples_progress.md` 更新不要:
review only; no runnable sample state changed in this task.

## Reviewer findings and follow-up

1. `alpha_network_runtime` is not directly reusable as the practical transport carrier because it still derives semantics from `NET-*` sample identity.
   - Evidence:
     - `NetworkScenario::parse()` selects the scenario from `sample_id`, then the scenario chooses capabilities, witnesses, auth evidence, payload ref, expected outcome, and proof wording:
       `crates/mir-runtime/src/alpha_network_runtime.rs:53-245`
     - the wire request is built from that scenario, not from an external transport-plan carrier:
       `crates/mir-runtime/src/alpha_network_runtime.rs:655-703`
     - world bootstrap also depends on the scenario:
       `crates/mir-runtime/src/alpha_network_runtime.rs:907-919`
     - the example CLI only exposes `run <NET-..>` and `participant-client <NET-..>` entrypoints:
       `crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs:12-17`
       `crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs:40-117`
   - Why this matters:
     - If `P-A1-05` simply maps package paths to invented `NET-*` identifiers, then carrier meaning still lives in the scenario table rather than in the practical package/runtime data.
     - That would be sample-ID theater, not a reusable practical transport surface.
   - Follow-up recommendation:
     - extract the generic transport evaluator beneath `NetworkScenario`
     - keep `NetworkScenario` and `NET-*` only as the alpha-0 Stage-C evidence wrapper

2. The narrowest honest practical split is not `checked package -> Docker/TCP report`, and not `runtime report -> Docker/TCP report`; it is a new carrier below the local-runtime report and above the socket bridge.
   - Evidence:
     - the practical package already carries runtime places, bootstrap participants, membership advances, envelopes, principal claim, auth evidence, capability requirements, authorization checks, and witness refs:
       `crates/mir-ast/src/practical_alpha1.rs:253-356`
     - the runtime-plan floor explicitly keeps Docker transport later:
       `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:23-29`
       `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:31-41`
     - the runtime-plan builder already validates the world/bootstrap/envelope inputs and guarantees a checked package first:
       `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:128-195`
       `crates/mir-ast/src/practical_alpha1_runtime_plan.rs:198-403`
   - Recommended split:
     - keep the already-actualized carrier:
       `checked package -> runtime plan`
     - introduce a new non-final transport carrier:
       `runtime plan -> practical transport plan/input`
     - then emit a distinct transport report:
       `practical transport plan/input -> local TCP or Docker runtime report`
   - Keep in the new transport carrier:
     - `package_id`, `world_id`, `entry_place`
     - runtime shell seeds:
       `runtime_places`, `bootstrap_participants`, `pre_dispatch_membership_advances`
     - current request envelope data:
       principal claim, auth evidence, membership epoch/incarnation, capability requirements, authorization checks, witness refs
     - requested transport surface/medium as a non-final execution selector
   - Avoid in the new transport carrier:
     - `sample_id`
     - `sample_name`
     - `what_it_proves`
     - Stage-C report narration
     - local-runtime report outputs such as owner/history/event DAG
   - Naming is still `UNRESOLVED`, but the carrier boundary itself is the narrowest honest cut.

3. `PracticalAlpha1LocalRuntimeReport` is too sample-shaped to be the upstream input for transport reuse.
   - Evidence:
     - the report surface includes `current_owner`, `visible_history`, full `event_dag`, and runtime-side claim flags:
       `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:77-121`
     - the report is built after executing the local queue and deriving Sugoroku-specific observations:
       `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:166-234`
     - event IDs and visible history are hard-coded to the current Sugoroku slice:
       `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:373-397`
       `crates/mir-runtime/src/practical_alpha1_local_runtime.rs:432-505`
   - Why this matters:
     - those fields are valid local-runtime evidence, but they already sit after execution and encode current sample/product behavior.
     - reusing them as transport input would couple `P-A1-05` to `RUN-01/02` output theater and blur transport with world-state execution.
   - Follow-up recommendation:
     - reuse the common upstream data and substrate:
       `PracticalAlpha1RuntimePlan`, `LogicalPlaceRuntimeShell`, `MessageEnvelope`
     - do not route `P-A1-05` through `PracticalAlpha1LocalRuntimeReport`

4. The reusable Stage-C kernel is the request/admission/observer-trace engine, not the `NET-*` report builder.
   - Evidence:
     - the generic wire carrier shape already exists in:
       `TransportWireRequest` / `TransportWireResponse`
       `crates/mir-runtime/src/alpha_network_runtime.rs:322-347`
     - the reusable report assembly boundary is:
       `crates/mir-runtime/src/alpha_network_runtime.rs:596-653`
     - the admission logic is content-driven over envelope/membership/capability/witness/auth lanes:
       `crates/mir-runtime/src/alpha_network_runtime.rs:706-905`
     - route-trace and auth-lane shaping are also generic once the request exists:
       `crates/mir-runtime/src/alpha_network_runtime.rs:921-1001`
   - Why this matters:
     - these are the pieces that should be extracted or wrapped for `P-A1-05`.
     - they already prove the desired lane separation:
       transport, auth, membership, capability, witness, and observer-safe trace.
   - Follow-up recommendation:
     - feed this kernel from a practical transport carrier built from runtime-plan data
     - replace `bootstrap_network_shell(scenario)` with bootstrap-from-plan
     - replace scenario-based `build_wire_request()` with lowering from practical transport input
     - keep Stage-C sample builders as regression adapters over the extracted engine

5. The current script surfaces confirm the sample-wrapper boundary and should not be mistaken for the future practical transport CLI.
   - Evidence:
     - `scripts/practical_alpha1_run_local.py` only knows `RUN-01/02` and validates exact report equality:
       `scripts/practical_alpha1_run_local.py:16-29`
       `scripts/practical_alpha1_run_local.py:98-137`
     - `scripts/alpha_network_docker_e2e.py` only knows `NET-02/03/04/05/07/09`, Docker-compose-specific env vars, and expected sidecars:
       `scripts/alpha_network_docker_e2e.py:23-66`
       `scripts/alpha_network_docker_e2e.py:137-219`
       `scripts/alpha_network_docker_e2e.py:228-345`
   - Why this matters:
     - these scripts are valid evidence runners for the current floors, but they are not yet neutral transport front doors.
   - Follow-up recommendation:
     - keep them as regression surfaces
     - add a separate practical transport runner that accepts a practical package path plus explicit transport mode, then emits/compares a transport report without a `NET-*` lookup table

## Skipped validations and reasons

- Did not run `python3 scripts/alpha_network_docker_e2e.py check-all --format json` because it depends on Docker/Compose availability and this task was a carrier-boundary review, not a Docker closeout or behavioral change.
- Did not run practical hot-plug or save/load validations because the review question was limited to runtime/transport reuse for `P-A1-05`.
- Did not run `cargo test -p mir-runtime --test alpha_network_runtime` because the loopback closeout example already re-executed all current Stage-C report builders for the purposes of this review.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent session was opened.
