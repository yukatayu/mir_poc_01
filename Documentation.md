# ドキュメント要約

## リポジトリの目的

このリポジトリは、次のシステム群を中心とした**仕様先行の出発点**である。

- **Mir** — 意味論コア言語
- **Mirrorea** — 分散 fabric と制御プレーン（control plane）
- **PrismCascade** — メディアグラフ kernel
- **Typed-Effect Wiring Platform** — inspectable・routable・contract-aware な effect 層

## 現在の状態

- プロジェクトは**実装前段階 / アーキテクチャ重視段階**にある。
- 最も強い設計上の焦点は、意味論、境界、不変条件、統合点にある。
- いくつかの実装 skeleton は、将来の作業整理をしやすくするためだけに存在している。

## Decision level 要約

- **L0（基盤）**
  - 因果は event graph / directed acyclic graph で表現される。
  - effect と contract は first-class である。
  - ownership / lifetime は後付けではなく本質的である。
  - 安全な進化は運用上の付随物ではなく設計目標である。
- **L1（強い方向性）**
  - Mir、Mirrorea、PrismCascade、Typed-Effect Wiring Platform は分離しつつ相互運用可能に保つ。
  - downstream addition と compatibility-preserving overlay を優先する。
- **L2（設計提案）**
  - Prism と Mir の正確な境界詳細
  - fallback / preference chains の完全意味論
  - 一部の concurrency / coroutine 詳細
- **L3（探索段階）**
  - Reversed Library の知識分類戦略
  - GUI プログラミング基盤
  - 一部の高度な patching / visualization の論点

## 図

`docs/diagrams/` を参照。

```mermaid
flowchart TD
    L0[既存の OS / Network / Device Runtime]
    L1[Mir Core]
    L2[Mirrorea Fabric]
    L3[共有空間 / 共有状態]
    L4[ドメインエンジン / フレームワーク<br/>PrismCascade / VR / Collaboration]
    L5[アプリケーション / コミュニティ / Reversed Library]

    L0 --> L1 --> L2 --> L3 --> L4 --> L5
```

```mermaid
flowchart LR
    A[Mir Core]
    B[Mirrorea Fabric]
    C[PrismCascade]
    D[Typed-Effect Wiring Platform]

    A <--> B
    A <--> D
    B <--> D
    C <--> D
    C <--> A
```

## 次にどこから読むか

1. `specs/00-document-map.md`
2. 次に `specs/01-charter-and-decision-levels.md`
3. 次に `specs/02-system-overview.md`
4. 次に `specs/03-layer-model.md` と `specs/09-invariants-and-constraints.md`
5. その後、必要な subsystem に進む
6. representative code で current L2 の書き味を確認したいときは `specs/examples/00-representative-mir-programs.md`
7. その examples で使う `perform`、option chain 参照、`try` / `fallback`、`require` / `ensure` clause、separator / block nesting の候補書式は `specs/examples/01-current-l2-surface-syntax-candidates.md`
8. parser なしで representative examples を machine-readable に扱う最小 AST fixture schema は `specs/examples/02-current-l2-ast-fixture-schema.md`、fixture 実体は `crates/mir-ast/tests/fixtures/current-l2/`
9. parser なし最小 interpreter に必要な evaluation state schema は `specs/examples/03-current-l2-evaluation-state-schema.md`
10. parser なし最小 interpreter の 1-step semantics は `specs/examples/04-current-l2-step-semantics.md`
11. 既存判断は `specs/12-decision-register.md` を参照する

## レポート

すべての non-trivial work は、`docs/reports/` 配下に新しいファイルを生成しなければならない。

## 現在の環境メモ

この scaffold は、Python は利用できるが `cargo` は利用できない環境で作成された。
Rust workspace skeleton は存在するが、コンパイル可能性の検証は Rust が使えるマシンで別途行う必要がある。
