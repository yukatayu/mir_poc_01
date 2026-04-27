# samples_progress

Last updated: 2026-04-27 23:26 JST
Current repo-local focus: clean near-end current layer と Sugoroku world / avatar follow representative slice を runnable floor として維持しつつ、Mirrorea future-axis を sample-first / docs-first に段階 actualize する
Current active packages: promoted Typed external boundary residual planned family review、reopen Projection / placement residual emitted-program gate

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

## Summary

| Layer | Overall % | Status | Current focus | Next validation |
|---|---:|---|---|---|
| Mir core | 90 | active current layer | `samples/current-l2/` base corpus と current-L2 source execution を維持 | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` |
| Typing / checker | 90 | active clean suite | finite-index first fragment と negative corpus を維持 | `python3 scripts/clean_near_end_samples.py run typing --format json` |
| Order / handoff | 90 | active clean suite | relation family / witness / authority handoff を維持 | `python3 scripts/clean_near_end_samples.py run order-handoff --format json` |
| Model-check | 90 | active clean suite | mutex / weak-memory second line と Sugoroku reset safety を維持 | `python3 scripts/clean_near_end_samples.py run model-check --format json` |
| Lean / theorem | 89 | active proof bridge | committed Lean bridge と repo-local stub alignment を維持 | `python3 scripts/current_l2_lean_sample_sync.py` |
| Sugoroku runtime | 90 | active vertical slice | attach / membership / handoff / reset model-check + hot-plug debug lane を維持 | `python3 scripts/sugoroku_world_samples.py closeout --format json` |
| Avatar follow | 90 | widened active representative slice + single residual planned family | `FAIRY-01/02/03/04/06` を維持しつつ `FAIRY-05` は docs-first gate fixed / runnable widening deferred に保つ | `python3 scripts/avatar_follow_samples.py closeout --format json` |
| External adapters | 75 | synthetic preview helper + residual planned family | `EXT-03` / `EXT-04` synthetic preview subset を維持しつつ residual reopen gate を整理する | `python3 scripts/typed_external_boundary_samples.py closeout --format json` |
| Network transport | 75 | helper-local canary family | `NET-01..05` helper-local canary を維持しつつ real socket / durable replay を deferred に保つ | `python3 scripts/network_transport_samples.py closeout --format json` |
| Visualization | 90 | helper-local + report-local first cut | `visualization_views` / `telemetry_rows` / redaction naming を維持 | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` |
| Projection / placement | 75 | helper-local + report-local preview floor | `projection_view` と `cross_place_projection` を維持しつつ emitted-program gate を docs-first に残す | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` |
| Hot-plug package | 75 | helper-local lifecycle canary | `hotplug_lifecycle` / `--debug hotplug` / attach-detach telemetry を維持しつつ real migration を deferred に保つ | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` |

## Active sample matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH0` | repository memory | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | dashboard / hierarchy check | 90 | mixed | 2026-04-27 23:26 JST | `0913`, `0920`, `0943` | source hierarchy と report discipline の current floor |
| `PH1` | Mir core | `samples/current-l2/` | base corpus | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | final parser / public API deferred |
| `PH6` | compile-ready minimal actualization | `samples/clean-near-end/` | active clean suite | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | public shell / packaging deferred |
| `SUG-01` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | active runnable | 90 | positive | 2026-04-27 19:56 JST | `0909`, `0916`, `0931` | runtime attach floor + helper-local attachpoint compatibility / activation evidence |
| `SUG-03` | Sugoroku runtime | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | active runnable E2E | 90 | positive | 2026-04-27 23:24 JST | `0909`, `0916`, `0918`, `0919`, `0942` | roll -> publish -> witness -> handoff。projection / visualization debug lane の基準点 |
| `SUG-05` | shared-space membership | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:59 JST | `0909`, `0916`, `0919` | membership timeline anchor |
| `SUG-08` | theorem / model-check boundary | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | reset safety bridge |
| `SUG-09` | hot-plug preview | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | active TODO boundary | 75 | explicit TODO + rejection evidence | 2026-04-27 19:56 JST | `0909`, `0916`, `0925`, `0931` | completion evidence ではなく stop line |
| `FAIRY-01` | avatar follow | `samples/clean-near-end/avatar-follow/01_follow_remote_head_with_local_fallback.mir` | active representative canary | 90 | positive | 2026-04-27 19:35 JST | `0917`, `0930` | visible remote head follow with explicit local fallback lineage |
| `FAIRY-02` | avatar follow | `samples/clean-near-end/avatar-follow/02_remote_head_not_visible_falls_back_to_local.mir` | active representative canary | 90 | positive + fallback | 2026-04-27 20:55 JST | `0917`, `0930`, `0933` | visibility loss falls back locally and does not claim transport recovery |
| `FAIRY-06` | avatar follow | `samples/clean-near-end/avatar-follow/06_model_check_no_detached_anchor_observed.mir` | active representative canary | 90 | verification | 2026-04-27 19:35 JST | `0917`, `0930` | detached-anchor safety verification bridge |
| `AUTH-01` | auth seam | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local envelope carrier | 90 | positive + negative | 2026-04-27 23:24 JST | `0921`, `0941` | local queue baseline with `auth none`, membership freshness, capability refs, witness refs |
| `VIS-01` | visualization | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local view/telemetry carrier | 90 | positive | 2026-04-27 23:24 JST | `0922`, `0942` | label / authority / redaction を持つ evidence-oriented view |
| `PRJ-01` | projection / placement | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local projection preview | 75 | positive | 2026-04-27 23:24 JST | `0924`, `0942` | `projection_view` で system source から place split と observer view refs を visible にする。final emitted place program ではない |
| `PRJ-02` | projection / placement | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | report-local provider placement preview | 75 | positive | 2026-04-27 23:24 JST | `0924`, `0942` | `cross_place_projection` で authority placement と provider placement を分けて visible にする |

## Base corpus / planned / spec-only matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH2` | parser-free PoC | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | detached loop | 75 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | dedicated detached-loop CLI refresh remains open |
| `PH3` | parser / checker cut | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | crate test surface | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | parser grammar freeze deferred |
| `PH4` | shared-space membership / room boundary | `samples/clean-near-end/sugoroku-world/`, `plan/16-shared-space-membership-and-example-boundary.md` | runnable room boundary | 90 | positive + negative | 2026-04-27 19:56 JST | `0909`, `0916` | late join / leave / owner reassign / history visibility が current anchor |
| `PH5` | theorem / model-check boundary | `samples/clean-near-end/{typing,model-check}`, `samples/lean/` | active bridge family | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | proof/model-check public contract absent |
| `PH7` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/`, `scripts/sugoroku_world_samples.py` | active vertical slice | 90 | positive + negative | 2026-04-27 19:56 JST | `0909`, `0916`, `0931` | attach / handoff / witness / reset model-check / detach TODO boundary |
| `PH8` | avatar follow | `samples/clean-near-end/avatar-follow/`, `samples/not_implemented/avatar-fairy-follow/` | widened active representative slice + residual planned family | 90 | positive + negative + verification | 2026-04-27 20:55 JST | `0917`, `0930`, `0933`, `0939` | active canary は `FAIRY-01/02/03/04/06`、residual planned family は `FAIRY-05` |
| `FAIRY-05` | avatar follow | `samples/not_implemented/avatar-fairy-follow/FAIRY-05_reacquire_after_return.md` | planned family | 10 | target only | 2026-04-27 21:38 JST | `0939` | explicit state timeline / anchor switch evidence が reopen 条件。carrier bundling は `UNRESOLVED` |
| `PH9` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview subset + residual planned family | 75 | positive + negative | 2026-04-27 23:24 JST | `0923`, `0941` | phase 9 `.mir` direct semantic execution ではなく helper self-consistency + anchor comparison |
| `EXT-01/02/05` | typed external boundary | `samples/not_implemented/typed-external-boundary/` | residual planned family | 10 | target only | 2026-04-27 21:55 JST | `0923`, `0941` | final host-facing contract は mixed gate |
| `EXT-03/04` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview helper subset | 75 | positive + negative | 2026-04-27 23:24 JST | `0941` | typed adapter failure lane、envelope split、redacted visualization lane |
| `PH10` | MessageEnvelope / auth seam | Sugoroku helper、clean near-end runtime report | helper-local + report-local carrier | 90 | positive + negative | 2026-04-27 23:24 JST | `0921` | transport / auth / membership / capability / witness を collapse しない baseline |
| `PH11` | TermSignature / LayerSignature | Sugoroku helper、clean near-end runtime report | helper-local + report-local inventory | 90 | positive | 2026-04-27 15:59 JST | `0918`, `0919` | final shared law schema ではない first cut |
| `PH12` | projection / placement | `plan/20-projection-and-placement-roadmap.md`, `docs/hands_on/projection_placement_views_01.md` | helper/report-local preview floor | 75 | positive | 2026-04-27 23:24 JST | `0924`, `0942` | preview floor only。final emitted program / optimizer / equivalence checker は deferred |
| `PH13` | network transport | `scripts/network_transport_samples.py`, `samples/not_implemented/network-transport/` | helper-local canary family + planned source family | 75 | positive + negative | 2026-04-27 20:27 JST | `0926`, `0929`, `0932` | local queue / loopback / reconnect / typed failure / redacted route trace |
| `NET-01` | network transport | `scripts/sugoroku_world_samples.py --transport loopback_socket` | helper-local loopback preview | 75 | positive + negative | 2026-04-27 20:27 JST | `0929`, `0932` | same-process parity preview |
| `NET-02..05` | network transport | `scripts/network_transport_samples.py` | helper-local canary family | 75 | positive + negative | 2026-04-27 20:27 JST | `0932` | subprocess JSON bridge / stale reconnect reject / typed failure / redacted route trace |
| `PH14` | hot-plug package | `plan/21-hotplug-attachpoint-roadmap.md`, `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | helper-local lifecycle canary + docs-first gate | 75 | positive + negative | 2026-04-27 19:56 JST | `0925`, `0931` | real migration / rollback / attachpoint runtime remain deferred |
| `PH15` | visualization / IDE | Sugoroku helper views、runtime report-local views | helper-local + report-local first cut | 90 | positive | 2026-04-27 23:24 JST | `0922` | final public viewer contract は deferred |
| `PH16` | compiler / backend / LLVM preparation | `/mnt/mirrorea-work`, `scripts/env/mirrorea_storage_env.sh`, `plan/23-compiler-backend-llvm-guardrail-roadmap.md` | storage / backend guardrail | 75 | operational positive | 2026-04-27 18:48 JST | `0913`, `0915`, `0927` | actual LLVM build / backend choice is still later |

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | Last result |
|---|---|---|---:|---|---|
| `E2E-CLEAN-SUITE` | current-L2 -> clean near-end | `python3 scripts/current_l2_guided_samples.py closeout --format json` | 90 | current-L2 / clean suite floor が still green | pass 2026-04-27 18:55 JST |
| `E2E-SUGOROKU` | membership -> attach -> roll -> publish -> handoff -> late join | `python3 scripts/sugoroku_world_samples.py closeout --format json` | 90 | current shared-space vertical slice の runnable floor | pass 2026-04-27 21:10 JST |
| `E2E-AVATAR` | follow -> fallback -> stale-anchor rejection -> safety property | `python3 scripts/avatar_follow_samples.py closeout --format json` | 90 | representative avatar slice の active floor | pass 2026-04-27 20:55 JST |
| `E2E-TYPED-EXTERNAL-TARGET` | source stub -> adapter preview -> anchor comparison | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | 75 | helper self-consistency + anchor comparison の current floor。phase 9 `.mir` direct execution ではない | pass 2026-04-27 23:18 JST |
| `E2E-TRANSPORT-CANARY` | loopback / reconnect / failure / redacted trace | `python3 scripts/network_transport_samples.py closeout --format json` | 75 | helper-local transport canary family | pass 2026-04-27 20:27 JST |
| `E2E-PROJECTION-TARGET` | system source -> emitted place program -> equivalent trace | future target: current `PRJ-01` / `PRJ-02` preview floor + later emitted-program runner | 10 | compositional projection E2E は still later。current package で actualize したのは helper/report-local preview floor だけ | target only |

## Build / storage environment

| Item | Status | Validation | Notes |
|---|---|---|---|
| external workdir `/mnt/mirrorea-work` | active | `findmnt /mnt/mirrorea-work` | `/dev/vdb1` ext4 mount、`target/` SSD cutover 済み |
| cargo registry cache | active guardrail | `bash scripts/env/mirrorea_storage_env.sh` | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache` を intended binding にしている |
| LLVM path readiness | guardrail only | `ls -ld /mnt/mirrorea-work/llvm/*` | directory はあるが actual LLVM build はまだ |

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|
| final public adapter / host schema scope | typed external boundary | high | user + repo | synthetic preview subset と public host-facing contract を分けたまま residual review を進める |
| emitted program / equivalence checker gate | projection / placement | high | repo | helper/report preview と final artifact family を docs-first に切り分ける |
| `FAIRY-05` carrier bundling | avatar follow | medium | repo | explicit state timeline / anchor switch evidence を保ったまま `UNRESOLVED` を解く |
| real migration / rollback | hot-plug | medium | repo | helper-local lifecycle canary の先に attachpoint migration contract を切り出す |
| real socket / durable replay | network transport | high | repo + user | helper-local canary を維持しつつ production transport line は defer する |
| actual LLVM build / backend choice | compiler / backend | medium | user + repo | guardrail を壊さず、later package で backend target を決める |

## Recent validation

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-04-27 23:24 JST | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | phase 9 current reading は synthetic preview helper + anchor comparison のまま |
| 2026-04-27 23:24 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` | pass | `projection_view` は helper-local preview floor のまま |
| 2026-04-27 23:24 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | `cross_place_projection` inventory は current report-local preview と一致 |
| 2026-04-27 23:26 JST | `python3 scripts/check_source_hierarchy.py` | pass | required source hierarchy is intact after `0943` report add |
| 2026-04-27 23:26 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete after snapshot cleanup and report add |
| 2026-04-27 23:26 JST | `git diff --check` | pass | whitespace-clean after final docs sync |
| 2026-04-27 22:52 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side projection view test included |
| 2026-04-27 22:52 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side `cross_place_projection` inventory test included |
| 2026-04-27 21:55 JST | `python3 scripts/typed_external_boundary_samples.py check-all --format json` | pass | `EXT-03` / `EXT-04` helper subset current floor is green |
| 2026-04-27 20:55 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | active avatar slice と residual `FAIRY-05` 分離が current queue と一致 |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | `NET-02..05` helper-local canary family is green |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` | pass | `SUG-09` は explicit stop line として維持されている |

## Historical / archived samples

| Old path | New path / status | Reason |
|---|---|---|
| `samples/prototype/*` | historical prototype | active runnable floor と混同しないため |
| `samples/old/2026-04-22-pre-clean-near-end/` | archive | pre-clean current-L2 sample line を保存 |
| `samples/not_implemented/avatar-fairy-follow/` | planned family | active avatar slice と residual gate を分離するため |
| `samples/not_implemented/typed-external-boundary/` | planned family + synthetic preview source stubs | phase 9 active execution と混同しないため |
| `samples/not_implemented/network-transport/` | planned family | helper-local canary と future source family を分離するため |
