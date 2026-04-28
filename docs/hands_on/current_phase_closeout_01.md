# current phase closeout 01

## この文書の役割

この文書は、2026-04-28 時点の **repo-local alpha current line** と
**Mirrorea future-axis docs-first line** を、実行コマンドと stop line 付きで短く確認するための hands-on closeout guide です。

- final public completion ではありません
- active sample と planned sample を混同しません
- helper-local debug output を final public API として扱いません

## まず実行するコマンド

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
find samples/generated -maxdepth 3 -type f | sort
bash scripts/env/mirrorea_storage_env.sh
```

## 追加で見る debug lane

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json
python3 scripts/network_transport_samples.py run NET-02 --debug route-trace --format json
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json
python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
```

## これで確認できること

- active clean near-end suite と Sugoroku world / avatar fairy follow representative slice が current runnable floor にあること
- `TermSignature`、`LayerSignature`、`MessageEnvelope`、`VisualizationProtocol` の helper-local / report-local first cut が current line に同期されていること
- `Network transport` の `NET-01` helper-local loopback preview と `NET-02..05` helper-local canary が actualize 済みであり、same-process parity、subprocess JSON bridge、stale reconnect reject、typed failure family、observer-safe redacted route trace を current evidence surface として確認できること
- phase 9 typed external boundary の `EXT-03` / `EXT-04` synthetic preview helper subset が actualize 済みであり、effect boundary / transport envelope / auth evidence / witness refs の non-collapse と typed adapter failure lane を helper self-consistency + anchor comparison の current evidence surface として確認できること
- `P2` residual planned family review が close 済みであり、`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate が current docs / helper closeout に固定されていること
- phase 12 projection / placement の helper/report-local preview floor が actualize 済みであり、`projection_view` と `cross_place_projection` によって system-wide source から authority place / participant place / adapter seam / observer view refs への split を current evidence surface として確認できること
- `P3` projection / placement residual emitted-program gate が close 済みであり、projection validity report minimum、generated artifact reserve policy、`P15` handoff line が current docs に固定されていること
- `HotPlug Patch / AttachPoint` の helper-local lifecycle canary が actualize 済みであり、`detach_request#1` / `detached_roll_request#1` / `hotplug_lifecycle` / attach-detach telemetry-view を envelope-derived evidence として確認できること
- phase 8 avatar representative slice が actualize 済みであり、follow / fallback / stale-anchor rejection / detached-anchor safety を helper-local evidence surface で確認できること
- `auth none` baseline のまま、transport / authentication / membership / capability / witness を collapse していないこと
- typed visualization / telemetry line が label / authority / redaction を意識した evidence carrier として置かれていること
- external workdir と `CARGO_TARGET_DIR` / `CARGO_HOME` binding により、small-VPS 前提の backend/LLVM guardrail が current snapshot に入っていること

## これではまだ確認できないこと

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public `AuthEvidence` schema
- final public adapter API / FFI
- exact host schema
- real network transport
- final projection / placement public API
- final hot-plug runtime lifecycle
- actual LLVM build
- installed binary / FFI / engine adapter / deployment contract

## current closeout の読み

current closeout で揃ったのは、**仕様・sample・helper・report・progress dashboard が同じ現在地を指す状態** です。

- active sample:
  `samples/clean-near-end/`
- active base source corpus:
  `samples/current-l2/`
- active proof evidence:
  `samples/lean/`
- planned sample:
  `samples/not_implemented/`
- prototype / historical:
  `samples/prototype/` と `samples/old/`
- generated artifact reserve:
  `samples/generated/`
- helper-local preview:
  script の `--debug` 出力、detached artifact、report-local inventory
- dashboard:
  `samples_progress.md`
- next queue:
  `tasks.md` と `docs/research_abstract/mirrorea_future_axis_01.md`
- final public API:
  まだ deferred
- deferred mixed gate:
  parser/public API、auth/public contract、adapter/public contract、exact host schema、visualization/public contract、projection/public API、hot-plug/public API

## remaining mixed gate

- final public auth / visualization / projection / hot-plug surface
- final public adapter / exact host schema
- transport canary から real socket / session / durable replay への widening
- detach lifecycle / `AttachPoint` residual contract
- `FAIRY-05` runnable widening decision
- actual LLVM artifact と backend choice

## remaining true user-spec gate

- packaging / installed binary
- FFI / engine adapter / host integration target
- broader application target
- final shared-space operational catalog

## next queue

1. `P4` `TermSignature` registry hardening
2. `P5` `LayerSignature` system hardening

`P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、`P2` Typed external boundary residual planned family review、`P3` Projection / placement residual emitted-program gate は close 済みです。
後続の full queue は `tasks.md` と `progress.md` の current snapshot を参照してください。

## 関連文書

- `../research_abstract/mirrorea_future_axis_01.md`
- `../research_abstract/network_transport_plan_01.md`
- `network_transport_canaries_01.md`
- `../research_abstract/avatar_fairy_follow_plan_01.md`
- `avatar_fairy_follow_representative_slice_01.md`
- `typed_external_boundary_canaries_01.md`
- `projection_placement_views_01.md`
- `../research_abstract/compiler_backend_llvm_preparation_01.md`
- `../../plan/19-repository-map-and-taxonomy.md`
- `../../samples_progress.md`
