# tasks

最終更新: 2026-04-27 20:07 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較は `plan/`、runnable sample 状態は `samples_progress.md`、詳細証跡は `docs/reports/` に置きます。
- append-only の履歴ではなく、現況に合わせて毎回全体を書き直す snapshot として扱います。

## current status at task level

- active clean near-end suite は runnable
- first strong typing layer は finite-index fragment として runnable
- order / handoff は high-level relation family として runnable
- mutex / weak-memory line は model-check second line として runnable
- Lean foundations と generated clean stub corpus は runnable
- Sugoroku world runtime attachment vertical slice は repo-local logical multi-place emulator として runnable
- `samples_progress.md` は phase 0〜16 matrix と storage row に加え、Sugoroku per-sample alignment と phase 8 avatar representative slice row を持つ current dashboard になった
- storage audit と external workdir cutover は `docs/reports/0913-*` と `0915-*` で close してあり、repo `target/` は `/mnt/mirrorea-work/cargo-target` への symlink で運用している
- `Sugoroku sample progress alignment` は `docs/reports/0916-*` で close 済み
- `Avatar fairy follow sample plan` は `docs/reports/0917-*` で close 済み
- `TermSignature registry / debug output` は `docs/reports/0918-*` で close し、Sugoroku `--debug signatures` と clean near-end report/closeout inventory を追加した
- `LayerSignature system` は `docs/reports/0919-*` で close し、Sugoroku helper の `verification` / `runtime_trace` / `membership` layer inventory と、clean near-end report-local `transport_provider_boundary` / `auth_authority_witness` / `verification_model_check` carrier を追加した
- `MessageEnvelope / Auth seam` は `docs/reports/0921-*` で close し、Sugoroku helper の `message_envelopes` / `--debug envelopes` と clean near-end report-local `MessageEnvelope` inventory を追加した
- `VisualizationProtocol` は `docs/reports/0922-*` で close し、Sugoroku helper の `visualization_views` / `telemetry_rows` / `--debug visualization` と clean near-end report-local `VisualizationView` / `TelemetryRow` inventory を追加した
- `Typed external boundary / adapter` は `docs/reports/0923-*` で close し、phase 9 planned family `EXT-01..05` を `samples/not_implemented/typed-external-boundary/` に置き、provider boundary / local queue / typed failure / debug label restriction の current evidence anchor を docs-first に固定した
- `Projection / placement` は `docs/reports/0924-*` で close し、`plan/20-projection-and-placement-roadmap.md` に system-wide source / place-specific program distinction、place split、validity checklist、stop line を追加した
- `HotPlug Patch / AttachPoint` は `docs/reports/0925-*` で close し、`plan/21-hotplug-attachpoint-roadmap.md` に compatibility / activation / migration stop line を追加した
- `HotPlug Patch / AttachPoint executable widening` は `docs/reports/0931-*` で close し、Sugoroku helper の `hotplug_lifecycle` / `--debug hotplug` / `hot-plug` layer inventory、attach / detach telemetry / visualization を current line に actualize した
- `Network transport` は `docs/reports/0926-*` で close し、`plan/22-network-transport-roadmap.md` と phase 13 planned family `samples/not_implemented/network-transport/` に loopback / reconnect / failure matrix の docs-first ladder を追加した
- `Compiler/backend/LLVM preparation` は `docs/reports/0927-*` で close し、`plan/23-compiler-backend-llvm-guardrail-roadmap.md`、`CARGO_HOME` binding、non-destructive probe floor を追加した
- `hands-on docs / closeout` は `docs/reports/0928-*` で close し、`docs/hands_on/README.md` と `docs/hands_on/current_phase_closeout_01.md` を current landing page として追加した
- `Network transport executable widening` は `docs/reports/0929-*` で close し、helper-local `NET-01` loopback preview、`--transport loopback_socket`、loopback parity test を追加した
- `Avatar fairy follow representative slice` は `docs/reports/0930-*` で close し、`scripts/avatar_follow_samples.py`、active sample root `samples/clean-near-end/avatar-follow/`、residual planned `FAIRY-02` / `FAIRY-05`、helper-local debug surface を追加した
- repository structure / layer-boundary staging は `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md`、`docs/research_abstract/repository_layer_structure_01.md` に docs-first で切り出し、high-risk move はまだ行っていない
- current promoted next line は **`Network transport NET-02..05`**
- next reopen point は **`Avatar fairy follow residual widening`**

## current executable floor

### Active clean near-end suite

- typing:
  `01_authorized_declassification`
  `02_unauthorized_declassification_rejected`
  `03_label_flow_rejected`
  `04_capture_escape_rejected`
  `05_cost_bound_rejected`
- order / handoff:
  `01_authorized_roll_publish_handoff`
  `02_missing_witness_rejected`
  `03_handoff_before_publication_rejected`
  `04_stage_block_authorized_handoff`
  `05_delegated_rng_service`
  `06_auditable_authority_witness`
- model-check:
  `01_peterson_sc_pass`
  `02_peterson_relaxed_counterexample`
  `03_broken_mutex_counterexample`
- modal:
  `01_stage_stable_later_minimal`
  `02_published_witnessed_mode_bridge`
- sugoroku-world:
  `00_world_bootstrap`
  `01_runtime_attach_game`
  `02_admin_start_reset`
  `03_roll_publish_handoff`
  `04_non_owner_roll_rejected`
  `05_late_join_history_visible`
  `06_leave_non_owner`
  `07_owner_leave_reassign`
  `08_reset_interleaving_model_check`
  `09_detach_todo`
- avatar-follow:
  `01_follow_remote_head_with_local_fallback`
  `03_remote_avatar_leaves_falls_back_to_local`
  `04_invalid_cross_anchor_chain_rejected`
  `06_model_check_no_detached_anchor_observed`

### Planned but not active sample family

- `samples/not_implemented/avatar-fairy-follow/`
  - `FAIRY-02` visibility-loss fallback
  - `FAIRY-05` target reacquire after return

### Helper entrypoints

- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/clean_near_end_samples.py smoke-all --format json`
- `python3 scripts/clean_near_end_samples.py run typing --format json`
- `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
- `python3 scripts/clean_near_end_samples.py run model-check --format json`
- `python3 scripts/clean_near_end_samples.py run modal --format json`
- `python3 scripts/clean_near_end_samples.py matrix --format json`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `python3 scripts/sugoroku_world_samples.py check-all`
- `python3 scripts/sugoroku_world_samples.py model-check`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/avatar_follow_samples.py check-all --format json`
- `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
- `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `cargo test -p mir-runtime --test clean_near_end_samples`
- `git diff --check`
- `bash scripts/storage/setup_mirrorea_workdisk_root.sh --plan`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/storage/detach_prepare.sh`
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`

## 自走可能な task package

### Package 1. Network transport `NET-02..05`

- phase / stage:
  `Macro 7`, `S1 -> S2`
- rough estimate:
  `1 package`
- objective:
  `plan/22` の two-process / reconnect / typed failure / route trace widening を後段 helper / process harness へ渡す
- expected deliverables:
  `NET-02` process-boundary canary、`NET-03` epoch guard、`NET-04` failure family、`NET-05` redacted route trace plan
- validation command:
  `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json`
- report requirement:
  新しい report、transport widening invariant、process-boundary stop line、auth/transport separation evidence を明記する
- stop line:
  production transport、public deployment contract、cryptographic session protocol は固定しない

### Package 2. Avatar fairy follow residual widening

- phase / stage:
  `Macro 7`, `S1 -> S2`
- rough estimate:
  `1 package`
- objective:
  `FAIRY-02` visibility-loss fallback と `FAIRY-05` reacquire-after-return を representative slice の外側で widen するか判断する
- expected deliverables:
  residual planned family の active promotion条件再評価、必要なら追加 canary
- validation command:
  `python3 scripts/avatar_follow_samples.py closeout --format json`
- report requirement:
  新しい report、representative slice と residual planned family の境界を明記する
- stop line:
  final game/runtime API や production avatar stack は固定しない

## research を通して見つけること

- `TermSignature` の最小粒度と residual obligation surface
- `LayerSignature` first cut の helper/runtime naming をどこまで共有 law surface に寄せるか
- `VisualizationProtocol` の最小 view kind / telemetry row / redaction wording
- avatar follow residual family をどこまで active helper に取り込むか
- projection / placement validity report の最小 shape
- `AttachPoint` compatibility と detach lifecycle の最小 contract

## user が決める必要があること

### Blocker 1. final public contract の固定範囲

- 概要:
  auth / visualization / projection / hot-plug をどこまで public API として固定するか
- 影響:
  package 2 以降の naming と stop line
- 主要な選択肢:
  repo-local helper 優先のまま進める / early public-contract drafting を始める
- current recommendation / 見解:
  まだ repo-local helper と docs-first carrier に留め、public contract は mixed gate に残す

### Blocker 2. packaging / FFI / host integration target

- 概要:
  installed binary、FFI、engine adapter、deployment contract の最終受け皿
- 影響:
  Macro 7 以降の acceptance criteria
- 主要な選択肢:
  CLI / library / engine-adapter / hybrid
- current recommendation / 見解:
  current task では固定せず、backend / LLVM preparation の後に user-spec gate として reopen する
