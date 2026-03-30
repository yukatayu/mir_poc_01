# 11 — ロードマップと Workstream

## 原則

このプロジェクトは、大規模な実装へ進む前に意味論を安定化することで前進すべきである。

## 推奨 workstream

### Workstream A — Mir 仕様コア

目標:

- Mir-0 の最小形式意味論を確定する
- 現時点で合意されている primitive の正確な syntax と type rule を記述する
- cut、effect、contract、failure、monotone な ownership / lifetime behavior を固定する

### Workstream B — Mir runtime proof of concept

目標:

- single-process interpreter を構築する
- event graph extraction を支援する
- fallback / try / cut を支援する
- trace artifact を生成する

### Workstream C — Mirrorea の最小 fabric

目標:

- 論理名
- route rebinding
- overlay registration
- downstream patch activation
- 基本的な audit

### Workstream D — PrismCascade の最小 kernel

目標:

- Meta / Core / Runtime 分離
- graph normalization
- 最小 scheduler と memory plan
- 最小 trace output

### Workstream E — 共有統合面

目標:

- 共有 identifier
- 適切な範囲での共有 contract schema
- link された tracing strategy
- Mir と Prism の最小 bridge

### Workstream F — 可視化と editor support

目標:

- language server の基礎
- graph view
- cut / route / patch visualization
- report-driven workflow support

### Workstream G — アプリケーション実験

目標:

- 少なくとも 1 つの小さな synchronized shared-space 例
- 1 つの小さな virtual-world または collaborative editing 例
- 1 つの小さな route / overlay insertion 例

## 推奨される phase 順序

1. Workstream A
2. Workstream B
3. Workstream C
4. Workstream D
5. Workstream E
6. Workstream F
7. Workstream G

## 仮の実装推奨（アーキテクチャ上の法則ではない）

- Core runtime、graph processing、tooling backend には Rust。
- Mir / Mirrorea / Prism component ごとに分離した native crate。
- Engine integration は adapter の背後に置く。
- 暗黙結合よりも、明示的 schema と version づけられた interface を優先する。
