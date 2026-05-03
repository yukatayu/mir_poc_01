# plan/01 — 現況サマリ

## この文書について

- この文書は repo-wide status-at-a-glance の repository memory であり、current queue authority は `progress.md` / `tasks.md` に置く
- ここに残す closeout chain と guardrail inventory は current snapshot の背景 memory として読む。active package order や fresh validation status の判定は `progress.md` / `tasks.md` / `docs/reports/` で再確認する
- storage / capacity / mount の具体的な数値は point-in-time anchor であり、運用判断の前には `df -h`、`du -sh`、`lsblk -f`、`findmnt` を rerun する

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
- `scripts/visual_debugger_viewer_samples.py` が helper/runtime typed visualization inventory を typed public prototype inventory へ正規化する current first-cut surface を持つ
- `scripts/alpha_visualization_samples.py` が alpha-local `VIS-01/02/03/05/06/07/08/10/11` を dedicated Stage-E subset runner として actualize し、`stage-e-closeout` で current-scope Stage E closeout surface を与えている
- `scripts/check_source_hierarchy.py` と `scripts/validate_docs.py` が repository memory / report / dashboard 側の baseline check になる

## storage guardrail memory

- fresh quantitative readings は `docs/reports/0972-*` と `docs/reports/1001-*` の storage audit evidence を入口にし、運用判断の前に再 probe する
- repo path `target/` は `/mnt/mirrorea-work/cargo-target` への symlink current line を維持する
- cargo registry cache は `/mnt/mirrorea-work/cargo-registry-cache` を current external cache path として扱う
- heavy disposable artifact 用の external workdir は `/mnt/mirrorea-work` mount を前提にしつつ、actual mount / capacity は毎回 probe で確認する
- `scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh` が current guardrail surface である
- `/mnt/mirrorea-work/llvm/{src,build,install}` は reserve path family として維持し、`/mnt/mirrorea-work/llvm` parent が `root:root` non-writable の間は helper が ownership repair を行わず、`llvm/src` cleanup exclusion と `llvm/build` / `llvm/install` cleanup guard だけを current line に固定する

## twin peaks の snapshot state

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
- `docs/reports/0971-*` で `P16` visual debugger / viewer first public prototype の current first-cut closeout を close し、`scripts/visual_debugger_viewer_samples.py`、`plan/26-visual-debugger-viewer-roadmap.md`、`docs/hands_on/visual_debugger_viewer_01.md`、`docs/research_abstract/visual_debugger_viewer_plan_01.md`、`P16-VIEW-01..05`、`viewer_panel_lanes` / `viewer_telemetry_lanes`、`actualized_panel_kinds`、`kept_later_gates` を current line に actualize した
- `docs/reports/0972-*` で `P17` storage / LLVM / backend preparation の current first-cut closeout を close し、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh --list`、`docs/hands_on/compiler_backend_llvm_preparation_01.md`、`plan/23` を通じて external workdir / cleanup / `llvm` owner-writable probe / non-writable cleanup guard を current line に actualize した
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
- `docs/reports/0975-*` で `R1` `VerificationLayer` widening threshold inventory first cut を close し、helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor、theorem / runtime-policy / visualization downstream stop line、`plan/29` / reader-facing docs を current repo に actualize した
- `docs/reports/0976-*` で `R1` closeout trail と `R2` promoted-next sync を close し、research landing stale drift、`progress.md` recent log、`samples_progress.md` `PH0` row、report trail completion を current repo に actualize した
- `docs/reports/0977-*` で `R2` `AttachPoint` compatibility / detach minimal contract を close し、helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row、grounding envelope / view / telemetry anchor、kept-later migration / rollback gate を `plan/30` / reader-facing docs / snapshot docs に actualize した
- `docs/reports/0978-*` で `R3` `FAIRY-05` visibility-return carrier bundling を close し、helper closeout implementation inventory `carrier_choice = UNRESOLVED` を保ったまま、carrier-choice matrix と provisional recommendation `typed bundle over state_timeline + anchor_switch` を `plan/31` / reader-facing docs / snapshot docs に actualize した
- `docs/reports/0979-*` で `R4` hot-plug real migration / rollback boundary を close し、helper-local evidence floor を widening せずに `activation_cut != distributed activation ordering`、`migration_contract row != protocol`、runtime-crate engine / rollback / durable migration / final public ABI kept-later boundary を `plan/32` / reader-facing docs / snapshot docs に actualize した
- `docs/reports/0982-*` で `R5` runtime-crate hot-plug engine ownership cut を close し、helper-local `hotplug_lifecycle` / sample anchor IDs、`mirrorea-core` generic carrier-substrate、`mir-runtime` thin runtime-report assembly の owner split と Python/Rust duplication != ownership migration complete を `plan/33` / reader-facing docs / snapshot docs に actualize した
- `docs/reports/0986-*` で `R6` runtime-crate hot-plug carrier admission cut を close し、post-`R5` の first admissible Rust-side hot-plug-specific family を engine-neutral request / verdict carrier に限定し、`P19` `mirrorea-core` hot-plug request/verdict carrier tranche と `P20` `mir-runtime` hot-plug orchestration skeleton first tranche の queue split を `plan/34` / reader-facing docs / snapshot docs に actualize した
- `docs/reports/0989-*` で `P20` `mir-runtime` hot-plug orchestration skeleton first tranche を close し、`crates/mir-runtime/src/hotplug_runtime.rs` に dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize した
- `docs/reports/0992-*` で `R7` post-`P20` hot-plug next-package inventory を close し、`plan/35` と companion docs により `P21` runtime-crate hot-plug engine-state narrow floor を historical narrow implementation cut として固定した
- `docs/reports/0993-*` で `P21` runtime-crate hot-plug engine-state narrow floor を close し、`HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、consumer-side `assemble_hotplug_runtime_engine_report()`、example `build_hotplug_runtime_engine_report()`、grouped later-family reading を snapshot / roadmap / reader-facing docs に actualize した
- `docs/reports/0994-*` で post-`P21` rollback / durable migration family historical first boundary family を close し、`plan/36`、closed trilogy ordering、keep-one-family vs split-again criteria を snapshot / roadmap / reader-facing docs に actualize した
- `docs/reports/0995-*` で post-`P21` distributed activation ordering family historical second boundary family を close し、`plan/37`、`activation_cut` widening criteria、durable activation commit / public ABI / transport-collapse stop line、then-remaining final-public-ABI family handoff を snapshot / roadmap / reader-facing docs に actualize した
- `docs/reports/0997-*` で post-`P21` final public hot-plug ABI family last historical boundary family を close し、`plan/38`、`freeze prerequisite fixed; public ABI still unfrozen`、helper-local preview naming / runtime-private naming / public candidate naming の non-equivalence、product-shaping gate = actual `U1` commitment を snapshot / roadmap / reader-facing docs に actualize した
- `docs/reports/0988-*` で `P19` `mirrorea-core` hot-plug request/verdict carrier tranche を close し、`crates/mirrorea-core/src/fabric.rs` に engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を actualize した
- `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md` で current repo taxonomy と staged migration plan を docs-first に固定した
- `P11` logical multi-place runtime tranche の current third cut は actualize 済みであり、`MembershipRegistry` / `PlaceCatalog` substrate の上に participant-place-kind-gated principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper を置いた
- `P12` external adapter / host boundary tranche の current first cut は close 済みであり、typed external helper subset / closeout に `host_boundary_scope` / `host_boundary_lanes` / `non_collapse_lanes` / `host_family_gates` / `host_boundary_inventory` を actualize 済みである
- `docs/reports/0973-*` で `P18` public API / parser grammar gate の repo-side first-cut closeout を close し、freeze checklist / public-boundary inventory / mixed-gate と true user-spec hold line の分離を current line に actualize した
- historical `P20` closeout memory として、admitted request/verdict carrier と existing substrate の thin runtime/report assembly までは current line に actualize 済みである。`P20` では dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize した。completed engine、rollback、durable migration、distributed activation ordering、final public hot-plug ABI は still later に残す
- `R7` は close 済みであり、post-`P20` kept-later lane を smallest plausible package cuts に分け、`P21` runtime-crate hot-plug engine-state narrow floor を historical narrow implementation cut として固定した。historical grouped-later memory は維持しつつ、repository snapshot では `rollback / durable migration` family、`distributed activation ordering` family、`final public hot-plug ABI` family を closed docs-first trilogy として扱う
- later reopen point retained in this memory は installed binary / packaging adoption target、FFI / engine adapter / host integration target、first shipped public surface scope、final shared-space operational catalog breadth の actual commitment であり、repo-side inventory だけでは閉じない。last historical boundary docs-first close の後も still later であり、maintenance-only line かどうかの queue authority は `progress.md` / `tasks.md` に残す

### Mirrorea future-axis carrier memory

- project axis:
  **正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム**
- `docs/reports/0912-*` で package 1 current-state audit と package 2 AGENTS/reporting discipline を close した
- repository-memory reading は、sample/storage foundation と `TermSignature` / `LayerSignature` / `MessageEnvelope` / `VisualizationProtocol` first cut、phase 9 typed external boundary docs-first sample plan と synthetic preview helper widening と residual reopen matrix fix、phase 12 projection docs-first plan / helper-report preview widening / emitted-program gate closeout、phase 14 hot-plug helper-local lifecycle canary、phase 13 reported `NET-01` Sugoroku parity anchor + `NET-02..05` runnable transport canary、phase 16 backend/LLVM guardrail、hands-on closeout landing page、phase 8 avatar representative slice widening、cross-package sweep、`FAIRY-05` residual reacquire design review、`P4` `TermSignature` registry hardening closeout、`P5` `LayerSignature` system hardening closeout、`P6` `MessageEnvelope / AuthEvidence` seam hardening closeout、`P7` `VisualizationProtocol / VisualizationSecurity` hardening closeout、`P8` Sugoroku runtime attach hardening closeout、`P9` avatar fairy follow hardening closeout、`P10` `mirrorea-core` first real implementation tranche closeout、`P11` logical multi-place runtime tranche の third cut actualization、`P12` external adapter / host boundary tranche の first cut closeout、`P13` network transport minimal alpha の first-cut closeout、`P14` hot-plug package-manager tranche の first-cut closeout、`P15` projection/codegen first emitted place-specific programs の first-cut closeout、`P16` visual debugger / viewer first public prototype の first-cut closeout、`P17` storage / LLVM / backend preparation の first-cut closeout、`P18` public API / parser grammar gate の repo-side first-cut closeout を先に入れた上で、public-freeze path の repo-side framingは一度閉じ、post-`P18` user-spec hold line として読む
- `docs/reports/1130-*` で `P-A0-23` Stage B alpha-0.5 closeout を追加し、`scripts/alpha_local_runtime_samples.py` によって `LR-01/02` dedicated runner と `CUT-04/17` supporting subset を組み合わせた current-scope Stage B closeout surface を actualize した。これは local runtime + local-only save/load subset の closeout であり、distributed save/load や CUT family completion ではない
- `docs/reports/1131-*` では `P-A0-24` Stage C transport closeout を追加し、`scripts/alpha_network_docker_e2e.py stage-c-closeout` によって `NET-02/03/04/05/07/09` existing Docker/local-subprocess floor を current-scope Stage C closeout surface として束ねる。これは alpha-0.7 transport closeout であり、`NET-06/08/10`、production WAN/session/replay、network partition completion、final public transport ABI ではない
- `docs/reports/1132-*` では `P-A0-25` Stage D lifecycle closeout を追加し、`scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout` によって `LI-01/02/03/04/05` と `AV-01/02/06/08/09` / `HP-11/12/15` existing floors を current-scope Stage D closeout surface として束ねる。これは alpha-0.8 hot-plug lifecycle closeout であり、detach runtime、durable migration、distributed activation ordering、native execution realization、final public layer/package/avatar ABI ではない
- `docs/reports/1133-*` では `P-A0-26` Stage E devtools closeout を追加し、`scripts/alpha_visualization_samples.py stage-e-closeout` によって `VIS-01/02/03/05/06/07/08/10/11` existing subset を current-scope Stage E closeout surface として束ねる。これは alpha-0.9 devtools closeout であり、`VIS-04/09/12`、final public viewer/telemetry API ではない
- `docs/reports/1134-*` では `P-A0-27` Stage F alpha closeout を追加し、`scripts/alpha_e2e_samples.py stage-f-closeout` によって `E2E-01/02/03/04/05/06/07/09/10` existing bridge rows と current-scope Stage E closeout surface を current-scope Stage F closeout として束ねる。これは alpha-1 current-scope closeout であり、`E2E-08`、public alpha / `U1`、distributed save/load completion、active runnable-root promotion ではない
- `docs/reports/1135-*` では `P-A0-28` Stage A imported-baseline reconciliation を追加し、current-L2 / clean-near-end / Lean / Sugoroku / avatar / typed external / network canary / projection / viewer / hot-plug narrow floor の imported validation line を rerun した上で、`specs/17`、`plan/43`、snapshot docs の large-stage reading を `Stage A..F` sequential closeout として同期した。これは imported baseline reconciliation であり、新しい runtime semantics、`samples/alpha/` runnable-root promotion、final public product claim ではない
- `docs/reports/0933-*` で `Avatar fairy follow residual widening` を close し、`FAIRY-02` visibility-loss fallback を active helper canary に昇格させ、residual planned family を `FAIRY-05` だけに縮めた
- `docs/reports/0934-*` で `cross-package sweep` を close し、active evidence / planned family / mixed gate / historical next-step reading を recut した
- `docs/reports/0939-*` で `FAIRY-05 residual reacquire design` を docs-first package として close し、sample は planned のままに保ちつつ、explicit state timeline / anchor switch evidence gate と exact carrier bundling `UNRESOLVED` だけを固定した
- `docs/reports/0956-*` で `P9` avatar fairy follow hardening を close し、avatar helper closeout `planned_sample_paths` と `fairy05_reopen_gate` を current line に actualize した
- `docs/reports/0978-*` で `R3` `FAIRY-05` visibility-return carrier bundling を close し、helper closeout implementation inventory は変えずに provisional recommendation `typed bundle over state_timeline + anchor_switch` を current memory に actualize した
- `docs/reports/0979-*` で `R4` hot-plug real migration / rollback boundary を close し、`activation_cut`、`migration_contract`、rejected `detached_roll_request#1` の non-equivalence と kept-later boundary matrix を current memory に actualize した
- `docs/reports/0945-mirrorea-next-stage-plan-integration.md` で next-stage future-plan integration / package-sequencing wording stabilization を close し、handoff 由来の project axis、source hierarchy、queue numbering、snapshot docs を repo へ再同期した
- phase 8 active representative slice は `samples/clean-near-end/avatar-follow/` と `scripts/avatar_follow_samples.py` に置き、`FAIRY-05` だけを residual planned family に残す
- closed chain の repository-memory reading は、`P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、phase 9 typed external docs-first / synthetic preview widening、`P2` typed external residual reopen matrix closeout、phase 12 projection docs-first / preview widening、phase 13 transport canary、phase 14 hot-plug lifecycle canary、phase 16 backend/LLVM guardrail、phase 8 avatar representative slice / residual design closeout、`P10` `mirrorea-core` first real implementation tranche、`P11` logical multi-place runtime tranche の third cut、`P12` external adapter / host boundary tranche の first cut、`P13` network transport minimal alpha の first-cut closeout、`P14` hot-plug package-manager tranche の first-cut closeout、`P15` projection/codegen first emitted place-specific programs の first-cut closeout、`P16` visual debugger / viewer first public prototype の first-cut closeout、`P17` storage / LLVM / backend preparation の first-cut closeout、`P18` public API / parser grammar gate の repo-side first-cut closeout までである
- `R2` closeout memory は `plan/30-attachpoint-detach-minimal-contract.md`、`docs/research_abstract/attachpoint_detach_minimal_contract_01.md`、`docs/hands_on/attachpoint_detach_minimal_contract_01.md` を入口にし、helper-local `hotplug_lifecycle` / `hotplug_lifecycle_lanes` / explicit detach TODO boundary の current minimal contract row と kept-later migration / rollback gate を current memory として読む
- `R3` closeout memory は `plan/31-fairy05-visibility-return-carrier-bundling.md`、`docs/research_abstract/fairy05_visibility_return_carrier_bundling_01.md`、`docs/hands_on/fairy05_visibility_return_carrier_bundling_01.md` を入口にし、helper closeout implementation inventory は `UNRESOLVED` のまま保ちつつ、provisional recommendation `typed bundle over state_timeline + anchor_switch` を current memory として読む
- `R4` closeout memory は `plan/32-hotplug-real-migration-rollback-boundary.md`、`docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`、`docs/hands_on/hotplug_real_migration_rollback_boundary_01.md` を入口にし、current helper-local evidence がまだ証明していない kept-later hot-plug boundary を current memory として読む
- `R5` closeout memory は `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`、`docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`、`docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md` を入口にし、helper-local preview / crate-side carrier-substrate / runtime thin assembly の owner split を current memory として読む
- `P21` runtime-crate hot-plug engine-state narrow floor は close 済みである。later family の exact label は intentionally unfixed のまま保ち、historical first boundary family `rollback / durable migration` と historical second boundary family `distributed activation ordering` は docs-first close 済みとして保つ
- `final public hot-plug ABI` family hardening も docs-first close 済みである。ここで fixed したのは `freeze prerequisite fixed; public ABI still unfrozen` までであり、actual final public ABI freeze は post-`P18` mixed gate / `U1` hold line の dependency を保ったまま still later に残す
- reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md`

## 2026-05-02 alpha-0 integration addendum

- current repo は maintenance-only line に固定せず、Mirrorea Spaces alpha-0 の **theory-freeze and roadmap synchronization lane** を reopen してよい
- normative anchor は `specs/13..17`
- repository memory anchor は `plan/39..43`
- sample scaffold root は `samples/alpha/`
- current package reading は `P-A0-01..28` が current repo state で close しうる line に入り、`P-A0-07` による first non-public Rust local-runtime floor、`P-A0-08` による `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs` と `samples/alpha/layer-insertion/LI-01..05` からなる first non-public Rust layer-insertion floor、`P-A0-09` による `crates/mir-runtime/src/alpha_network_runtime.rs`、example `mirrorea_alpha_network_runtime`、thin runner `scripts/alpha_network_docker_e2e.py`、and `samples/alpha/network-docker/NET-02/03/04/05/07/09` からなる first non-public Rust Stage-C network / Docker floor、`P-A0-10` による `crates/mir-runtime/src/alpha_avatar_runtime.rs`、example `mirrorea_alpha_avatar_runtime`、thin runner `scripts/alpha_avatar_runtime_samples.py`、and `samples/alpha/avatar-runtime/AV-01/02/06/08/09` + `samples/alpha/hotplug-runtime/HP-11/12/15` からなる first non-public runtime-private package/avatar manifest-admission floor、`P-A0-25` による `scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout` over `LI-01/02/03/04/05` + `AV-01/02/06/08/09` + `HP-11/12/15`、`P-A0-11` / `P-A0-27` による thin integrated bridge runner `scripts/alpha_e2e_samples.py` の `E2E-01/02/03/04/05/06/07/09/10` + `stage-f-closeout` current-scope completion surface、`P-A0-12` による `scripts/alpha_cut_save_load_samples.py` + `CUT-04` local save/load positive bridge + `E2E-06` actualization、`P-A0-13` / `P-A0-15` / `P-A0-26` による `scripts/alpha_visualization_samples.py` の `VIS-01/02/03/05/06/07/08/10/11` actualization + `stage-e-closeout` current-scope completion surface、`P-A0-14` による `CUT-17` local stale-membership rejection bridge + `CUT-11` checker-backed Z-cycle inadmissibility row actualization、`P-A0-16` による `LIF-01` raw dangling reference reject + `VAR-05` synthetic mutable/read-write invariance negative row の checker-floor actualization、`P-A0-17` による `LIF-02/03/04` + `VAR-01/04/06` helper-local synthetic acceptance-floor actualization、`P-A0-18` による `VAR-08/11/13` runtime-mirror actualization、`P-A0-19` による `LIF-11/13/15` + `VAR-14` remaining positive-row carrier inventory closeout、`P-A0-20` による `LIF-13` helper-local synthetic snapshot-selected actualization、`P-A0-21` による `LIF-11` helper-local synthetic anchor-handoff actualization、`P-A0-22` による `LIF-15` planned-only `alpha-remote-observe-floor` / `VAR-14` planned-only `alpha-adapter-transform-floor` blocker split closeout、and `P-A0-28` による imported Stage A baseline reconciliation を含む状態を前提にし、current line が acceptance/snapshot/runtime-mirror/runtime bridge を widen せず、queue authority は `progress.md` / `tasks.md` に残す
- Stage A current floor と Stage B local runtime target を混同しない
  - Stage A evidence:
    clean near-end suite、current-L2 corpus、Lean evidence、Sugoroku helper、avatar helper、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory、hot-plug carrier/runtime narrow floor
    `P-A0-28` rerun により、この imported baseline は current alpha line の Stage A 100% evidence として扱う
  - Stage B target:
    integrated local runtime、event DAG export、Place graph export、fallback degradation trace、hot-plug request/verdict trace
    current-scope closeout は local queue / `MessageEnvelope` dispatch / membership freshness / event DAG export hook に `CUT-04/17` local-only save/load subset を加えたものに限る
- `samples/alpha/` は expected-verdict sidecar 付き scaffold であり、active runnable root へ silent promotion しない
- `samples/alpha/e2e/` の current reading は runner-backed non-public bridge + planned-only upper-layer row の mixed familyであり、current-scope Stage F closeout は fixed 済みだが、public alpha / `U1` completion claim ではない
- `samples/not_implemented/` は residual planned family preservation root として残す
- public-boundary `U1` gate は消えていない
  - packaging / host target / first shipped public surface / final catalog breadth の user-facing decision は still later

## current stop line

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public theorem / model-check / witness-provider contract
- final public auth / visualization / projection / hot-plug API
- full dependent type theory
- real network / durable distributed commit / multi-server consensus
- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope

## standing guardrails in this memory

- active current layer、sample progress discipline、Mirrorea future-axis carrier memory を 1 本の “実装済み” line に潰さない
- `world` は current Sugoroku sample では host/server-side sugar として読み、Mir core primitive として既成事実化しない
- authentication / authorization / membership / capability / witness / visualization / telemetry を transport や debug leak に潰さない
- effect-based OS-like substrate という内側の解釈を使っても、Mir core standard I/O primitive や subsystem collapse を既成事実化しない
- `VerificationLayer` composition は helper `verification_handoff_witness` と runtime `verification_model_check` を emitted floor とする typed layer composition の repository-memory reading に留め、hidden verifier builtin や final public verifier contract と混同しない
- post-`P18` line では `library-first` ordering、`native-process-first` host target、minimal shared-space subset keep、core surface first / integration surface second の two-step split を provisional recommendation として並べてよいが、actual product target と final public ABI は user choice なしに fixed しない
- `auth none` baseline を final public auth protocol と混同せず、helper-local / report-local carrier の stop line を明記する
- root disk 上の `target/` 膨張を放置せず、external workdir が使えるなら heavy disposable artifact をそこへ逃がす
- old sample archive と current active sample の区別を、README / Documentation / progress / tasks / `samples_progress.md` / research_abstract で常に対にして書く
