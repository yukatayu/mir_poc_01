# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform

この repository は、4 系統を分離可能なまま扱う **specification-first research repo** です。

- **Mir**
  因果、effect、ownership、lifetime、contract、安全な進化を扱う意味論コア
- **Mirrorea**
  logical naming、routing、overlay insertion、audit、dynamic reconfiguration を扱う fabric/runtime 層
- **PrismCascade**
  media domain の独立 kernel
- **Typed-Effect Wiring Platform**
  inspectable / routable な effect integration 層

repo が主として検証しているのは、Mir current-L2 の **repo-local alpha-ready current layer** です。
これは final public product ではありませんが、docs-only の構想メモでもありません。active sample、helper CLI、Lean foundations、report 群を通して、現時点でどこまで実装と検証が進んでいるかを repo 内で再確認できます。

## 現在の到達点

- active sample suite は `samples/clean-near-end/`
- runnable sample dashboard は `samples_progress.md`
- first strong typing layer は **finite decidable index fragment**
- authority hierarchy / security label hierarchy / capture / region / cost は **user-defined finite theory**
- order / handoff は `publication_order`、`witness_order`、`scoped_happens_before` などの高水準関係で扱う
- mutex / weak-memory / broken mutex は **model-check second line**
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/`
  と `scripts/sugoroku_world_samples.py` で repo-local に実行可能
- Lean 側は
  - `samples/lean/foundations/` の小さな実証明
  - `samples/lean/clean-near-end/` の generated theorem stub
  に分かれている

## 明示的にまだ完了していないもの

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public auth / adapter / visualization / projection / hot-plug / transport surface
- full dependent type theory
- concrete theorem prover / model-checker への production binding
- low-level `memory_order_*` を source principal syntax としてどう公開するか
- final public witness / provider / emitted-artifact contract
- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope
- final shared-space operational catalog breadth

## Mirrorea の次軸

Mirrorea future-axis は current promoted line ではなく、docs-first / repo-local integration の roadmap-memory family です。2026-05-05 時点では、その中に 3 つの line を分けて持ちます。

- **Mirrorea Spaces alpha-0 evidence line**
  規範判断は `specs/13..17`、repository memory は `plan/39..43`、phase-indexed sample scaffold は `samples/alpha/` に置きます。これは active runnable root の置換ではなく、current-scope evidence closeout を蓄積する alpha-local scaffolding です。
- **Mirrorea Spaces practical alpha-1 line**
  規範判断は `specs/18-practical-alpha1-scope.md`、repository memory は `plan/44-practical-alpha1-roadmap.md` に置きます。ここは source front-door、checker、runtime、package/hot-plug、transport、devtools、local save/load、product preview を揃える first-floor toolchain line であり、operational α-0.5 / α-0.8 readiness そのものではありません。
  ただし、これは promoted work queue であり、active canonical runnable root への昇格を意味しません。2026-05-05 時点の latest package closeout は `P-A1-23` practical α-1 integrated workflow carrier です。
  2026-05-05 時点では、`samples/practical-alpha1/` の limited `package.mir.json` fixtures と `crates/mir-ast::practical_alpha1` library loader、`crates/mir-ast::practical_alpha1_checker` と `scripts/practical_alpha1_check.py` による first practical checker floor、`crates/mir-ast::practical_alpha1_runtime_plan` と `crates/mir-runtime::practical_alpha1_local_runtime` / example `mir_practical_alpha1_run_local` / `scripts/practical_alpha1_run_local.py` による first practical local-runtime floor、`crates/mir-ast::practical_alpha1_hotplug_plan` と `crates/mir-runtime::practical_alpha1_hotplug` / example `mir_practical_alpha1_attach` / `scripts/practical_alpha1_attach.py` による non-final practical hot-plug floor、`crates/mir-runtime::practical_alpha1_avatar` / example `mir_practical_alpha1_avatar` / `scripts/practical_alpha1_avatar.py` による distinct avatar preview companion floor、`crates/mir-ast::practical_alpha1_transport_plan` と `crates/mir-runtime::practical_alpha1_transport` / example `mir_practical_alpha1_transport` / `scripts/practical_alpha1_transport.py` による current practical transport floor、`scripts/practical_alpha1_export_devtools.py` による widened practical devtools export floor、`crates/mir-ast::practical_alpha1_save_load_plan` と `crates/mir-runtime::practical_alpha1_save_load` / example `mir_practical_alpha1_save_load` / `scripts/practical_alpha1_save_load.py` による widened practical local save/load floor、`samples/practical-alpha1/previews/` と `scripts/practical_alpha1_product_preview.py` による widened practical product-preview floor、さらに `scripts/practical_alpha1_integrated_workflow.py` による bounded practical α-1 integrated workflow carrier が入っています。現在 actualize 済みの hot-plug row は `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`、`HP-A1-07`、avatar preview row は `AV-A1-01/02/03`、transport row は `TR-A1-01..07`、devtools row は `VIS-A1-01/02/03/04/05/06/07`、save/load row は `SL-A1-01/02/03`、preview row は `PE2E-01..09`、integrated workflow row は `PA1W-01..08` に限ります。avatar preview floor の carrier split は `checked package -> hotplug plan -> exact hot-plug report -> distinct avatar preview report` であり、`AV-A1-02` は `mir_humanoid_runtime_preview` を選ぶ non-final custom Mir avatar preview、`AV-A1-03` は source hot-plug report を missing host capability rejected のまま保って visible monotone placeholder fallback を示す companion preview に限られます。preview floor の carrier split は `preview manifest -> exact practical reports / exact practical devtools bundles -> non-final product-preview bundle` であり、`PE2E-04` は引き続き `HP-A1-06` placeholder object preview companion evidence に narrow され、`PE2E-06` は exact `SL-A1-03` save-load preflight reject report を consume する invalid distributed save rejected preview に限られ、`PE2E-08/09` は `AV-A1-02/03` exact avatar preview reports を thin companion bundles として consume します。`PA1W-01..08` は source front-door / checker / same-session runtime / typed host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 つの developer workflow として束ねますが、final public parser / viewer / telemetry ABI、distributed durable save/load、WAN/federation、product/public-ready α-1 を意味しません。`VIS-A1-03` は exact `SL-A1-02` save-load report から saved frontier、later live membership advance、restored frontier、stale-membership reject を保つ membership timeline export に限り、distributed durable membership timeline や witness/lease co-timeline completion を意味しません。`VIS-A1-04` は exact practical hotplug reports から attach accepted boundary、membership snapshot、deferred detach boundary を export する observable であり、alpha-0 Stage D lifecycle closeout や detach runtime lifecycle completion を意味しません。`VIS-A1-05` は exact `AV-A1-03` avatar preview report から rejected source lane、degraded roles、missing host capability を保つ fallback degradation export に限り、native execution や unsupported-runtime execution success を意味しません。`VIS-A1-07` は exact `SL-A1-02` save-load report に追加した report-local `retained_artifacts` catalog と `retention_scope = report_local_inventory` を source にして、hit/miss 付き on-demand retention query trace を devtools bundle へ下ろす narrow widening に限り、durable retained-artifact catalog service、cross-session or remote retrieval API、retention expiry/lease lifecycle completion を意味しません。save/load floor は runtime-backed branch と checker-backed preflight branch を分けて持ち、`SL-A1-01/02` は one exact practical local-runtime frontier と distinct save-load plan から saved local frontier と non-final save-load report を組み立てる local-only rows、`SL-A1-03` は exact rejected `CHK-CUT-01` checker report を distinct save-load preflight reject report へ写す invalid distributed-cut row に限られます。`CHK-CUT-01` reuse は orphan-receive guard reuse に留まり、full consistent-cut / `Z-cycle` / distributed durable save/load completion を意味しません。first practical avatar preview floor は native execution、final avatar ABI、same-session product runtime を意味せず、widened practical product-preview floor と bounded practical integrated workflow carrier も unsupported-runtime execution success、final product prototype completion、product-like CLI、final public runtime/devtools/transport/save-load/package-avatar API を意味しません。

- **Operational alpha theory-freeze / session-runtime line**
  規範判断は `specs/19..24`、repository memory は `plan/45..49` に置きます。ここでは runtime を広げずに、verification stratification、`atomic_cut` / consistent cut / save-load semantics、auth / rate-limit / debug の contract-transformer 理論、typed observability、typed external host boundary、そして α-0.5 / α-0.8 / α-0.9 の operational readiness 条件を固定します。
  `P-A1-19`、`P-A1-20`、`P-A1-21`、`P-A1-22` により、same-session α-0.5 session carrier、typed external `AddOne` direct execution lane、debug / auth / rate-limit / object preview / deferred detach の same-session attach lane、そして event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace の session-bound devtools export が actualize され、bounded operational α-0.5 / α-0.8 / α-0.9 line は揃いました。`P-A1-23` はその line と practical first floors を bounded practical α-1 workflow として束ねました。final public viewer / telemetry ABI、durable audit、distributed durable save/load、product-ready α-1 は引き続き未固定です。

現行の Stage A..F `100%` は `100% current-scope evidence closeout`、practical alpha-1 row の `100%` は `100% first-floor closeout` と明示して読みます。裸の `100%` は operational-layer-ready 以上にだけ使います。live queue authority と next reopen point は `progress.md` / `tasks.md` を参照してください。

current line で reader が押さえるべき点は次です。

- 境界条件:
  `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読みます。standard I/O は Mir core に入れず、external world とは typed effect adapter で接続します。
- security / observation split:
  authentication / authorization / membership / capability / witness は transport に潰さず、visualization / telemetry も `label` / `authority` / `redaction` / `retention` を持つ typed effect layer として扱います。
- current repo-local evidence surfaces:
  Sugoroku world runtime attachment vertical slice、avatar fairy follow representative slice、typed external `EXT-03/04` preview、network `NET-02..05` canary、projection/codegen manifest bridge evidence、viewer prototype inventory、hot-plug runtime narrow floor は runnable または closeout-backed です。加えて `samples/alpha/` には alpha-local theory-freeze / checker skeleton / runtime roadmap 用の sample matrix scaffold を置きますが、これは expected-verdict 付き skeleton であり、まだ runnable root ではありません。`FAIRY-05` は `samples/not_implemented/avatar-fairy-follow/` に residual planned family として残しています。
  projection/codegen current `equivalence` reading は committed generated manifest と helper/report-local anchor の review-category alignment inventory に限り、generated place-program synthesis、placement optimizer、deployment planner、cross-place equivalence checker、proof completion、final emitted executable family、final public emitted-program ABI ではありません。
- current runtime / carrier floor:
  `TermSignature` / `LayerSignature` / `MessageEnvelope` / `AuthEvidence` / helper `verification_handoff_witness` / runtime `verification_model_check`、`MembershipRegistry` / `PlaceCatalog` / `LogicalPlaceRuntimeShell`、engine-neutral `HotPlugRequest` / `HotPlugVerdict`、runtime-side hot-plug skeleton/engine reports までは repo-local actualization 済みです。
- kept-later boundaries:
  final public auth / adapter / visualization / projection / hot-plug / transport surface、real socket / session / durable replay、rollback / durable migration / distributed activation ordering、final public viewer API / telemetry service、exact host schema、installed binary / backend / packaging adoption は unresolved または deferred のままです。

詳細は次を使い分けてください。

- short current snapshot:
  `Documentation.md`
- live status / next reopen point:
  `progress.md`、`tasks.md`
- practical alpha-1 scope / roadmap:
  `specs/18-practical-alpha1-scope.md`、`plan/44-practical-alpha1-roadmap.md`
- hands-on closeout commands:
  `docs/hands_on/current_phase_closeout_01.md`
- reader-facing roadmap summary:
  `docs/research_abstract/mirrorea_future_axis_01.md`
- operational alpha theory freeze / roadmap:
  `specs/19-verification-stratification.md`、`specs/20-cut-save-load-semantics.md`、`specs/21-auth-layer-algebra.md`、`specs/22-observability-devtools-semantics.md`、`specs/23-typed-external-host-boundary.md`、`specs/24-operational-alpha05-alpha08-readiness.md`
  `plan/45-operational-alpha05-roadmap.md`、`plan/46-operational-alpha08-roadmap.md`、`plan/47-operational-alpha09-devtools-roadmap.md`、`plan/48-theory-freeze-proof-obligations.md`、`plan/49-host-io-and-session-runtime-roadmap.md`
- future-axis repository memory:
  `plan/28-post-p18-true-user-spec-hold-option-matrix.md` と `plan/29..49`

## 何が built-in で、何が user-defined か

current clean near-end layer では、次を built-in vocabulary として扱います。

- `module`
- `index`
- `policy`
- `principal`
- `resource`
- `effect`
- `place`
- `option`
- `chain`
- `fallback`
- `lineage`
- `perform`
- `via`
- `require`
- `ensure`
- `atomic_cut`
- `transition`
- `stage`
- `publish`
- `observe`
- `handoff`
- `witness`
- `model`
- `property`

一方で、次のような domain vocabulary は built-in ではありません。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `KeyMaterial`
- `Observer`
- `Releaser`
- `Admin`
- `RoomHistory`
- `EphemeralToken`

つまり、旧来の権限専用 predicate 名を magical builtin として言語が暗黙に持つのではなく、sample 側が有限理論として宣言し、その上で checker / helper が読む構成です。

## まず実行するコマンド

active clean near-end suite の確認:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

family ごとの確認:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
```

Sugoroku world runtime attachment vertical slice:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
```

sample code viewer:

```text
mir_hilight.html
```

`mir_hilight.html` は repo 直下の単一 HTML です。ブラウザで開くと
`samples/clean-near-end/**/*.mir` の current active sample を Solarized Dark
標準で表示し、行番号、スマホ対応、theme 切替、予約語と sample 内定義名の
別色 highlight を確認できます。custom source panel に任意の Mir 風コードを貼ると、
同じ highlighter で browser-local preview できます。CSS は外部 framework ではなく
HTML 内の hand-written original CSS です。final parser / checker ではなく readable viewer
です。文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わった場合は、
HTML 内の embedded samples / syntax token list / symbol extraction rule と docs
を同じ task で更新してください。

Lean foundations と generated stub の同期:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

practical alpha-1 front-door / checker / first local-runtime floor:

```bash
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture
cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
```

## 読み始める順序

この README の次は、原則として次の順で読みます。

1. `Documentation.md`
2. current status / roadmap / remaining steps を扱う task なら `progress.md` と `tasks.md`
3. phase recut / roadmap rewrite / progress/tasks reorganization を扱う task なら `.docs/progress-task-axes.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. 必要な subsystem spec と `plan/00-index.md`
10. practical alpha-1 を扱う task なら `specs/18-practical-alpha1-scope.md` と `plan/44-practical-alpha1-roadmap.md`
11. operational readiness / theory freeze を扱う task なら `specs/19..24` と `plan/45..49`

task が specific `sub-agent-pro/*.md` handoff を名指しした場合は、その handoff を user 指示順で先に読みます。
ただし handoff は規範正本ではなく、必要な内容は `specs/` / `plan/` / docs / report へ mirror して扱います。

## いま参照すべき docs

- `Documentation.md`
  現在の repo を短く読むための入口
- `progress.md`
  現在地、rough progress、recent log
- `tasks.md`
  自走可能な package と mixed gate / user-spec gate の整理
- `samples_progress.md`
  phase / layer ごとの runnable sample、E2E、debug surface、build / storage 環境の dashboard
- `samples/README.md`
  active / base corpus / planned / prototype / archive / generated sample の置き場所
- `scripts/README.md`
  active runner、repo-local helper、detached loop、storage/env script の current taxonomy
- `docs/research_abstract/README.md`
  日本語での短い研究概要と `_detail` への導線
- `docs/hands_on/README.md`
  実行コマンド付きの hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  current phase closeout memory、remaining mixed gate、snapshot docs への入口
- `docs/hands_on/visual_debugger_viewer_01.md`
  `P16` typed public prototype inventory の最短入口
- `docs/hands_on/network_transport_canaries_01.md`
  phase 13 helper-local canary の入口
- `docs/reports/`
  実行証跡と変更履歴

## active path と archive path

- active sample:
  `samples/clean-near-end/`
- active base current-L2 corpus:
  `samples/current-l2/`
- active Sugoroku world vertical slice:
  `samples/clean-near-end/sugoroku-world/`
- active Lean material:
  `samples/lean/`
- alpha-local scaffold root:
  `samples/alpha/`
- planned skeleton family:
  `samples/not_implemented/` (residual planned family)
- prototype / compatibility anchor:
  `samples/prototype/`
- historical archive:
  `samples/old/2026-04-22-pre-clean-near-end/`
  と
  `samples/lean/old/2026-04-22-pre-clean-near-end/`
- generated artifact reserve:
  `samples/generated/`

`samples/not_implemented/` は archive ではなく planned family です。
archive は比較用の履歴であり、active canonical sample としては扱いません。
generated artifact reserve と helper-local preview は source sample と混同しません。
