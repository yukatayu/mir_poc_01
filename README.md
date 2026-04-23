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

2026-04-23 時点で repo が主として検証しているのは、Mir current-L2 の **repo-local alpha-ready current layer** です。
これは final public product ではありませんが、docs-only の構想メモでもありません。active sample、helper CLI、Lean foundations、report 群を通して、現時点でどこまで実装と検証が進んでいるかを repo 内で再確認できます。

## 現在の到達点

- active sample suite は `samples/clean-near-end/`
- first strong typing layer は **finite decidable index fragment**
- authority hierarchy / security label hierarchy / capture / region / cost は **user-defined finite theory**
- order / handoff は `publication_order`、`witness_order`、`scoped_happens_before` などの高水準関係で扱う
- mutex / weak-memory / broken mutex は **model-check second line**
- Lean 側は
  - `samples/lean/foundations/` の小さな実証明
  - `samples/lean/clean-near-end/` の generated theorem stub
  に分かれている

## 明示的にまだ完了していないもの

- final public parser grammar
- final public parser / checker / runtime / verifier API
- full dependent type theory
- concrete theorem prover / model-checker への production binding
- low-level `memory_order_*` を source principal syntax としてどう公開するか
- final public witness / provider / emitted-artifact contract
- packaging / installed binary / FFI / engine adapter

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

Lean foundations と generated stub の同期:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## 読み始める順序

1. `AGENTS.md`
2. `Documentation.md`
3. `progress.md`
4. `tasks.md`
5. `specs/00-document-map.md`
6. `specs/01-charter-and-decision-levels.md`
7. `specs/02-system-overview.md`
8. `specs/03-layer-model.md`
9. `specs/09-invariants-and-constraints.md`
10. 必要な subsystem spec と `plan/00-index.md`

## いま参照すべき docs

- `Documentation.md`
  現在の repo を短く読むための入口
- `progress.md`
  現在地、rough progress、recent log
- `tasks.md`
  自走可能な package と mixed gate / user-spec gate の整理
- `docs/research_abstract/README.md`
  日本語での短い研究概要と `_detail` への導線
- `docs/reports/`
  実行証跡と変更履歴

## active path と archive path

- active sample:
  `samples/clean-near-end/`
- active Lean material:
  `samples/lean/`
- historical archive:
  `samples/old/2026-04-22-pre-clean-near-end/`
  と
  `samples/lean/old/2026-04-22-pre-clean-near-end/`

archive は比較用の履歴です。active canonical sample としては扱いません。
