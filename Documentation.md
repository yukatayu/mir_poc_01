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
2026-04-23 時点の主眼は、そのうち Mir current-L2 の repo-local alpha-ready current layer にあります。

この layer で重要なのは、次の 2 つを混同しないことです。

- **repo-local alpha-ready current layer**
  repo 内の sample、helper、Lean foundation、report まで含めて動かせる現行の足場
- **final public product**
  final parser grammar、public checker/runtime/verifier API、packaging、external contract まで含む最終形

現在 reachable なのは前者であり、後者ではありません。

## current active line

### 1. Clean near-end sample suite

active canonical sample は `samples/clean-near-end/` です。

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
- auth / membership / capability / witness / visualization / telemetry は transport や debug hack に潰さず、typed layer として合成する予定です。
- `TermSignature registry / debug output` の first cut は close してあり、Sugoroku helper の `--debug signatures` と clean near-end report / closeout inventory に helper-local / report-local signature carrier を追加しました。
- `LayerSignature system` の first cut も close してあり、Sugoroku helper の `--debug layers` と clean near-end report / closeout の `layer_signatures` inventory を追加しました。
- `MessageEnvelope / Auth seam` の first cut も close してあり、Sugoroku helper の `message_envelopes` / `--debug envelopes` と clean near-end report / closeout の `MessageEnvelope` inventory を追加しました。
- current none-auth baseline では、transport は local queue / provider boundary のまま、auth evidence は `none`、membership freshness は `membership_epoch` / `member_incarnation`、witness は `witness_refs` として separate lane に保ちます。
- 次の package は `VisualizationProtocol`、typed external boundary / adapter、projection / placement、hot-plug patch / `AttachPoint` です。
- avatar fairy follow は next representative slice 候補ですが、current active sample ではありません。
  現在は `samples/not_implemented/avatar-fairy-follow/` に planned skeleton family を置き、helper / validation / debug surface は still later としています。

この line の reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md` に置きます。そこは roadmap summary であり、規範判断の正本ではありません。

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
  phase 8 avatar fairy follow skeleton family と historical prototype の境界
- 各 `_detail.md`
  full sample code と actual output

## この文書で意図的に省いていること

この文書は current snapshot の入口なので、pre-clean-near-end の古い sample line や古い proposal chain の詳細はここでは再説明しません。必要な場合は archive、`plan/90-source-traceability.md`、`docs/reports/` を参照してください。
