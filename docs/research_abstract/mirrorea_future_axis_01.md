# Mirrorea future axis 01

## この文書の役割

この文書は、Mirrorea の **次の docs-first / repo-local integration line** を日本語で短く読むための summary です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 実行証跡は `docs/reports/`
- ここは current reading と next package queue を reader-friendly にまとめる入口です

## 現在地

2026-04-28 時点で repo が実装済みの current line は次です。

- clean near-end suite
- current-L2 base source corpus
- finite-index first strong typing layer
- order / handoff relation family
- model-check second line
- Lean foundations / generated stub
- Sugoroku world runtime attachment vertical slice
- avatar fairy follow representative slice

これらは **repo-local alpha-ready current layer** です。
Mirrorea full runtime、final public API、real network、final auth stack、final visualization API まで完了したことを意味しません。

同時に、Mirrorea future-axis 側では `TermSignature registry / debug output` の first cut を close し、さらに `P4` `TermSignature` registry hardening も close しました。

- Sugoroku helper に `--debug signatures`
- clean near-end report / closeout に `term_signatures` と `signature_kinds`
- helper / runtime closeout に `signature_lanes`、`signature_scope`、`signature_evidence_roles`

を追加し、helper-local / report-local の evidence carrier を先に actualize しています。
current line では `signature_lanes = kind/name/evidence_role`、helper closeout `signature_scope = representative_slice`、clean near-end closeout `signature_scope = clean_near_end_canonical_inventory`、active kind family `effect / transition / witness / relation / property`、reserved `message` / `adapter` / `layer` split まで fixed しました。
これは final public signature schema や final public message / adapter contract を意味しません。

さらに `P5` `LayerSignature system` hardening も close しました。

- Sugoroku helper に `--debug layers`
- clean near-end report / closeout に `layer_signatures`

を追加し、helper-local representative inventory と runtime report-local canonical inventory を
どちらも `name / requires / provides / transforms / checks / emits / obligations / laws` で読めるようにしました。
helper closeout は `representative_slice` scope で `verification_handoff_witness` / `runtime_turn_trace` / `membership_*` / `hotplug_*` を持ち、runtime closeout は `clean_near_end_canonical_inventory` scope で `auth_authority_witness` / `transport_provider_boundary` / `verification_model_check` を持ちます。scope split は current repo memory であり、final public layer law schema を意味しません。

さらに `P6` `MessageEnvelope / AuthEvidence` seam hardening も close しました。
さらに `P10` `mirrorea-core` first real implementation tranche も close しました。
さらに `P11` logical multi-place runtime tranche では、current third cut として `MembershipRegistry`、`PlaceCatalog`、participant-place-kind-gated `LogicalPlaceRuntimeShell`、principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper を `mirrorea-core` に actualize しました。
さらに `VisualizationProtocol` の first cut も close しました。
さらに `Typed external boundary / adapter` の docs-first sample plan も close しました。
さらに `Typed external boundary executable widening` も close しました。
さらに `P12` external adapter / host boundary tranche の current first cut も close しており、typed external helper subset / closeout に helper-local `host_boundary` preview inventory を actualize しました。さらに `P13` network transport minimal alpha の current first-cut closeout も close しており、helper closeout に helper-local `process_boundary` inventory を actualize しました。さらに `P14` hot-plug package-manager tranche の current first-cut closeout も close しており、helper closeout に `hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor` を含む helper-local package-manager inventory を actualize しました。さらに `P15` projection/codegen first emitted place-specific programs の current first-cut closeout も close しており、`scripts/projection_codegen_samples.py`、`samples/generated/projection-placement/manifest.json`、`P15-GEN-01..04` committed generated bridge evidence、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor` を current line に actualize しました。さらに `P16` visual debugger / viewer first public prototype の current first-cut closeout も close しており、`scripts/visual_debugger_viewer_samples.py`、`P16-VIEW-01..05`、`viewer_panel_lanes` / `viewer_telemetry_lanes`、`actualized_panel_kinds`、`kept_later_gates` を current line に actualize しました。さらに `P17` storage / LLVM / backend preparation の current first-cut closeout と `P18` public API / parser grammar gate の repo-side first-cut closeout も close しており、public-boundary inventory / mixed-gate と true user-spec hold line の分離を current line に actualize しました。`U1` post-`P18` true user-spec hold option matrix は close 済みであり、packaging shape / host integration target / first shipped public surface / final shared-space operational catalog breadth は provisional recommendation 付きの option inventory として整理しつつ、actual commitment は user choice に残しています。`R1` `VerificationLayer` widening threshold inventory も close 済みであり、helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor と widening threshold matrix を current memory に固定しました。`R2` `AttachPoint` compatibility / detach minimal contract も close 済みであり、helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row と kept-later migration / rollback gate を current memory に固定しました。current promoted next line は `R3` `FAIRY-05` visibility-return carrier bundling であり、active representative slice を保ったまま residual planned family の carrier choice を docs-first に narrow にします。
さらに `Projection / placement` の docs-first plan も close しました。
さらに `Projection / placement executable widening` も close しました。
さらに `HotPlug Patch / AttachPoint` の docs-first plan も close しました。
さらに `Network transport` の docs-first plan も close しました。
さらに `Compiler/backend/LLVM preparation` guardrail も close しました。
さらに `hands-on docs / closeout` も close しました。

- Sugoroku helper に `message_envelopes` と `--debug envelopes`
- Sugoroku helper に `projection_view` と `--debug projection`
- clean near-end report / closeout に `MessageEnvelope` inventory
- clean near-end report-local inventory に `cross_place_projection`
- `scripts/typed_external_boundary_samples.py` と `samples/not_implemented/typed-external-boundary/`
  に synthetic preview subset `EXT-03` / `EXT-04`

を追加し、current none-auth baseline のまま transport / auth / membership / capability / witness を
separate lane で読めるようにしています。2026-04-28 時点では helper/runtime `message_envelope_scope`、
`transport_medium` / `transport_seam`、`emitter_principal`、`freshness_checks`、shared `auth_evidence_lanes = kind / subject / issuer / bindings / notes` まで current line に上げてよいです。
ここでの `auth none` は temporary repo-local baseline であり、session token / signature /
federation protocol を fixed した意味ではありません。

## 主軸

守るべき project axis は次です。

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

局所最適化でこの軸を崩してはいけません。

## 高位の読み

### Place

`Place` は participant そのものではありません。

- physical process
- machine / node
- virtual thread
- authority / permission compartment
- state / queue / visibility / observation frontier を持つ execution locus

したがって `Alice is a Place` ではなく、
`Alice` は participant / principal、
`ParticipantPlace[Alice]` は Place と読みます。

### external world boundary

- standard I/O は Mir core primitive ではありません。
- `stdin` / `stdout` / `stderr` を core built-in として固定しません。
- external world とは typed external effect adapter で接続します。

ここでいう adapter は、unsafe FFI をそのまま privileged 化するものではなく、
contract と effect を保った boundary です。

### effect-based OS-like substrate

Mirrorea / adapter / visualization / telemetry の内側を、
effect-based OS-like substrate として読むこと自体は有用です。

ただしこれは **内側の解釈** です。

- Mir core に privileged standard I/O primitive を足す理由にはしない
- Mir / Mirrorea / Typed-Effects Wiring Platform を 1 つの runtime へ collapse しない
- authentication / authorization / membership / capability / witness を transport に潰さない

### transport と auth の分離

次の層は潰さずに分けます。

- transport
- authentication
- authorization
- membership
- capability
- witness / audit

current repo-local line では Sugoroku sample が membership epoch / incarnation / witness を先に示しており、
helper-local / report-local first cut では `MessageEnvelope` carrier で transport insertion seam を visible にしました。
未決なのは final public `AuthEvidence` kind、real transport、session / signature schema です。

### visualization / telemetry

可視化は optional debug toy ではありません。
ただし visualization も telemetry も、外へ情報を出す effect です。

そのため次を要求します。

- label
- authority
- redaction
- retention
- evidence-oriented rendering

`untyped debug leak` として出してよいものではありません。
performance telemetry も同様に information-bearing effect であり、typed telemetry として扱います。current line では helper/runtime security envelope に `retention_scope` を持たせ、NET-05 observer route trace は fail-closed にしています。
さらに `P16` current first-cut closeout では helper/runtime surface を `viewer_panel_lanes` / `viewer_telemetry_lanes` と `P16-VIEW-01..05` に正規化し、`typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` を current stop line にしました。

### VerificationLayer composition

`VerificationLayer` は、finite-index checker、theorem bridge、model-check second line、
runtime policy preview、visualization / telemetry lane を typed layer として合成する current reading を指します。current emitted floor は helper `verification_handoff_witness` と runtime `verification_model_check` までです。

- helper-local signature dump や report-local inventory は first cut の evidence carrier に留める
- hidden verifier builtin や final public verifier contract を既成事実化しない
- exact public law surface と composition contract は `UNRESOLVED` のまま残す

### projection / placement

高位の source から、後で place-specific program へ projection できる性質を保ちます。

- server-side authoritative path
- participant-side view / local debug path
- adapter path
- visualizer path

のどれかに source principal を早期固定しないことが重要です。

2026-04-28 時点では、`plan/20-projection-and-placement-roadmap.md` に
system-wide source / place-specific program distinction、place split、validity checklist、helper/report-local preview floor、stop line を置いてあります。
さらに helper-local / report-local current evidence として、

- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`

で `projection_view` と `cross_place_projection` を読めます。
さらに `python3 scripts/projection_codegen_samples.py closeout --format json`
で committed generated bridge evidence と live-anchor alignment surface を読めます。
ただし、これは final projection IR や emitted executable place program を意味しません。

### hot-plug

Mirrorea の中心は runtime hot-plug です。

- Patch
- AttachPoint
- compatibility check
- activation cut
- migration contract

を今後の principal package として扱います。

2026-04-28 時点では、`plan/21-hotplug-attachpoint-roadmap.md` に
compatibility checklist、activation cut、migration stop line、`SUG-01` attach と `SUG-09` detach TODO boundary を置いてあります。
さらに helper-local executable widening として、Sugoroku helper の `hotplug_lifecycle`、
`--debug hotplug`、`detach_request#1` auth-none envelope canary、attach-detach telemetry / visualization current view も actualize しました。
storage detach script は operational cleanup concern であり、この runtime hot-plug lifecycle そのものではありません。

### network transport

transport widening は `local_queue` baseline をそのまま final public transport ABI にしないための docs-first line です。

- loopback
- reconnect
- membership epoch / member incarnation guard
- typed transport failure
- redacted route trace

2026-04-28 時点では、`plan/22-network-transport-roadmap.md` に
current anchor、`NET-01..05` ladder、transport widening invariant、stop line を置いてあります。
`NET-01` だけは helper-local `--transport loopback_socket` preview として actualize し、
same-process emulator のまま envelope field / reject path parity を確認できます。
ここでも transport と auth / membership / capability / witness / visualization を collapse しません。

### compiler/backend/LLVM preparation

toolchain / backend 側では、small VPS と detachable workdir を前提に、
root disk を build cache や LLVM artifact で既成事実化しない guardrail を先に固定します。

- external workdir
- `CARGO_TARGET_DIR`
- `CARGO_HOME`
- LLVM path readiness
- non-destructive cleanup

2026-04-28 時点では、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` に
current anchor、non-destructive probe floor、stop line を置いてあります。
actual LLVM build、final backend choice、final packaging / FFI / engine adapter target はまだ fixed しません。

current closeout を実行コマンド付きで追う landing page は `docs/hands_on/current_phase_closeout_01.md` です。

## representative slices

### current

- Sugoroku world runtime attachment vertical slice
- avatar fairy follow representative slice

### residual candidate

- `FAIRY-05` reacquire-after-return widening

avatar fairy follow では、`scripts/avatar_follow_samples.py` と
`samples/clean-near-end/avatar-follow/` によって、
`FAIRY-01` / `FAIRY-02` / `FAIRY-03` / `FAIRY-04` / `FAIRY-06` を widened active representative slice として追えます。
`FAIRY-05` だけが residual planned family に残ります。
2026-04-27 の docs-first fixation では、`FAIRY-05` を planned family に留め、
visibility-return witness をどの carrier に載せるかを `UNRESOLVED` のまま残しました。
2026-04-28 の `P9` helper/test/docs hardening で、reopen 前に positive
reacquire-after-return sample、negative missing-return-witness または stale-membership companion、
explicit `state_timeline` / `anchor_switch` evidence、docs/report/snapshot sync が要ることを
helper closeout / dashboard / snapshot docs に actualize しました。
current helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` を返し、
この reopen gate を planning-only inventory として visible にします。

## 文書の読み分け

混同しやすいものは次のように分けて読みます。

- 規範 spec:
  `specs/`
- repository memory:
  `plan/`
- historical report:
  `docs/reports/`
- current sample:
  `samples/clean-near-end/`
- active base source corpus:
  `samples/current-l2/`
- active proof evidence:
  `samples/lean/`
- planned sample:
  `samples/not_implemented/`
- prototype / compatibility anchor:
  `samples/prototype/`
- historical old sample:
  `samples/old/`
- generated artifact reserve:
  `samples/generated/`
- helper-local debug output:
  script の `--debug` や detached artifact
- final public API:
  **まだ deferred**
- deferred mixed gate:
  parser/public API、auth/public contract、visualization/public contract、projection/public API、hot-plug/public API

## next package queue

`P0` current-state audit と `P1` repository layer map / `samples_progress.md` stabilization は close 済みです。
future-axis の first-cut / widening package は次まで close 済みです。

- `TermSignature registry / debug output`
- `LayerSignature system`
- `MessageEnvelope / AuthEvidence` seam
- `VisualizationProtocol`
- `Typed external boundary / adapter` docs-first sample plan
- `Typed external boundary executable widening`
- `Projection / placement` docs-first plan
- `HotPlug Patch / AttachPoint` docs-first plan
- `Network transport` docs-first plan
- `Compiler/backend/LLVM preparation` guardrail

現在の stabilized queue は次です。

`P2` Typed external boundary residual planned family review は close 済みであり、
`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate は current docs / helper closeout に固定されています。
`P3` Projection / placement residual emitted-program gate も close 済みであり、
projection validity report minimum、generated artifact reserve policy、actual emitted executable family は `P15` family 以降へ残す handoff line を current docs に固定しています。current `P15` first cut で actualize したのは committed generated bridge evidence only です。

1. `R3` `FAIRY-05` visibility-return carrier bundling
   - active representative slice を保ったまま residual planned family `FAIRY-05` の carrier choice を docs-first に narrow にする
2. `R2` closeout memory
   - `plan/30-attachpoint-detach-minimal-contract.md`、`attachpoint_detach_minimal_contract_01.md`、`../hands_on/attachpoint_detach_minimal_contract_01.md`
   - helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row と kept-later migration / rollback gate を current memory に残した
3. `R1` closeout memory
   - helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor と widening threshold matrix を current memory として残した
   - `plan/29-verification-layer-widening-threshold.md`、`verification_layer_widening_threshold_01.md`、`../hands_on/verification_layer_widening_threshold_01.md`
4. `U1` closeout memory
   - `plan/28` と reader-facing summary / landing page に packaging shape / installed binary target / host integration target / first shipped public surface / final shared-space operational catalog breadth の option inventory と provisional recommendation を残した
5. `P18` repo-side first cut memory
   - `plan/27-public-api-parser-gate-roadmap.md` と `public_api_parser_gate_plan_01.md` を current inventory の読み直し入口として維持する

`U1` の reader-facing 入口は `post_p18_true_user_spec_hold_option_matrix_01.md` と
`../hands_on/post_p18_true_user_spec_hold_01.md` です。

この queue は repo-local current reading であり、final public package structure や public API freeze を意味しません。

## stop line

この summary を読んでも、次を「すでに実装済み」とは読まないでください。

- real network transport
- multi-server consensus
- durable distributed commit
- final public auth protocol
- final public visualization API
- final public projection API
- final public hot-plug API
- final parser / checker / runtime / verifier API
- final product completion
