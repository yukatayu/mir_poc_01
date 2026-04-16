# 11 — ロードマップと Workstream

## 原則

このプロジェクトは、大規模な実装へ進む前に意味論を安定化することで前進すべきである。
意味の切り分けが曖昧なまま public surface や backend を凍らせてはならない。

ここでいう roadmap は strict waterfall ではない。
mainline actualization と docs-first research package が並走してよい。

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

## cross-cutting docs-first theory packages

上の workstream に加えて、current repo では **meaning first, implementation later** を守るための
cross-cutting theory package を docs-first に進めてよい。

### A/E/G に跨る理論線

1. **cut family / order family**
   - `atomic_cut` の local nucleus を維持したまま、
     observation / snapshot、ordering-only barrier、commit-bearing durable cut、
     publication / observation / witness / finalization relation を比較する。
2. **authority-handoff / shared-space order**
   - authority placement、provider placement、witness、fairness source、replay attachment を同軸化せずに比較する。
3. **thread / node parity**
   - source-level causal language の平等性と、
     lowering / transport / evidence / failure / durability policy の非対称性を切り分ける。
4. **syntax / semantics honesty**
   - compactness ではなく semantic honesty、checker legibility、modal adequacy、misreading resistance を主軸に syntax candidate を比較する。
5. **modal foundation / verifier boundary**
   - `lambda-circle-box` を partial basis candidate に留めつつ、
     guarded / modal dependent / multimodal line を stronger candidate として比較する。
   - property-to-boundary matrix は
     `core_static_checker` / `theorem_prover_boundary` /
     `protocol_verifier_boundary` / `runtime_policy_boundary`
     を維持して整理する。

### package discipline

- these theory packages are **comparison / adequacy / operating-model work**, not immediate public API work.
- final parser grammar、final public API、shared-space final catalog、backend success criteria、upper-layer app contract はここで固定しない。
- 必要なら tiny non-production prototype / simulator / compare helper を使ってよいが、mainline runtime semantics へ直結させない。

## 推奨される phase 順序

1. Workstream A
2. Workstream B
3. Workstream C
4. Workstream D
5. Workstream E
6. Workstream F
7. Workstream G

この順序は mainline の大勢であり、cross-cutting theory package を否定しない。
current repo では、Workstream A/E/G に跨る docs-first theory package を、
mainline actualization と separable に ratchet 方式で進めてよい。

## 仮の実装推奨（アーキテクチャ上の法則ではない）

- Core runtime、graph processing、tooling backend には Rust。
- Mir / Mirrorea / Prism component ごとに分離した native crate。
- Engine integration は adapter の背後に置く。
- 暗黙結合よりも、明示的 schema と version づけられた interface を優先する。
