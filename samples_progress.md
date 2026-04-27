# samples_progress

Last updated: 2026-04-27 22:23 JST
Current repo-local focus: clean near-end current layer と Sugoroku world vertical slice を runnable floor として維持しつつ、Mirrorea future-axis を sample-first / docs-first に段階 actualize する
Current active packages: promoted Projection / placement executable widening、reopen Typed external boundary residual planned family review

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
| Model-check | 90 | active clean suite | mutex / weak-memory second line と Sugoroku reset safety | `python3 scripts/clean_near_end_samples.py run model-check --format json` |
| Lean / theorem | 89 | active proof bridge | committed Lean bridge と repo-local stub alignment を維持 | `python3 scripts/current_l2_lean_sample_sync.py` |
| Sugoroku runtime | 90 | active vertical slice | attach / membership / handoff / reset model-check + hot-plug debug lane | `python3 scripts/sugoroku_world_samples.py closeout --format json` |
| Avatar follow | 90 | widened active representative slice + single residual planned family | active helper canary `FAIRY-01/02/03/04/06` を維持しつつ `FAIRY-05` は docs-first gate fixed / exact carrier bundling unresolved / runnable widening deferred として保つ | `python3 scripts/avatar_follow_samples.py closeout --format json` |
| External adapters | 75 | synthetic preview helper + residual planned family | `EXT-03` / `EXT-04` synthetic preview subset を維持しつつ `EXT-01` / `EXT-02` / `EXT-05` residual reopen gate を保つ | `python3 scripts/typed_external_boundary_samples.py closeout --format json` |
| Network transport | 75 | helper-local canary family | `NET-01..05` helper-local canary を維持しつつ real socket / session / durable replay を deferred に保つ | `python3 scripts/network_transport_samples.py check-all --format json` |
| Visualization | 90 | helper-local + report-local first cut | `visualization_views` / `telemetry_rows` / redaction naming を維持しつつ future executable widening に備える | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` |
| Projection / placement | 10 | docs-first plan | `plan/20` の validity checklist と stop line を維持しつつ future transport / hot-plug widening に備える | `python3 scripts/validate_docs.py` |
| Hot-plug package | 75 | helper-local lifecycle canary | `hotplug_lifecycle` / `--debug hotplug` / attach-detach telemetry を維持しつつ real migration / rollback を deferred に保つ | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` |

## Active sample matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH0` | repository memory | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | dashboard / hierarchy check | 90 | mixed | 2026-04-27 17:03 JST | `0913`, `0920` | source hierarchy と report discipline の current floor |
| `PH1` | Mir core | `samples/current-l2/` | base corpus | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | final parser / public API deferred |
| `PH2` | parser-free PoC | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | detached loop | 75 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | dedicated detached-loop CLI refresh remains open |
| `PH3` | parser / checker cut | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | crate test surface | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | parser grammar freeze deferred |
| `PH5` | theorem / model-check boundary | `samples/clean-near-end/{typing,model-check}`, `samples/lean/` | active bridge family | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | proof/model-check public contract absent |
| `PH6` | compile-ready minimal actualization | `samples/clean-near-end/` | active clean suite | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | public shell / packaging deferred |
| `SUG-00` | shared-space membership | `samples/clean-near-end/sugoroku-world/00_world_bootstrap.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | empty world baseline |
| `SUG-01` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | active runnable | 90 | positive | 2026-04-27 19:56 JST | `0909`, `0916`, `0931` | runtime attach floor + helper-local attachpoint compatibility / activation evidence |
| `SUG-02` | shared-space membership | `samples/clean-near-end/sugoroku-world/02_admin_start_reset.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | admin-only start/reset |
| `SUG-03` | Sugoroku runtime | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:59 JST | `0909`, `0916`, `0918`, `0919` | roll -> publish -> witness -> handoff |
| `SUG-04` | shared-space membership | `samples/clean-near-end/sugoroku-world/04_non_owner_roll_rejected.mir` | active runnable | 90 | negative | 2026-04-27 15:21 JST | `0909`, `0916` | wrong owner rejection |
| `SUG-05` | shared-space membership | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:59 JST | `0909`, `0916`, `0919` | membership timeline anchor |
| `SUG-06` | shared-space membership | `samples/clean-near-end/sugoroku-world/06_leave_non_owner.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | epoch invalidation |
| `SUG-07` | shared-space membership | `samples/clean-near-end/sugoroku-world/07_owner_leave_reassign.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | owner reassignment |
| `SUG-08` | theorem / model-check boundary | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | reset safety bridge |
| `SUG-09` | hot-plug preview | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | active TODO boundary | 75 | explicit TODO + rejection evidence | 2026-04-27 19:56 JST | `0909`, `0916`, `0925`, `0931` | completion evidence ではなく stop line; `hotplug_lifecycle` / post-detach rejection / helper-local telemetry-view current anchor |
| `LAY-01` | LayerSignature | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `verification` / `runtime_trace` layer inventory |
| `LAY-02` | LayerSignature | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | helper-local inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `membership` layer inventory |
| `LAY-03` | LayerSignature | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | runtime report inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `transport_provider_boundary` lane |
| `LAY-04` | LayerSignature | `samples/clean-near-end/order-handoff/06_auditable_authority_witness.mir` | runtime report inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `auth_authority_witness` lane |
| `LAY-05` | LayerSignature | `samples/clean-near-end/model-check/01_peterson_sc_pass.mir` | runtime report inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `verification_model_check` lane |
| `AUTH-01` | auth seam | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local envelope carrier | 90 | positive + negative | 2026-04-27 17:46 JST | `0921` | local queue baseline with `auth none`, membership freshness, capability refs, witness refs |
| `AUTH-02` | auth seam | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | runtime report-local envelope carrier | 90 | positive | 2026-04-27 17:46 JST | `0921` | provider boundary envelope keeps transport / witness / auth split explicit |
| `AUTH-03` | auth seam | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir`, `09_detach_todo.mir` | helper-local hot-plug envelope canary | 75 | positive + negative | 2026-04-27 19:56 JST | `0921`, `0931` | `attach_request#1` / `detach_request#1` / `detached_roll_request#1` keep auth none, membership freshness, capability split explicit |
| `VIS-01` | visualization | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local view/telemetry carrier | 90 | positive | 2026-04-27 18:04 JST | `0922` | `turn_timeline` / `message_route` / `verification_summary` with label-authority-redaction |
| `VIS-02` | visualization | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | helper-local membership view | 90 | positive | 2026-04-27 18:04 JST | `0922` | membership timeline stays redacted and published-history-only |
| `VIS-03` | visualization | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | runtime report-local view | 90 | positive | 2026-04-27 18:04 JST | `0922` | provider-boundary view stays downstream of layer/message inventory |
| `VIS-04` | visualization | `samples/clean-near-end/order-handoff/06_auditable_authority_witness.mir` | runtime report-local telemetry | 90 | positive | 2026-04-27 18:04 JST | `0922` | authority witness remains evidence, not auth |
| `FAIRY-01` | avatar follow | `samples/clean-near-end/avatar-follow/01_follow_remote_head_with_local_fallback.mir` | active representative canary | 90 | positive | 2026-04-27 19:35 JST | `0917`, `0930` | visible remote head follow with explicit local fallback lineage |
| `FAIRY-02` | avatar follow | `samples/clean-near-end/avatar-follow/02_remote_head_not_visible_falls_back_to_local.mir` | active representative canary | 90 | positive + fallback | 2026-04-27 20:55 JST | `0917`, `0930`, `0933` | visibility loss falls back locally without claiming transport recovery |
| `FAIRY-03` | avatar follow | `samples/clean-near-end/avatar-follow/03_remote_avatar_leaves_falls_back_to_local.mir` | active representative canary | 90 | negative + fallback | 2026-04-27 19:35 JST | `0917`, `0930` | leave invalidates stale anchor and falls back locally |
| `FAIRY-04` | avatar follow | `samples/clean-near-end/avatar-follow/04_invalid_cross_anchor_chain_rejected.mir` | active representative canary | 90 | negative | 2026-04-27 19:35 JST | `0917`, `0930` | invalid cross-anchor lineage stays explicit rejection |
| `FAIRY-06` | avatar follow | `samples/clean-near-end/avatar-follow/06_model_check_no_detached_anchor_observed.mir` | active representative canary | 90 | verification | 2026-04-27 19:35 JST | `0917`, `0930` | detached-anchor safety verification bridge |
## Base corpus / planned / spec-only matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH8` | avatar follow | `samples/clean-near-end/avatar-follow/`, `samples/not_implemented/avatar-fairy-follow/`, `scripts/avatar_follow_samples.py` | widened active representative slice + residual planned family | 90 | positive + negative + verification | 2026-04-27 20:55 JST | `0917`, `0930`, `0933` | active canary は `FAIRY-01` / `02` / `03` / `04` / `06`; residual planned family は `FAIRY-05` のみ |
| `FAIRY-05` | avatar follow | `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir` | residual planned skeleton | 10 | not yet | 2026-04-27 21:22 JST | `0917`, `0930`, `0939` | reacquire-after-return remains planned; explicit state timeline / anchor switch evidence gate fixed docs-first, exact witness bundling remains `UNRESOLVED` |
| `PH9` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview helper + residual planned row | 75 | synthetic positive + synthetic negative + debug | 2026-04-27 22:21 JST | `0913`, `0920`, `0923`, `0941` | `EXT-03` / `EXT-04` は planned source stub を読む synthetic preview subset。current validation は helper self-consistency と anchor comparison であり、phase 9 `.mir` の direct semantic execution ではない |
| `PH10` | MessageEnvelope / auth seam | `scripts/sugoroku_world_samples.py`, `crates/mir-runtime/src/clean_near_end.rs` | helper-local / report-local first cut | 90 | positive + negative | 2026-04-27 17:46 JST | `0912`, `0913`, `0920`, `0921` | `auth none` baseline、local queue / provider boundary、public auth contract deferred |
| `EXT-01` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | `LogText` adapter local console; exact host schema open |
| `EXT-02` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | `ShowFloatingText` world overlay; visualization label line remains typed |
| `EXT-03` | typed external boundary | `samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir`, `scripts/typed_external_boundary_samples.py` | synthetic preview helper lane | 75 | synthetic positive | 2026-04-27 22:21 JST | `0923`, `0941` | helper self-consistency preview only。scenario label は helper-local working name であり、phase 9 `.mir` の direct semantic execution や final effect 名ではない |
| `EXT-04` | typed external boundary | `samples/not_implemented/typed-external-boundary/04_adapter_failure_typed_result.mir`, `scripts/typed_external_boundary_samples.py` | synthetic preview helper lane | 75 | synthetic negative + typed failure | 2026-04-27 22:21 JST | `0923`, `0941` | helper self-consistency preview only。typed adapter failure lane は explicit だが、phase 9 `.mir` の direct semantic execution ではない |
| `EXT-05` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | debug visualization label restriction; no untyped leak |
| `PH12` | projection / placement | `plan/20-projection-and-placement-roadmap.md`, `docs/research_abstract/projection_placement_plan_01.md` | docs-first row | 10 | planned | 2026-04-27 18:15 JST | `0912`, `0913`, `0920`, `0924` | source-to-place validity checklist、place split、stop line を固定 |
| `PH13` | network transport | `plan/22-network-transport-roadmap.md`, `samples/clean-near-end/network-transport/README.md`, `scripts/network_transport_samples.py` | helper-local canary family | 75 | positive + negative + typed failure | 2026-04-27 20:27 JST | `0913`, `0920`, `0926`, `0929`, `0932` | `NET-01..05` は current helper-local canary; real socket / public ABI は still later |
| `NET-01` | network transport | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir`, `03_roll_publish_handoff.mir`, `04_non_owner_roll_rejected.mir` | helper-local loopback preview | 90 | positive + negative | 2026-04-27 18:58 JST | `0926`, `0929` | `--transport loopback_socket` で attach / handoff / reject parity を same-process emulator 上で確認 |
| `NET-02` | network transport | `samples/clean-near-end/network-transport/README.md`, `scripts/network_transport_samples.py` | helper-local process-boundary canary | 90 | positive | 2026-04-27 20:27 JST | `0932` | subprocess JSON bridge で `attach_request#1` / `roll_request#1` / `handoff_notice#1` と witness lane を維持 |
| `NET-03` | network transport | `samples/clean-near-end/network-transport/README.md`, `scripts/network_transport_samples.py` | helper-local reconnect guard | 75 | negative | 2026-04-27 20:27 JST | `0932` | stale `membership_epoch` / `member_incarnation` reconnect を hidden repair せず reject |
| `NET-04` | network transport | `samples/clean-near-end/network-transport/README.md`, `scripts/network_transport_samples.py` | helper-local typed failure matrix | 75 | negative + typed failure | 2026-04-27 20:27 JST | `0932` | timeout / queue-full / route-not-found / detach-after-send を retryable/terminal に分離 |
| `NET-05` | network transport | `samples/clean-near-end/network-transport/README.md`, `scripts/network_transport_samples.py` | helper-local redacted route trace | 75 | positive | 2026-04-27 20:27 JST | `0932` | observer-safe route trace で auth / capability payload を leak しない |
| `PH15` | visualization / IDE | `scripts/sugoroku_world_samples.py`, `crates/mir-runtime/src/clean_near_end.rs`, `mir_hilight.html` | helper-local / report-local first cut | 90 | positive | 2026-04-27 18:04 JST | `0910`, `0911`, `0913`, `0918`, `0920`, `0922` | typed/redacted visualization actualized; final public viewer and retention remain deferred |
| `PH16` | compiler/backend/LLVM prep | `scripts/env/mirrorea_storage_env.sh`, `scripts/storage/*`, `plan/23-compiler-backend-llvm-guardrail-roadmap.md` | ops guardrail | 75 | partial | 2026-04-27 18:48 JST | `0913`, `0915`, `0920`, `0927` | `CARGO_HOME` probe と LLVM path readiness は actualize; actual LLVM build はまだない |

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | Last result |
|---|---|---|---:|---|---|
| `E2E-CLEAN-SMOKE` | clean near-end family run | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | 90 | typing / order-handoff / model-check / modal の active clean suite floor | pass 2026-04-27 15:59 JST |
| `E2E-SUG-ATTACH` | membership -> attach -> start -> roll -> publish -> witness -> handoff | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json` | 90 | logical multi-Place vertical slice の中心線 | pass 2026-04-27 15:59 JST |
| `E2E-SUG-HISTORY` | membership -> late join -> published history visibility | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership` | 90 | epoch / active-pending / published-history visibility line | pass 2026-04-27 15:59 JST |
| `E2E-SUG-RESET` | runtime attach -> reset -> stale handoff invalidation | `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification` | 90 | reset safety と model-check second line の bridge | pass 2026-04-27 15:21 JST |
| `E2E-TERM-LAYER-INV` | source sample -> helper/runtime inventory | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures` and `--debug layers` | 90 | helper-local `TermSignature` / `LayerSignature` inventory floor | pass 2026-04-27 15:59 JST |
| `E2E-AVATAR-FALLBACK` | follow -> fallback -> reject -> verification | `python3 scripts/avatar_follow_samples.py check-all --format json` | 90 | representative slice keeps follow / fallback / stale-anchor rejection / detached-anchor safety visible in one helper family | pass 2026-04-27 19:35 JST |
| `E2E-ADAPTER-BOUNDARY` | source -> adapter lane -> typed effect receipt/failure | future target: `scripts/typed_external_boundary_samples.py` preview commands + `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` + `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | 10 | compositional adapter E2E を later に作るとき、provider-boundary / local-queue anchor と synthetic preview subset をどこまで統合するかの target | planned target only; current validation is helper self-consistency plus anchor comparison |
| `E2E-NET-LOOPBACK` | local queue -> loopback attach / handoff / reject parity | `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json` | 90 | helper-local loopback preview keeps envelope / reject parity across current Sugoroku family | pass 2026-04-27 20:27 JST |
| `E2E-NET-PROCESS` | subprocess JSON bridge / reconnect guard / failure / route trace | `python3 scripts/network_transport_samples.py check-all --format json` | 90 | helper-local transport canary family keeps process-boundary / freshness / failure / redaction evidence explicit | pass 2026-04-27 20:27 JST |
| `E2E-MSG-AUTH` | envelope -> auth -> authz -> membership -> dispatch | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | 90 | helper-local envelope carrier keeps auth / membership / capability / witness separate | pass 2026-04-27 17:46 JST |
| `E2E-VIS-TRACE` | source -> runtime trace -> visualization view -> telemetry row | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | 90 | helper-local typed/redacted visualization first cut | pass 2026-04-27 18:04 JST |
| `E2E-HOTPLUG-ATTACH` | patch -> compatibility -> attach -> activate -> detach request -> post-detach rejection | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` | 90 | helper-local `hotplug_lifecycle` keeps attachpoint compatibility / activation cut / detach request / post-detach rejection visible without claiming final ABI | pass 2026-04-27 19:56 JST |
| `E2E-PROJECTION` | system source -> place-specific projection -> equivalent trace | `plan/20-projection-and-placement-roadmap.md` | 10 | docs-first validity checklist fixed; executable projection helper is not yet present | plan fixed 2026-04-27 18:15 JST |

## Build/storage environment

| Item | Status | Path | Notes |
|---|---|---|---|
| External workdir | mounted | `/mnt/mirrorea-work` | `/dev/vdb1` ext4 `mirrorea-work`, UUID `a87650a8-e3e9-4977-8940-6c293a0ee23c`, backed by `fstab` |
| Root setup helper | verified | `scripts/storage/setup_mirrorea_workdisk_root.sh` | GPT + ext4 + `/mnt/mirrorea-work` + UUID `fstab`; current session で mount 済み |
| Cargo target | externalized | `target/` -> `/mnt/mirrorea-work/cargo-target` | existing build artifact は SSD 側へ移送済み; current external usage `5.3G` |
| Storage env script | yes | `scripts/env/mirrorea_storage_env.sh` | mounted default を確認しつつ safe path を exportし、`CARGO_HOME` も external workdir へ向ける |
| Cargo registry cache probe | verified | `/mnt/mirrorea-work/cargo-registry-cache` | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` pass |
| LLVM build | path ready | `/mnt/mirrorea-work/llvm/{src,build,install}` | directory existence は確認済み; actual LLVM artifact はまだない |
| Generated artifacts | policy only | `/mnt/mirrorea-work/generated-artifacts` | heavy disposable artifact は root よりこちらを優先 |
| Detach prep script | yes | `scripts/storage/detach_prepare.sh` | non-destructive; status print only |
| Cleanup script | yes | `scripts/storage/cleanup_disposable_artifacts.sh` | requires `--confirm`; current default では safe guard 付き |

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|
| detach lifecycle is still an explicit TODO boundary | hot-plug | medium | CodeX | keep `SUG-09` visible even after helper-local lifecycle actualization until real attachpoint / migration helper exists |
| `FAIRY-05` residual family remains planned | avatar follow | low | CodeX | keep planned until explicit state timeline / anchor switch evidence and a negative companion package are worth carrying together |
| final public visualization contract / retention / multi-tenant telemetry が未決 | visualization | medium | mixed gate | keep helper/report-local first cut only and reopen after adapter / projection packages |
| final public `AuthEvidence` kind と real transport widening が未決 | auth / transport | medium | mixed gate | keep helper-local `auth none` baseline and reopen with adapter / network packages |
| transport canary is helper-local only; real socket / session / durable replay remain absent | network transport | medium | mixed gate | keep `NET-01..05` as evidence surface only and reopen real transport later |
| actual LLVM artifact / backend choice / packaging contract はまだ absent | backend / storage | low | CodeX | keep `plan/23` guardrail fixed and reopen only when a real LLVM/build package is promoted |
| final public adapter contract / host schema scope が未決 | external adapters | medium | mixed gate | keep `EXT-03` / `EXT-04` synthetic preview subset only and reopen `EXT-01` / `EXT-02` / `EXT-05` after projection / visualization pressure clarifies |
| repository taxonomy drift can reintroduce active/planned/generated confusion | docs / structure | medium | CodeX | keep `plan/19`, `samples/README.md`, `scripts/README.md`, `0920` report in sync |

## Recent validation

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-04-27 21:55 JST | `python3 -m unittest scripts.tests.test_typed_external_boundary_samples` | pass | dedicated typed external helper test suite; 10 tests green |
| 2026-04-27 22:21 JST | `python3 scripts/typed_external_boundary_samples.py check-all --format json` | pass | synthetic preview subset `EXT-03` / `EXT-04`; residual planned `EXT-01` / `EXT-02` / `EXT-05`; helper self-consistency only |
| 2026-04-27 22:21 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json` | pass | envelope id / auth none / membership freshness / capability / witness split visible in synthetic preview helper |
| 2026-04-27 22:21 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json` | pass | label / authority / redaction preview visible in synthetic preview helper |
| 2026-04-27 22:21 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json` | pass | typed adapter failure family remains explicit in synthetic preview helper |
| 2026-04-27 21:38 JST | `python3 scripts/check_source_hierarchy.py` | pass | reviewer re-check report `0940` added after `0939`; required `23`, present `23`, missing `0` |
| 2026-04-27 21:38 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 938 numbered report(s).` after `0940` rereview report |
| 2026-04-27 21:38 JST | `git diff --check` | pass | whitespace-clean after reviewer rereview follow-up |
| 2026-04-27 21:34 JST | `python3 scripts/avatar_follow_samples.py check-all --format json` | pass | active avatar helper slice remains green while `FAIRY-05` stays planned |
| 2026-04-27 21:34 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | `planned_sample_ids = [\"05_follow_target_reacquired_after_return\"]` and active sample IDs remain unchanged |
| 2026-04-27 21:34 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` after `FAIRY-05` docs-first design review sync |
| 2026-04-27 21:34 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 937 numbered report(s).` after `0938` review + `0939` package report |
| 2026-04-27 21:34 JST | `git diff --check` | pass | whitespace-clean after `FAIRY-05` docs-first design review sync |
| 2026-04-27 21:19 JST | `python3 scripts/check_source_hierarchy.py` | pass | final cross-package sweep sync still keeps required `23`, present `23`, missing `0` |
| 2026-04-27 21:19 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 935 numbered report(s).` after `0934`..`0937` sync |
| 2026-04-27 21:19 JST | `git diff --check` | pass | whitespace-clean after cross-package sweep final follow-up cleanup |
| 2026-04-27 21:16 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 934 numbered report(s).` after `0934`/`0935` sync |
| 2026-04-27 21:16 JST | `git diff --check` | pass | whitespace-clean after cross-package sweep follow-up cleanup |
| 2026-04-27 21:10 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | widened avatar slice / single residual `FAIRY-05` still visible |
| 2026-04-27 21:10 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | helper-local `NET-02..05` closeout still matches queue recut |
| 2026-04-27 21:10 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | `hotplug_lifecycle` / `message_envelopes` / `visualization_views` stay visible |
| 2026-04-27 21:10 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | clean near-end runtime closeout still matches report-local carrier inventory |
| 2026-04-27 21:10 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` before final docs sync |
| 2026-04-27 21:00 JST | `python3 scripts/check_source_hierarchy.py` | pass | reviewer follow-up after avatar taxonomy cleanup still keeps required `23`, present `23`, missing `0` |
| 2026-04-27 21:00 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 931 numbered report(s).` after reviewer follow-up |
| 2026-04-27 21:00 JST | `git diff --check` | pass | whitespace-clean after reviewer follow-up cleanup |
| 2026-04-27 20:55 JST | `python3 -m unittest scripts.tests.test_avatar_follow_samples` | pass | widened avatar helper suite; `FAIRY-02` active / `FAIRY-05` single residual planned |
| 2026-04-27 20:55 JST | `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json` | pass | visibility-loss fallback stays local and does not claim transport recovery |
| 2026-04-27 20:55 JST | `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json` | pass | detached-anchor safety regression remains explicit |
| 2026-04-27 20:55 JST | `python3 scripts/avatar_follow_samples.py check-all --format json` | pass | `FAIRY-01` / `02` / `03` / `04` / `06` all green |
| 2026-04-27 20:55 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | active sample IDs, debug modes, residual `FAIRY-05` only |
| 2026-04-27 20:55 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` after avatar widening sync |
| 2026-04-27 20:55 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 931 numbered report(s).` |
| 2026-04-27 20:55 JST | `git diff --check` | pass | whitespace-clean after `0933` report and snapshot sync |
| 2026-04-27 20:27 JST | `python3 -m unittest scripts.tests.test_network_transport_samples` | pass | dedicated transport helper test suite |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py run NET-02 --debug route-trace` | pass | subprocess JSON bridge route trace visible |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect` | pass | stale epoch/incarnation reconnect reject visible |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py run NET-04 --debug failures` | pass | typed failure family visible with retryable/terminal split |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace` | pass | observer-safe redacted route trace visible |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py check-all --format json` | pass | `NET-02..05` helper-local canary family all green |
| 2026-04-27 20:27 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | active sample root, helper script, debug modes, limitations visible |
| 2026-04-27 20:27 JST | `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json` | pass | `NET-01` loopback floor remains green after transport helper addition |
| 2026-04-27 20:27 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | provider-boundary anchor remains separate from transport canary lane |
| 2026-04-27 20:27 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 20:27 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 930 numbered report(s).` |
| 2026-04-27 20:27 JST | `git diff --check` | pass | whitespace-clean after transport helper/doc sync |
| 2026-04-27 20:07 JST | `python3 scripts/check_source_hierarchy.py` | pass | docs-only stale-reference cleanup still keeps required `23/23` paths present |
| 2026-04-27 20:07 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 929 numbered report(s).` |
| 2026-04-27 20:07 JST | `git diff --check` | pass | whitespace-clean after final hot-plug snapshot cleanup |
| 2026-04-27 20:05 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | final rerun after `0931` report add; 34 tests green |
| 2026-04-27 20:05 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 929 numbered report(s).` |
| 2026-04-27 20:05 JST | `git diff --check` | pass | whitespace-clean after `0931` report sync |
| 2026-04-27 20:03 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | hot-plug lifecycle / detach request / telemetry-view assertions included |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json` | pass | attachpoint compatibility / activation summary visible on `SUG-01` |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` | pass | `detach_request#1` + post-detach rejection grounded in `MessageEnvelope` carrier |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug visualization --format json` | pass | detach lifecycle visualization view references real envelope ids |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | `hotplug_lifecycle_states`, `hotplug_activation`, `hotplug_detach`, `--debug hotplug` visible |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py check-all --format json` | pass | all 10 Sugoroku samples pass after hot-plug widening |
| 2026-04-27 20:03 JST | `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json` | pass | loopback preview remains green after hot-plug widening |
| 2026-04-27 20:03 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 20:03 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 928 numbered report(s).` before adding `0931` report |
| 2026-04-27 20:03 JST | `git diff --check` | pass | whitespace-clean after hot-plug code/doc sync |
| 2026-04-27 19:35 JST | `python3 -m unittest scripts.tests.test_avatar_follow_samples` | pass | dedicated avatar helper test suite |
| 2026-04-27 19:35 JST | `python3 scripts/avatar_follow_samples.py check-all --format json` | pass | `FAIRY-01` / `03` / `04` / `06` representative slice all green |
| 2026-04-27 19:35 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | active sample IDs, planned residual IDs, debug modes, model-check property visible |
| 2026-04-27 19:46 JST | `python3 scripts/check_source_hierarchy.py` | pass | avatar representative slice sync kept required `23/23` paths present after reviewer follow-up |
| 2026-04-27 19:46 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 928 numbered report(s).` |
| 2026-04-27 19:46 JST | `git diff --check` | pass | whitespace-clean after `0930` reviewer follow-up sync |
| 2026-04-27 19:17 JST | `python3 scripts/check_source_hierarchy.py` | pass | `Network transport executable widening` final sync kept required `23/23` paths present |
| 2026-04-27 19:17 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 927 numbered report(s).` |
| 2026-04-27 19:17 JST | `git diff --check` | pass | whitespace-clean after `0929` final sync |
| 2026-04-27 18:57 JST | `python3 scripts/check_source_hierarchy.py` | pass | hands-on closeout sync did not introduce hierarchy drift |
| 2026-04-27 18:57 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 926 numbered report(s).` |
| 2026-04-27 18:55 JST | `python3 scripts/current_l2_guided_samples.py closeout --format json` | pass | current-L2 closeout still matches built-in/user-defined boundary and report-local carrier inventory |
| 2026-04-27 18:55 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | Sugoroku closeout still reports message envelopes / visualization / telemetry / layer signatures |
| 2026-04-27 18:55 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout still reports message / visualization / layer carriers on clean near-end suite |
| 2026-04-27 18:55 JST | `git diff --check` | pass | whitespace-clean after hands-on closeout sync |
| 2026-04-27 18:48 JST | `python3 scripts/check_source_hierarchy.py` | pass | backend/LLVM guardrail sync did not introduce hierarchy drift |
| 2026-04-27 18:48 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 925 numbered report(s).` |
| 2026-04-27 18:48 JST | `bash scripts/env/mirrorea_storage_env.sh` | pass | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache` remains the intended external binding |
| 2026-04-27 18:48 JST | `bash scripts/storage/detach_prepare.sh` | pass | non-destructive status and disposable-candidate inventory remained readable after guardrail sync |
| 2026-04-27 18:48 JST | `bash scripts/storage/cleanup_disposable_artifacts.sh --list` | pass | delete still requires explicit confirmation |
| 2026-04-27 18:48 JST | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` | pass | cargo registry cache path remained usable on external workdir |
| 2026-04-27 18:48 JST | `git diff --check` | pass | whitespace-clean after backend/LLVM guardrail sync |
| 2026-04-27 18:35 JST | `df -h / /mnt/mirrorea-work` | pass | root `32G` free, external workdir `181G` free |
| 2026-04-27 18:35 JST | `findmnt /mnt/mirrorea-work` | pass | `/dev/vdb1` ext4 mount confirmed |
| 2026-04-27 18:35 JST | `bash scripts/env/mirrorea_storage_env.sh` | pass | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache` exported |
| 2026-04-27 18:35 JST | `bash scripts/storage/detach_prepare.sh` | pass | non-destructive status output and disposable candidates confirmed |
| 2026-04-27 18:35 JST | `bash scripts/storage/cleanup_disposable_artifacts.sh --list` | pass | delete requires explicit confirmation |
| 2026-04-27 18:35 JST | `ls -ld /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install` | pass | LLVM directories exist; artifact absent |
| 2026-04-27 18:35 JST | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` | pass | cargo registry cache path is usable on external workdir |
| 2026-04-27 18:32 JST | `python3 scripts/check_source_hierarchy.py` | pass | network transport docs-first package did not introduce hierarchy drift |
| 2026-04-27 18:32 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 924 numbered report(s).` |
| 2026-04-27 18:32 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | local-queue envelope remains current phase 13 evidence anchor |
| 2026-04-27 18:32 JST | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership` | pass | membership freshness remains reconnect planning anchor |
| 2026-04-27 18:32 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | provider boundary remains current phase 13 evidence anchor |
| 2026-04-27 18:23 JST | `python3 scripts/check_source_hierarchy.py` | pass | hot-plug docs-first package did not introduce hierarchy drift |
| 2026-04-27 18:23 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 923 numbered report(s).` |
| 2026-04-27 18:23 JST | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --format json` | pass | attach request / runtime attach envelope remain current hot-plug anchor |
| 2026-04-27 18:23 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --format json` | pass | detach TODO boundary remains explicit stop line, not completion evidence |
| 2026-04-27 18:23 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | all 10 Sugoroku samples remain green after hot-plug docs-first sync |
| 2026-04-27 18:15 JST | `python3 scripts/check_source_hierarchy.py` | pass | projection docs-first package did not introduce hierarchy drift |
| 2026-04-27 18:15 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 922 numbered report(s).` |
| 2026-04-27 18:15 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | provider boundary remains current projection evidence anchor |
| 2026-04-27 18:15 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | place split remains visible on helper-local visualization view |
| 2026-04-27 18:10 JST | `python3 scripts/check_source_hierarchy.py` | pass | planned phase 9 sample root added without hierarchy drift |
| 2026-04-27 18:10 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 921 numbered report(s).` |
| 2026-04-27 18:10 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | provider boundary remains current phase 9 evidence anchor |
| 2026-04-27 18:10 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | local-queue envelope remains current phase 9 evidence anchor |
| 2026-04-27 18:04 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 18:04 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 920 numbered report(s).` |
| 2026-04-27 18:04 JST | `git diff --check` | pass | whitespace-clean after visualization sync |
| 2026-04-27 18:04 JST | `cargo test -p mir-runtime` | pass | full runtime suite remained green after visualization carrier additions |
| 2026-04-27 18:04 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout emits `visualization_views` / `telemetry_rows` |
| 2026-04-27 18:04 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side visualization/telemetry tests included |
| 2026-04-27 18:04 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout emits visualization/telemetry inventory |
| 2026-04-27 18:04 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | helper-local typed/redacted view and telemetry visible |
| 2026-04-27 18:04 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | all 10 Sugoroku samples still pass with visualization carrier enabled |
| 2026-04-27 18:04 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side visualization tests included |
| 2026-04-27 17:46 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 17:46 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 919 numbered report(s).` |
| 2026-04-27 17:46 JST | `git diff --check` | pass | whitespace-clean after `0921` report and snapshot sync |
| 2026-04-27 17:46 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side MessageEnvelope inventory tests included |
| 2026-04-27 17:46 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | runtime report-local `message_envelopes` visible on provider boundary |
| 2026-04-27 17:46 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | closeout emits `message_envelope_lanes`, `auth_evidence_kinds`, `transport_seams` |
| 2026-04-27 17:46 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | helper-local `message_envelopes` visible on local-queue baseline |
| 2026-04-27 17:46 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | 10 Sugoroku samples still pass with envelope carrier enabled |
| 2026-04-27 17:46 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side MessageEnvelope tests included |
| 2026-04-27 17:03 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 17:03 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 918 numbered report(s).` |
| 2026-04-27 17:03 JST | `git diff --check` | pass | report / snapshot sync after final reviewer fixes is whitespace-clean |
| 2026-04-27 13:20 JST | `findmnt /mnt/mirrorea-work` | pass | external workdir mount confirmed |
| 2026-04-27 13:20 JST | `cargo test -p mir-ast --no-run` | pass | externalized `target/` path usable |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | 10 Sugoroku samples passed |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace` | pass | publish -> handoff trace readable |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification` | pass | reset safety bridge readable |
| 2026-04-27 15:59 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side Term/Layer inventory tests |
| 2026-04-27 15:59 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side Term/Layer inventory tests |
| 2026-04-27 15:59 JST | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | pass | current-L2 smoke after Term/Layer work |
| 2026-04-27 15:59 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers` | pass | helper-local layer inventory view |
| 2026-04-27 15:59 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout emits `layer_signatures` |
| 2026-04-27 15:59 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, missing `0` |
| 2026-04-27 15:59 JST | `python3 scripts/validate_docs.py` | pass | documentation scaffold complete |
| 2026-04-27 15:59 JST | `git diff --check` | pass | whitespace-clean at `0919` close |
| 2026-04-27 15:21 JST | `python3 scripts/current_l2_guided_samples.py run-source-sample samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt --host-plan samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json --format json` | expected fail | prototype is not active runnable surface |

## Historical / archived samples

| Old path | New path / status | Reason |
|---|---|---|
| `samples/old/2026-04-22-pre-clean-near-end/` | archived; active replacement is `samples/clean-near-end/` | historical pre-clean-near-end corpus; not active |
| `samples/lean/old/2026-04-22-pre-clean-near-end/` | archived; active replacement is `samples/lean/clean-near-end/` | historical Lean carry-over only |
| `samples/prototype/current-l2-dynamic-attach-detach/` | prototype reference only | useful anchor for avatar/hot-plug planning, but not active |
