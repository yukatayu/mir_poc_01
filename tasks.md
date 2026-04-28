# tasks

最終更新: 2026-04-28 09:26 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較は `plan/`、runnable sample 状態は `samples_progress.md`、詳細証跡は `docs/reports/` に置きます。
- append-only 履歴ではなく、現況に合わせて毎回全体を書き直す snapshot として扱います。

## current status at task level

- active clean near-end suite、Sugoroku world vertical slice、avatar follow representative slice は runnable です。
- `P0` current-state audit と `P1` repository layer map / `samples_progress.md` stabilization は close 済みです。
- `TermSignature`、`LayerSignature`、`MessageEnvelope / Auth seam`、`VisualizationProtocol`、typed external synthetic preview helper、projection preview、hot-plug helper-local lifecycle canary、network transport helper-local canary、storage / LLVM guardrail は first cut / widening close 済みです。
- `P2` Typed external boundary residual planned family review は close 済みです。
- current promoted next line は **`P3` Projection / placement residual emitted-program gate** です。
- next reopen point は **`P4` `TermSignature` registry hardening** です。
- current snapshot を短く追う入口は `progress.md`、`samples_progress.md`、`docs/hands_on/current_phase_closeout_01.md` です。

## current executable floor

| lane | current floor | note |
|---|---|---|
| Mir current-L2 | `samples/current-l2/` | base corpus と current-L2 source execution を維持 |
| clean near-end suite | `samples/clean-near-end/{typing,order-handoff,model-check,modal}/` | finite-index typing、order / handoff、model-check second line、modal family が runnable |
| Sugoroku world | `samples/clean-near-end/sugoroku-world/` + `scripts/sugoroku_world_samples.py` | attach / membership / handoff / reset model-check / hot-plug helper-local lifecycle canary |
| avatar follow | `samples/clean-near-end/avatar-follow/` + `scripts/avatar_follow_samples.py` | `FAIRY-01/02/03/04/06` active、`FAIRY-05` は planned |
| typed external / transport | `scripts/typed_external_boundary_samples.py`、`scripts/network_transport_samples.py` | helper-local synthetic preview / canary。typed external residual reopen matrix は fixed 済みだが、final public adapter / transport contract ではない |
| projection / placement | Sugoroku `projection_view`、runtime `cross_place_projection` | helper-local / report-local preview floor。final emitted program ではない |
| storage / backend guardrail | `/mnt/mirrorea-work`、`scripts/env/mirrorea_storage_env.sh` | root disk を既成事実化しない external workdir floor |

## ordered self-driven packages

| Package | Macro phase | Stage | Status | Rough estimate | Current reading |
|---|---|---|---|---|---|
| `P0` current-state audit | `Macro 0` | `S6 -> S6` | closed | closed | source hierarchy / stale reference audit |
| `P1` repo layer map / samples dashboard stabilization | `Macro 0` | `S6 -> S6` | closed | closed | taxonomy / dashboard synchronization |
| `P2` typed external residual review | `Macro 6` reserve | `S5 -> S6` | closed | closed | residual planned family review |
| `P3` projection residual emitted-program gate | `Macro 6` reserve | `S5 -> S6` | next | 1-2 tasks | emitted-program stop-line definition |
| `P4` TermSignature hardening | `Macro 6` | `S4 -> S5` | reopen next | ~1 task | first cut exists, naming/law hardening remains |
| `P5` LayerSignature hardening | `Macro 6` | `S4 -> S5` | queued | ~1 task | first cut exists, law surface remains |
| `P6` MessageEnvelope/Auth seam hardening | `Macro 6` | `S4 -> S5` | queued | ~1 task | first cut exists, public seam remains open |
| `P7` VisualizationProtocol/Security | `Macro 6-7` | `S4 -> S5` | queued | ~1 task | first cut exists, viewer/security hardening remains |
| `P8` Sugoroku runtime attach hardening | `Macro 6` | `S5 -> S6` | queued | 1-2 tasks | representative runtime slice hardening |
| `P9` avatar fairy follow hardening | `Macro 6` | `S5 -> S6` | queued | 1-2 tasks | residual `FAIRY-05` gate remains |
| `P10` mirrorea-core first real implementation tranche | `Macro 6-7` | `S1 -> S4` | later | multi-task | placeholder -> first real core |
| `P11` logical multi-place runtime tranche | `Macro 6-7` | `S1 -> S4` | later | multi-task | helper emulator -> crate substrate |
| `P12` external adapter / host boundary tranche | `Macro 7` | `S1 -> S4` | later | multi-task | host-facing adapter seam |
| `P13` network transport minimal alpha | `Macro 6-7` | `S1 -> S4` | later | multi-task | helper canary -> real transport alpha |
| `P14` hot-plug package-manager tranche | `Macro 6-7` | `S1 -> S4` | later | multi-task | compatibility / migration / rollback |
| `P15` projection/codegen emitted programs | `Macro 7` | `S1 -> S4` | later | multi-task | preview floor -> emitted programs |
| `P16` visual debugger / viewer prototype | `Macro 7` | `S1 -> S4` | later | multi-task | typed visualization public prototype |
| `P17` storage / LLVM / backend preparation | `Macro 7` | `S3 -> S5` | later | 1-3 tasks | guardrail -> implementation-ready |
| `P18` public API / parser grammar gate | `Macro 7` mixed gate | `S0 -> S3` | final gate | user + repo dependent | final freeze remains last |

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
  current promoted next line

- macro phase / stage:
  `Macro 6` reserve, `S5 -> S6`
- objective:
  helper/report-local preview floor と emitted place program / optimizer / equivalence gate を docs-first に切り分ける
- deliverables:
  emitted-program stop line、validity report minimum、preview floor と generated artifact family の境界
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `python3 scripts/validate_docs.py`
- debug / visualization output:
  `projection_view`、`cross_place_projection`
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/20`、relevant docs を同期する
- stop line:
  final projection IR、optimizer、equivalence checker、deployment planner を固定しない

### P4. `TermSignature` registry hardening

- macro phase / stage:
  `Macro 6`, `S4 -> S5`
- objective:
  signature kind / granularity / reserved-kind wording を tighten し、helper/runtime/report naming drift を減らす
- deliverables:
  current signature inventory rule、reserved kind policy、report-local mirror rule
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- debug / visualization output:
  `term_signatures`、`signature_kinds`、`reserved_signature_kinds`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/09`、relevant docs を同期する
- stop line:
  final public signature schema や final public message contract を既成事実化しない

### P5. `LayerSignature` system hardening

- macro phase / stage:
  `Macro 6`, `S4 -> S5`
- objective:
  `requires / provides / transforms / checks / emits / laws` carrier の naming と law wording を tighten する
- deliverables:
  `LayerSignature` wording、`VerificationLayer` composition current reading、report-local mirror rule
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- debug / visualization output:
  `layer_signatures`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/09`、`plan/14`、relevant docs を同期する
- stop line:
  final public layer law schema や hidden verifier builtin を既成事実化しない

### P6. `MessageEnvelope` / `AuthEvidence` seam hardening

- macro phase / stage:
  `Macro 6`, `S4 -> S5`
- objective:
  message/auth seam を tighten し、transport / auth / membership / capability / witness split を明確に保つ
- deliverables:
  current `AuthEvidence` baseline wording、transport insertion seam wording、public stop line
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  `python3 scripts/network_transport_samples.py closeout --format json`
- debug / visualization output:
  `message_envelopes`、`auth_evidence_kinds`、`transport_seams`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/09`、relevant docs を同期する
- stop line:
  session / signature protocol、final public auth schema、final public transport ABI を固定しない

### P7. `VisualizationProtocol` + `VisualizationSecurity`

- macro phase / stage:
  `Macro 6-7`, `S4 -> S5`
- objective:
  visualization / telemetry を typed effect として tighten し、label / authority / redaction / retention を整理する
- deliverables:
  current static/runtime view family、typed telemetry wording、security stop line
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
- debug / visualization output:
  `visualization_views`、`telemetry_rows`、route-trace redaction
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/14`、relevant docs を同期する
- stop line:
  final public viewer contract、retention policy、multi-tenant telemetry service を固定しない

### P8. Sugoroku runtime attach hardening

- macro phase / stage:
  `Macro 6`, `S5 -> S6`
- objective:
  attach / membership / handoff / late join / detach TODO boundary を representative runtime slice として harden する
- deliverables:
  membership registry wording、world sugar boundary、detach lifecycle current stop line
- validation command:
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- debug / visualization output:
  `message_envelopes`、`hotplug_lifecycle`、attach-detach telemetry / visualization current view
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/16`、relevant docs を同期する
- stop line:
  real network、consensus、durable distributed commit、final public runtime API を claim しない

### P9. avatar fairy follow hardening

- macro phase / stage:
  `Macro 6`, `S5 -> S6`
- objective:
  active representative slice を保ちながら residual `FAIRY-05` gate を整理する
- deliverables:
  explicit state timeline / anchor switch evidence gate、carrier `UNRESOLVED` の current reading
- validation command:
  `python3 scripts/avatar_follow_samples.py closeout --format json`
  `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
  `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
- debug / visualization output:
  `anchors`、`membership`、`verification`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/24`、relevant docs を同期する
- stop line:
  `FAIRY-05` を evidence なしに active 化しない。public avatar / visualization API へ飛ばない

### P10. `mirrorea-core` first real implementation tranche

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  placeholder crate から first real Mirrorea runtime substrate を起こす
- deliverables:
  crate responsibility boundary、minimal core carrier、report-local invariants
- validation command:
  `UNRESOLVED`. first acceptable command candidate は `cargo test -p mirrorea-core`
- debug / visualization output:
  `UNRESOLVED`. expected first surface は place graph / effect route graph / layer inventory
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/19` を同期する
- stop line:
  `mir-runtime` や helper script へ責務を無言で押し戻さない。public API freeze もしない

### P11. logical multi-place runtime tranche

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  helper-local logical multi-place emulator を crate-side runtime substrate へ寄せる
- deliverables:
  event DAG、message flow、membership timeline の runtime-side carrier
- validation command:
  `UNRESOLVED`. current anchor は `python3 scripts/sugoroku_world_samples.py closeout --format json`
- debug / visualization output:
  event DAG、message flow、state timeline、membership timeline
- docs / report requirement:
  新しい report、`samples_progress.md`、runtime docs を同期する
- stop line:
  real network / consensus / durable replay を先に claim しない

### P12. external adapter / host boundary tranche

- macro phase / stage:
  `Macro 7`, `S1 -> S4`
- objective:
  typed external boundary を host-facing adapter seam として tighten する
- deliverables:
  adapter responsibility boundary、host boundary wording、carrier split
- validation command:
  `UNRESOLVED`. current anchor は `python3 scripts/typed_external_boundary_samples.py closeout --format json`
- debug / visualization output:
  `envelopes`、`visualization`、`failures`
- docs / report requirement:
  新しい report、`samples_progress.md`、adapter docs を同期する
- stop line:
  final public adapter ABI、browser/network/VR family contract を premature に固定しない

### P13. network transport minimal alpha

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  helper-local canary から real socket / session / replay minimal alpha へ widening する
- deliverables:
  minimal transport alpha、reconnect guard、failure classification
- validation command:
  `python3 scripts/network_transport_samples.py closeout --format json`
  additional real-socket command は `UNRESOLVED`
- debug / visualization output:
  `route-trace`、`reconnect`、`failures`
- docs / report requirement:
  新しい report、`samples_progress.md`、transport docs を同期する
- stop line:
  consensus、durable replay、multi-server production transport を claim しない

### P14. hot-plug `Patch` / `AttachPoint` package-manager tranche

- macro phase / stage:
  `Macro 6-7`, `S1 -> S4`
- objective:
  compatibility、activation、migration、rollback を package-manager concern として widen する
- deliverables:
  attachpoint compatibility contract、migration stop line、rollback wording
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  package-manager-specific command は `UNRESOLVED`
- debug / visualization output:
  `hotplug_lifecycle`
- docs / report requirement:
  新しい report、`samples_progress.md`、`plan/21`、relevant docs を同期する
- stop line:
  deployment-grade migration / rollback / durable upgrade engine を premature に固定しない

### P15. projection/codegen first emitted server/client programs

- macro phase / stage:
  `Macro 7`, `S1 -> S4`
- objective:
  preview floor から emitted place program への first actualization を行う
- deliverables:
  emitted server/client artifact minimum、equivalence review surface、generated artifact policy
- validation command:
  `UNRESOLVED`. current guard は `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
- debug / visualization output:
  projection view、place graph、generated artifact inventory
- docs / report requirement:
  新しい report、`samples_progress.md`、projection docs を同期する
- stop line:
  final optimizer / equivalence checker / deployment planner を freeze しない

### P16. visual debugger / viewer first public prototype

- macro phase / stage:
  `Macro 7`, `S1 -> S4`
- objective:
  typed visualization / telemetry を public prototype viewer へ widen する
- deliverables:
  first public prototype boundary、viewer security wording、redaction policy draft
- validation command:
  `UNRESOLVED`. current guard は `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
- debug / visualization output:
  TermSignature view、layer view、event/message/state/membership/witness/telemetry views
- docs / report requirement:
  新しい report、`samples_progress.md`、viewer docs を同期する
- stop line:
  helper-local preview をそのまま final public viewer API と呼ばない

### P17. storage / LLVM / backend preparation

- macro phase / stage:
  `Macro 7`, `S3 -> S5`
- objective:
  guardrail-only current line を implementation-ready な storage / LLVM / backend preparation に進める
- deliverables:
  external workdir policy、LLVM staging policy、cleanup safety、backend-prep stop line
- validation command:
  `bash scripts/env/mirrorea_storage_env.sh`
  `bash scripts/storage/detach_prepare.sh`
  `df -h`
  `lsblk -f`
  `findmnt`
- debug / visualization output:
  storage audit evidence、workdir mount status
- docs / report requirement:
  新しい report、storage audit、`samples_progress.md`、relevant plan/docs を同期する
- stop line:
  device format、mount rewrite、destructive cleanup を無断で実行しない

### P18. public API / parser grammar gate

- macro phase / stage:
  `Macro 7` mixed gate, `S0 -> S3`
- objective:
  final parser grammar、public API、public verifier / viewer / adapter contract の freeze 条件を定義する
- deliverables:
  final freeze checklist、mixed gate と true user-spec gate の切り分け
- validation command:
  `UNRESOLVED`. prior packages の closeout と user-spec gate の充足が前提
- debug / visualization output:
  final freeze checklist 自体。runtime debug lane は前段 package の成果を参照する
- docs / report requirement:
  新しい report、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `specs/` / `plan/` を同期する
- stop line:
  `P10-P17` 未成熟のまま final parser / public API / public verifier を freeze しない

## research を通して見つけること

- `VerificationLayer` composition の exact law surface、machine-check / theorem / runtime policy / visualization の責務分担
- effect-based OS-like substrate という内側の解釈を prose に留めるか、formal layer naming に上げるか
- projection / placement validity report の最小 shapeと、generated artifact policy の最小 shape
- `AttachPoint` compatibility と detach lifecycle の最小 contract
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
  まだ repo-local helper と docs-first carrier に留め、public contract は mixed gate に残す

### Blocker 2. packaging / FFI / host integration target

- 概要:
  installed binary、FFI、engine adapter、deployment contract の最終受け皿
- 影響:
  `P12`、`P17`、`P18`
- 主要な選択肢:
  CLI / library / engine-adapter / hybrid
- current recommendation / 見解:
  current task では固定せず、backend / LLVM preparation の後に user-spec gate として reopen する

### Blocker 3. final shared-space operational catalog

- 概要:
  authoritative-room minimal subset から portal / multi-world / fairness / quorum をどこまで final catalog に含めるか
- 影響:
  `P8`、`P9`、`P11`、`P13`、`P14`、`P18`
- 主要な選択肢:
  minimal subset を長く維持する / catalog drafting を早める
- current recommendation / 見解:
  まず minimal subset と residual gate を保ち、final catalog は true user-spec gate に残す
