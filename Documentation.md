# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 現在の進捗 snapshot は `progress.md`
- 現在の task map は `tasks.md`
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

旧 active sample line は active path から外し、archive に退避しています。

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

Lean:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

docs:

```bash
python3 scripts/validate_docs.py
```

## どこを見ると理解しやすいか

- `docs/research_abstract/README.md`
  日本語の簡潔な全体像
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
- 各 `_detail.md`
  full sample code と actual output

## この文書で意図的に省いていること

この文書は current snapshot の入口なので、pre-clean-near-end の古い sample line や古い proposal chain の詳細はここでは再説明しません。必要な場合は archive、`plan/90-source-traceability.md`、`docs/reports/` を参照してください。
