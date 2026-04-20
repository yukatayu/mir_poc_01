# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform

このリポジトリは、4 つの関連システムを **specification-first** に整理しながら、
current-L2 の非 production 実装と検証サンプルを積み上げている研究用ワークスペースである。

- **Mir**: 因果、contract、effect、ownership、lifetime、安全な進化を扱う意味論コア
- **Mirrorea**: 論理名、routing、overlay insertion、audit を扱う fabric / runtime 層
- **PrismCascade**: media processing 用の独立 kernel
- **Typed-Effect Wiring Platform**: inspectable / routable な effect integration 層

この 4 つは密接に関係するが、repo では **意図的に separable** に扱う。

## 現在の立ち位置

この repo は **完成済み製品** でも **final public language implementation** でもない。
一方で、単なる docs-only skeleton でもない。

2026-04-21 時点の実務的な読みは次のとおり。

- current-L2 authored sixteen と corrected prototype set `p01 ... p16` は runnable
- Problem 1 の current first line
  - typed / IFC sample
  - theorem-first emitted artifact loop
  - model-check second-line reserve summary
  は repo-local helper / CLI / Lean foundation 付きで確認できる
- Problem 2 の current first line
  - order / handoff / authoritative-room representative pair
  - reserve strengthening lane
  - negative static-stop pair
  は runnable scenario loop まで確認できる
- Lean は 2 層ある
  - `samples/lean/foundations/`: actual small proof fragment
  - `samples/lean/current-l2/`: Lean が受理する generated stub corpus

ただし、次は **まだ done ではない**。

- final public parser / checker / runtime API
- full strong typed surface
- concrete theorem prover binding
- concrete model-check tool binding
- final public verifier contract
- low-level `memory_order` exact source surface
- final source wording / witness-provider public contract
- packaging / installed binary / FFI / engine adapter

## 先に読む順序

1. `AGENTS.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. 必要に応じて subsystem spec
10. current task map は `tasks.md`
11. long-lived repository memory は `plan/00-index.md` と `plan/01-status-at-a-glance.md`

## まず確かめるコマンド

全体の representative bundle を一度に見る:

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

Problem 1 の typed / theorem / model-check current cut を見る:

```bash
python3 scripts/current_l2_guided_samples.py bundle problem1
python3 scripts/current_l2_guided_samples.py emit-theorem problem1
python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line
```

Problem 2 の order / handoff current cut を見る:

```bash
python3 scripts/current_l2_guided_samples.py bundle problem2
python3 scripts/current_l2_guided_samples.py emit-scenario problem2
```

Lean foundation を見る:

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

## 作業ルール

- non-trivial task ごとに `docs/reports/` 配下へ **新しい report** を作る
- 規範判断の正本は `specs/`
- current snapshot は `Documentation.md` / `progress.md` / `tasks.md`
- `plan/` は snapshot ではなく repository memory

report 作成:

```bash
python3 scripts/new_report.py --slug <short-name>
```
