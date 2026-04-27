# tasks

最終更新: 2026-04-27 13:22 JST

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
- `samples_progress.md` は phase 0〜16 matrix と storage row を持つ current dashboard として追加済み
- storage audit の current result は、root `/dev/vda2` 99G 中 32G free、`/dev/vdb1` ext4 `mirrorea-work` が `/mnt/mirrorea-work` に mounted、repo `target/` は `/mnt/mirrorea-work/cargo-target` への symlink に切り替え済み
- `phase-sample-progress-storage-foundation` package は `docs/reports/0913-*` で close 済み
- current promoted next line は **`Sugoroku sample progress alignment`**
- next semantic carrier package は **`TermSignature registry / debug output`**

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
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `bash scripts/storage/setup_mirrorea_workdisk_root.sh --plan`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/storage/detach_prepare.sh`
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`

## 自走可能な task package

### Package 1. Sugoroku sample progress alignment

- phase / stage:
  `Macro 0 + Macro 6`, `S5-S6 maintenance`
- objective:
  `samples_progress.md` の phase 4 / 7 row を Sugoroku helper の actual command / debug surface / report reference と tighter に揃える
- expected deliverables:
  per-sample or finer-grained Sugoroku row split、`summary` / `membership` / `verification` debug surface の row 反映、closeout evidence refresh
- validation command:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、`samples_progress.md` / `progress.md` / `tasks.md` 同期
- stop line:
  real network、consensus、durable distributed commit、final detach lifecycle は固定しない

### Package 2. Avatar fairy follow sample plan

- phase / stage:
  `Macro 6`, `S0 -> S1`
- objective:
  phase 8 representative sample family、helper surface、negative case、prototype anchor を current active plan として固める
- expected deliverables:
  `FAIRY-01..06` sample plan、active/historical boundary note、必要なら sample skeleton path decision
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report、prototype sample と active sample の境界を明記
- stop line:
  final avatar API、engine binding、real network avatar sync は固定しない

### Package 3. TermSignature registry / debug output

- phase / stage:
  `Macro 6`, `S1 -> S2`
- objective:
  term / transition / effect / message / adapter を横断して、static checker / theorem / model-check / runtime guard / debug output / hot-plug compatibility が同じ signature carrier を参照できるようにする
- expected deliverables:
  `TermSignature`、`SignatureRegistry`、signature dump、docs、sample-facing explanation
- validation command:
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json`
  に加え、package close までに `--debug signatures` 相当の entry
- report requirement:
  新しい report を追加し、`samples_progress.md` / `progress.md` / `tasks.md` を同じ task で更新する
- stop line:
  final public signature schema、generic public API、concrete network transport bindingは固定しない

### Package 4. LayerSignature system

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

### Package 5. MessageEnvelope / Auth seam

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

### Package 6. VisualizationProtocol first implementation

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

### Package 7. Typed external boundary / adapter sample plan

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

### Package 8. Projection / placement plan

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

### Package 9. Hot-plug Patch / AttachPoint

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

### Package 10. Network transport plan

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

### Package 11. Compiler/backend/LLVM preparation guardrail

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

### Package 12. Hands-on docs / closeout

- phase / stage:
  `Macro 0-6` closeout
- objective:
  docs / specs / plan / `samples_progress.md` / progress / tasks / report / validation evidence を再同期し、残 gate を圧縮して示す
- expected deliverables:
  closeout report、snapshot refresh、remaining mixed gate と true user-spec gate の整理
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `python3 scripts/clean_near_end_samples.py smoke-all --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo test -p mir-ast`
  `cargo test -p mir-runtime`
  `cargo test -p mir-semantics`
  に、当時点で追加された helper closeout command を足す
- report requirement:
  新しい report、exact validation result と skip reason を残す
- stop line:
  final public completion、production-ready、all theory solved とは書かない

## research を通して見つけること

### phase/sample percentage calibration

- 概要:
  phase row を coarse 1 行で持つか、sample family ごとに細分化するか
- 何に影響するか:
  `samples_progress.md` の保守性、100% 過剰申告リスク、report granularity
- 主要な選択肢:
  - phase row を維持して representative sample を cell に持つ
  - Sugoroku / current-L2 から先に finer row へ割る
- current recommendation:
  Sugoroku alignment package で phase 4 / 7 を先に finer row へ寄せる

### detached artifact relocation strategy

- 概要:
  `target/current-l2-detached/` default を external workdir へどう ratchet するか
- 何に影響するか:
  small VPS root pressure、report path stability、helper CLI default
- 主要な選択肢:
  - default は repo-local のまま、heavy run だけ override
  - env script で global redirect を入れる
- current recommendation:
  まず override path と cleanup policy を固め、default 切替は later

### LayerSignature law / composition order

- 概要:
  no hidden authority / no hidden downgrade / evidence preservation / placement preservation を、どの layer order で最も自然に保てるか
- 何に影響するか:
  TermSignature、viewer、auth seam、theorem / model-check handoff
- 主要な選択肢:
  - layer metadata だけ先に置く
  - helper-local canary implementation まで同時に入れる
- current recommendation:
  先に metadata / law / docs を固め、canary implementation は narrow に足す

## user が決める必要があること

### final public host / packaging target

- 概要:
  repo-local helper floor を installed binary / host-facing contract に進めるか
- 何に影響するか:
  Macro 7、artifact retention、CI、engine / host integration
- 主要な選択肢:
  - repo-local helper floor を維持する
  - packaging / installed binary を first-class target にする
  - FFI / engine adapter を先に設計する
- current recommendation:
  まだ repo-local floor を維持する

### exhaustive shared-space / product target

- 概要:
  authoritative-room minimal subset の次に、どの shared-space catalog / application target を first-class にするか
- 何に影響するか:
  Macro 6-8、auth policy、projection scope、public API、non-functional requirement
- 主要な選択肢:
  - minimal subset を維持し、future-axis package を先に閉じる
  - exhaustive shared-space catalog を先に固定する
  - broader product target を先に固定する
- current recommendation:
  まず future-axis package を self-driven で進め、catalog / product target は user specification を待つ
