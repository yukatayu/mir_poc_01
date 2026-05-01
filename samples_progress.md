# samples_progress

Last updated: 2026-05-01 10:46 JST
Current repo-local focus: current-L2 / clean near-end runnable floor, Sugoroku world and avatar follow representative slices, typed external / network / projection / viewer helper evidence, hot-plug P21 narrow runtime floor, and actual `U1` commitment gate.
Current active packages: no new implementation package is promoted. Maintenance packages remain active. post-`P21` later-family docs-first trilogy is closed; the next product-shaping work is actual `U1` commitment.

## Legend

Progress:
- 0%: not scheduled
- 1%: started; sample ID and goal exist
- 10%: spec/sample skeleton exists
- 25%: parser/loader/static carrier exists
- 50%: minimal implementation passes primary positive sample
- 65%: negative/rejection samples pass
- 75%: debug/visualization output exists
- 90%: E2E/regression/closeout validation passes
- 100%: complete for current scope: implementation + positive/negative samples + debug/visualization + docs + report + tests + progress update + git commit/push

Notes:
- `100%` is always current-scope only.
- conceptual-only rows must stay at or below `25%`.
- helper-local preview, report-local inventory, and generated bridge evidence are not final public API.

## Summary

| Layer | Overall % | Status | Current focus | Next validation |
|---|---:|---|---|---|
| Mir core | 90 | active current layer | `samples/current-l2/` base corpus and current-L2 execution | `python3 scripts/current_l2_guided_samples.py closeout --format json` |
| clean near-end suite | 90 | active clean suite | typing / order-handoff / model-check / modal runnable floor | `python3 scripts/clean_near_end_samples.py closeout` |
| Lean / theorem | 89 | active proof bridge | small proof foundations + clean-near-end generated theorem stubs; live subject = `e5`, compare floor = `05_delegated_rng_service`, committed bridge floor = `samples/lean/foundations` + `samples/lean/clean-near-end`, `e2` remains foundation/contrast anchor | `python3 scripts/current_l2_lean_sample_sync.py` |
| Sugoroku runtime | 90 | active vertical slice | attach / membership / handoff / late join / detach TODO boundary | `python3 scripts/sugoroku_world_samples.py closeout --format json` |
| Avatar follow | 90 | active representative slice | `FAIRY-01/02/03/04/06`; `FAIRY-05` remains planned | `python3 scripts/avatar_follow_samples.py closeout --format json` |
| External adapters | 75 | synthetic preview + host-boundary inventory | `EXT-03` / `EXT-04` helper subset; `EXT-01/02/05` planned | `python3 scripts/typed_external_boundary_samples.py closeout --format json` |
| Network transport | 100 | helper-local canary family | `NET-02..05` current-scope canaries; `NET-01` は Sugoroku loopback parity anchor として reported | `python3 scripts/network_transport_samples.py check-all --format json` |
| Projection / placement | 90 | preview + generated bridge evidence | helper/report preview + committed manifest bridge | `python3 scripts/projection_codegen_samples.py closeout --format json` |
| Visualization / viewer | 100 | typed prototype inventory | helper/runtime typed panel and telemetry inventory | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` |
| Hot-plug package | 90 | P21 narrow runtime floor + docs-first trilogy closed | helper lifecycle + request/verdict carrier + runtime engine-state narrow floor; public ABI still unfrozen | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` |
| Storage / backend guardrail | 100 | current first-cut closeout | external workdir, cargo cache/target binding, LLVM staging visibility | `bash scripts/storage/detach_prepare.sh` |
| Docs / traceability | 90 | active maintenance | current snapshot, reports, source hierarchy, stale wording cleanup | `python3 scripts/check_source_hierarchy.py && python3 scripts/validate_docs.py` |

## Active sample matrix

| Sample ID | Layer | Path / command | Kind | Progress | Positive/Negative | Last validation | Docs / reports | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH0` | repository memory | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | dashboard / hierarchy check | 90 | mixed | 2026-05-01 10:26 JST | `0945`, `0996`, `0997`, `0998`, `1001`, `1053`, `1066` | Snapshot docs are maintenance artifacts, not normative specs |
| `PH1` | Mir current-L2 | `samples/current-l2/` | base corpus | 90 | positive + negative | 2026-05-01 10:26 JST | `0904`, `0913`, `0998`, `1066` | final parser / public API deferred |
| `PH6` | clean near-end | `samples/clean-near-end/` | active clean suite | 90 | positive + negative | 2026-05-01 10:26 JST | `0945`, `0959`, `0988`, `0989`, `0998`, `1066` | current canonical runnable suite |
| `SUG-01` | Sugoroku attach | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | active runnable | 90 | positive | 2026-05-01 10:26 JST | `0955`, `0977`, `0986`, `0997`, `0998`, `1066` | attach lifecycle / compatibility anchor |
| `SUG-03` | Sugoroku handoff | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | active runnable E2E | 90 | positive | 2026-05-01 10:26 JST | `0950`, `0952`, `0954`, `0955`, `0998`, `1066` | roll -> publish -> witness -> handoff |
| `SUG-05` | membership | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | active runnable E2E | 90 | positive | 2026-05-01 10:26 JST | `0955`, `0998`, `1066` | published history visible before turn insertion |
| `SUG-08` | reset model-check | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | active verification E2E | 90 | positive | 2026-05-01 10:26 JST | `0945`, `0998`, `1066` | reset safety bridge |
| `SUG-09` | hot-plug detach TODO | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | active TODO boundary | 75 | rejection evidence | 2026-05-01 10:26 JST | `0977`, `0979`, `0986`, `0994`, `0995`, `0998`, `1066` | explicit stop line; not completed migration / rollback |
| `FAIRY-01/02/03/04/06` | avatar follow | `samples/clean-near-end/avatar-follow/` | active representative slice | 90 | positive + negative + verification | 2026-05-01 10:26 JST | `0956`, `0978`, `0998`, `1066` | follow / fallback / stale-anchor rejection / detached-anchor safety |
| `FAIRY-05` | avatar follow | `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir` | planned family | 10 | target only | 2026-04-28 21:45 JST | `0956`, `0978` | carrier choice remains `UNRESOLVED`; not active |
| `EXT-03/04` | typed external | `scripts/typed_external_boundary_samples.py` | helper synthetic preview | 75 | positive + negative | 2026-05-01 10:26 JST | `0966`, `0998`, `1066` | helper synthetic preview over planned-path sources; not final adapter API |
| `EXT-01/02/05` | typed external | `samples/not_implemented/typed-external-boundary/` | planned residual family | 10 | target only | 2026-04-28 09:26 JST | `0946` | reopen criteria fixed; not active |
| `NET-02..05` | network transport | `scripts/network_transport_samples.py`, Sugoroku loopback mode | helper-local canary family | 100 | positive + negative | 2026-05-01 10:26 JST | `0967`, `0998`, `1066` | `NET-01` is only a reported Sugoroku loopback parity anchor; not production transport |
| `PRJ-01/02` | projection / placement | Sugoroku `projection_view`, runtime `cross_place_projection` | helper/report preview | 75 | positive | 2026-05-01 10:26 JST | `0948`, `0998`, `1066` | preview floor; not emitted executable program |
| `P15-GEN-01..04` | projection / codegen | `samples/generated/projection-placement/manifest.json` | committed generated bridge evidence | 90 | positive | 2026-05-01 10:26 JST | `0970`, `0998`, `1066` | generated artifact; not source sample / final executable |
| `P16-VIEW-01..05` | viewer | `scripts/visual_debugger_viewer_samples.py` | typed public prototype inventory | 100 | positive | 2026-05-01 10:26 JST | `0971`, `0998`, `1066` | typed inventory over helper/runtime surfaces; not final viewer API |
| `P19-HOTPLUG-CARRIER` | hot-plug carrier | `crates/mirrorea-core/src/fabric.rs` | Rust carrier | 90 | positive + negative | 2026-05-01 10:26 JST | `0988`, `0998`, `1066` | engine-neutral request/verdict carrier |
| `P20-HOTPLUG-SKELETON` | hot-plug runtime | `crates/mir-runtime/src/hotplug_runtime.rs` | thin runtime/report assembly | 90 | positive + negative | 2026-05-01 10:26 JST | `0989`, `0991`, `0998`, `1066` | not completed engine |
| `P21-HOTPLUG-ENGINE-STATE` | hot-plug runtime | `crates/mir-runtime/src/hotplug_runtime.rs` | narrow runtime engine-state floor | 90 | positive + negative | 2026-05-01 10:26 JST | `0993`, `0996`, `0997`, `0998`, `1066` | non-public runtime-private state progression; freeze prerequisite fixed, public ABI still unfrozen |
| `STORAGE-01` | storage / backend | `/mnt/mirrorea-work`, `scripts/env/`, `scripts/storage/` | operational guardrail | 100 | positive | 2026-05-01 10:46 JST | `0972`, `0998`, `1066`, `1072` | no destructive cleanup / no actual LLVM build |

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | What it does not prove |
|---|---|---|---:|---|---|
| `E2E-CLEAN-SUITE` | current-L2 -> clean near-end | `python3 scripts/clean_near_end_samples.py closeout` | 90 | active clean suite positive/negative floor | final language completion |
| `E2E-SUGOROKU` | membership -> attach -> roll -> publish -> witness -> handoff -> late join | `python3 scripts/sugoroku_world_samples.py closeout --format json` | 90 | current shared-space vertical slice | real network / durable distributed runtime |
| `E2E-AVATAR` | follow -> fallback -> stale-anchor rejection -> safety property | `python3 scripts/avatar_follow_samples.py closeout --format json` | 90 | representative avatar floor | `FAIRY-05` reacquire implementation |
| `E2E-TYPED-EXTERNAL` | typed effect request -> receipt/failure -> redacted view | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | 75 | helper host-boundary inventory | final adapter API / exact host schema |
| `E2E-TRANSPORT-CANARY` | loopback / subprocess bridge / reconnect / failure / redacted trace | `python3 scripts/network_transport_samples.py check-all --format json` | 100 | current-scope helper transport canary execution; `NET-01` parity remains separately reported from Sugoroku anchors | production socket / durable replay |
| `E2E-PROJECTION-BRIDGE` | system source -> preview -> committed manifest bridge -> live-anchor alignment | `python3 scripts/projection_codegen_samples.py closeout --format json` | 90 | generated bridge evidence alignment | final emitted executable family |
| `E2E-VIEWER-INVENTORY` | helper/runtime inventory -> typed viewer panels/telemetry | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | 100 | typed prototype inventory | final viewer / telemetry service |
| `E2E-HOTPLUG-RUNTIME` | request/verdict carrier -> runtime snapshot -> engine-state report | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | 90 | P21 narrow runtime floor | rollback / durable migration / distributed ordering / final ABI |

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|
| actual `U1` commitment | product / public boundary | high | repo + user | choose packaging target, host target, first shipped public surface, and final catalog breadth |
| final public parser / checker / runtime / verifier API | public-freeze mixed gate | high | repo + user | keep `P18` repo-side inventory separate from actual public freeze |
| final public adapter / host schema | typed external boundary | high | user + repo | choose host target before final adapter ABI |
| final public hot-plug ABI | hot-plug | high | repo + user | use `plan/38`; do not import helper/runtime-private names as public ABI |
| rollback / durable migration engine | hot-plug | high | repo | later implementation after `U1` / public-boundary choices |
| distributed activation ordering | hot-plug / transport | high | repo | later implementation after multi-place / durable activation pressure exists |
| real socket / durable replay | network transport | high | repo + user | preserve helper canaries while production transport remains deferred |
| actual LLVM build / backend choice | compiler / backend | medium | user + repo | preserve storage guardrail; choose backend target before heavy build |

## Recent validation

| Time | Command | Result | Notes |
|---|---|---|---|
| 2026-05-01 10:46 JST | storage guardrail freshness audit | pass with known storage warning | `df -h`, `free -h`, `lsblk -f`, `findmnt`, repo/external `du`, and `bash scripts/storage/detach_prepare.sh` ran. `/mnt/mirrorea-work` remained mounted with external workdir available; repo root stayed small; `detach_prepare.sh` reported the known `/mnt/mirrorea-work/llvm` root-owned warning, listed disposable candidates, and deleted no files. |
| 2026-05-01 10:26 JST | full validation + Lean/storage supplemental floor | pass with known storage warning | 16-command full floor passed at this checkpoint: source hierarchy / docs scaffold, current-L2, clean near-end, Sugoroku, avatar, typed external, network, projection, viewer, Cargo crate tests, `cargo fmt --check`, and `git diff --check`; log review found no actual Rust warnings / panics / failed targets. Supplemental `current_l2_lean_sample_sync.py` passed and left the tree clean. `detach_prepare.sh` passed with known `/mnt/mirrorea-work/llvm` root-owned warning and no deletion. |
| 2026-05-01 09:34 JST | warning-noise focused floor | pass | `RUSTFLAGS="-D warnings" cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike`、`RUSTFLAGS="-D warnings" cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe`、`cargo test -p mir-ast`、`cargo test -p mir-semantics`、`cargo fmt --check`、`git diff --check` passed; cleanup was item-level allow for target-specific shared support helpers, not sample behavior change |
| 2026-05-01 09:26 JST | full validation freshness floor | pass with existing warnings | source hierarchy / docs scaffold passed at that checkpoint; later guardrail packages expanded the required path set. current-L2 / clean near-end / Lean sync / Sugoroku / avatar / typed external / network / projection / viewer closeouts passed、`detach_prepare.sh` passed with known `/mnt/mirrorea-work/llvm` root-owned warning、Cargo floor and `cargo fmt --check` passed with existing dead-code warnings only、`git diff --check` passed |
| 2026-04-30 19:54 JST | model-check + mixed-helper focused floor | pass | `python3 scripts/clean_near_end_samples.py run model-check --format json`、`python3 scripts/current_l2_guided_samples.py smoke-all --format json`、`python3 scripts/current_l2_lean_sample_sync.py`、`cargo test -q -p mir-runtime --test current_l2_operational_cli`、`cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json`、`cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json`、`python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`git diff --check` passed; historical `p08/p09` prototype anchors stay report-local because the current accepted sample set does not re-run them |
| 2026-04-30 19:29 JST | theorem-side focused floor | pass with warnings | `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder`、`cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support`、`cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe`、`cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json`、`python3 scripts/clean_near_end_samples.py closeout --format json` passed; warnings are existing `dead_code` warnings in theorem support helpers |
| 2026-04-30 15:00 JST | `python3 scripts/check_source_hierarchy.py` | pass | historical checkpoint; source hierarchy passed before later required-path expansion |
| 2026-04-30 15:00 JST | `python3 scripts/validate_docs.py` | pass | documentation scaffold complete at that historical checkpoint |
| 2026-04-30 15:00 JST | current sample helper closeouts + Lean sync | pass | current-L2 compatibility wrapper, clean near-end, Sugoroku, avatar, typed external, network, projection, viewer closeouts passed; `current_l2_lean_sample_sync.py` refreshed `samples/lean/manifest.json` and left the working tree clean |
| 2026-04-30 15:00 JST | storage scripts | pass with warning | `/mnt/mirrorea-work` remained mounted; LLVM root is root-owned / non-writable; no files deleted |
| 2026-04-30 15:00 JST | Cargo validation floor | pass with warnings | `mir-ast`, `mirrorea-core`, `mir-runtime`, `mir-semantics`, and `cargo fmt --check` passed; warnings are existing dead-code / storage-env warnings |
| 2026-04-30 15:00 JST | `git diff --check` | pass | post-closeout docs diff was whitespace-clean |
| 2026-04-29 12:02 JST | `python3 scripts/check_source_hierarchy.py` | pass | historical checkpoint; source hierarchy passed before later required-path expansion |
| 2026-04-29 12:02 JST | `python3 scripts/validate_docs.py` | pass | documentation scaffold complete at that historical checkpoint |
| 2026-04-29 12:02 JST | current sample helper closeouts | pass | current-L2, clean near-end, Sugoroku, avatar, typed external, network, projection, viewer closeouts all passed |
| 2026-04-29 12:02 JST | storage scripts | pass with warning | `/mnt/mirrorea-work` mounted; LLVM root is root-owned / non-writable; no files deleted |
| 2026-04-29 12:02 JST | Cargo validation floor | pass with warnings | `mir-ast`, `mirrorea-core`, `mir-runtime`, `mir-semantics`, and `cargo fmt --check` passed; warnings are existing dead-code / storage-env warnings |
| 2026-04-29 12:02 JST | `git diff --check` | pass | docs freshness diff is whitespace-clean |
| 2026-04-29 11:50 JST | `cargo fmt --check` | pass | after applying Rust formatting cleanup |
| 2026-04-29 11:50 JST | `cargo test -p mir-ast` | pass | formatting cleanup touched `mir-ast`; existing dead-code warnings only |
| 2026-04-29 11:50 JST | `cargo test -p mirrorea-core` | pass | carriers 12/12 and runtime substrate 12/12 passed |
| 2026-04-29 11:50 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | clean near-end regression 27/27 passed |
| 2026-04-29 11:50 JST | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | pass | hot-plug runtime skeleton / engine-state regression 8/8 passed |
| 2026-04-29 11:50 JST | `git diff --check` | pass | formatting cleanup diff was whitespace-clean before commit `b213721` |
| 2026-04-29 04:47 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | final public hot-plug ABI family docs-first close kept public ABI unfrozen |
| 2026-04-29 04:47 JST | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | pass | runtime-private hot-plug anchor remained non-public |
| 2026-04-29 04:47 JST | `python3 scripts/check_source_hierarchy.py` | pass | `plan/38`, reports `0996` / `0997` present |
| 2026-04-29 04:47 JST | `python3 scripts/validate_docs.py` | pass | documentation scaffold complete at that checkpoint |
| 2026-04-29 04:47 JST | `git diff --check` | pass | post-`P21` final-public-hotplug docs-first diff was whitespace-clean |

## Historical / archived samples

- Historical samples live under `samples/old/` and are not active runnable roots.
- Planned skeletons live under `samples/not_implemented/` and must not be silently promoted.
- Generated bridge evidence lives under `samples/generated/` and is not source sample material.
- Prototype / compatibility anchors are historical or explanatory unless a current runner and validation row explicitly make them active.
