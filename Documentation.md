# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 現在の進捗 snapshot は `progress.md`
- 現在の task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
- 実行証跡と詳細経緯は `docs/reports/`

## まず repo をどう読むべきか

この repo は、Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を **意図的に separable** に保った研究用 workspace です。
主眼は、そのうち Mir current-L2 の repo-local alpha-ready current layer にあります。

この layer で重要なのは、次の 2 つを混同しないことです。

- **repo-local alpha-ready current layer**
  repo 内の sample、helper、Lean foundation、report まで含めて動かせる現行の足場
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 reachable なのは前者であり、後者ではありません。

## current active line

### 1. Clean near-end sample suite

active canonical sample は `samples/clean-near-end/` です。
base current-L2 corpus は `samples/current-l2/`、active Lean mechanization evidence は `samples/lean/` に置きます。

- `typing/`
  finite-index first strong typing layer
- `order-handoff/`
  publication / witness / handoff relation family
- `model-check/`
  Peterson / broken mutex による second-line verification
- `modal/`
  `stable` / `later` / `published(room)` / `witnessed(...)` の current mode line
- `sugoroku-world/`
  empty world server へ SugorokuGame を runtime attach する Mir / Mirrorea vertical slice。
  これは separate helper `scripts/sugoroku_world_samples.py` で実行する repo-local logical multi-place emulator です。

旧 active sample line は active path から外し、archive に退避しています。

### 1.1 Mirrorea future integration line

Mirrorea future-axis は current sample floor の次の promoted line ではなく、整理済みの docs-first / repo-local actualization roadmap-memory family です。2026-05-03 時点では、その中に **Mirrorea Spaces alpha-0 theory-freeze lane** を追加しており、`specs/13..17` が alpha-local 規範判断、`plan/39..43` が roadmap memory、`samples/alpha/` が mixed scaffold / non-public runner root を担います。これは active runnable root の昇格宣言ではありません。current line ではさらに `scripts/alpha_lifetime_fallback_checker.py`、`scripts/alpha_contract_variance_checker.py`、`scripts/alpha_cut_save_load_checker.py` が selected sidecar-declared negative reason rows（現時点では `LIF-01/05..08`、`VAR-02/03/05/07/09/10/15`、`CUT-05/07/08/09/11/13/14/15`）を synthetic detached artifacts と照合する non-public checker floor を担い、artifact 側の `reason_codes_scope` が family floor と一致する場合だけ matched と扱います。加えて `scripts/alpha_lifetime_fallback_acceptance.py` と `scripts/alpha_contract_variance_acceptance.py` が `LIF-02/03/04` と `VAR-01/04/06` の selected positive rows を helper-local synthetic acceptance floor として actualize し、artifact 側の `acceptance_scope` が `alpha-acceptance-floor` と一致し、explicit acceptance rows が sidecar の `expected_acceptance.checked_acceptance_rows` と一致する場合だけ matched と扱います。さらに `scripts/alpha_lifetime_fallback_snapshot.py` が `LIF-13` を selected snapshot floor として actualize し、artifact 側の `snapshot_scope = alpha-snapshot-selected-floor` が一致し、selected option / appended fallback / excluded options を含む explicit snapshot row が sidecar の `expected_snapshot.checked_snapshot_rows` と一致する場合だけ matched と扱います。`scripts/alpha_lifetime_fallback_anchor_handoff.py` は `LIF-11` を selected anchor-handoff floor として actualize し、artifact 側の `anchor_handoff_scope = alpha-anchor-handoff-floor` が一致し、deleted object lifetime extension 不在 / hidden object reference retained 不在 / explicit later degradation order を含む anchor-handoff row が sidecar の `expected_anchor_handoff.checked_anchor_handoff_rows` と一致する場合だけ matched と扱います。`scripts/alpha_contract_variance_runtime_mirror.py` は `VAR-08/11/13` を selected runtime-mirror floor として actualize し、target sidecar の `runtime_mirror.scope = alpha-runtime-mirror-floor` と source runtime-floor sidecar `LI-04/01/03` の `status` / `claims` / `current_validation.mode` / `mirrors` / `expected_runtime` が一致する場合だけ matched と扱います。`reason_codes_scope` confinement、`acceptance_scope` confinement、`snapshot_scope` confinement、`anchor_handoff_scope` confinement、`runtime_mirror.scope` confinement は別であり、positive row の証拠は negative reason code の不在ではありません。`snapshot_selected` は `inherit_chain` と `plain ref` とも別であり、selected option capture と non-selected option exclusion を explicit synthetic row で示す必要があります。`anchor_handoff` floor は deleted object lifetime extension を意味せず、Sparkle が explicit inherited chain evidence を持つことだけを意味します。`runtime_mirror` は `contract-variance/` sample 自体の parser/runtime execution を意味せず、`snapshot` floor も parser/runtime execution や runtime object deletion behavior を意味しません。`anchor_handoff` floor も runtime deletion behavior、`FAIRY-05` completion、parser/runtime execution を意味しません。`P-A0-22` は actualized floor を増やさず、`LIF-15` を planned-only `remote_observe_scope = alpha-remote-observe-floor`、`VAR-14` を planned-only `adapter_transform_scope = alpha-adapter-transform-floor` の blocker split として固定しました。これら future carriers は current five-floor actualization に含まれず、parser/runtime bridge、runnable root promotion、final public checker/API completion を意味しません。`crates/mir-runtime/src/alpha_local_runtime.rs` と example `mirrorea_alpha_local_runtime` が `samples/alpha/local-runtime/` の `LR-01/02` に対する non-public in-memory Rust local-runtime floor を actualize し、`scripts/alpha_local_runtime_samples.py` がその dedicated sample-ID keyed runner と current-scope Stage B closeout surface を提供します。current-scope Stage B closeout は `LR-01/02` に加えて `CUT-04` local-only save/load positive bridge と `CUT-17` local savepoint stale-membership rejection bridge を **supporting subset としてだけ**再利用しており、CUT family 全体や distributed save/load completion を意味しません。`scripts/alpha_cut_save_load_samples.py` は引き続き `CUT-04/17` dedicated validation floor と `CUT-11` checker-backed structural negative row を担い、`CUT-10/12/16` は lease/witness store や communication-induced checkpoint repair を伴うため still later です。`crates/mir-runtime/src/alpha_layer_insertion_runtime.rs` と example `mirrorea_alpha_layer_insertion_runtime` は `samples/alpha/layer-insertion/` の `LI-01..05` に対する non-public Rust layer-insertion floor を actualize し、`crates/mir-runtime/src/alpha_network_runtime.rs` と example `mirrorea_alpha_network_runtime`、thin runner `scripts/alpha_network_docker_e2e.py`、`samples/alpha/network-docker/docker-compose.alpha-net.yml` は `samples/alpha/network-docker/` の `NET-02/03/04/05/07/09` に対する first non-public Rust Stage-C network / Docker floor を actualize し、`crates/mir-runtime/src/alpha_avatar_runtime.rs`、example `mirrorea_alpha_avatar_runtime`、thin runner `scripts/alpha_avatar_runtime_samples.py` は `samples/alpha/avatar-runtime/` の `AV-01/02/06/08/09` と `samples/alpha/hotplug-runtime/` の `HP-11/12/15` に対する first non-public runtime-private package/avatar manifest-admission floor を actualize し、`scripts/alpha_visualization_samples.py` は `samples/alpha/visualization/` の `VIS-01/02/03/05/06/07/08/10/11` を dedicated Stage-E subset runner として actualize し、`stage-e-closeout` で current-scope Stage E closeout surface を与え、さらに `scripts/alpha_e2e_samples.py` が `samples/alpha/e2e/` の `E2E-01/02/03/04/05/06/07/09/10` を thin integrated Stage-F bridge として actualize しています。local-runtime cut は queue / `MessageEnvelope` dispatch / membership freshness / event DAG export hook に限り、local save/load cut は room-local runtime savepoint / restored runtime snapshot / saved owner marker / visible-history frontier / resumed dispatch / later membership-frontier advance rejection に限り、layer-insertion cut は attach-time contract comparison / authority-gated debug attach / declared-failure rate-limit preview / explicit contract-update auth path / incompatible patch reject に限り、network cut は TCP process boundary / Docker Compose bridge・explicit membership/capability/witness/auth admission・observer-safe route trace・auth lane preservation に限り、avatar/package cut は placeholder/custom avatar acceptance・visible fallback・undeclared effect reject・unsigned/over-capability/revoked native package reject に限り、visualization cut は event DAG / place-catalog projection / route trace / membership epoch-incarnation timeline / hot-plug lifecycle bundle / fallback degradation / observer-redacted route trace / on-demand trace gate / retention-floor enforcement に限り、integrated E2E bridge は those existing floors の composition + checker-backed distributed non-claim に限ります。`VIS-04/09/12` は still later planned families ですが、current-scope Stage E closeout 自体は implemented subset で固定されています。Stage F completion 自体は still later です。いずれも alpha parser/runtime execution、completed hot-plug lifecycle、native binary execution、full VRM / VRChat / Unity compatibility、distributed save/load completion、final public avatar/package ABI、final public viewer/telemetry API、production WAN/session/replay、final public transport/hot-plug ABI ではありません。helper-local acceptance floor、snapshot floor、anchor-handoff floor、runtime-mirror floor も parser/runtime implementation や final public checker API ではありません。live queue authority と next reopen point は `progress.md` / `tasks.md` を参照してください。

`P-A0-24` では、既存の Stage-C network floor に新しい transport semantics を追加せず、`scripts/alpha_network_docker_e2e.py stage-c-closeout` を current-scope closeout surface として追加する。admissible rows は `NET-02/03/04/05/07/09` に限り、これは Docker/local-subprocess floor の alpha-0.7 transport closeout であって、`NET-06/08/10`、production WAN/session/replay、network partition completion、transport-medium substitution completion、final public transport ABI を意味しない。

`P-A0-25` では、existing Stage-D layer/package/avatar floors に新しい hot-plug semantics を追加せず、`scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout` を current-scope closeout surface として追加する。admissible rows は `LI-01/02/03/04/05`、`AV-01/02/06/08/09`、`HP-11/12/15` に限り、これは alpha-0.8 hot-plug lifecycle closeout であって、detach runtime、durable migration、distributed activation ordering、native execution realization、`HP-08/09/13/14`、`AV-03/04/05/07/10`、final public layer/package/avatar ABI を意味しない。

`P-A0-26` では、existing Stage-E visualization subset に新しい devtools semantics を追加せず、`scripts/alpha_visualization_samples.py stage-e-closeout` を current-scope closeout surface として追加する。admissible rows は `VIS-01/02/03/05/06/07/08/10/11` に限り、これは alpha-0.9 devtools closeout であって、`VIS-04/09/12`、final public viewer API、final public telemetry service、Stage F completion を意味しない。

- conceptual boundary:
  `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読みます。Sugoroku sample の `world` は host/server-side sugar であり、Mir core primitive にはしません。
- external / security boundary:
  standard I/O は Mir core に入れず、external world とは typed external effect adapter で接続します。auth / membership / capability / witness / visualization / telemetry は transport や debug hack に潰さず、typed layer として合成します。
- current repo-local actualized families:
  `TermSignature` / `LayerSignature` / `MessageEnvelope` / `AuthEvidence` / helper `verification_handoff_witness` / runtime `verification_model_check`、`MembershipRegistry` / `PlaceCatalog` / `LogicalPlaceRuntimeShell`、engine-neutral `HotPlugRequest` / `HotPlugVerdict`、runtime-side hot-plug skeleton/engine reports、typed external `EXT-03/04`、helper-local network canary `NET-02..05`、Alpha-0 Stage-C network floor `NET-02/03/04/05/07/09`、projection/codegen manifest bridge evidence、viewer prototype inventory は current runnable または closeout-backed surface です。
  projection/codegen current `equivalence` reading は committed generated manifest と helper/report-local anchor の review-category alignment inventory に限り、generated place-program synthesis、placement optimizer、deployment planner、cross-place equivalence checker、proof completion、final emitted executable family、final public emitted-program ABI ではありません。
- representative slices and kept-later boundaries:
  Sugoroku world runtime attachment と avatar fairy follow representative slice は active ですが、`FAIRY-05` は planned のままです。real socket / session / durable replay、rollback / durable migration / distributed activation ordering、exact host schema、final public auth / adapter / visualization / projection / hot-plug / transport surface、final public viewer API / telemetry service、installed binary / backend / packaging adoption は未決または deferred のままです。
- where to read details:
  short snapshot はこの `Documentation.md`、hands-on closeout command set は `docs/hands_on/current_phase_closeout_01.md`、reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md`、future-axis repository memory は `plan/28-post-p18-true-user-spec-hold-option-matrix.md` と `plan/29..43` を参照してください。

### 1.2 Sample highlighter

repo 直下の `mir_hilight.html` は、current active `.mir` sample を読むための単一 HTML viewer です。
ブラウザだけで動き、外部 asset は読みません。標準 theme は Solarized Dark で、VS Code Dark、GitHub Light / Dark、Dracula、Monokai などへ切り替えられます。
行番号、スマホ対応、予約語 highlight、sample 内で宣言された user-defined symbol の別色 highlight、browser-local custom source input を備えています。
CSS は外部 framework ではなく、HTML 内に直接書いた hand-written original CSS です。

これは final public parser / checker / verifier ではありません。
文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わった場合は、`mir_hilight.html` の embedded samples、syntax token list、symbol extraction rule、関連 docs / tests を同じ task で更新してください。

### 1.3 Samples progress and storage foundation

`samples_progress.md` は、phase / layer ごとの runnable sample、unit validation、E2E、debug / visualization、blocker、report、build / storage 環境を一覧する dashboard です。

- progress% は runnable sample と validation に紐づけて更新します。
- `100%` は implementation、positive/negative sample、debug/visualization、docs、report、tests、git commit/push まで揃った current scope に限ります。
- build / storage では root disk を既成事実化せず、heavy disposable artifact は external workdir を優先します。

### 1.4 Repository structure reading

repo の current filesystem は flat な部分を残していますが、責務は分けて読みます。

- `crates/mir-ast`:
  parser / AST carrier
- `crates/mir-semantics`:
  semantics / proof / model-check bridge
- `crates/mir-runtime`:
  current runner / CLI / report-local evidence carrier
- `crates/mirrorea-*`, `crates/prism-*`, `crates/engine-abi`, `crates/mir-lsp`:
  separable subsystem boundary の current minimal carrier lane、placeholder、または future lane
- `samples/README.md`:
  active / base corpus / planned / prototype / archive / generated sample taxonomy
- `scripts/README.md`:
  active runner、repo-local helper、detached loop、storage/env script の taxonomy
- `plan/19-repository-map-and-taxonomy.md`:
  staged migration plan と “いま動かさないもの” の repository memory

### 2. first strong typing layer

current typing judgment の読みは、概ね次の形です。

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

- `Σ`
  user-defined index theory、policy、有限 preorder / lattice / powerset / bound
- `Ψ`
  mode / stage / place / visibility / witness / durability の環境
- `Γ`
  unrestricted context
- `Δ`
  linear / affine / capability context
- `A`
  ordinary type
- `μ`
  mode
- `ε`
  effect row
- `C`
  finite decidable constraint
- `O`
  first decidable fragment の外側へ残す residual obligation

ここで強調すべき点は、domain predicate を magical builtin にしていないことです。

- authority hierarchy は user-defined finite preorder
- security label hierarchy は user-defined finite lattice
- capture は finite capture set
- lifetime / region は finite preorder
- cost は simple decidable bound

### 3. order / handoff と `memory_order` 再解釈

current active source line は、低レベル `memory_order_release` などを source principal に据えていません。
代わりに次の高水準関係を principal にしています。

- `program_order`
- `dependency_order`
- `publication_order`
- `observation_order`
- `witness_order`
- `finalization_order`
- `scoped_happens_before`

`atomic_cut` も global mutex や global fence ではなく、**local finalization / rollback frontier** として扱います。

### 4. model-check second line

mutex / weak-memory family は static typing first line に押し込めず、**model-check second line** として分離しています。

- `01_peterson_sc_pass`
  sequential consistency では mutual exclusion が保たれる
- `02_peterson_relaxed_counterexample`
  publication / observation edge が欠けると counterexample が出る
- `03_broken_mutex_counterexample`
  古典的 broken mutex は interleaving / visibility の問題として second line で捉える

### 5. Lean

Lean material は 2 段です。

- `samples/lean/foundations/`
  小さな actual proof fragment
- `samples/lean/clean-near-end/`
  active sample suite から生成した theorem stub 群

重要なのは、generated stub は **proof completion の完了宣言ではない** ことです。
repo は Lean を current layer の mechanization spine として使っていますが、全 sample の domain proof が discharge 済みとは言っていません。

## built-in と user-defined の境界

current helper / sample line で built-in vocabulary に入るのは、repo が closeout で明示している最小の構文語です。

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

それ以外の security label 名、authority 名、capture capability 名、region 名、cost counter 名、witness field 名は user-defined です。

## 現在すぐ動く確認コマンド

suite 全体:

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

family ごと:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
```

Sugoroku world vertical slice:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
```

Lean:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

docs:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
```

Alpha-0 avatar/package floor:

```bash
cargo test -p mir-runtime --test alpha_avatar_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
```

Alpha-0 cut/save-load floor:

```bash
cargo test -p mirrorea-core --test runtime_substrate
cargo test -p mir-runtime --test alpha_local_runtime
cargo test -p mir-runtime --test alpha_cut_save_load_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
```

Alpha-0 visualization/devtools floor:

```bash
python3 scripts/alpha_visualization_samples.py check-all --format json
python3 scripts/alpha_visualization_samples.py closeout --format json
python3 -m unittest scripts.tests.test_alpha_visualization_samples
```

Alpha-0 integrated E2E bridge:

```bash
python3 scripts/alpha_e2e_samples.py run E2E-06 --format json
python3 scripts/alpha_e2e_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py closeout --format json
```

## どこを見ると理解しやすいか

- `docs/research_abstract/README.md`
  日本語の簡潔な全体像
- `docs/hands_on/README.md`
  current runnable floor と remaining gate を短く辿る hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  current phase closeout guide
- `samples_progress.md`
  phase / layer ごとの runnable sample、E2E、debug surface、build / storage 環境の dashboard
- `samples/README.md`
  active / base corpus / planned / prototype / archive / generated sample の taxonomy
- `scripts/README.md`
  active runner / helper / detached loop / storage/env / tests の taxonomy
- `docs/research_abstract/clean_near_end_typing_01.md`
  finite-index typing の要点
- `docs/research_abstract/clean_near_end_order_model_01.md`
  order / handoff と model-check second line の関係
- `docs/research_abstract/clean_near_end_modal_01.md`
  modal / stage / witnessed bridge の current reading
- `docs/research_abstract/clean_near_end_lean_01.md`
  Lean foundations と generated stub の current reading
- `docs/research_abstract/hands_on_typing.md`
  clean near-end typing をコマンド実行、sample code、output から読む初心者向け手順
- `docs/research_abstract/hands_on_order_model.md`
  publication / witness / handoff と `memory_order` 再解釈を手で確認する手順
- `docs/research_abstract/hands_on_model_checking.md`
  Peterson / relaxed memory / broken mutex を model-check second line として確認する手順
- `docs/research_abstract/hands_on_modal.md`
  `stable` / `later` / `published(room)` / `witnessed(...)` を stage ごとに読む手順
- `docs/research_abstract/hands_on_lean.md`
  Lean foundation proof と generated theorem stub の違いを確認する手順
- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
  Sugoroku world runtime attachment vertical slice の初心者向け入口
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
  Sugoroku 10 sample が phase 4 / 7 / 14 のどの証拠なのかを短く追うための matrix
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
  phase 8 active representative slice / residual planned family / historical prototype の境界
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
  phase 8 current runnable slice の最短入口
- 各 `_detail.md`
  full sample code と actual output

## この文書で意図的に省いていること

この文書は current snapshot の入口なので、pre-clean-near-end の古い sample line や古い proposal chain の詳細はここでは再説明しません。必要な場合は archive、`plan/90-source-traceability.md`、`docs/reports/` を参照してください。
