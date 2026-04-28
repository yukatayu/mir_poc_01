# tasks

最終更新: 2026-04-29 03:08 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較は `plan/`、runnable sample 状態は `samples_progress.md`、詳細証跡は `docs/reports/` に置きます。
- append-only 履歴ではなく、現況に合わせて毎回全体を書き直す snapshot として扱います。

## current status at task level

- active clean near-end suite、Sugoroku world vertical slice、avatar follow representative slice は runnable です。
- `P0` current-state audit と `P1` repository layer map / `samples_progress.md` stabilization は close 済みです。
- `TermSignature`、`LayerSignature`、`MessageEnvelope / AuthEvidence` seam、`VisualizationProtocol`、typed external synthetic preview helper、projection preview、hot-plug helper-local lifecycle canary、network transport helper-local canary、storage / LLVM guardrail は first cut / widening close 済みです。
- `P2` Typed external boundary residual planned family review は close 済みです。
- `P10` `mirrorea-core` first real implementation tranche は close 済みです。
- `P11` logical multi-place runtime tranche の current third cut は close 済みで、`MembershipRegistry` / `PlaceCatalog` substrate と participant-place-kind-gated shell の上に principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper まで actualize 済みです。
- `P12` external adapter / host boundary tranche の current first cut は close 済みで、typed external helper subset / closeout に `host_boundary_scope`、`host_boundary_lanes`、`non_collapse_lanes`、`host_family_gates`、`host_boundary_inventory` を actualize 済みです。
- `P15` projection/codegen first emitted place-specific programs の current first cut は close 済みで、`scripts/projection_codegen_samples.py` と `samples/generated/projection-placement/manifest.json` によって committed generated bridge evidence / live-anchor alignment / `kept_later_gates` を current line に actualize 済みです。
- `P16` visual debugger / viewer first public prototype の current first cut は close 済みで、`scripts/visual_debugger_viewer_samples.py`、`plan/26-visual-debugger-viewer-roadmap.md`、`P16-VIEW-01..05`、`viewer_panel_lanes` / `viewer_telemetry_lanes`、`kept_later_gates` によって typed public prototype inventory over helper/runtime surfaces を current line に actualize 済みです。
- `P17` storage / LLVM / backend preparation の current first cut も close 済みで、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh --list`、`docs/hands_on/compiler_backend_llvm_preparation_01.md`、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` によって external workdir / cleanup / LLVM staging ownership mismatch を non-destructive probe floor として current line に actualize 済みです。
- `R6` runtime-crate hot-plug carrier admission cut は close 済みです。
- `P19` `mirrorea-core` hot-plug request/verdict carrier tranche も close 済みであり、engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を `crates/mirrorea-core/src/fabric.rs` に actualize 済みです。
- `P20` `mir-runtime` hot-plug orchestration skeleton first tranche も close 済みであり、`crates/mir-runtime/src/hotplug_runtime.rs` に dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize 済みです。
- `R7` post-`P20` hot-plug next-package inventory も close 済みであり、`plan/35-post-p20-hotplug-next-package-inventory.md` と companion docs により post-`P20` kept-later lane を smallest plausible package cuts に分け、`P21` runtime-crate hot-plug completed-engine narrow cut を current narrow 実装対象として固定済みです。
- `P21` runtime-crate hot-plug completed-engine narrow cut も close 済みであり、`crates/mir-runtime/src/hotplug_runtime.rs` に `HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、consumer-side `assemble_hotplug_runtime_engine_report()`、example `build_hotplug_runtime_engine_report()` を actualize 済みです。
- current promoted next exact label は **intentionally unfixed** です。`rollback / durable migration`、`distributed activation ordering`、`final public ABI` は later family の grouped reading に戻し、premature に collapse しません。
- next reopen point は **installed binary / packaging adoption target、FFI / engine adapter / host integration target、first shipped public surface scope、final shared-space operational catalog breadth の actual commitment** です。
- current snapshot を短く追う入口は `progress.md`、`samples_progress.md`、`docs/hands_on/current_phase_closeout_01.md` です。

## current executable floor

| lane | current floor | note |
|---|---|---|
| Mir current-L2 | `samples/current-l2/` | base corpus と current-L2 source execution を維持 |
| clean near-end suite | `samples/clean-near-end/{typing,order-handoff,model-check,modal}/` | finite-index typing、order / handoff、model-check second line、modal family が runnable |
| Sugoroku world | `samples/clean-near-end/sugoroku-world/` + `scripts/sugoroku_world_samples.py` | attach / membership / handoff / reset model-check / hot-plug helper-local lifecycle canary |
| avatar follow | `samples/clean-near-end/avatar-follow/` + `scripts/avatar_follow_samples.py` | `FAIRY-01/02/03/04/06` active、`FAIRY-05` は planned |
| typed external / transport | `scripts/typed_external_boundary_samples.py`、`scripts/network_transport_samples.py` | helper-local synthetic preview / canary。typed external residual reopen matrix は fixed 済みだが、final public adapter / transport contract ではない |
| projection / placement | Sugoroku `projection_view`、runtime `cross_place_projection`、`scripts/projection_codegen_samples.py`、`samples/generated/projection-placement/manifest.json` | helper-local / report-local preview floor + committed generated bridge evidence。final emitted executable program ではない |
| storage / backend guardrail | `/mnt/mirrorea-work`、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh` | root disk を既成事実化しない external workdir floor と `llvm` staging ownership mismatch visibility |

## ordered self-driven packages

| Package | Macro phase | Stage | Status | Rough estimate | Current reading |
|---|---|---|---|---|---|
| `P0` current-state audit | `Macro 0` | `S6 -> S6` | closed | closed | source hierarchy / stale reference audit |
| `P1` repo layer map / samples dashboard stabilization | `Macro 0` | `S6 -> S6` | closed | closed | taxonomy / dashboard synchronization |
| `P2` typed external residual review | `Macro 6` reserve | `S5 -> S6` | closed | closed | residual planned family review |
| `P3` projection residual emitted-program gate | `Macro 6` reserve | `S5 -> S6` | closed | closed | emitted-program boundary fixation |
| `P4` TermSignature hardening | `Macro 6` | `S4 -> S5` | closed | ~1 task | current first-cut lanes / scope / reserved split fixed |
| `P5` LayerSignature hardening | `Macro 6` | `S4 -> S5` | closed | closed | row schema / obligations lane / scope split fixed |
| `P6` MessageEnvelope/AuthEvidence seam hardening | `Macro 6` | `S4 -> S5` | closed | closed | medium/seam split、shared auth lane inventory、freshness lane fixed |
| `P7` VisualizationProtocol/Security | `Macro 6-7` | `S4 -> S5` | closed | closed | security envelope / typed telemetry / fail-closed route trace fixed |
| `P8` Sugoroku runtime attach hardening | `Macro 6` | `S5 -> S6` | closed | closed | MembershipRegistry source-of-truth / world sugar / hot-plug stop line fixed |
| `P9` avatar fairy follow hardening | `Macro 6` | `S5 -> S6` | closed | closed | helper closeout `FAIRY-05` reopen gate / planned path inventory fixed |
| `P10` mirrorea-core first real implementation tranche | `Macro 6-7` | `S1 -> S4` | closed | closed | placeholder -> first real minimal carrier core |
| `P11` logical multi-place runtime tranche | `Macro 6-7` | `S1 -> S4` | closed (third cut current scope) | closed | third cut actualized; non-blocking follow-up only |
| `P12` external adapter / host boundary tranche | `Macro 7` | `S1 -> S4` | closed (first cut) | closed | helper-local host-boundary inventory first cut |
| `P13` network transport minimal alpha | `Macro 6-7` | `S1 -> S4` | closed (first cut) | closed | helper-local process-boundary closeout inventory fixed |
| `P14` hot-plug package-manager tranche | `Macro 6-7` | `S1 -> S4` | closed (first cut) | closed | helper-local package-manager inventory fixed |
| `P15` projection/codegen emitted programs | `Macro 7` | `S1 -> S4` | closed (first cut) | closed | preview floor -> committed generated bridge evidence + alignment surface |
| `P16` visual debugger / viewer prototype | `Macro 7` | `S1 -> S4` | closed (first cut) | closed | typed public prototype inventory over helper/runtime surfaces |
| `P17` storage / LLVM / backend preparation | `Macro 7` | `S3 -> S5` | closed (first cut) | closed | guardrail -> implementation-ready staging closeout |
| `P18` public API / parser grammar gate | `Macro 7` mixed gate | `S0 -> S3` | closed (repo-side first cut) | closed for repo-side scope | freeze checklist / public-boundary inventory / hold-line split fixed; final freeze remains last |
| `U1` post-`P18` true user-spec hold option matrix | `Macro 8` prep | `S0 -> S2` | closed | closed | packaging / host target / shipped-surface / final-catalog options を decision-ready に整理した |
| `R1` `VerificationLayer` widening threshold inventory | `Macro 8` prep | `S0 -> S2` | closed | closed | helper/runtime verification lanes の widening threshold matrix と stop lineを docs-first に固定した |
| `R2` `AttachPoint` compatibility / detach minimal contract | `Macro 8` prep | `S0 -> S2` | closed | closed | helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row と kept-later migration / rollback gate を docs-first に固定した |
| `R3` `FAIRY-05` visibility-return carrier bundling | `Macro 8` prep | `S0 -> S2` | closed | closed | helper closeout implementation inventory を変えずに carrier-choice matrix と provisional recommendation を docs-first に固定した |
| `R4` hot-plug real migration / rollback boundary | `Macro 8` prep | `S0 -> S2` | closed | closed | helper-local evidence floor を widening せずに kept-later hot-plug boundary matrix を docs-first に固定した |
| `R5` runtime-crate hot-plug engine ownership cut | `Macro 8` prep | `S0 -> S2` | closed | closed | helper-local preview / crate-side carrier / runtime orchestration の owner split を docs-first に固定した |
| `R6` runtime-crate hot-plug carrier admission cut | `Macro 8` prep | `S0 -> S2` | closed | closed | post-`R5` の first admissible Rust-side hot-plug-specific family を engine-neutral request / verdict carrier に限定し、`P19` / `P20` queue split を docs-first に固定した |
| `P19` `mirrorea-core` hot-plug request/verdict carrier tranche | `Macro 6-7` | `S1 -> S4` | closed | closed | engine-neutral `HotPlugRequest` / `HotPlugVerdict` と lane inventory を `mirrorea-core` に actualize し、helper-local lifecycle と engine actualization を kept-later に残した |
| `P20` `mir-runtime` hot-plug orchestration skeleton first tranche | `Macro 6-7` | `S1 -> S4` | closed | closed | dedicated `HotPlugRuntimeSkeletonReport` と consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` により、`P19` carrier と existing substrate の上に thin runtime/report assembly を narrow に actualize した |
| `R7` post-`P20` hot-plug next-package inventory | `Macro 8` prep | `S0 -> S2` | closed | closed | post-`P20` kept-later lane を smallest plausible package cuts に分け、`P21` runtime-crate hot-plug completed-engine narrow cut を next narrow implementation line として固定した |
| `P21` runtime-crate hot-plug completed-engine narrow cut | `Macro 6-7` | `S1 -> S4` | closed | closed | admitted request/verdict carrier と existing substrate の上に canonical runtime-side engine state progression を narrow に actualize した |

### P0. Current-state audit and source-hierarchy validation

- macro phase / stage:
  `Macro 0`, `S6 -> S6`
- objective:
  handoff と repo 現況の差分を洗い、source hierarchy、stale active reference、snapshot drift を補正する
- deliverables:
  handoff mirror、front-door docs repair、stale/current/planned/old/helper/final-public の区別
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `samples_progress.md` の `PH0` row と report evidence
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `plan/` / `specs/` / docs の同期
- stop line:
  risky な path move や public API freeze を audit task の名目で滑り込ませない

### P1. Repository layer map and `samples_progress.md` stabilization

- macro phase / stage:
  `Macro 0`, `S6 -> S6`
- objective:
  repo taxonomy、sample taxonomy、dashboard semantics、queue reading を current line に揃える
- deliverables:
  active / planned / prototype / old / generated / helper-local preview / final public API / mixed gate の docs-first separation
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `samples_progress.md` summary、`docs/hands_on/current_phase_closeout_01.md`、`docs/research_abstract/mirrorea_future_axis_01.md`
- docs / report requirement:
  新しい report、`plan/19`、`samples_progress.md`、front-door docs の同期
- stop line:
  planned skeleton や helper-local preview を active canonical sample に昇格させない

### P2. Typed external boundary residual planned family review

- status:
  close 済み。`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate を helper closeout と docs / plan / snapshot に固定した。

- macro phase / stage:
  `Macro 6` reserve, `S5 -> S6`
- objective:
  `EXT-01` / `EXT-02` / `EXT-05` residual planned family と `EXT-03` / `EXT-04` synthetic preview subset を projection / visualization / host-schema pressure と照らして整理し、residual reopen matrix を固定する
- deliverables:
  residual planned family の reopen criterion、indirect anchor matrix、host-facing stop line、synthetic preview helper と final public adapter contract の境界
- validation command:
  `python3 scripts/typed_external_boundary_samples.py list`
  `python3 scripts/typed_external_boundary_samples.py check-all`
  `python3 scripts/typed_external_boundary_samples.py closeout`
  `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `python3 -m unittest scripts.tests.test_typed_external_boundary_samples`
- debug / visualization output:
  `envelopes`、`visualization`、`failures`、clean near-end `provider_boundary`、helper closeout `residual_review_matrix`
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/25`、relevant docs を同期する
- stop line:
  exact host schema、final public adapter API、console / overlay / viewer contract を premature に固定しない

### P3. Projection / placement residual emitted-program gate

- status:
  close 済み。helper/report-local preview floor と later emitted place-specific program family の boundary、projection validity report minimum、generated artifact reserve policy、`P15` handoff line を docs-first に固定した。

- macro phase / stage:
  `Macro 6` reserve, `S5 -> S6`
- objective:
  helper/report-local preview floor と emitted place program / optimizer / equivalence gate を docs-first に切り分ける
- deliverables:
  emitted-program stop line、projection validity report minimum、preview floor と generated place-specific program family の境界、actual emitted executable family は `P3` の外へ handoff する historical line
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  `cargo test -p mir-runtime --test clean_near_end_samples`
  `find samples/generated -maxdepth 3 -type f | sort`
  `python3 scripts/validate_docs.py`
- debug / visualization output:
  `projection_view`、`cross_place_projection`、generated artifact reserve inventory
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/20`、`samples/generated/README.md`、relevant docs を同期する
- stop line:
  final projection IR、actual emitted place-specific program family、optimizer、cross-place equivalence checker、deployment planner を固定しない

### P4. `TermSignature` registry hardening

- macro phase / stage:
  `Macro 6`, `S4 -> S5`
- objective:
  signature kind / granularity / reserved-kind wording を tighten し、helper/runtime/report naming drift を減らす
- deliverables:
  current `signature_lanes = kind/name/evidence_role`、helper `signature_scope = representative_slice`、clean near-end `signature_scope = clean_near_end_canonical_inventory`、current `signature_evidence_roles` inventory、reserved kind policy、report-local mirror rule
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  `cargo test -p mir-runtime --test clean_near_end_samples`
- debug / visualization output:
  `term_signatures`、`signature_lanes`、`signature_scope`、`signature_kinds`、`signature_evidence_roles`、`reserved_signature_kinds`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/09`、`plan/14`、`specs/10`、relevant docs を同期する
- stop line:
  final public signature schema や final public message / adapter contract を既成事実化しない

### P5. `LayerSignature` system hardening

- status:
  close 済み。helper/runtime `LayerSignature` row key を `name` に揃え、`obligations` lane、helper/runtime `layer_signature_scope` distinction、helper representative inventory と runtime canonical inventory の split を front-door docs / plan / specs / closeout に固定した。

- macro phase / stage:
  `Macro 6`, `S4 -> S5`
- objective:
  `name / requires / provides / transforms / checks / emits / obligations / laws` carrier の naming と law wording を tighten する
- deliverables:
  `LayerSignature` wording、helper representative inventory / runtime canonical inventory distinction、`VerificationLayer` composition current reading、report-local mirror rule
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  `cargo test -p mir-runtime --test clean_near_end_samples`
- debug / visualization output:
  `layer_signatures`、`layer_signature_scope`、`layer_signature_lanes`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/09`、`plan/14`、`specs/10`、relevant docs を同期する
- stop line:
  final public layer law schema や hidden verifier builtin を既成事実化しない

### P6. `MessageEnvelope` / `AuthEvidence` seam hardening

- status:
  close 済み。helper/runtime `message_envelope_scope`、`transport_medium` / `transport_seam`、`emitter_principal`、`freshness_checks`、shared `auth_evidence_lanes`、helper medium inventory / runtime seam inventory distinctionを fixed した。`session_token` / `signature`、final public transport ABI、`witness_refs` role taxonomy は deferred のまま残す。

- macro phase / stage:
  `Macro 6`, `S4 -> S5`
- objective:
  message/auth seam を tighten し、transport / auth / membership / capability / witness split を明確に保つ
- deliverables:
  current `AuthEvidence` baseline wording、transport medium / seam wording、subject/emitter distinction、public stop line
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/network_transport_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  `python3 -m unittest scripts.tests.test_network_transport_samples -v`
  `cargo test -p mir-runtime --test clean_near_end_samples`
  `cargo test -p mir-runtime`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `message_envelopes`、`message_envelope_scope`、`auth_evidence_lanes`、`auth_evidence_kinds`、`transport_mediums`、`transport_seams`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/09`、relevant docs を同期する
- stop line:
  session / signature protocol、final public auth schema、final public transport ABI、`witness_refs` role taxonomy を固定しない

### P7. `VisualizationProtocol` + `VisualizationSecurity`

- status:
  close 済み。Sugoroku helper と clean near-end runtime の view / telemetry security envelope に `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を追加し、NET-05 observer route trace を fail-closed に固定した。typed telemetry は security boundary の内側に戻し、helper `helper_local_ephemeral` / runtime `report_local_inventory` retention floor を current line に actualize した。

- macro phase / stage:
  `Macro 6-7`, `S4 -> S5`
- objective:
  visualization / telemetry を typed effect として tighten し、label / authority / redaction / retention を整理する
- deliverables:
  current static/runtime view family、typed telemetry wording、helper/runtime retention floor、observer-safe route-trace security stop line
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/network_transport_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  `python3 -m unittest scripts.tests.test_network_transport_samples -v`
  `cargo test -p mir-runtime --test clean_near_end_samples`
  `cargo test -p mir-runtime`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `visualization_views`、`telemetry_rows`、`retention_scope_names`、route-trace redaction
- docs / report requirement:
  新しい report、`samples_progress.md`、`progress.md`、`plan/14`、relevant docs を同期する
- stop line:
  final public viewer contract、retention policy、multi-tenant telemetry service を固定しない

### P8. Sugoroku runtime attach hardening

- status:
  close 済み。Sugoroku helper closeout に `world_surface = host_server_side_sugar`、`membership_model.source_of_truth = MembershipRegistry`、`membership_model.late_join_handoff_boundary`、`hotplug_stop_line` を追加し、attach / membership / handoff / late join / detach TODO boundary の current helper/test/docs contract を tighten した。

- macro phase / stage:
  `Macro 6`, `S5 -> S6`
- objective:
  attach / membership / handoff / late join / detach TODO boundary を representative runtime slice として harden する
- deliverables:
  membership registry wording、world sugar boundary、late-join/handoff boundary、detach lifecycle current stop line
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `message_envelopes`、`membership`、`hotplug_lifecycle`、attach-detach telemetry / visualization current view
- docs / report requirement:
  新しい report、`samples_progress.md`、`progress.md`、`plan/16`、`plan/21`、relevant docs を同期する
- stop line:
  real network、consensus、durable distributed commit、rollback protocol、durable migration engine、final public runtime / hot-plug ABI を claim しない

### P9. avatar fairy follow hardening

- status:
  close 済み。avatar helper closeout に `planned_sample_paths` と `fairy05_reopen_gate = { sample_status = planned_only, required_evidence = [...], carrier_choice = UNRESOLVED, planning_only_candidate_labels = state_timeline / anchor_switch }` を追加し、active representative slice を保ったまま residual `FAIRY-05` gate を tighten した。

- macro phase / stage:
  `Macro 6`, `S5 -> S6`
- objective:
  active representative slice を保ちながら residual `FAIRY-05` gate を整理する
- deliverables:
  explicit state timeline / anchor switch evidence gate、carrier `UNRESOLVED` の current reading、planned sample path inventory
- validation command:
  `python3 -m unittest scripts.tests.test_avatar_follow_samples -v`
  `python3 scripts/avatar_follow_samples.py check-all --format json`
  `python3 scripts/avatar_follow_samples.py closeout --format json`
  `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
  `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
  `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
  `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `anchors`、`membership`、`verification`
- docs / report requirement:
  新しい report、`samples_progress.md`、`progress.md`、`plan/24`、relevant docs を同期する
- stop line:
  `FAIRY-05` を evidence なしに active 化しない。public avatar / visualization API、real transport / session / auth semantics、`mirrorea-core` / Rust runtime へ飛ばない

### P10. `mirrorea-core` first real implementation tranche

- status:
  close 済み。`crates/mirrorea-core` に `LayerSignature`、`PrincipalClaim`、`AuthEvidence`、`MessageEnvelope`、lane inventory、duplicate-name merge helper、carrier validation を actualize し、`mir-runtime` clean near-end report がそれを利用する current ownership cut を入れた。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  placeholder crate から first real Mirrorea runtime substrate を起こす
- deliverables:
  crate responsibility boundary、minimal core carrier、report-local invariants
- validation command:
  `cargo test -p mirrorea-core`
  `cargo test -p mir-runtime --test clean_near_end_samples`
  `cargo test -p mir-runtime`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples scripts.tests.test_avatar_follow_samples`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  clean near-end closeout `layer_signature_lanes` / `message_envelope_lanes` / `auth_evidence_lanes`
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/09`、`plan/19` を同期する
- stop line:
  visualization / telemetry catalog、`MembershipRegistry`、projection object model、hot-plug runtime、final auth / transport ABI、public API freeze をここで固定しない

### P11. logical multi-place runtime tranche

- status:
  close 済み。current third cut では `crates/mirrorea-core` に `MembershipRegistry` typed source-of-truth substrate、`PlaceCatalog` logical multi-place catalog substrate、participant-place-kind-gated `LogicalPlaceRuntimeShell`、principal-derived `ParticipantPlace[{principal}]` shell-backed `add_initial_participant` / `add_participant` / `leave_participant` parity helper を actualize し、helper-local emulator 全体を丸ごと移す前に crate-side runtime substrate の thin composition frontier を切り出した。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  helper-local logical multi-place emulator を crate-side runtime substrate へ寄せる。ただし current third cut でも domain runtime aggregate には広げず、`MembershipRegistry` / `PlaceCatalog`、participant-place-kind-gated thin composition、principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper に絞り、helper-specific `WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` は kept-later に残す
- deliverables:
  `MembershipRegistry` typed source-of-truth substrate、`PlaceCatalog` logical multi-place catalog、participant-place-kind-gated `LogicalPlaceRuntimeShell`、principal-derived `ParticipantPlace[{principal}]` shell-backed `add_initial_participant` / `add_participant` / `leave_participant` parity helper、typed membership / runtime snapshot、explicit kept-later boundary
- validation command:
  `cargo test -p mirrorea-core`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  helper closeout `place_model` / `membership_model` / `runtime_components`、helper-local `membership_snapshot`、crate-side `MembershipSnapshot` / `PlaceCatalogSnapshot` / `LogicalPlaceRuntimeSnapshot`
- docs / report requirement:
  新しい report、`samples_progress.md`、runtime docs、`plan/16`、`plan/19` を同期する
- stop line:
  `WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState`、turn-order insertion policy、session/auth/signature、queue/bridge/failure/route-trace、hot-plug lifecycle ownership、avatar `state_timeline` / `anchor_switch`、real network / consensus / durable replay を先に claim しない

### P12. external adapter / host boundary tranche

- status:
  close 済み。current first cut では typed external helper subset / closeout に `host_boundary_scope = helper_local_synthetic_preview`、`host_boundary_lanes = request / receipt / failure / visualization`、`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、`host_family_gates = final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`、`host_boundary_inventory` を actualize した。`engine-abi` は placeholder のまま保ち、crate-side adapter ABI はまだ起動していない。

- macro phase / stage:
  `Macro 7`, `S1 -> S4`
- objective:
  typed external boundary を host-facing adapter seam として tighten する
- deliverables:
  adapter responsibility boundary、host boundary wording、carrier split、helper-local `host_boundary` preview inventory、explicit non-collapse lane inventory
- validation command:
  `python3 -m unittest scripts.tests.test_typed_external_boundary_samples`
  `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json`
  `python3 scripts/typed_external_boundary_samples.py run EXT-04 --format json`
  `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- debug / visualization output:
  `summary`、`envelopes`、`visualization`、`failures`、helper closeout `host_boundary_inventory`
- docs / report requirement:
  新しい report、`samples_progress.md`、adapter docs、`plan/25`、snapshot docs を同期する
- stop line:
  final public adapter ABI、exact host schema、browser/network/VR family contract を premature に固定しない。`engine-abi` placeholder を current helper inventory task の名目で実装済み扱いしない

### P13. network transport minimal alpha

- status:
  close 済み。current safest first cut は helper-local `process_boundary` closeout であり、`mirrorea-core` transport runtime や real socket/broker を先に作るのではなく、`loopback_socket` parity、subprocess JSON bridge、stale epoch/incarnation reject、typed transport failure family、observer-safe redacted route trace を current validation floor と stop line に固定した。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  helper-local transport canary family を `process_boundary` closeout として tighten し、real transport line に入る前の最小 honest scope を固定する
- deliverables:
  `transport_scope = helper_local_process_boundary`、`process_boundary_canaries`、`loopback_parity_sources`、`loopback_socket` parity、subprocess JSON bridge、stale epoch/incarnation reject、typed transport failure family、observer-safe redacted route trace、`non_collapse_lanes`、`kept_later_gates`、`validation_floor`
- validation command:
  `python3 scripts/network_transport_samples.py list`
  `python3 scripts/network_transport_samples.py run NET-02 --debug route-trace --format json`
  `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json`
  `python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json`
  `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
  `python3 scripts/network_transport_samples.py check-all --format json`
  `python3 scripts/network_transport_samples.py closeout --format json`
  `python3 -m unittest scripts.tests.test_network_transport_samples`
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json`
  `python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --transport loopback_socket --format json`
- debug / visualization output:
  `route-trace`、`reconnect`、`failures`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/22`、transport docs、snapshot docs を同期する
- stop line:
  real socket/broker、crypto session、durable replay/commit、continuous shared runtime state across process、final public transport ABI、production telemetry/viewer contract を claim しない

### P14. hot-plug `Patch` / `AttachPoint` package-manager tranche

- status:
  close 済み。current safest first cut は helper/test/docs closeout hardening に留め、既存 `hotplug_lifecycle` / attach-detach canary / compatibility stop line を package-manager inventory surface として揃えた。runtime crate 側の migration engine や durable rollback protocol はまだ進めていない。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  compatibility、activation、migration、rollback を package-manager concern として widen する前に、helper-local package-manager inventory surface と stop line を current docs / closeout / tests に揃える
- deliverables:
  `hotplug_scope = helper_local_package_manager_preview`、`hotplug_anchor_samples`、`hotplug_package_concerns`、`hotplug_lifecycle_lanes`、`hotplug_anchor_envelopes`、`hotplug_view_ids`、`hotplug_telemetry_row_ids`、`hotplug_kept_later_gates`、`hotplug_validation_floor`、`hotplug_lifecycle` package-manager inventory view、attachpoint compatibility contract、activation cut wording、explicit TODO detach boundary、migration stop line、rollback wording、historical/active/helper/final-public split
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
- debug / visualization output:
  `hotplug_lifecycle`、attach/detach lifecycle rows、package-manager inventory wording
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/21`、hot-plug docs、snapshot docs を同期する
- stop line:
  runtime crate 側の migration engine、deployment-grade rollback、durable upgrade engine、final public hot-plug ABI を premature に固定しない

### P15. projection/codegen first emitted place-specific programs

- status:
  close 済み。current safest first cut は helper/report/generated-reserve closeout hardening に留め、`projection_view` / `cross_place_projection` / generated reserve policy を committed generated bridge evidence、live-anchor alignment、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor` へ揃えた。manifest bridge evidence only の stop line を保ち、optimizer や deployment planner はまだ進めていない。

- macro phase / stage:
  `Macro 7`, `S1 -> S4`
- objective:
  preview floor から projection/codegen current first cut を actualize し、generated reserve と live anchor の間に honest な bridge evidence surface を置く
- deliverables:
  `scripts/projection_codegen_samples.py`、`samples/generated/projection-placement/manifest.json`、`P15-GEN-01..04` artifact inventory、live-anchor alignment check、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor`
- validation command:
  `python3 -m unittest scripts.tests.test_projection_codegen_samples`
  `python3 scripts/projection_codegen_samples.py list --format json`
  `python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json`
  `python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json`
  `python3 scripts/projection_codegen_samples.py check-all --format json`
  `python3 scripts/projection_codegen_samples.py closeout --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `find samples/generated -maxdepth 3 -type f | sort`
- debug / visualization output:
  `projection_view`、`cross_place_projection`、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`
- docs / report requirement:
  新しい report、`samples_progress.md`、projection docs を同期する
- stop line:
  manifest bridge evidence only を actual emitted executable family と混同しない。final projection IR / cross-place equivalence checker / optimizer / deployment planner / final public emitted-program ABI を freeze しない

### P16. visual debugger / viewer first public prototype

- status:
  close 済み。helper-local / report-local typed visualization inventory を `typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` として widen し、`scripts/visual_debugger_viewer_samples.py` に `P16-VIEW-01..05`、`viewer_panel_lanes`、`viewer_telemetry_lanes`、`actualized_panel_kinds`、`kept_later_gates` を actualize した。

- macro phase / stage:
  `Macro 7`, `S1 -> S4`
- objective:
  typed visualization / telemetry を public prototype viewer へ widen する
- deliverables:
  `scripts/visual_debugger_viewer_samples.py`、`P16-VIEW-01..05` typed bundle inventory、`viewer_panel_lanes` / `viewer_telemetry_lanes`、`first_public_prototype_over_typed_inventories` scope、viewer security wording、redaction/retention stop line
- validation command:
  `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples`
  `python3 scripts/visual_debugger_viewer_samples.py list --format json`
  `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json`
  `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json`
  `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json`
  `python3 scripts/visual_debugger_viewer_samples.py check-all --format json`
  `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
  `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- debug / visualization output:
  `turn_timeline`、`message_route`、`verification_summary`、`projection_view`、`membership_snapshot`、`hotplug_lifecycle`、`route_trace`、`audit_trace`、typed telemetry inventory
- docs / report requirement:
  新しい report、`samples_progress.md`、viewer docs を同期する
- stop line:
  helper-local preview / runtime canonical inventory をそのまま final public viewer API、final public visualization schema、final public telemetry schema、Event DAG / place graph / effect route graph / proof obligation graph、production telemetry backend と呼ばない

### P17. storage / LLVM / backend preparation

- status:
  close 済み。external workdir / LLVM guardrail を implementation-ready staging に寄せ、`llvm` owner/writable visibility、`llvm/src` cleanup exclusion、parent non-writable 時の `llvm/build` / `llvm/install` cleanup guard を current closeout line に固定した。device rewrite や backend freeze はまだ行わない。

- macro phase / stage:
  `Macro 7`, `S3 -> S5`
- objective:
  guardrail-only current line を implementation-ready な storage / LLVM / backend preparation に進める
- deliverables:
  external workdir policy、LLVM staging policy、cleanup safety、owner/writable probe、backend-prep stop line
- validation command:
  `bash scripts/env/mirrorea_storage_env.sh`
  `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`
  `bash scripts/storage/detach_prepare.sh`
  `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
  `df -h`
  `free -h`
  `lsblk -f`
  `findmnt`
  `ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install`
  `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  storage audit evidence、workdir mount status、LLVM owner/writable status、cleanup candidate inventory
- docs / report requirement:
  新しい report、storage audit、`samples_progress.md`、relevant plan/docs を同期する
- stop line:
  device format、mount rewrite、destructive cleanup、root-owned llvm parent の ownership repair、actual LLVM build を無断で実行しない

### P18. public API / parser grammar gate

- status:
  close 済み。repo-side first cut として final freeze checklist、public-boundary inventory、mixed gate と true user-spec hold line の分離を `plan/27-public-api-parser-gate-roadmap.md` と関連 snapshot docs に固定した。actual final parser grammar や actual final public API は still later に残す。

- macro phase / stage:
  `Macro 7` mixed gate, `S0 -> S3`
- objective:
  final parser grammar、public API、public verifier / viewer / adapter contract を premature に freeze しないための repo-side checklist と boundary inventory を定義する
- deliverables:
  final freeze checklist、public-boundary inventory、mixed gate と true user-spec gate の切り分け、preview/prototype/evidence qualifier の再確認
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  `python3 scripts/network_transport_samples.py closeout --format json`
  `python3 scripts/projection_codegen_samples.py closeout --format json`
  `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `bash scripts/env/mirrorea_storage_env.sh`
  `git diff --check`
- debug / visualization output:
  final freeze checklist 自体。runtime debug lane は前段 package の成果を参照する
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `specs/` / `plan/` を同期する
- stop line:
  actual final parser grammar / public API / public verifier / public adapter / public viewer / installed-binary commitment を `P18` repo-side first cut と混同しない

### U1. post-`P18` true user-spec hold option matrix

- status:
  close 済み。`P18` で閉じた repo-side framing の後に残る actual commitment を user choice として保ったまま、packaging shape / host integration target / first shipped public surface / final shared-space operational catalog breadth を option inventory と provisional recommendation 付きで整理した。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  user-spec gate へ入る前に、repo docs / plan / snapshot 上で choice axes と non-goal / stop line を揃える
- deliverables:
  `U1` option matrix、blocker refresh、mixed gate と user-spec gate の packaging split 明文化、reader-facing summary
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `progress.md` / `tasks.md` / `samples_progress.md` 上の current focus と blocker matrix
- docs / report requirement:
  新しい report、`plan/28`、relevant `plan/` / docs / snapshot の同期
- stop line:
  option inventory task の名目で actual product target、final public parser / checker / runtime / verifier API、final public adapter / viewer / projection / hot-plug / transport ABI を勝手に freeze しない

### R1. `VerificationLayer` widening threshold inventory

- status:
  close 済み。helper `verification_handoff_witness` / runtime `verification_model_check` を current emitted floor に保ったまま、finite-index checker preview / theorem bridge / runtime policy / visualization-telemetry を evidence carrier または downstream consumer に留める widening threshold matrix と stop line を `plan/29` と reader-facing docs に固定した。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  current evidence carrier、emitted layer candidate、kept-later gate を `VerificationLayer` family で切り分け、public-freeze mixed gate と user-spec hold line を増やさずに core-side public-surface narrowing criteria を明文化する
- deliverables:
  widening threshold matrix、helper/runtime verification lane inventory の current reading、hidden verifier builtin / final public verifier contract / final public layer law schema を避ける stop line、reader-facing summary
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `git diff --check`
- debug / visualization output:
  `--debug layers`、helper `verification_handoff_witness`、runtime `verification_model_check`、`layer_signatures`
- docs / report requirement:
  新しい report、relevant `plan/` / docs / snapshot の同期
- stop line:
  hidden verifier builtin、final public verifier contract、final public layer law schema、concrete external theorem / model-check binding を premature に固定しない

### R2. `AttachPoint` compatibility / detach minimal contract

- status:
  close 済み。`P14` helper-local package-manager first-cut closeout と `plan/21-hotplug-attachpoint-roadmap.md` の current memory を前提に、helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row、grounding envelope / view / telemetry anchor、kept-later migration / rollback gate を `plan/30` と reader-facing docs に固定した。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  current helper-local lifecycle canary、package-manager inventory surface、kept-later migration / rollback gate を分け、final public hot-plug ABI や runtime crate hot-plug engine を増やさずに minimal contract row を明文化する
- deliverables:
  minimal contract matrix、helper-local lifecycle lane inventory の current reading、storage detach と runtime detach の non-equivalence 再確認、migration / rollback / final public hot-plug ABI を避ける stop line、reader-facing summary
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `git diff --check`
- debug / visualization output:
  `--debug hotplug`、helper `hotplug_lifecycle`、`attach_request#1`、`detach_request#1`、`detached_roll_request#1`、`hotplug_lifecycle_lanes`
- docs / report requirement:
  新しい report、relevant `plan/` / docs / snapshot の同期
- stop line:
  runtime crate hot-plug engine、rollback protocol、durable migration engine、final public hot-plug ABI、storage detach と runtime detach の collapse を premature に固定しない

### R3. `FAIRY-05` visibility-return carrier bundling

- status:
  close 済み。`P9` helper closeout `fairy05_reopen_gate` と `plan/24-avatar-follow-representative-slice-roadmap.md` の current memory を前提に、active representative slice を保ったまま residual planned family `FAIRY-05` の carrier-choice matrix と provisional recommendation `typed bundle over state_timeline + anchor_switch` を `plan/31` と reader-facing docs に固定した。helper closeout implementation inventory `carrier_choice = UNRESOLVED` はそのまま保った。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  explicit `state_timeline` / `anchor_switch` evidence requirement を保ったまま、visibility-return witness を timeline event / anchor-switch event / witness event / typed bundle のどこへ載せるかを narrow にし、sample 自体の active promotion と final public avatar / visualization API を増やさずに current carrier matrix を明文化する
- deliverables:
  carrier-choice matrix、active representative slice と residual planned family の boundary 再確認、planning-only candidate label と current helper debug surface の非同一視、reader-facing summary
- validation command:
  `python3 -m unittest scripts.tests.test_avatar_follow_samples`
  `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
  `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
  `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
  `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
  `python3 scripts/avatar_follow_samples.py closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  avatar helper `planned_sample_paths`、`fairy05_reopen_gate`、`--debug anchors`、`--debug membership`、`--debug verification`
- docs / report requirement:
  新しい report、relevant `plan/` / docs / snapshot の同期
- stop line:
  `FAIRY-05` を evidence なしに active runnable widening へ昇格させない。planning-only candidate label を current public debug mode と混同しない。final public avatar / visualization API や real transport / session / auth semantics へ飛ばない

### R4. hot-plug real migration / rollback boundary

- status:
  close 済み。`R2` current minimal contract row と `plan/21-hotplug-attachpoint-roadmap.md` / `plan/30-attachpoint-detach-minimal-contract.md` の current memory を前提に、real migration / rollback / runtime-crate hot-plug engine / distributed activation ordering を helper-local current evidence から切り離した kept-later boundary として `plan/32` と reader-facing docs に固定した。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  helper-local `hotplug_lifecycle` / explicit detach TODO boundary を completed migration や rollback protocol と混同させないように、current minimal contract row と kept-later engine boundary の split を reader-facing に明文化する
- deliverables:
  kept-later boundary matrix、storage detach / network replay との非同一視、real migration / rollback / runtime-crate engine / distributed activation ordering の stop line、reader-facing summary
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `hotplug_lifecycle`、`attach_request#1`、`detach_request#1`、`detached_roll_request#1`、`hotplug_kept_later_gates`
- docs / report requirement:
  新しい report、relevant `plan/` / docs / snapshot の同期
- stop line:
  helper-local package-manager closeout inventory を completed migration / rollback / runtime engine ownership と取り違えない。storage detach や network replay と collapse しない。final public hot-plug ABI を premature に固定しない

### R5. runtime-crate hot-plug engine ownership cut

- status:
  close 済み。`R4` closeout memory を前提に、helper-local hot-plug evidence、crate-side carrier/runtime substrate、later runtime orchestration の owner split を docs-first に narrow に固定した。hot-plug engine actualization や final public ABI の固定はまだ行わない。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  `hotplug_lifecycle`、`attach_request#1`、`detach_request#1`、`detached_roll_request#1` をどこが owner として持つかを narrow にし、helper-local preview と crate-side substrate の境界を reader-facing に固定する
- deliverables:
  owner split matrix、`mirrorea-core` / `mir-runtime` / helper-local crate-anchor map、rollback / durable migration / distributed activation ordering / final public ABI の kept-later table、reader-facing summary
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo test -p mirrorea-core`
  `cargo test -p mir-runtime`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  `hotplug_lifecycle`、`attach_request#1`、`detach_request#1`、`detached_roll_request#1`、runtime closeout inventory、crate-side ownership note
- docs / report requirement:
  新しい report、`plan/21`、新設する `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`、`README.md` / `Documentation.md` / `progress.md` / `tasks.md` / `docs/research_abstract/mirrorea_future_axis_01.md` / `docs/hands_on/current_phase_closeout_01.md` の同期。`samples_progress.md 更新不要` なら report に明記する
- stop line:
  hot-plug engine を実装しない。`hotplug_lifecycle` や `LogicalPlaceRuntimeShell` を既成 engine と見なさない。rollback / migration / distributed activation ordering / final public ABI を同じ package で fixed にしない

### R6. runtime-crate hot-plug carrier admission cut

- status:
  close 済み。`R5` closeout memory を前提に、post-`R5` の first admissible Rust-side hot-plug-specific family を engine-neutral request / verdict carrier に限定し、`P19` `mirrorea-core` current closeout と `P20` `mir-runtime` current closeout へ進む queue split を docs-first に固定した。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  first admissible Rust-side hot-plug-specific family を narrow に決め、helper-local preview / reusable carrier ownership / later runtime orchestration を collapse しない queue cut を固定する
- deliverables:
  owner/admission matrix、allowed-first-family list、kept-later table、`P19` / `P20` sequence、reader-facing summary
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `cargo test -p mirrorea-core`
  `cargo test -p mir-runtime`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  helper `hotplug_lifecycle`、sample-grounded attach-detach IDs、closeout `hotplug_kept_later_gates`、crate-side owner/admission reading
- docs / report requirement:
  新しい report、`plan/21`、新設する `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`、`README.md` / `Documentation.md` / `progress.md` / `tasks.md` / `samples_progress.md` / reader-facing docs の同期
- stop line:
  helper-local lifecycle / sample-grounded IDs / attach-detach view-telemetry IDs を Rust canonical engine family と読まない。request/verdict carrier を rollback / migration / distributed ordering / final public ABI と混同しない

### P19. `mirrorea-core` hot-plug request/verdict carrier tranche

- status:
  close 済み。`crates/mirrorea-core/src/fabric.rs` に engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を actualize し、`verdict_kind` は `accepted / rejected / deferred` の narrow family に留めた。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  `R6` で admissible にした reusable engine-neutral request / verdict carrier だけを `mirrorea-core` に actualize する
- deliverables:
  request-side carrier、verdict-side carrier、tests、report、snapshot sync
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  `cargo test -p mirrorea-core`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  helper `hotplug_lifecycle` / closeout を grounding evidence にしつつ、Rust-side carrier ownership note を report に残す
- docs / report requirement:
  新しい report、`plan/34`、relevant front-door docs / snapshot docs / `samples_progress.md` の同期
- stop line:
  lifecycle state machine、rollback state、migration protocol、final public seam naming を同じ tranche で actualize しない

### P20. `mir-runtime` hot-plug orchestration skeleton first tranche

- status:
  close 済み。`P19` carrier と existing substrate の上で thin runtime/report assembly だけを narrow に actualize し、dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を `mir-runtime` に置いた。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  `P19` carrier と existing substrate の上に、thin runtime/report assembly 側の hot-plug orchestration skeleton だけを narrow に置く
- deliverables:
  dedicated runtime/report assembly skeleton module、tests、report、snapshot sync
- validation command:
  `cargo test -p mir-runtime`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  runtime report-local carrier note over admitted request/verdict carrier + substrate snapshot
- docs / report requirement:
  新しい report、`plan/34` と relevant roadmap / snapshot docs の同期
- stop line:
  completed engine、rollback protocol、durable migration engine、distributed activation ordering、final public hot-plug ABI を claim しない

### R7. post-`P20` hot-plug next-package inventory

- status:
  close 済み。`plan/35-post-p20-hotplug-next-package-inventory.md` と companion docs により、post-`P20` kept-later lane を smallest plausible package cuts に分け、`P21` runtime-crate hot-plug completed-engine narrow cut を next narrow implementation line として固定した。

- macro phase / stage:
  `Macro 8` prep, `S0 -> S2`
- objective:
  `P20` current closeout の先に残る hot-plug kept-later lane を docs-first に decomposition し、next narrow implementation line と later-family grouping を固定する
- deliverables:
  `plan/35`、reader-facing summary / landing page、snapshot / roadmap sync、report
- validation command:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  existing `hotplug_lifecycle` / `HotPlugRuntimeSkeletonReport` evidence を grounding にした package-cut matrix
- docs / report requirement:
  新しい report、`plan/35`、relevant front-door docs / snapshot docs / reader-facing docs の同期
- stop line:
  `R7` の next-line recommendation を `P21` actualization と混同しない。exact later package labels を evidence なしに固定しない

### P21. runtime-crate hot-plug completed-engine narrow cut

- status:
  close 済み。`HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、consumer-side `assemble_hotplug_runtime_engine_report()`、example `build_hotplug_runtime_engine_report()` を `mir-runtime` に actualize し、request/verdict consumption、runtime snapshot mirror の `active_membership_epoch` / `reason_refs`、`attach|detach x accepted|rejected|deferred` の canonical runtime-side state progression を narrow に actualize した。

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  admitted request/verdict carrier と existing substrate の上に canonical runtime-side hot-plug engine state progression を narrow に actualize する
- deliverables:
  runtime-side engine/state carrier、request/verdict consumption path、state-transition tests、report、snapshot sync
- validation command:
  `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
  `cargo test -p mir-runtime`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
  `git diff --check`
- debug / visualization output:
  engine-side lifecycle/state note over admitted request/verdict carrier + substrate snapshot
- docs / report requirement:
  新しい report、`plan/35` と relevant roadmap / snapshot docs の同期
- stop line:
  rollback protocol、durable migration / reattach semantics、distributed activation ordering、final public hot-plug ABI を同じ tranche に混ぜない

## research を通して見つけること

- effect-based OS-like substrate という内側の解釈を prose に留めるか、formal layer naming に上げるか
- projection / placement validity report の最小 shapeと、generated artifact policy の最小 shape
- `FAIRY-05` visibility-return witness をどの carrier へ載せるか

## user が決める必要があること

### Blocker 1. final public contract の固定範囲

- 概要:
  auth / visualization / projection / hot-plug / verifier をどこまで public API として固定するか
- 影響:
  `P6`、`P7`、`P15`、`P16`、`P18`
- 主要な選択肢:
  repo-local helper 優先のまま進める / early public-contract drafting を始める
- current recommendation / 見解:
  repo-side framingは `P18` で済ませたので、actual public contract commitment は post-`P18` user-spec hold line として扱う

### Blocker 2. packaging / FFI / host integration target

- 概要:
  installed binary、FFI、engine adapter、deployment contract の最終受け皿
- 影響:
  `P12`、`P17`、`P18`
- 主要な選択肢:
  CLI / library / engine-adapter / hybrid
- current recommendation / 見解:
  library-first ordering を保ち、installed binary promotion は second gate として扱う。host target は browser / native process / engine / mixed の option matrix を明示した上で、current provisional bias は native-process-first とするが、actual adoption は user choice に残す

### Blocker 3. final shared-space operational catalog

- 概要:
  authoritative-room minimal subset から portal / multi-world / fairness / quorum をどこまで final catalog に含めるか
- 影響:
  `P8`、`P9`、`P11`、`P13`、`P14`、`P18`
- 主要な選択肢:
  minimal subset を長く維持する / catalog drafting を早める
- current recommendation / 見解:
  まず minimal subset と residual gate を保ち、final catalog は true user-spec gate に残す

### Blocker 4. first shipped public surface scope

- 概要:
  final public ship を parser/checker/runtime/verifier first に切るか、adapter/viewer/projection/hot-plug first に切るか、two-step split にするか
- 影響:
  `P12`、`P15`、`P16`、`P18`
- 主要な選択肢:
  parser/checker/runtime/verifier first / adapter-viewer-projection-hot-plug first / two-step split
- current recommendation / 見解:
  packaging shape と host target が決まる前に 1 bucket へ潰さず、core library surface first と integration surface second の two-step split を provisional recommendation に置く
