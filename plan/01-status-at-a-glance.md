# plan/01 — 現況サマリ

## repo 全体の主眼

- 主眼は、Mir current-L2 の repo-local alpha-ready current layer を保ちながら、Mirrorea future-axis を sample / progress / storage discipline と結びつけて前進させることにある
- current active sample suite は `samples/clean-near-end/`
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/` と `scripts/sugoroku_world_samples.py` で runnable
- `samples_progress.md` は phase 0〜16 の runnable sample dashboard として current snapshot に入る
- `crates/mirrorea-core` は subsystem boundary を明示する current minimal carrier crate であり、`LayerSignature` / `PrincipalClaim` / `AuthEvidence` / `MessageEnvelope` と lane inventory / invariant helper に加え、`MembershipRegistry` typed source-of-truth substrate と `PlaceCatalog` logical multi-place catalog substrate を持つ。`crates/mirrorea-control` は引き続き placeholder skeleton である
- Mirrorea / PrismCascade / Typed-Effect Wiring Platform は separable track として扱う

## current executable floor

- `samples/clean-near-end/` active suite 16 本は runnable
- `samples/clean-near-end/sugoroku-world/` vertical slice 10 本は runnable
- `crates/mir-runtime/src/clean_near_end.rs` が finite-index typing / order-handoff / model-check / modal current layer を持つ
- `scripts/sugoroku_world_samples.py` が logical multi-place runtime attachment emulator を持つ
- `scripts/projection_codegen_samples.py` と `samples/generated/projection-placement/manifest.json` が projection/codegen current first-cut の committed generated bridge evidence と live-anchor alignment floor を持つ
- `scripts/check_source_hierarchy.py` と `scripts/validate_docs.py` が repository memory / report / dashboard 側の baseline check になる

## current storage audit snapshot

- root disk:
  `/dev/vda2` 99G 中 32G free
- repo size:
  `90M`
- `target/`:
  repo path は `/mnt/mirrorea-work/cargo-target` への symlink に切り替え済み
  - external usage:
    `5.3G`
- cargo registry cache:
  `/mnt/mirrorea-work/cargo-registry-cache`
  - current probe:
    `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`
- `.git/`:
  `69M`
- extra storage:
  `/dev/vdb1` ext4 `mirrorea-work` が `/mnt/mirrorea-work` に mounted
  - UUID:
    `a87650a8-e3e9-4977-8940-6c293a0ee23c`
  - `/etc/fstab`:
    UUID-based `defaults,nofail`
  - current active cutover:
    `target/` は SSD 側へ移送済み
  - LLVM path readiness:
    `/mnt/mirrorea-work/llvm/{src,build,install}` は作成済み、actual artifact はまだない

## twin peaks の current state

### Problem 1

- current first line:
  finite-index first strong typing layer、Lean-first proof skeleton、model-check second-line handoff
- active sample:
  clean typing 5 本
- repo-local evidence:
  `run typing`
  `matrix`
  `closeout`
  `samples/lean/foundations/`
  `samples/lean/clean-near-end/`
- still later:
  final typed source principal、final theorem result object、final public checker / verifier contract

### Problem 2

- current first line:
  order / handoff relation decomposition、witness / publication discipline、model-check second-line split
- active sample:
  clean order-handoff 6 本 + model-check 3 本
- repo-local evidence:
  `run order-handoff`
  `run model-check`
  `closeout`
- still later:
  final source wording、final emitted-artifact / public contract、exhaustive shared-space catalog

### Sugoroku world / Mirrorea vertical slice

- current first line:
  empty world server、runtime attach、membership epoch / incarnation、publish / witness / handoff、late join、leave、owner leave、reset model-check
- active sample:
  `samples/clean-near-end/sugoroku-world/00...09`
- repo-local evidence:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py model-check`
  `python3 scripts/sugoroku_world_samples.py closeout`
- still later:
  real network、multi-server consensus、durable distributed commit、detach lifecycle implementation、final public API

### phase / sample / storage foundation lane

- `samples_progress.md` を current runnable dashboard として追加した
- `scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh` を small-VPS safe default で置く
- `docs/reports/0913-*` で phase-sample-progress-storage-foundation package を close する
- `docs/reports/0915-*` で `/mnt/mirrorea-work` mount verification、`target/` SSD cutover、`cargo test -p mir-ast --no-run` の軽量確認を追加した
- `docs/reports/0916-*` で Sugoroku per-sample alignment と debug surface mapping を close した
- `docs/reports/0917-*` で phase 8 avatar fairy follow skeleton family と prototype boundary を close した
- `docs/reports/0918-*` で `TermSignature registry / debug output` を close し、Sugoroku `--debug signatures` と clean near-end report/closeout inventory を追加した
- `docs/reports/0919-*` で `LayerSignature system` first cut を close し、helper-local / runtime report-local lane inventory を追加した
- `docs/reports/0952-*` で `P5` `LayerSignature` system hardening を close し、helper/runtime `name` row key、`obligations` lane、`layer_signature_scope` distinction、representative/canonical inventory split を current line に actualize した
- `docs/reports/0953-*` で `P6` `MessageEnvelope / AuthEvidence` seam hardening を close し、helper/runtime `message_envelope_scope`、`transport_medium` / `transport_seam`、`emitter_principal`、`freshness_checks`、shared `auth_evidence_lanes` を current line に actualize した
- `docs/reports/0954-*` で `P7` `VisualizationProtocol / VisualizationSecurity` hardening を close し、helper/runtime view / telemetry security envelope、typed telemetry retention floor、NET-05 fail-closed observer route trace を current line に actualize した
- `docs/reports/0955-*` で `P8` Sugoroku runtime attach hardening を close し、Sugoroku helper closeout `world_surface`、`membership_model.source_of_truth`、`membership_model.late_join_handoff_boundary`、`hotplug_stop_line` を current line に actualize した
- `docs/reports/0921-*` で `MessageEnvelope / Auth seam` を close し、Sugoroku helper の `message_envelopes` / `--debug envelopes` と clean near-end report-local `MessageEnvelope` inventory を追加した
- `docs/reports/0922-*` で `VisualizationProtocol` を close し、Sugoroku helper の `visualization_views` / `telemetry_rows` / `--debug visualization` と clean near-end report-local `VisualizationView` / `TelemetryRow` inventory を追加した
- `docs/reports/0923-*` で `Typed external boundary / adapter` docs-first sample plan を close し、phase 9 planned family `EXT-01..05` を `samples/not_implemented/typed-external-boundary/` に置いた
- `docs/reports/0941-*` で `Typed external boundary executable widening` を close し、`scripts/typed_external_boundary_samples.py`、`samples/not_implemented/typed-external-boundary/` を読む `EXT-03` / `EXT-04` synthetic preview helper subset、residual planned family `EXT-01` / `EXT-02` / `EXT-05`、`plan/25-typed-external-boundary-executable-roadmap.md` を追加した
- `docs/reports/0924-*` で `Projection / placement` docs-first plan を close し、`plan/20-projection-and-placement-roadmap.md` を追加した
- `docs/reports/0942-*` で `Projection / placement executable widening` を close し、Sugoroku helper `projection_view` / `--debug projection` と clean near-end runtime report-local `cross_place_projection` を追加した
- `docs/reports/0970-*` で `P15` projection/codegen first emitted place-specific programs の current first-cut closeout を close し、`scripts/projection_codegen_samples.py`、`samples/generated/projection-placement/manifest.json`、`P15-GEN-01..04` committed generated bridge evidence、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor` を current line に actualize した
- `docs/reports/0925-*` で `HotPlug Patch / AttachPoint` docs-first plan を close し、`plan/21-hotplug-attachpoint-roadmap.md` を追加した
- `docs/reports/0926-*` で `Network transport` docs-first plan を close し、`plan/22-network-transport-roadmap.md` と phase 13 planned family `samples/not_implemented/network-transport/` を追加した
- `docs/reports/0927-*` で `Compiler/backend/LLVM preparation` guardrail を close し、`plan/23-compiler-backend-llvm-guardrail-roadmap.md`、`CARGO_HOME` binding、non-destructive probe evidence を追加した
- `docs/reports/0928-*` で `hands-on docs / closeout` を close し、`docs/hands_on/README.md` と `docs/hands_on/current_phase_closeout_01.md` を追加した
- `docs/reports/0929-*` で `Network transport executable widening` を close し、helper-local `NET-01` loopback preview、transport seam parameter、loopback parity test を追加した
- `docs/reports/0930-*` で `Avatar fairy follow representative slice` を close し、`scripts/avatar_follow_samples.py`、active sample root `samples/clean-near-end/avatar-follow/`、initial residual planned `FAIRY-02` / `FAIRY-05`、hands-on landing page を追加した
- `docs/reports/0931-*` で `HotPlug Patch / AttachPoint executable widening` を close し、Sugoroku helper の `hotplug_lifecycle`、`--debug hotplug`、attach / detach telemetry-view、`detach_request#1` auth-none envelope canary を追加した
- `docs/reports/0932-*` で `Network transport helper-local canaries` を close し、`scripts/network_transport_samples.py`、active landing page `samples/clean-near-end/network-transport/README.md`、`NET-02..05` process-boundary / reconnect / typed failure / redacted route trace canary を追加した
- `docs/reports/0946-*` で `P2` Typed external boundary residual planned family review を close し、typed external helper の pretty `list` / `check-all` / `closeout` bug を修正した上で residual reopen matrix を current repo に actualize した
- `docs/reports/0950-*` で `P4` `TermSignature` registry hardening を close し、helper / runtime closeout の `signature_lanes` / `signature_scope` / `signature_evidence_roles`、active kind family、reserved kind split を current repo に actualize した
- `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md` で current repo taxonomy と staged migration plan を docs-first に固定した
- `P11` logical multi-place runtime tranche の current third cut は actualize 済みであり、`MembershipRegistry` / `PlaceCatalog` substrate の上に participant-place-kind-gated principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper を置いた
- `P12` external adapter / host boundary tranche の current first cut は close 済みであり、typed external helper subset / closeout に `host_boundary_scope` / `host_boundary_lanes` / `non_collapse_lanes` / `host_family_gates` / `host_boundary_inventory` を actualize 済みである
- current promoted next package は `P16` visual debugger / viewer first public prototype であり、current safest first cut は helper/report-local typed visualization inventory を public prototype boundary と誤読させずに widen することである
- next reopen point は `P17` storage / LLVM / backend preparation

### Mirrorea future-axis carrier lane

- project axis:
  **正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム**
- `docs/reports/0912-*` で package 1 current-state audit と package 2 AGENTS/reporting discipline を close した
- current reading は、sample/storage foundation と `TermSignature` / `LayerSignature` / `MessageEnvelope` / `VisualizationProtocol` first cut、phase 9 typed external boundary docs-first sample plan と synthetic preview helper widening と residual reopen matrix fix、phase 12 projection docs-first plan / helper-report preview widening / emitted-program gate closeout、phase 14 hot-plug helper-local lifecycle canary、phase 13 `NET-01..05` helper-local transport canary、phase 16 backend/LLVM guardrail、hands-on closeout landing page、phase 8 avatar representative slice widening、cross-package sweep、`FAIRY-05` residual reacquire design review、`P4` `TermSignature` registry hardening closeout、`P5` `LayerSignature` system hardening closeout、`P6` `MessageEnvelope / AuthEvidence` seam hardening closeout、`P7` `VisualizationProtocol / VisualizationSecurity` hardening closeout、`P8` Sugoroku runtime attach hardening closeout、`P9` avatar fairy follow hardening closeout、`P10` `mirrorea-core` first real implementation tranche closeout、`P11` logical multi-place runtime tranche の current third cut actualization、`P12` external adapter / host boundary tranche の current first cut closeout、`P13` network transport minimal alpha の current first-cut closeout、`P14` hot-plug package-manager tranche の current first-cut closeout、`P15` projection/codegen first emitted place-specific programs の current first-cut closeout を先に入れた上で、current promoted line を `P16` visual debugger / viewer first public prototype へ進めると読む
- `docs/reports/0933-*` で `Avatar fairy follow residual widening` を close し、`FAIRY-02` visibility-loss fallback を active helper canary に昇格させ、residual planned family を `FAIRY-05` だけに縮めた
- `docs/reports/0934-*` で `cross-package sweep` を close し、active evidence / planned family / mixed gate / next queue の current reading を recut した
- `docs/reports/0939-*` で `FAIRY-05 residual reacquire design` を docs-first package として close し、sample は planned のままに保ちつつ、explicit state timeline / anchor switch evidence gate と exact carrier bundling `UNRESOLVED` だけを固定した
- `docs/reports/0956-*` で `P9` avatar fairy follow hardening を close し、avatar helper closeout `planned_sample_paths` と `fairy05_reopen_gate` を current line に actualize した
- `docs/reports/0945-mirrorea-next-stage-plan-integration.md` で next-stage future-plan integration / next package queue stabilization を close し、handoff 由来の project axis、source hierarchy、queue numbering、snapshot docs を current repo へ再同期した
- phase 8 active representative slice は `samples/clean-near-end/avatar-follow/` と `scripts/avatar_follow_samples.py` に置き、`FAIRY-05` だけを residual planned family に残す
- closed chain の current reading は、`P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、phase 9 typed external docs-first / synthetic preview widening、`P2` typed external residual reopen matrix closeout、phase 12 projection docs-first / preview widening、phase 13 transport canary、phase 14 hot-plug lifecycle canary、phase 16 backend/LLVM guardrail、phase 8 avatar representative slice / residual design closeout、`P10` `mirrorea-core` first real implementation tranche、`P11` logical multi-place runtime tranche の current third cut、`P12` external adapter / host boundary tranche の current first cut、`P13` network transport minimal alpha の current first-cut closeout、`P14` hot-plug package-manager tranche の current first-cut closeout、`P15` projection/codegen first emitted place-specific programs の current first-cut closeout までである
- current next line は `P16` visual debugger / viewer first public prototype であり、safest first cut は helper/report-local typed visualization inventory を public prototype boundary と誤読させずに widen すること、その reopen point は `P17` storage / LLVM / backend preparation と読む
- reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md`

## current stop line

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public theorem / model-check / witness-provider contract
- final public auth / visualization / projection / hot-plug API
- full dependent type theory
- real network / durable distributed commit / multi-server consensus
- packaging / installed binary / FFI / engine adapter

## current recommendation

- active current layer、sample progress discipline、Mirrorea future-axis carrier queue を 1 本の “実装済み” line に潰さない
- `world` は current Sugoroku sample では host/server-side sugar として読み、Mir core primitive として既成事実化しない
- authentication / authorization / membership / capability / witness / visualization / telemetry を transport や debug leak に潰さない
- effect-based OS-like substrate という内側の解釈を使っても、Mir core standard I/O primitive や subsystem collapse を既成事実化しない
- `VerificationLayer` composition は helper `verification_handoff_witness` と runtime `verification_model_check` を current emitted floor とする typed layer composition の current reading に留め、hidden verifier builtin や final public verifier contract と混同しない
- `auth none` baseline を final public auth protocol と混同せず、helper-local / report-local carrier の stop line を明記する
- root disk 上の `target/` 膨張を放置せず、external workdir が使えるなら heavy disposable artifact をそこへ逃がす
- old sample archive と current active sample の区別を、README / Documentation / progress / tasks / `samples_progress.md` / research_abstract で常に対にして書く
