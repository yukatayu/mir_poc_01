# 01 — 憲章と Decision Levels

## Charter

このプロジェクトの目的は、次を満たす一貫したスタックを設計することである。

1. 意味論コア言語（**Mir**）が、因果、effect、contract、ownership、lifetime、安全な進化を記述できること。
2. 分散 fabric（**Mirrorea**）が、その意味論の下でシステムを実行・進化できること。
3. media kernel（**PrismCascade**）が、高性能で plan-first、UI-independent な engine として独立性を保ちながら、より大きなシステムに統合できること。
4. typed-effect wiring platform が、container 化された、あるいはその他の opaque な software を routing layer でより inspectable・contract-aware・rewritable にできること。
5. 上位システム（virtual reality social spaces、synchronized web views、collaborative editors、knowledge spaces、Reversed Library）を、その下位層を application-specific assumption に潰さずに構築できること。

### Current operational axis

この repo で shorthand として使う current operational axis は次である。

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

これは Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform の separability を消す宣言ではない。
どの層をどこまで正本化するかは、引き続き decision level と subsystem boundary に従う。

## Non-goals

このプロジェクトは、現時点では次を目的としない。

- 既存の物理ネットワークや operating-system network stack の置き換え
- 最終製品の user interface 定義
- 単一の consensus algorithm へのコミット
- 単一の game engine へのコミット
- すべての subsystem を時期尚早に 1 つの runtime へ統合すること

## Decision levels

### L0 — 基盤

ここを変えると、スタック全体の意味が変わる。
例:

- Mir の四本柱
- 有向非循環グラフ（directed acyclic graph）の discipline
- 明示的な contracts / effects
- ownership / lifetime の monotonicity
- 安全な進化の原則

### L1 — 強い方向性

高い確信を持つアーキテクチャ上の方向性だが、構文や運用上の精密化の余地はある。
例:

- Mir、Mirrorea、PrismCascade、Typed-Effect Wiring Platform を分離しつつ相互運用可能に保つこと
- downstream addition を既定の進化規則とすること
- compatibility-preserving overlay

### L2 — 設計提案

有用な作業設計だが、今後の改訂余地がある。
例:

- overlay の正確な構文
- preference chain の正確な形
- effect-wiring layer の正確な実装分割

### L3 — 探索段階

重要ではあるが未確定の事項。
例:

- Reversed Library の最終的な知識分類戦略
- 最終的な graphical-programming substrate
- 完全な visual debugger model

## 未解決事項に関するルール

何かが未解決なら、文書はそれを明示しなければならない。
未解決とは重要でないことを意味せず、まだ決定が存在すると装わないとプロジェクトが選んでいる、という意味である。
