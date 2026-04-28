# samples_progress

Last updated: 2026-04-28 13:09 JST
Current repo-local focus: clean near-end current layer と Sugoroku world / avatar follow representative slice の runnable floor を維持したまま、`P8` Sugoroku runtime attach hardening で MembershipRegistry source-of-truth / world sugar / hot-plug stop line を fixed し、`P9` avatar fairy follow hardening へ進める
Current active packages: `P9` avatar fairy follow hardening

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
| Avatar follow | 90 | widened active representative slice + single residual planned family | `FAIRY-01/02/03/04/06` を維持しつつ `FAIRY-05` は docs-first gate fixed / runnable widening deferred に保つ | `python3 scripts/avatar_follow_samples.py closeout --format json` |
| External adapters | 75 | synthetic preview helper + residual review closed | `EXT-03` / `EXT-04` synthetic preview subset を維持しつつ `EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen matrix を fixed 済みとして保つ | `python3 scripts/typed_external_boundary_samples.py closeout --format json` |
| Network transport | 75 | helper-local canary family | `NET-01..05` helper-local canary を維持しつつ real socket / durable replay を deferred に保つ | `python3 scripts/network_transport_samples.py closeout --format json` |
| Visualization | 100 | helper-local + report-local security hardening closed | `visualization_views` / `telemetry_rows` / `retention_scope` / `source_refs` / fail-closed observer route trace を current floor として維持 | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` |
| Projection / placement | 75 | helper-local + report-local preview floor + emitted-program gate closed | `projection_view` と `cross_place_projection` を維持しつつ validity report minimum / generated artifact reserve policy を docs-first に固定した | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` |
| Hot-plug package | 75 | helper-local lifecycle canary | `hotplug_lifecycle` / `--debug hotplug` / attach-detach telemetry を維持しつつ real migration を deferred に保つ | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json` |

## Active sample matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH0` | repository memory | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | dashboard / hierarchy check | 90 | mixed | 2026-04-28 13:09 JST | `0913`, `0920`, `0943`, `0945`, `0955` | source hierarchy と report discipline の current floor |
| `PH1` | Mir core | `samples/current-l2/` | base corpus | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | final parser / public API deferred |
| `PH6` | compile-ready minimal actualization | `samples/clean-near-end/` | active clean suite | 90 | positive + negative | 2026-04-28 03:27 JST | `0904`, `0913`, `0945` | public shell / packaging deferred |
| `SUG-01` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | active runnable | 90 | positive | 2026-04-28 13:09 JST | `0909`, `0916`, `0931`, `0945`, `0955` | runtime attach floor + helper-local attachpoint compatibility / activation evidence。current closeout では `world_surface` と `MembershipRegistry` source-of-truth の anchor にもなる |
| `SUG-03` | Sugoroku runtime | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | active runnable E2E | 90 | positive | 2026-04-28 13:09 JST | `0909`, `0916`, `0918`, `0919`, `0942`, `0945`, `0950`, `0952`, `0954`, `0955` | roll -> publish -> witness -> handoff。`TermSignature` / `LayerSignature` inventory、projection preview、visualization security envelope の基準点であり、current P8 close では handoff target activity boundary の anchor でもある |
| `SUG-05` | shared-space membership | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | active runnable E2E | 90 | positive | 2026-04-28 13:09 JST | `0909`, `0916`, `0919`, `0945`, `0955` | membership timeline anchor。published history visible before turn-order insertion の closeout wording anchor |
| `SUG-08` | theorem / model-check boundary | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | active runnable E2E | 90 | positive | 2026-04-28 03:27 JST | `0909`, `0916`, `0945` | reset safety bridge |
| `SUG-09` | hot-plug preview | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | active TODO boundary | 75 | explicit TODO + rejection evidence | 2026-04-28 13:09 JST | `0909`, `0916`, `0925`, `0931`, `0955` | completion evidence ではなく stop line。detach は explicit TODO stop line であり completed migration ではない |
| `FAIRY-01` | avatar follow | `samples/clean-near-end/avatar-follow/01_follow_remote_head_with_local_fallback.mir` | active representative canary | 90 | positive | 2026-04-28 03:27 JST | `0917`, `0930`, `0945` | visible remote head follow with explicit local fallback lineage |
| `FAIRY-02` | avatar follow | `samples/clean-near-end/avatar-follow/02_remote_head_not_visible_falls_back_to_local.mir` | active representative canary | 90 | positive + fallback | 2026-04-28 03:27 JST | `0917`, `0930`, `0933`, `0945` | visibility loss falls back locally and does not claim transport recovery |
| `FAIRY-06` | avatar follow | `samples/clean-near-end/avatar-follow/06_model_check_no_detached_anchor_observed.mir` | active representative canary | 90 | verification | 2026-04-28 03:27 JST | `0917`, `0930`, `0945` | detached-anchor safety verification bridge |
| `AUTH-01` | auth seam | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local envelope carrier | 90 | positive + negative | 2026-04-27 23:24 JST | `0921`, `0941` | local queue baseline with `auth none`, membership freshness, capability refs, witness refs |
| `VIS-01` | visualization | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local view/telemetry carrier | 100 | positive | 2026-04-28 12:42 JST | `0922`, `0942`, `0954` | `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を持つ evidence-oriented view と typed telemetry。final public viewer contract ではない |
| `PRJ-01` | projection / placement | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local projection preview | 75 | positive | 2026-04-28 09:51 JST | `0924`, `0942`, `0948` | `projection_view` で system source から place split と observer view refs を visible にする。final emitted place program ではない |
| `PRJ-02` | projection / placement | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | report-local provider placement preview | 75 | positive | 2026-04-28 09:51 JST | `0924`, `0942`, `0948` | `cross_place_projection` で authority placement と provider placement を分けて visible にする |

## Base corpus / planned / spec-only matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH2` | parser-free PoC | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | detached loop | 75 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | dedicated detached-loop CLI refresh remains open |
| `PH3` | parser / checker cut | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | crate test surface | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | parser grammar freeze deferred |
| `PH4` | shared-space membership / room boundary | `samples/clean-near-end/sugoroku-world/`, `plan/16-shared-space-membership-and-example-boundary.md` | runnable room boundary | 90 | positive + negative | 2026-04-27 19:56 JST | `0909`, `0916` | late join / leave / owner reassign / history visibility が current anchor |
| `PH5` | theorem / model-check boundary | `samples/clean-near-end/{typing,model-check}`, `samples/lean/` | active bridge family | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | proof/model-check public contract absent |
| `PH7` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/`, `scripts/sugoroku_world_samples.py` | active vertical slice + helper/test/docs hardening closed | 90 | positive + negative | 2026-04-28 13:09 JST | `0909`, `0916`, `0931`, `0945`, `0955` | attach / handoff / witness / late join / reset model-check / detach TODO boundary。current closeout は world sugar / MembershipRegistry / stop line を explicit に保つ |
| `PH8` | avatar follow | `samples/clean-near-end/avatar-follow/`, `samples/not_implemented/avatar-fairy-follow/` | widened active representative slice + residual planned family | 90 | positive + negative + verification | 2026-04-28 03:27 JST | `0917`, `0930`, `0933`, `0939`, `0945` | active canary は `FAIRY-01/02/03/04/06`、residual planned family は `FAIRY-05` |
| `FAIRY-05` | avatar follow | `samples/not_implemented/avatar-fairy-follow/FAIRY-05_reacquire_after_return.md` | planned family | 10 | target only | 2026-04-27 21:38 JST | `0939` | explicit state timeline / anchor switch evidence が reopen 条件。carrier bundling は `UNRESOLVED` |
| `PH9` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview subset + residual planned family | 75 | positive + negative | 2026-04-28 09:26 JST | `0923`, `0941`, `0945`, `0946` | phase 9 `.mir` direct semantic execution ではなく helper self-consistency + anchor comparison。residual reopen matrix は fixed |
| `EXT-01/02/05` | typed external boundary | `samples/not_implemented/typed-external-boundary/` | residual planned family | 10 | target only | 2026-04-28 09:34 JST | `0923`, `0941`, `0946` | indirect anchor / reopen criterion / kept-later gate は fixed。final host-facing contract は mixed gate |
| `EXT-03/04` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `scripts/typed_external_boundary_samples.py` | synthetic preview helper subset | 75 | positive + negative | 2026-04-28 09:26 JST | `0941`, `0945`, `0946` | typed adapter failure lane、envelope split、redacted visualization lane |
| `PH10` | MessageEnvelope / AuthEvidence seam | Sugoroku helper、clean near-end runtime report | helper-local + report-local carrier | 100 | positive + negative | 2026-04-28 12:18 JST | `0921`, `0953` | helper `message_envelope_scope = representative_slice`、runtime `message_envelope_scope = clean_near_end_canonical_inventory`、legacy `transport` alias は seam 意味へ正規化済みで、`transport_medium` / `transport_seam` / `emitter_principal` / `freshness_checks` / shared `auth_evidence_lanes` を fixed。final public auth schema / transport ABI / `witness_refs` role taxonomy ではない |
| `PH11` | TermSignature / LayerSignature | Sugoroku helper、clean near-end runtime report | helper-local + report-local inventory | 100 | positive | 2026-04-28 11:19 JST | `0918`, `0919`, `0950`, `0952` | `TermSignature` は current `kind/name/evidence_role` lanes を固定済み。`LayerSignature` は current `name/requires/provides/transforms/checks/emits/obligations/laws` row schema、helper `representative_slice` inventory、runtime `clean_near_end_canonical_inventory` inventory、helper representative names `verification_handoff_witness` / `runtime_turn_trace` / `membership_*` / `hotplug_*`、runtime canonical names `auth_authority_witness` / `transport_provider_boundary` / `verification_model_check` を fixed。final shared law schema ではない |
| `PH12` | projection / placement | `plan/20-projection-and-placement-roadmap.md`, `docs/hands_on/projection_placement_views_01.md` | helper/report-local preview floor + docs-first emitted-program gate | 75 | positive | 2026-04-28 09:51 JST | `0924`, `0942`, `0948` | preview floor に加えて validity report minimum / generated artifact reserve policy / `P15` handoff line を fixed。actual emitted program family は deferred |
| `PH13` | network transport | `scripts/network_transport_samples.py`, `samples/not_implemented/network-transport/` | helper-local canary family + planned source family | 75 | positive + negative | 2026-04-28 03:27 JST | `0926`, `0929`, `0932`, `0945` | local queue / loopback / reconnect / typed failure / redacted route trace |
| `NET-01` | network transport | `scripts/sugoroku_world_samples.py --transport loopback_socket` | helper-local loopback preview | 75 | positive + negative | 2026-04-28 03:27 JST | `0929`, `0932`, `0945` | same-process parity preview |
| `NET-02..05` | network transport | `scripts/network_transport_samples.py` | helper-local canary family | 75 | positive + negative | 2026-04-28 03:27 JST | `0932`, `0945` | subprocess JSON bridge / stale reconnect reject / typed failure / redacted route trace |
| `PH14` | hot-plug package | `plan/21-hotplug-attachpoint-roadmap.md`, `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | helper-local lifecycle canary + docs-first gate | 75 | positive + negative | 2026-04-28 13:09 JST | `0925`, `0931`, `0955` | real migration / rollback / attachpoint runtime remain deferred |
| `PH15` | visualization / IDE | Sugoroku helper views、runtime report-local views | helper-local + report-local security hardening closed | 100 | positive | 2026-04-28 12:42 JST | `0922`, `0954` | helper/runtime security envelope、typed telemetry retention floor、NET-05 fail-closed route trace まで current scope close。final public viewer contract は deferred |
| `PH16` | compiler / backend / LLVM preparation | `/mnt/mirrorea-work`, `scripts/env/mirrorea_storage_env.sh`, `plan/23-compiler-backend-llvm-guardrail-roadmap.md` | storage / backend guardrail | 75 | operational positive | 2026-04-27 18:48 JST | `0913`, `0915`, `0927` | actual LLVM build / backend choice is still later |

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | Last result |
|---|---|---|---:|---|---|
| `E2E-CLEAN-SUITE` | current-L2 -> clean near-end | `python3 scripts/clean_near_end_samples.py closeout` | 90 | active clean near-end suite の positive / negative floor と closeout snapshot が still green | pass 2026-04-28 03:27 JST |
| `E2E-SUGOROKU` | membership -> attach -> roll -> publish -> handoff -> late join | `python3 scripts/sugoroku_world_samples.py closeout --format json` | 90 | current shared-space vertical slice の runnable floor | pass 2026-04-28 13:09 JST |
| `E2E-AVATAR` | follow -> fallback -> stale-anchor rejection -> safety property | `python3 scripts/avatar_follow_samples.py closeout --format json` | 90 | representative avatar slice の active floor | pass 2026-04-28 03:27 JST |
| `E2E-TYPED-EXTERNAL-TARGET` | source stub -> adapter preview -> anchor comparison | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | 75 | helper self-consistency + anchor comparison の current floor。phase 9 `.mir` direct execution ではない | pass 2026-04-28 09:26 JST |
| `E2E-TRANSPORT-CANARY` | loopback / reconnect / failure / redacted trace | `python3 scripts/network_transport_samples.py closeout --format json` | 75 | helper-local transport canary family | pass 2026-04-28 12:42 JST |
| `E2E-PROJECTION-TARGET` | system source -> emitted place program -> equivalent trace | future target: current `PRJ-01` / `PRJ-02` preview floor + later emitted-program runner | 10 | compositional projection E2E は still later。`P3` current package で actualize したのは preview floor と emitted-program gate の docs-first boundaryだけ | target only |

## Build / storage environment

| Item | Status | Validation | Notes |
|---|---|---|---|
| external workdir `/mnt/mirrorea-work` | active | `findmnt /mnt/mirrorea-work` | `/dev/vdb1` ext4 mount、`target/` SSD cutover 済み |
| cargo registry cache | active guardrail | `bash scripts/env/mirrorea_storage_env.sh` | `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache` を intended binding にしている |
| LLVM path readiness | guardrail only | `ls -ld /mnt/mirrorea-work/llvm/*` | directory はあるが actual LLVM build はまだ |

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|
| final public adapter / host schema scope | typed external boundary | high | user + repo | residual reopen matrix は fixed 済みとし、host-facing contract 本体は `P12` / `P18` に残す |
| actual emitted program / cross-place equivalence checker family | projection / placement | high | repo | `P3` docs-first boundary は fixed 済みとし、`P15` で emitted family / generated artifact inventory / equivalence review surface を actualize する |
| final public viewer contract / retention policy / multi-tenant telemetry service | visualization / telemetry | high | repo + user | `P7` current scope は close 済みとして保ち、public contract 自体は `P16` / `P18` に残す |
| `FAIRY-05` carrier bundling | avatar follow | medium | repo | explicit state timeline / anchor switch evidence を保ったまま `UNRESOLVED` を解く |
| real migration / rollback | hot-plug | medium | repo | helper-local lifecycle canary の先に attachpoint migration contract を切り出す |
| real socket / durable replay | network transport | high | repo + user | helper-local canary を維持しつつ production transport line は defer する |
| actual LLVM build / backend choice | compiler / backend | medium | user + repo | guardrail を壊さず、later package で backend target を決める |

## Recent validation

| Date | Command | Result | Notes |
|---|---|---|---|
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
