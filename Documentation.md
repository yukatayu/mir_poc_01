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
2026-04-27 時点の主眼は、そのうち Mir current-L2 の repo-local alpha-ready current layer にあります。

この layer で重要なのは、次の 2 つを混同しないことです。

- **repo-local alpha-ready current layer**
  repo 内の sample、helper、Lean foundation、report まで含めて動かせる現行の足場
- **final public product**
  final parser grammar、public checker/runtime/verifier API、packaging、external contract まで含む最終形

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

current sample floor の次に進む promoted line は、Mirrorea future-axis の整理と repo-local actualization です。

- `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読みます。
- Sugoroku sample に出てくる `world` は current sample surface の host/server-side sugar として読み、Mir core primitive だと固定しません。
- standard I/O は Mir core に入れず、external world とは typed external effect adapter で接続する方向です。
- Mirrorea / adapter / visualization / telemetry の内側は、effect-based OS-like substrate として読むこと自体はできます。
  ただしこれは内側の解釈に留め、standard I/O core built-in 化、Mir / Mirrorea / adapter 境界の collapse、transport-auth collapse の根拠にはしません。
- auth / membership / capability / witness / visualization / telemetry は transport や debug hack に潰さず、typed layer として合成する予定です。
- `VerificationLayer` composition は、現時点では helper `verification_handoff_witness` と clean near-end runtime `verification_model_check` を中核に、finite-index checker / theorem bridge / runtime policy / visualization lane へ later widening しうる typed layer family として読みます。
  hidden verifier builtin や final public verifier contract を既成事実化する意味ではありません。
- `TermSignature registry / debug output` の first cut は close してあり、Sugoroku helper の `--debug signatures` と clean near-end report / closeout inventory に helper-local / report-local signature carrier を追加しました。
- `P5` `LayerSignature` system hardening も close してあり、Sugoroku helper と clean near-end runtime の両方で `LayerSignature` row key を `name` に揃え、`obligations` lane と closeout `layer_signature_scope` / `layer_signature_lanes` を追加しました。
- `P6` `MessageEnvelope / AuthEvidence` seam hardening も close してあり、Sugoroku helper と clean near-end runtime の両方で `message_envelope_scope` を追加し、current carrier を `transport_medium / transport_seam / emitter_principal / freshness_checks / capability_requirements / authorization_checks / witness_refs` に widen しました。
- `P7` `VisualizationProtocol / VisualizationSecurity` hardening も close してあり、Sugoroku helper と clean near-end runtime の view / telemetry security envelope に `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を追加しました。NET-05 observer route trace は fail-closed にし、raw trace fallback と分離しています。
- performance telemetry も information-bearing effect として扱い、typed telemetry と label / authority / redaction / retention を security boundary の内側で保ちます。final public viewer contract、retention policy、multi-tenant telemetry service は未決のままです。
- `P8` Sugoroku runtime attach hardening も close してあり、Sugoroku helper closeout に `world_surface = host_server_side_sugar`、`membership_model.source_of_truth = MembershipRegistry`、`membership_model.late_join_handoff_boundary`、`hotplug_stop_line` を追加しました。attach / membership / handoff / late join / detach TODO boundary の representative runtime slice は current helper/test/docs hardening まで actualize しましたが、real network、consensus、durable distributed commit、rollback protocol、durable migration engine、final public runtime / hot-plug ABI は未決のままです。
- `P9` avatar fairy follow hardening も close してあり、avatar helper closeout に `planned_sample_paths` と `fairy05_reopen_gate = { sample_status = planned_only, required_evidence = [...], carrier_choice = UNRESOLVED, planning_only_candidate_labels = state_timeline / anchor_switch }` を追加しました。active representative slice `FAIRY-01/02/03/04/06` は維持したままですが、`FAIRY-05` 自体は planned のまま、public avatar / visualization API、real transport / session / auth semantics は未決のままです。
- current none-auth baseline では、helper active medium は `local_queue` / `loopback_socket`、runtime canonical seam は `provider_boundary` / `audit_trace_boundary`、auth evidence kind は `none`、membership freshness は `freshness_checks` + `membership_epoch` / `member_incarnation`、subject claim と emitter は `principal_claim` / `emitter_principal` に分けて保ちます。`session_token` / `signature`、final public transport ABI、`witness_refs` role taxonomy は未決のままです。
- `Typed external boundary / adapter` の docs-first sample plan も close してあり、phase 9 `EXT-01..05` planned family を `samples/not_implemented/typed-external-boundary/` に置きました。さらに `Typed external boundary executable widening` として `scripts/typed_external_boundary_samples.py` に `EXT-03` / `EXT-04` synthetic preview helper subset を追加し、`EXT-01` / `EXT-02` / `EXT-05` は residual planned family のまま残します。これは phase 9 `.mir` の direct semantic execution ではありません。
- `Projection / placement` の docs-first plan も close してあり、`plan/20-projection-and-placement-roadmap.md` に validity checklist と stop line を置きました。
- `HotPlug Patch / AttachPoint` の executable widening も current line に入り、Sugoroku helper の `hotplug_lifecycle` / `--debug hotplug` / `hot-plug` layer inventory で compatibility / activation / post-detach rejection evidence を helper-local に読めます。
- `Network transport` の docs-first plan も close してあり、`plan/22-network-transport-roadmap.md` に loopback / reconnect / failure matrix と stop line を置きました。さらに `NET-01` helper-local loopback preview に加えて、`scripts/network_transport_samples.py` の `NET-02..05` canary で process-boundary JSON bridge / reconnect epoch guard / typed transport failure / redacted route trace を helper-local evidence surface として確認できます。
- `Compiler/backend/LLVM preparation` guardrail も close してあり、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` に external workdir、`CARGO_HOME`、LLVM path、cleanup safety、stop line を置きました。
- `hands-on docs / closeout` も close し、avatar representative slice も active にし、hot-plug helper-local lifecycle canary、transport helper-local canary、projection / placement helper/report-local preview floor も actualize しました。phase 9 executable widening は `EXT-03` / `EXT-04` の thin synthetic preview helper cut まで actualize し、sample 自体は planned のままに保っています。さらに `P2` Typed external boundary residual planned family review、`P3` Projection / placement residual emitted-program gate、`P4` `TermSignature` registry hardening、`P5` `LayerSignature` system hardening、`P6` `MessageEnvelope / AuthEvidence` seam hardening、`P7` `VisualizationProtocol / VisualizationSecurity` hardening、`P8` Sugoroku runtime attach hardening、`P9` avatar fairy follow hardening も close し、`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate、projection validity report minimum、generated artifact reserve policy、current `signature_lanes = kind/name/evidence_role`、helper/runtime `signature_scope` distinction、`signature_evidence_roles`、reserved `message` / `adapter` / `layer` split、`LayerSignature` row schema、`MessageEnvelope` medium/seam split、shared `AuthEvidence` lane inventory、view / telemetry security envelope、fail-closed observer route trace、MembershipRegistry source-of-truth wording、world sugar boundary、hot-plug stop line、`FAIRY-05` reopen gate と planned path inventory を fixed しました。current next package は `P10` `mirrorea-core` first real implementation tranche で、その次が `P11` logical multi-place runtime tranche です。
- avatar fairy follow は `samples/clean-near-end/avatar-follow/` と `scripts/avatar_follow_samples.py` で active representative slice を持ちます。
  `samples/not_implemented/avatar-fairy-follow/` には `FAIRY-05` の residual planned family だけを残します。
  これを reopen する前に positive reacquire-after-return sample、negative missing-return-witness または stale-membership companion、explicit `state_timeline` / `anchor_switch` evidence、docs/report/snapshot sync が要ること、visibility-return witness をどの carrier へ載せるかは `UNRESOLVED` であること、helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` までしか actualize していないこと、という current line だけを固定しました。public avatar / visualization API を示唆するものではありません。

この line の reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md` に置きます。そこは roadmap summary であり、規範判断の正本ではありません。
current closeout を実行コマンド付きで読む最短入口は `docs/hands_on/current_phase_closeout_01.md` です。

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
  separable subsystem boundary の placeholder または future lane
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

## どこを見ると理解しやすいか

- `docs/research_abstract/README.md`
  日本語の簡潔な全体像
- `docs/hands_on/README.md`
  current runnable floor と remaining gate を短く辿る hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  2026-04-27 時点の current phase closeout guide
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
