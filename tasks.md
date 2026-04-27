# tasks

最終更新: 2026-04-27 15:59 JST

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
- `samples_progress.md` は phase 0〜16 matrix と storage row に加え、Sugoroku per-sample alignment と phase 8 avatar skeleton line を持つ current dashboard になった
- storage audit と external workdir cutover は `docs/reports/0913-*` と `0915-*` で close してあり、repo `target/` は `/mnt/mirrorea-work/cargo-target` への symlink で運用している
- `Sugoroku sample progress alignment` は `docs/reports/0916-*` で close 済み
- `Avatar fairy follow sample plan` は `docs/reports/0917-*` で close 済み
- `TermSignature registry / debug output` は `docs/reports/0918-*` で close し、Sugoroku `--debug signatures` と clean near-end report/closeout inventory を追加した
- current promoted next line は **`LayerSignature system`**
- next semantic carrier package は **`MessageEnvelope / Auth seam`**

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

### Planned but not active sample family

- `samples/not_implemented/avatar-fairy-follow/`
  - `FAIRY-01` remote head follow with local fallback
  - `FAIRY-02` visibility-loss fallback
  - `FAIRY-03` leave-triggered stale-anchor rejection
  - `FAIRY-04` invalid cross-anchor rejection
  - `FAIRY-05` target reacquire after return
  - `FAIRY-06` detached-anchor safety model-check

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
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `bash scripts/storage/setup_mirrorea_workdisk_root.sh --plan`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/storage/detach_prepare.sh`
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`

## 自走可能な task package

### Package 1. LayerSignature system

- phase / stage:
  `Macro 6`, `S1 -> S2`
- objective:
  auth / verification / visualization / transport / telemetry を typed layer として合成し、law surface を明示する
- expected deliverables:
  `LayerSignature`、requires / provides / transforms / checks / emits / laws の整理、docs、examples
- validation command:
  `python3 scripts/current_l2_guided_samples.py closeout --format json`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、snapshot 同期、layer law と stop line の明記
- stop line:
  final public plugin API、tool-brand 固定、production verifier binding は行わない

### Package 2. MessageEnvelope / Auth seam

- phase / stage:
  `Macro 6`, `S2 -> S3`
- objective:
  transport、authentication、authorization、membership、capability、witness を分けた envelope carrier を置く
- expected deliverables:
  `MessageEnvelope`、`AuthEvidence`、`PrincipalClaim`、membership epoch / incarnation carrier、transport adapter seam、docs
- validation command:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py model-check`
- report requirement:
  新しい report、current none-auth baseline と future session / signature path の切り分けを記録する
- stop line:
  production auth protocol、federation、real socket transport は固定しない

### Package 3. VisualizationProtocol first implementation

- phase / stage:
  `Macro 6`, `S2 -> S3`
- objective:
  static view / runtime trace / redaction-aware debug output / typed telemetry をまとめる
- expected deliverables:
  static snapshot view、runtime trace view、membership / witness timeline、redaction policy、docs
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json`
  `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership`
- report requirement:
  新しい report、helper-local debug output と final public viewer の区別を明記する
- stop line:
  final public visualization protocol、retention policy、multi-tenant telemetry service は固定しない

### Package 4. Typed external boundary / adapter sample plan

- phase / stage:
  `Macro 6`, `S1 -> S2`
- objective:
  phase 9 `EXT-01..05` representative sample と adapter failure / debug label restriction line を sample-first に整理する
- expected deliverables:
  sample plan、command plan、debug route plan、docs
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、stdio 非採用と typed adapter path の boundary を明記する
- stop line:
  concrete browser/network/VR adapter 実装は固定しない

### Package 5. Projection / placement plan

- phase / stage:
  `Macro 6`, `S1 -> S2`
- objective:
  system-wide source から server / participant / adapter / visualizer へ projection する validity line を整理する
- expected deliverables:
  projection doc、validity checklist、place split examples、必要なら helper-local report
- validation command:
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、projection invariant と stop line を明記する
- stop line:
  generated public backend、placement optimizer、final operational scheduler は実装しない

### Package 6. Hot-plug Patch / AttachPoint

- phase / stage:
  `Macro 6`, `S1 -> S2`
- objective:
  `Patch Req Prov Δ`、`AttachPoint`、compatibility / activation / migration contract の最小設計を置く
- expected deliverables:
  docs-first design、sample sketch、compatibility checklist、optional helper-local canary
- validation command:
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、activation cut と migration stop line を明記する
- stop line:
  distributed activation / rollback、durable state migration、production attach/detach API は固定しない

### Package 7. Network transport plan

- phase / stage:
  `Macro 6 -> Macro 7`, `S0 -> S1`
- objective:
  local queue -> two-process loopback -> explicit transport failure の docs-first sample ladder を切る
- expected deliverables:
  `NET-01..05` plan、reconnect / membership epoch line、transport failure explicit reject path
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、logical emulator と network plan の boundary を明記する
- stop line:
  production transport 実装や public deployment contract は固定しない

### Package 8. Compiler/backend/LLVM preparation guardrail

- phase / stage:
  `Macro 7`, `S0 -> S1`
- objective:
  small VPS 上で LLVM / build artifact / cache を root に溜めない guardrail を先に固める
- expected deliverables:
  storage env hardening、backend probe plan、artifact location report、cargo registry cache / LLVM path actual probe
- validation command:
  `bash scripts/env/mirrorea_storage_env.sh`
  `bash scripts/storage/detach_prepare.sh`
  `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、mounted/unmounted state、actual probe、cutover条件を明記する
- stop line:
  actual LLVM build や backend public contract は固定しない

## research を通して見つけること

- `TermSignature` の最小粒度と residual obligation surface
- `LayerSignature` law surface のどこまでを runtime / theorem / visualization に配るか
- avatar fairy helper surface を Sugoroku helper extension と専用 helper のどちらに置くか
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
