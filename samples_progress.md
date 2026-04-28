# samples_progress

Last updated: 2026-04-28 20:31 JST
Current repo-local focus: clean near-end current layer と Sugoroku world / avatar follow representative slice の runnable floor、viewer typed public prototype inventory、storage/backend current first-cut closeout、`U1` option-matrix closeout を保ったまま、`R1` `VerificationLayer` widening threshold inventory で core-side public-surface narrowing criteria を docs-first に揃える
Current active packages: `R1` `VerificationLayer` widening threshold inventory (`U1` post-`P18` true user-spec hold option matrix は close 済み。actual commitment は still later のまま保ち、次は verification lane widening criteria だけを整理する)

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
| Sugoroku runtime | 90 | active vertical slice + attach hardening closed | attach / membership / handoff / late join / detach TODO boundary の current closeout contract を維持 | `python3 scripts/sugoroku_world_samples.py closeout --format json` |
| Avatar follow | 90 | widened active representative slice + gate hardening closed | `FAIRY-01/02/03/04/06` を維持しつつ `FAIRY-05` は helper closeout `fairy05_reopen_gate` / `planned_sample_paths` まで actualize し、runnable widening は deferred に保つ | `python3 scripts/avatar_follow_samples.py closeout --format json` |
| External adapters | 75 | synthetic preview helper + residual review closed + host-boundary inventory first cut | `EXT-03` / `EXT-04` synthetic preview subset を維持しつつ helper closeout `host_boundary_inventory` / `non_collapse_lanes` を current evidence surface に載せ、`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen matrix を fixed 済みとして保つ | `python3 scripts/typed_external_boundary_samples.py closeout --format json` |
| Network transport | 100 | helper-local canary family + current first-cut closeout closed | `NET-01..05` helper-local canary に加えて helper closeout `transport_scope` / `process_boundary_canaries` / `loopback_parity_sources` / `non_collapse_lanes` / `kept_later_gates` / `validation_floor` を固定し、real socket / durable replay は deferred に保つ | `python3 scripts/network_transport_samples.py closeout --format json` |
| Visualization | 100 | helper-local + report-local security hardening closed | `visualization_views` / `telemetry_rows` / `retention_scope` / `source_refs` / fail-closed observer route trace を current floor として維持 | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` |
| Viewer prototype | 100 | typed public prototype inventory first-cut closeout closed | `scripts/visual_debugger_viewer_samples.py` により helper/runtime typed inventory を `viewer_panel_lanes` / `viewer_telemetry_lanes` へ正規化し、`P16-VIEW-01..05` と `kept_later_gates` を current floor に固定した | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` |
| Projection / placement | 90 | helper-local + report-local preview floor + generated bridge first-cut closeout closed | `projection_view` と `cross_place_projection` を維持しつつ `scripts/projection_codegen_samples.py`、committed generated bridge manifest、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor` を current line に固定した | `python3 scripts/projection_codegen_samples.py closeout --format json` |
| Hot-plug package | 100 | helper-local lifecycle canary + package-manager first-cut closeout closed | `hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor` を closeout に固定しつつ real migration を deferred に保つ | `python3 scripts/sugoroku_world_samples.py closeout --format json` |
| Storage / backend guardrail | 100 | external workdir / cleanup / LLVM staging current first-cut closeout closed | `scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh --list`、`docs/hands_on/compiler_backend_llvm_preparation_01.md`、`plan/23` を通じて owner/writable probe と cleanup guard を current line に固定した | `bash scripts/storage/detach_prepare.sh` |

## Active sample matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH0` | repository memory | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | dashboard / hierarchy check | 90 | mixed | 2026-04-28 20:31 JST | `0913`, `0920`, `0943`, `0945`, `0955`, `0956`, `0957`, `0958`, `0959`, `0960`, `0961`, `0962`, `0963`, `0964`, `0965`, `0966`, `0967`, `0968`, `0969`, `0970`, `0971`, `0972`, `0973`, `0974` | source hierarchy と report discipline の current floor |
| `PH1` | Mir core | `samples/current-l2/` | base corpus | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | final parser / public API deferred |
| `PH6` | compile-ready minimal actualization | `samples/clean-near-end/` | active clean suite | 90 | positive + negative | 2026-04-28 15:32 JST | `0904`, `0913`, `0945`, `0959`, `0960`, `0961`, `0962`, `0963`, `0964`, `0965` | public shell / packaging deferred。`P10` で `mirrorea-core` minimal carrier ownership cut と `transport == transport_seam` compatibility invariant、`P11` で `MembershipRegistry` / `PlaceCatalog` substrate、bootstrap/epoch guard、participant-place-kind-gated `LogicalPlaceRuntimeShell`、principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper が actualize 済み |
| `SUG-01` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | active runnable | 90 | positive | 2026-04-28 13:09 JST | `0909`, `0916`, `0931`, `0945`, `0955` | runtime attach floor + helper-local attachpoint compatibility / activation evidence。current closeout では `world_surface` と `MembershipRegistry` source-of-truth の anchor にもなる |
| `SUG-03` | Sugoroku runtime | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | active runnable E2E | 90 | positive | 2026-04-28 13:09 JST | `0909`, `0916`, `0918`, `0919`, `0942`, `0945`, `0950`, `0952`, `0954`, `0955` | roll -> publish -> witness -> handoff。`TermSignature` / `LayerSignature` inventory、projection preview、visualization security envelope の基準点であり、current P8 close では handoff target activity boundary の anchor でもある |
| `SUG-05` | shared-space membership | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | active runnable E2E | 90 | positive | 2026-04-28 13:09 JST | `0909`, `0916`, `0919`, `0945`, `0955` | membership timeline anchor。published history visible before turn-order insertion の closeout wording anchor |
| `SUG-08` | theorem / model-check boundary | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | active runnable E2E | 90 | positive | 2026-04-28 03:27 JST | `0909`, `0916`, `0945` | reset safety bridge |
| `SUG-09` | hot-plug preview | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | active TODO boundary | 75 | explicit TODO + rejection evidence | 2026-04-28 13:09 JST | `0909`, `0916`, `0925`, `0931`, `0955` | completion evidence ではなく stop line。detach は explicit TODO stop line であり completed migration ではない |
| `FAIRY-01` | avatar follow | `samples/clean-near-end/avatar-follow/01_follow_remote_head_with_local_fallback.mir` | active representative canary | 90 | positive | 2026-04-28 13:37 JST | `0917`, `0930`, `0945`, `0956` | visible remote head follow with explicit local fallback lineage |
| `FAIRY-02` | avatar follow | `samples/clean-near-end/avatar-follow/02_remote_head_not_visible_falls_back_to_local.mir` | active representative canary | 90 | positive + fallback | 2026-04-28 13:37 JST | `0917`, `0930`, `0933`, `0945`, `0956` | visibility loss falls back locally and does not claim transport recovery |
| `FAIRY-03` | avatar follow | `samples/clean-near-end/avatar-follow/03_remote_avatar_leaves_falls_back_to_local.mir` | active representative canary | 90 | negative + fallback | 2026-04-28 13:37 JST | `0917`, `0930`, `0945`, `0956` | leave invalidates stale anchor, rejects it, and falls back locally |
| `FAIRY-04` | avatar follow | `samples/clean-near-end/avatar-follow/04_invalid_cross_anchor_chain_rejected.mir` | active representative canary | 90 | negative | 2026-04-28 13:37 JST | `0917`, `0930`, `0945`, `0956` | invalid cross-anchor lineage is rejected without hidden repair |
| `FAIRY-06` | avatar follow | `samples/clean-near-end/avatar-follow/06_model_check_no_detached_anchor_observed.mir` | active representative canary | 90 | verification | 2026-04-28 13:37 JST | `0917`, `0930`, `0945`, `0956` | detached-anchor safety verification bridge |
| `AUTH-01` | auth seam | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local envelope carrier | 90 | positive + negative | 2026-04-27 23:24 JST | `0921`, `0941` | local queue baseline with `auth none`, membership freshness, capability refs, witness refs |
| `VIS-01` | visualization | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local view/telemetry carrier | 100 | positive | 2026-04-28 12:42 JST | `0922`, `0942`, `0954` | `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を持つ evidence-oriented view と typed telemetry。final public viewer contract ではない |
| `P16-VIEW-01..05` | viewer prototype | `scripts/visual_debugger_viewer_samples.py` | typed public prototype inventory | 100 | positive | 2026-04-28 18:44 JST | `0971` | Sugoroku helper view/telemetry、Sugoroku closeout catalog、`NET-05` route trace、runtime canonical inventory、typed external `EXT-03` host-boundary preview を `viewer_panel_lanes` / `viewer_telemetry_lanes` に正規化する。duplicate helper panel-id は explicit canonicalization を行い、final public viewer API / telemetry backend ではない |
| `PRJ-01` | projection / placement | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local projection preview | 75 | positive | 2026-04-28 09:51 JST | `0924`, `0942`, `0948` | `projection_view` で system source から place split と observer view refs を visible にする。final emitted place program ではない |
| `PRJ-02` | projection / placement | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | report-local provider placement preview | 75 | positive | 2026-04-28 09:51 JST | `0924`, `0942`, `0948` | `cross_place_projection` で authority placement と provider placement を分けて visible にする |
| `PRJ-GEN-01..04` | projection / placement | `samples/generated/projection-placement/manifest.json`, `scripts/projection_codegen_samples.py` | committed generated bridge evidence | 90 | positive | 2026-04-28 17:19 JST | `0970` | `P15-GEN-01..04` は generated artifact であり、source sample でも final emitted executable program でもない。live-anchor alignment と `generated_reserve_inventory` / `generated_bridge_artifact_inventory` / `kept_later_gates` を current line に固定する |

## Base corpus / planned / spec-only matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH2` | parser-free PoC | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | detached loop | 75 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | dedicated detached-loop CLI refresh remains open |
| `PH3` | parser / checker cut | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | crate test surface | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | parser grammar freeze deferred |
| `PH4` | shared-space membership / room boundary | `samples/clean-near-end/sugoroku-world/`, `plan/16-shared-space-membership-and-example-boundary.md` | runnable room boundary | 90 | positive + negative | 2026-04-27 19:56 JST | `0909`, `0916` | late join / leave / owner reassign / history visibility が current anchor |
| `PH5` | theorem / model-check boundary | `samples/clean-near-end/{typing,model-check}`, `samples/lean/` | active bridge family | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | proof/model-check public contract absent |
| `PH7` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/`, `scripts/sugoroku_world_samples.py` | active vertical slice + helper/test/docs hardening closed | 90 | positive + negative | 2026-04-28 13:09 JST | `0909`, `0916`, `0931`, `0945`, `0955` | attach / handoff / witness / late join / reset model-check / detach TODO boundary。current closeout は world sugar / MembershipRegistry / stop line を explicit に保つ |
| `PH8` | avatar follow | `samples/clean-near-end/avatar-follow/`, `samples/not_implemented/avatar-fairy-follow/` | widened active representative slice + residual planned family | 90 | positive + negative + verification | 2026-04-28 13:37 JST | `0917`, `0930`, `0933`, `0939`, `0945`, `0956` | active canary は `FAIRY-01/02/03/04/06`、helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` を返し、residual planned family は `FAIRY-05` のまま保つ |
| `FAIRY-05` | avatar follow | `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir` | planned family | 10 | target only | 2026-04-28 13:37 JST | `0939`, `0956` | positive sample、negative companion、explicit `state_timeline` / `anchor_switch` evidence、docs/report/snapshot sync が reopen 条件。helper closeout path inventory と carrier bundling `UNRESOLVED` だけを actualize |
| `PH9` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview subset + residual planned family | 75 | positive + negative | 2026-04-28 16:00 JST | `0923`, `0941`, `0945`, `0946`, `0966` | phase 9 `.mir` direct semantic execution ではなく helper self-consistency + anchor comparison。`P12` current first cut closeout では `host_boundary_scope` / `host_boundary_lanes` / `non_collapse_lanes` / `host_family_gates` / `host_boundary_inventory` を helper closeout に actualize |
| `EXT-01/02/05` | typed external boundary | `samples/not_implemented/typed-external-boundary/` | residual planned family | 10 | target only | 2026-04-28 09:34 JST | `0923`, `0941`, `0946` | indirect anchor / reopen criterion / kept-later gate は fixed。final host-facing contract は mixed gate |
| `EXT-03/04` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview helper subset | 75 | positive + negative | 2026-04-28 16:00 JST | `0941`, `0945`, `0946`, `0966` | typed adapter failure lane、envelope split、redacted visualization lane に加え、`P12` current first cut closeout で `host_boundary` preview が request / receipt / failure / visualization split を返す |
| `PH10` | MessageEnvelope / AuthEvidence seam | Sugoroku helper、clean near-end runtime report | helper-local + report-local carrier | 100 | positive + negative | 2026-04-28 12:18 JST | `0921`, `0953` | helper `message_envelope_scope = representative_slice`、runtime `message_envelope_scope = clean_near_end_canonical_inventory`、legacy `transport` alias は seam 意味へ正規化済みで、`transport_medium` / `transport_seam` / `emitter_principal` / `freshness_checks` / shared `auth_evidence_lanes` を fixed。final public auth schema / transport ABI / `witness_refs` role taxonomy ではない |
| `PH11` | TermSignature / LayerSignature | Sugoroku helper、clean near-end runtime report | helper-local + report-local inventory | 100 | positive | 2026-04-28 11:19 JST | `0918`, `0919`, `0950`, `0952` | `TermSignature` は current `kind/name/evidence_role` lanes を固定済み。`LayerSignature` は current `name/requires/provides/transforms/checks/emits/obligations/laws` row schema、helper `representative_slice` inventory、runtime `clean_near_end_canonical_inventory` inventory、helper representative names `verification_handoff_witness` / `runtime_turn_trace` / `membership_*` / `hotplug_*`、runtime canonical names `auth_authority_witness` / `transport_provider_boundary` / `verification_model_check` を fixed。final shared law schema ではない |
| `PH12` | projection / placement | `plan/20-projection-and-placement-roadmap.md`, `docs/hands_on/projection_placement_views_01.md`, `samples/generated/projection-placement/manifest.json`, `scripts/projection_codegen_samples.py` | helper/report-local preview floor + generated bridge first-cut closeout | 90 | positive | 2026-04-28 17:19 JST | `0924`, `0942`, `0948`, `0970` | preview floor と docs-first gate に加えて committed generated bridge evidence、`P15-GEN-01..04` alignment surface、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`validation_floor` を fixed。actual emitted executable family は deferred |
| `PH13` | network transport | `scripts/network_transport_samples.py`, `samples/not_implemented/network-transport/` | helper-local canary family + planned source family | 100 | positive + negative | 2026-04-28 16:29 JST | `0926`, `0929`, `0932`, `0945`, `0967` | local queue / loopback / reconnect / typed failure / redacted route trace に加え helper closeout `process_boundary_canaries` / `loopback_parity_sources` / `non_collapse_lanes` / `kept_later_gates` / `validation_floor` を fixed。real transport alpha ではない |
| `NET-01` | network transport | `scripts/sugoroku_world_samples.py --transport loopback_socket` | helper-local loopback preview | 90 | positive + negative | 2026-04-28 16:29 JST | `0929`, `0932`, `0945`, `0967` | same-process parity preview。`P13` current first-cut closeout では Sugoroku parity source の 1 つとして使う |
| `NET-02..05` | network transport | `scripts/network_transport_samples.py` | helper-local canary family | 100 | positive + negative | 2026-04-28 16:29 JST | `0932`, `0945`, `0967` | subprocess JSON bridge / stale reconnect reject / typed failure / redacted route trace と closeout inventory を固定 |
| `PH14` | hot-plug package | `plan/21-hotplug-attachpoint-roadmap.md`, `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | helper-local lifecycle canary + package-manager first-cut closeout | 100 | positive + negative | 2026-04-28 16:53 JST | `0925`, `0931`, `0955`, `0968`, `0969` | helper closeout `hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor` を fixed。real migration / rollback / attachpoint runtime remain deferred |
| `PH15` | visualization / IDE | Sugoroku helper views、runtime report-local views、`scripts/visual_debugger_viewer_samples.py` | helper-local + report-local security hardening + typed public prototype inventory closed | 100 | positive | 2026-04-28 18:44 JST | `0922`, `0954`, `0971` | helper/runtime security envelope、typed telemetry retention floor、NET-05 fail-closed route trace、`P16-VIEW-01..05` typed public prototype inventory まで current scope close。duplicate helper panel-id は explicit canonicalization し、final public viewer contract は deferred |
| `PH16` | compiler / backend / LLVM preparation | `/mnt/mirrorea-work`, `scripts/env/mirrorea_storage_env.sh`, `scripts/storage/detach_prepare.sh`, `plan/23-compiler-backend-llvm-guardrail-roadmap.md` | storage / backend guardrail current first-cut closeout | 100 | operational positive | 2026-04-28 19:09 JST | `0913`, `0915`, `0927`, `0972` | external workdir / cleanup / LLVM staging owner-writable probe と guard implementation / stop line は close 済み。cleanup refusal branch と actual LLVM build / backend choice / ownership repair は still later |

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | Last result |
|---|---|---|---:|---|---|
| `E2E-CLEAN-SUITE` | current-L2 -> clean near-end | `python3 scripts/clean_near_end_samples.py closeout` | 90 | active clean near-end suite の positive / negative floor と closeout snapshot が still green | pass 2026-04-28 03:27 JST |
| `E2E-SUGOROKU` | membership -> attach -> roll -> publish -> handoff -> late join | `python3 scripts/sugoroku_world_samples.py closeout --format json` | 90 | current shared-space vertical slice の runnable floor | pass 2026-04-28 16:53 JST |
| `E2E-AVATAR` | follow -> fallback -> stale-anchor rejection -> safety property | `python3 scripts/avatar_follow_samples.py closeout --format json` | 90 | representative avatar slice の active floor | pass 2026-04-28 13:37 JST |
| `E2E-TYPED-EXTERNAL-TARGET` | source stub -> adapter preview -> anchor comparison | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | 75 | helper self-consistency + anchor comparison の current floor。`P12` current first cut closeout では `host_boundary_inventory` / `host_family_gates` も返す。phase 9 `.mir` direct execution ではない | pass 2026-04-28 16:00 JST |
| `E2E-TRANSPORT-CANARY` | loopback / reconnect / failure / redacted trace | `python3 scripts/network_transport_samples.py closeout --format json` | 75 | helper-local transport canary family | pass 2026-04-28 12:42 JST |
| `E2E-PROJECTION-BRIDGE` | system source -> preview floor -> committed generated bridge evidence -> live-anchor alignment | `python3 scripts/projection_codegen_samples.py closeout --format json` | 90 | current `P15` first cut の generated bridge evidence floor。helper/runtime anchor と committed manifest の alignment を確認するが、final emitted executable family は証明しない | pass 2026-04-28 17:19 JST |
| `E2E-PROJECTION-TARGET` | system source -> emitted place program -> equivalent trace | future target: current `PRJ-01` / `PRJ-02` preview floor + later emitted-program runner | 10 | compositional projection E2E は still later。`P3` current package で actualize したのは preview floor と emitted-program gate の docs-first boundaryだけ | target only |

## Build / storage environment

| Item | Status | Validation | Notes |
|---|---|---|---|
| external workdir `/mnt/mirrorea-work` | active | `findmnt /mnt/mirrorea-work` | `/dev/vdb1` ext4 mount、`target/` SSD cutover 済み |
| cargo registry cache | active guardrail | `bash scripts/env/mirrorea_storage_env.sh` | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache` を intended binding にしている |
| LLVM path readiness | current first-cut closeout | `ls -ld /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/*` | `llvm/src` / `build` / `install` は staging path。`/mnt/mirrorea-work/llvm` parent の owner/writable status を visible にし、routine helper は ownership repair を行わない |

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|
| final parser / public verifier / public-boundary freeze scope | public-freeze mixed gate | high | repo + user | `P18` repo-side first cut は close 済み。helper-local threshold / prototype / inventory を final freeze evidence と混同せず、actual commitment は post-`P18` line に残す |
| final public adapter / host schema scope | typed external boundary | high | user + repo | residual reopen matrix と `host_boundary_inventory` は fixed 済み。host-facing contract 本体は post-`P18` line に残す |
| packaging shape / installed binary target | productization / packaging | high | user + repo | `U1` で `CLI / library / engine-adapter / hybrid` matrix を揃え、installed-binary promotion と current shell actualization を混同しない |
| actual emitted executable family / optimizer / deployment planner beyond manifest bridge | projection / placement | high | repo | `P15` current first cut は committed generated bridge evidence only として close 済み。final emitted executable family と optimizer / planner は kept-later gate に残す |
| final public viewer contract / retention policy / multi-tenant telemetry service | visualization / telemetry | high | repo + user | `P7` current scope と `P16` typed public prototype inventory は close 済みとして保ち、public contract 自体は post-`P18` line に残す |
| first shipped public surface scope | public-freeze mixed gate | high | repo + user | parser/checker/runtime/verifier first か integration surface first かをまだ決めず、`U1` では two-step split provisional recommendation だけを docs-first に置く |
| `VerificationLayer` widening threshold | theorem / model-check / runtime policy / visualization bridge | medium | repo | `R1` で helper `verification_handoff_witness` / runtime `verification_model_check` を current emitted floor に保ったまま widening threshold matrix と stop line を整理する |
| `FAIRY-05` carrier bundling | avatar follow | medium | repo | explicit state timeline / anchor switch evidence を保ったまま `UNRESOLVED` を解く |
| real migration / rollback | hot-plug | medium | repo | helper-local lifecycle canary の先に attachpoint migration contract を切り出す |
| real socket / durable replay | network transport | high | repo + user | helper-local canary を維持しつつ production transport line は defer する |
| actual LLVM build / backend choice | compiler / backend | medium | user + repo | `P17` first-cut closeout と `P18` inventory framingは完了済み。guardrail を壊さず、backend target と actual build は post-`P18` line で決める |

## Recent validation

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-04-28 20:31 JST | `python3 scripts/check_source_hierarchy.py` | pass | `U1` close / `R1` promoted-next wording / report `0974` 追加後も required hierarchy is intact |
| 2026-04-28 20:31 JST | `python3 scripts/validate_docs.py` | pass | documentation scaffold is complete。numbered reports は 972 |
| 2026-04-28 20:31 JST | `git diff --check` | pass | `U1` closeout / `R1` promoted-next diff は whitespace-clean |
| 2026-04-28 19:41 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | `message_envelope_scope = representative_slice`、`signature_lanes = kind / name / evidence_role`、`hotplug_scope = helper_local_package_manager_preview` を維持し、`P18` repo-side inventory の representative anchor として再確認 |
| 2026-04-28 19:41 JST | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | `host_boundary_scope = helper_local_synthetic_preview`、`host_boundary_lanes = request / receipt / failure / visualization`、residual reopen matrix を再確認し、final public adapter contract ではないことを current line に維持 |
| 2026-04-28 19:41 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | `transport_scope = helper_local_process_boundary` と `process_boundary_canaries = [NET-02, NET-03, NET-04, NET-05]` を維持し、transport non-collapse boundary を再確認 |
| 2026-04-28 19:41 JST | `python3 scripts/projection_codegen_samples.py closeout --format json` | pass | `artifact_boundary = committed manifest bridge evidence only` と `generated_bridge_artifact_inventory` を維持し、final emitted executable program 未着手 line を再確認 |
| 2026-04-28 19:41 JST | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | pass | `prototype_boundary = typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` と `viewer_panel_lanes` / `viewer_telemetry_lanes` を再確認 |
| 2026-04-28 19:41 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | canonical `message_envelope_scope = clean_near_end_canonical_inventory`、`transport_seams = provider_boundary / audit_trace_boundary`、`LayerSignature` / visualization / telemetry inventories を維持 |
| 2026-04-28 19:52 JST | `python3 scripts/check_source_hierarchy.py` | pass | reviewer follow-up と `plan/11` chronology fix 後も required hierarchy is intact |
| 2026-04-28 19:52 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。numbered reports は 971 |
| 2026-04-28 19:52 JST | `git diff --check` | pass | reviewer follow-up を含む `P18` repo-side first-cut closeout diff は whitespace-clean |
| 2026-04-28 19:11 JST | `python3 scripts/check_source_hierarchy.py` | pass | report `0972` 追加後も required hierarchy is intact |
| 2026-04-28 19:11 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。numbered reports は 970 |
| 2026-04-28 19:11 JST | `git diff --check` | pass | `P17` closeout diff は whitespace-clean |
| 2026-04-28 19:09 JST | `bash scripts/env/mirrorea_storage_env.sh` | pass | `MIRROREA_WORKDIR` / `CARGO_HOME` / `MIRROREA_LLVM_*` を返し、`llvm` parent `root:root` と non-writable warning を visible にする |
| 2026-04-28 19:09 JST | `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs` | pass | mounted workdir 下の required dirs を再確認 |
| 2026-04-28 19:09 JST | `bash scripts/storage/detach_prepare.sh` | pass | repo `102M`、`.git` `80M`、external workdir `6.0G`、cargo target `5.9G`、`llvm` owner/writable status、`llvm/src` cleanup exclusion を出力 |
| 2026-04-28 19:09 JST | `bash scripts/storage/cleanup_disposable_artifacts.sh --list` | pass | disposable candidate inventory と `llvm` root owner/writable status を出力し、`llvm/src` を除外する。refusal branch 自体は list-mode なので未実行 |
| 2026-04-28 19:09 JST | `ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install` | pass | `target` symlink、`llvm` parent `root:root`、child staging dirs user-owned を再確認 |
| 2026-04-28 19:09 JST | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` | pass | external cargo cache / target binding で build probe は green。existing warnings only |
| 2026-04-28 18:44 JST | `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples` | pass | viewer helper suite 8/8 green。duplicate `panel_id` canonicalization と live telemetry-kind aggregation regression を含む |
| 2026-04-28 18:44 JST | `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-02 --format json` | pass | helper closeout catalog bundle は duplicate `verification_summary` を `verification_summary@2` へ canonicalize し、silent shadowing を起こさない |
| 2026-04-28 18:44 JST | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | pass | `actualized_telemetry_kinds` は live normalized bundles から `history_visibility` / `hotplug_activation` / `hotplug_detach` / `membership_update` / `message_dispatch` / `model_check_summary` / `published_roll` / `route_hop` / `typed_effect_request` / `typed_effect_receipt` を返す |
| 2026-04-28 18:31 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | Sugoroku helper view bundle は `turn_timeline` / `message_route` / `verification_summary` / `projection_view` を typed panel lane に保ったまま green |
| 2026-04-28 18:31 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | Sugoroku closeout catalog は `membership_snapshot` / `hotplug_lifecycle` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` を viewer prototype backing inventory として返す |
| 2026-04-28 18:31 JST | `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json` | pass | observer-safe route trace は auth/capability payload を漏らさず `route_trace` panel bundle を支える |
| 2026-04-28 18:31 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json` | pass | host-boundary preview は `room_message_route` / `host_boundary_scope` / `non_collapse_lanes` を typed viewer pressure として返す |
| 2026-04-28 18:31 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime canonical inventory は `authority_trace_redacted_view` / `provider_boundary_redacted_flow` / `cross_place_projection` と `report_local_inventory` retention を維持する |
| 2026-04-28 18:31 JST | `python3 scripts/check_source_hierarchy.py` | pass | report `0971` 追加後も required hierarchy is intact |
| 2026-04-28 18:31 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。report count は 969 |
| 2026-04-28 18:31 JST | `git diff --check` | pass | `P16` closeout diff は whitespace-clean |
| 2026-04-28 17:19 JST | `python3 -m unittest scripts.tests.test_projection_codegen_samples` | pass | projection/codegen helper suite 10/10 green。manifest boundary / validation floor regression を含む |
| 2026-04-28 17:19 JST | `python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json` | pass | Sugoroku helper `projection_view` anchor と committed generated bridge evidence の alignment を確認 |
| 2026-04-28 17:19 JST | `python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json` | pass | clean near-end runtime `cross_place_projection` anchor と committed generated bridge evidence の alignment を確認 |
| 2026-04-28 17:19 JST | `python3 scripts/projection_codegen_samples.py check-all --format json` | pass | `P15-GEN-01..04` current first-cut inventory は all green |
| 2026-04-28 17:19 JST | `python3 scripts/projection_codegen_samples.py closeout --format json` | pass | `generated_bridge_artifact_inventory` / `generated_reserve_inventory` / `equivalence_review_categories` / `validation_floor` を返す |
| 2026-04-28 17:19 JST | `find samples/generated -maxdepth 3 -type f | sort` | pass | current committed generated files は `README.md` と `projection-placement/manifest.json` のみ。heavy disposable artifact は external workdir 側に残す |
| 2026-04-28 17:19 JST | `python3 scripts/check_source_hierarchy.py` | pass | report `0970` 追加後も required hierarchy is intact |
| 2026-04-28 17:19 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。report count は 968 |
| 2026-04-28 17:19 JST | `git diff --check` | pass | `P15` closeout diff は whitespace-clean |
| 2026-04-28 16:53 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper Sugoroku suite 46/46 green。`hotplug_scope` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` regression を含む |
| 2026-04-28 16:53 JST | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json` | pass | attach lifecycle は `attach_request#1`、`AttachPoint[SugorokuGame#1]`、`attach_lifecycle`、`attach_activation#1` を current line に保つ |
| 2026-04-28 16:53 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` | pass | detach lifecycle は `detach_request#1`、`detached_roll_request#1`、`detach_lifecycle`、`detach_boundary#1`、post-detach reject を current line に保つ |
| 2026-04-28 16:53 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout は `hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor` を返す |
| 2026-04-28 16:53 JST | `python3 scripts/check_source_hierarchy.py` | pass | report `0969` 追加後も required hierarchy is intact |
| 2026-04-28 16:53 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。report count は 967 |
| 2026-04-28 16:53 JST | `git diff --check` | pass | `P14` closeout diff は whitespace-clean |
| 2026-04-28 16:29 JST | `python3 -m unittest scripts.tests.test_network_transport_samples` | pass | helper network transport suite 11/11 green。`process_boundary_canaries` / `loopback_parity_sources` / `non_collapse_lanes` / `kept_later_gates` / `validation_floor` regression を含む |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py list` | pass | `NET-02..05` process-boundary canary family is present |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py run NET-02 --debug route-trace --format json` | pass | `transport_scope = helper_local_process_boundary`、`bridge_kind = subprocess_json_bridge`、`attach_request#1` / `roll_request#1` / `handoff_notice#1` parity を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json` | pass | `reason_family = stale_membership_epoch`、stale epoch/incarnation reject を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json` | pass | typed failure family `timeout / queue_full / route_not_found / detach_after_send` を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json` | pass | observer-safe redacted route trace と `retention_scope = helper_local_ephemeral` を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py check-all --format json` | pass | `passed = [NET-02, NET-03, NET-04, NET-05]` |
| 2026-04-28 16:29 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | helper closeout は `process_boundary_canaries` / `loopback_parity_sources` / `non_collapse_lanes` / `kept_later_gates` / `validation_floor` を返す |
| 2026-04-28 16:29 JST | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json` | pass | `attach_request#1` parity anchor を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json` | pass | `roll_request#1` / `handoff_notice#1` parity anchors を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --transport loopback_socket --format json` | pass | reject parity anchor を再確認 |
| 2026-04-28 16:29 JST | `python3 scripts/check_source_hierarchy.py` | pass | report `0967` 追加後も required hierarchy is intact |
| 2026-04-28 16:29 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。numbered reports は 965 |
| 2026-04-28 16:29 JST | `git diff --check` | pass | `P13` closeout report / queue-sync diff は whitespace-clean |
| 2026-04-28 16:00 JST | `python3 -m unittest scripts.tests.test_typed_external_boundary_samples` | pass | helper typed-external suite 17/17 green。`host_boundary` preview inventory と closeout inventory regressions を含む |
| 2026-04-28 16:00 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json` | pass | `host_boundary.scope = helper_local_synthetic_preview`、`request_lane = typed_effect_request`、`receipt_lane = typed_effect_receipt`、`visualization_lane = redacted_observer_view` を再確認 |
| 2026-04-28 16:00 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-04 --format json` | pass | `failure_lane = typed_adapter_failure`、`receipt_lane = null`、`terminal_outcome = typed_failure` を再確認 |
| 2026-04-28 16:00 JST | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | helper closeout は `host_boundary_scope`、`host_boundary_lanes`、`non_collapse_lanes`、`host_family_gates`、`host_boundary_inventory` を返し、`P12` first-cut closeout を current line に保つ |
| 2026-04-28 16:00 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | helper visualization anchor は `retention_scope = helper_local_ephemeral` と `source_refs` を保ったまま green |
| 2026-04-28 16:00 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | clean near-end provider-boundary anchor は `transport_provider_boundary` と `provider_request#1` / `provider_receipt#1` を current line に保つ |
| 2026-04-28 16:00 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P12` closeout wording / `P13` promoted-next wording 後も required hierarchy is intact |
| 2026-04-28 16:00 JST | `python3 scripts/validate_docs.py` | pass | docs scaffold is complete。numbered reports は 964 |
| 2026-04-28 16:00 JST | `git diff --check` | pass | `P12` closeout / `P13` promoted-next wording diff は whitespace-clean |
| 2026-04-28 15:32 JST | `cargo test -p mirrorea-core` | pass | `mirrorea-core` carrier/runtime substrate suite 19/19 green。participant-place derivation、failure-atomic shell helper、bootstrap / join / leave parity regression、duplicate-principal no-epoch-drift を含む |
| 2026-04-28 15:32 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper join/leave direct regression 42 tests green。`06_leave_non_owner` / `07_owner_leave_reassign` と layer-signature drift fix を含む |
| 2026-04-28 15:32 JST | `python3 scripts/sugoroku_world_samples.py run 06_leave_non_owner --format json` | pass | non-owner leave は `membership_epoch = 1`、`member_incarnation = 1`、`pending_actions_invalidated = true` を返す |
| 2026-04-28 15:32 JST | `python3 scripts/sugoroku_world_samples.py run 07_owner_leave_reassign --format json` | pass | owner leave reassignment は `new_dice_owner = Alice`、`phase_after = Running`、`membership_epoch = 2` を返し、layer transform drift も修正済み |
| 2026-04-28 15:32 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout は logical multi-place emulator / `MembershipRegistry` source-of-truth を current line のまま返し、Rust shell third cut との boundary を保つ |
| 2026-04-28 15:32 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P11` third-cut report / docs / sample-reader edits後も required hierarchy is intact |
| 2026-04-28 15:32 JST | `python3 scripts/validate_docs.py` | pass | front-door / snapshot docs は `P11` active third cut line と整合し、report count は 963 |
| 2026-04-28 15:32 JST | `git diff --check` | pass | `P11` shell parity diff は whitespace-clean |
| 2026-04-28 15:04 JST | `cargo test -p mirrorea-core` | pass | `mirrorea-core` carrier/runtime substrate suite 15/15 green。participant-place-kind gate、wrong-kind rejection、bootstrap-only guard、no phantom epoch drift、`transport == transport_seam` invariant regression を含む |
| 2026-04-28 15:04 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout は logical multi-place emulator / `MembershipRegistry` source-of-truth を current line のまま返し、crate-side participant-place-kind shell cut と helper-local world/game boundary を保つ |
| 2026-04-28 15:06 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P11` shell hardening report / snapshot edits後も required hierarchy is intact |
| 2026-04-28 15:06 JST | `python3 scripts/validate_docs.py` | pass | front-door / snapshot docs は `P11` active second cut line と整合し、report count は 960 |
| 2026-04-28 15:06 JST | `git diff --check` | pass | `P11` shell hardening diff は whitespace-clean |
| 2026-04-28 14:22 JST | `cargo test -p mir-runtime` | pass | runtime crate floor 全体 green。clean near-end 27/27、current-L2 runtime / lowering / verification ladder regression を含む |
| 2026-04-28 14:22 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples scripts.tests.test_avatar_follow_samples` | pass | helper Sugoroku + avatar suites 52 tests green |
| 2026-04-28 14:22 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout は logical multi-place emulator / `MembershipRegistry` source-of-truth / representative `LayerSignature` inventory を current line のまま返す |
| 2026-04-28 14:22 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout は canonical `message_envelope_lanes` / `auth_evidence_lanes` / `layer_signature_lanes` を `mirrorea-core` shape と整合させたまま返す |
| 2026-04-28 14:22 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P10` report / follow-up edits後も required hierarchy is intact |
| 2026-04-28 14:22 JST | `python3 scripts/validate_docs.py` | pass | front-door / snapshot docs は `P10` close / `P11` next line と整合 |
| 2026-04-28 14:22 JST | `git diff --check` | pass | `P10` closeout diff は whitespace-clean |
| 2026-04-28 13:37 JST | `python3 -m unittest scripts.tests.test_avatar_follow_samples -v` | pass | helper avatar suite 13/13 green。`fairy05_reopen_gate`、`planned_sample_paths`、missing planned-path fail-fast regression を含む |
| 2026-04-28 13:37 JST | `python3 scripts/avatar_follow_samples.py check-all --format json` | pass | active canaries 5/5 green |
| 2026-04-28 13:37 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` を返し、missing planned path は fail-fast する |
| 2026-04-28 13:37 JST | `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json` | pass | explicit follow/fallback lineage remains intact |
| 2026-04-28 13:37 JST | `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json` | pass | visibility-loss fallback stays local without transport recovery claim |
| 2026-04-28 13:37 JST | `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json` | pass | stale-membership rejection path remains intact |
| 2026-04-28 13:37 JST | `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json` | pass | detached-anchor safety remains a verification canary |
| 2026-04-28 13:37 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P9` report / queue promotion 後も required hierarchy is intact |
| 2026-04-28 13:37 JST | `python3 scripts/validate_docs.py` | pass | snapshot/front-door docs は `P9` close / `P10` next line と整合 |
| 2026-04-28 13:37 JST | `git diff --check` | pass | `P9` closeout diff は whitespace-clean |
| 2026-04-28 12:42 JST | `python3 -m unittest scripts.tests.test_network_transport_samples -v` | pass | NET-05 fail-closed regression を含む helper transport suite 10/10 green |
| 2026-04-28 13:09 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v` | pass | `world_surface` / `MembershipRegistry` source-of-truth / late-join-handoff boundary / `hotplug_stop_line` regression を含む 39/39 green |
| 2026-04-28 13:09 JST | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json` | pass | attachpoint compatibility / activation cut / imported membership epoch を helper-local hot-plug laneで確認 |
| 2026-04-28 13:09 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | handoff target activity / freshness / witness ordering を `MessageEnvelope` laneで再確認 |
| 2026-04-28 13:09 JST | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json` | pass | `MembershipRegistry` timeline と published-history-before-turn-order-insertion boundary を確認 |
| 2026-04-28 13:09 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` | pass | detach remains explicit TODO stop line with post-detach rejection and deferred rollback/migration |
| 2026-04-28 13:09 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout now returns `world_surface`, `membership_model.source_of_truth`, `membership_model.late_join_handoff_boundary`, `hotplug_stop_line` |
| 2026-04-28 13:09 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P8` report / queue promotion 後も required hierarchy is intact |
| 2026-04-28 13:09 JST | `python3 scripts/validate_docs.py` | pass | snapshot/front-door docs は `P8` close / `P9` next line と整合 |
| 2026-04-28 13:09 JST | `git diff --check` | pass | `P8` closeout diff は whitespace-clean |
| 2026-04-28 12:42 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | helper-local view / telemetry carrier は `retention_scope = helper_local_ephemeral` と `source_refs` を返し、P7 security envelope floor を確認 |
| 2026-04-28 12:42 JST | `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json` | pass | observer route trace は `authority` / `retention_scope` / `source_refs` を返しつつ principal/auth/freshness/witness raw detail を漏らさない |
| 2026-04-28 12:42 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout は `retention_scope_names = helper_local_ephemeral` を返し、visualization / telemetry security inventory を current line に揃える |
| 2026-04-28 12:42 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | helper-local transport canary family は fail-closed route trace を維持したまま green |
| 2026-04-28 12:42 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | runtime provider-boundary view は `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を返す |
| 2026-04-28 12:42 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json` | pass | runtime audit-trace view / telemetry row は typed telemetry security envelope を保ったまま green |
| 2026-04-28 12:42 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout は `visualization_view_lanes` / `telemetry_row_lanes` / `retention_scope_names` を canonical inventory として返す |
| 2026-04-28 12:42 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime current package sample tests 26/26 green、visualization / telemetry inventory regressions を含む |
| 2026-04-28 12:42 JST | `cargo test -p mir-runtime` | pass | runtime crate floor 全体は green |
| 2026-04-28 12:42 JST | `python3 scripts/check_source_hierarchy.py` | pass | required hierarchy intact |
| 2026-04-28 12:42 JST | `python3 scripts/validate_docs.py` | pass | snapshot/front-door docs は P7 close / P8 next line と整合 |
| 2026-04-28 12:42 JST | `git diff --check` | pass | P7 closeout diff は whitespace-clean |
| 2026-04-28 09:51 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` | pass | helper-local `projection_view` は preview-only のまま、place split / authority place / observer view refs を返す |
| 2026-04-28 09:51 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | projection gate close後も visualization lane が壊れていないことを確認 |
| 2026-04-28 09:51 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | report-local `cross_place_projection` が authority/provider placement split を維持していることを確認 |
| 2026-04-28 09:51 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side projection preview regression guard を含めて green |
| 2026-04-28 09:51 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side `cross_place_projection` / layer inventory regression guard を含めて green |
| 2026-04-28 09:51 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | Sugoroku closeout でも `projection_view` は preview-only carrier のまま |
| 2026-04-28 09:51 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | clean near-end closeout でも `cross_place_projection` は report-local preview carrier のまま |
| 2026-04-28 09:51 JST | `find samples/generated -maxdepth 3 -type f | sort` | pass | generated reserve path は `samples/generated/README.md` のみで、actual emitted artifact は未配置 |
| 2026-04-28 09:51 JST | `python3 scripts/check_source_hierarchy.py` | pass | `P3` closeout 後も required source hierarchy is intact |
| 2026-04-28 09:51 JST | `python3 scripts/validate_docs.py` | pass | `P3` queue promotion / generated reserve policy 追記後も docs scaffold は current line と整合 |
| 2026-04-28 09:51 JST | `git diff --check` | pass | `P3` closeout docs/report edits後も whitespace-clean |
| 2026-04-28 09:26 JST | `python3 -m unittest scripts.tests.test_typed_external_boundary_samples` | pass | pretty `list` / `check-all` / `closeout` と `residual_review_matrix` regression を含めて green |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py list` | pass | default pretty `list` formatting bug を修正し、preview subset と residual family split を human-readable に確認 |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py check-all` | pass | default pretty `check-all` formatting bug を修正し、`EXT-03` / `EXT-04` helper subset current floor を再確認 |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py closeout` | pass | default pretty `closeout` formatting bug を修正し、`residual_review_matrix` を human-readable に確認 |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | current executable subset と residual planned family split を JSON closeout でも再確認 |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json` | pass | typed adapter / envelope / witness non-collapse current lane を確認 |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json` | pass | redacted visualization / authority lane を helper-local synthetic preview として確認 |
| 2026-04-28 09:26 JST | `python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json` | pass | typed adapter failure lane と domain mutation non-commit を確認 |
| 2026-04-28 09:26 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | `EXT-02` residual family の indirect envelope anchor を再確認 |
| 2026-04-28 09:34 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` | pass | `EXT-02` residual family の projection preview floor anchor を再確認 |
| 2026-04-28 09:26 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | `EXT-02` / `EXT-05` residual family の indirect visualization anchor を再確認 |
| 2026-04-28 09:26 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | `EXT-01` residual family の `provider_boundary` indirect anchor を再確認 |
| 2026-04-28 09:34 JST | `python3 scripts/check_source_hierarchy.py` | pass | reviewer follow-up edits後も required source hierarchy is intact |
| 2026-04-28 09:34 JST | `python3 scripts/validate_docs.py` | pass | projection anchor 追記と current-status wording 修正後も docs scaffold は current line と整合 |
| 2026-04-28 03:27 JST | `python3 scripts/clean_near_end_samples.py smoke-all` | pass | active clean suite の smoke floor は green |
| 2026-04-28 03:27 JST | `python3 scripts/clean_near_end_samples.py closeout` | pass | active clean suite の closeout snapshot は green |
| 2026-04-28 03:27 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | Sugoroku world vertical slice の runnable floor を再確認 |
| 2026-04-28 03:27 JST | `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | avatar representative slice と residual `FAIRY-05` separation は current queue と一致 |
| 2026-04-28 03:27 JST | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | typed external synthetic preview subset / residual planned family split は維持された |
| 2026-04-28 03:27 JST | `python3 scripts/network_transport_samples.py closeout --format json` | pass | helper-local transport canary family は green |
| 2026-04-28 03:27 JST | `cargo test -p mir-ast` | pass | current-L2 / AST layer の crate test floor は green。support test warning のみ |
| 2026-04-28 03:27 JST | `cargo test -p mir-runtime` | pass | clean near-end runtime / CLI / sample support を含む crate floor は green |
| 2026-04-28 03:27 JST | `cargo test -p mir-semantics` | pass | Lean actual probe と model-check / proof notebook support を含む semantics floor は green |
| 2026-04-28 09:34 JST | `git diff --check` | pass | reviewer follow-up edits後も whitespace-clean |
| 2026-04-27 23:24 JST | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | phase 9 current reading は synthetic preview helper + anchor comparison のまま |
| 2026-04-27 23:24 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` | pass | `projection_view` は helper-local preview floor のまま |
| 2026-04-27 23:24 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | `cross_place_projection` inventory は current report-local preview と一致 |
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
