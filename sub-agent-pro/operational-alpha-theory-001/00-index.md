# operational-alpha-theory-001 — index

この handoff は、Mirrorea Spaces の α-0.5 / α-0.8 / α-0.9 を **artifact closeout ではなく operational readiness** として再定義し、そのために必要な理論 freeze / sample matrix / validation / docs update を Codex が新規コンテキストで実行できるようにするための情報パッケージである。

## 背景

現行 repo には、Mir current-L2、clean-near-end、Sugoroku、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer inventory、hot-plug narrow floor、さらに `samples/alpha/` と `samples/practical-alpha1/` に多数の first-floor evidence がある。

しかし、ユーザが期待する「実用できる α-0.5 / α-0.8」は、単なる sidecar / exact report / preview bundle / first-floor closeout ではない。開発者が動かし、観測し、状態推移を確認しながら実装を進められる **operational layer** が必要である。

この handoff は、その gap を理論と計画へ落とす。

## files

- `01-current-state-gap.md`
  - 現 repo の first-floor と operational readiness の gap
- `02-operational-readiness-definition.md`
  - evidence closeout / first-floor / operational-ready / product-ready の区別
- `03-decisions.md`
  - 今回固定する判断
- `04-verification-stratification.md`
  - 型システム、静的検証、model-check、proof side line
- `05-cut-save-load-theory.md`
  - `atomic_cut`、consistent cut、save/load、Z-cycle
- `06-auth-layer-theory.md`
  - auth / layer stack / contract transformer
- `07-observability-devtools-theory.md`
  - typed debug / observation / redaction / retention
- `08-typed-external-host-boundary.md`
  - host input/output、EchoText / AddOne、no stdio builtin
- `09-alpha05-roadmap.md`
  - α-0.5 local observable runtime の operational plan
- `10-alpha08-roadmap.md`
  - α-0.8 same-session hot-plug runtime の operational plan
- `11-alpha09-roadmap.md`
  - α-0.9 session-bound devtools の operational plan
- `12-sample-matrix.md`
  - required samples / negative tests / current evidence / missing rows
- `13-repository-change-plan.md`
  - 追加 specs / plan / docs / snapshot の構成案
- `14-validation-and-commands.md`
  - validation commands と report / commit protocol
- `15-subagent-plan.md`
  - reviewer / sub-agent 使用計画
- `16-codex-operational-rules.md`
  - Codex の行動規則と stop lines
- `17-risk-register.md`
  - 理論・実装・docs の risk register
- `18-done-criteria.md`
  - P-A1-18 closeout condition と次 reopen 条件

## package name

今回の推奨 package 名:

```text
P-A1-18 operational-alpha05-alpha08-theory-freeze
```

## core output

この package の core output は runtime 実装ではなく、以下の規範・計画文書である。

```text
specs/19-verification-stratification.md
specs/20-cut-save-load-semantics.md
specs/21-auth-layer-algebra.md
specs/22-observability-devtools-semantics.md
specs/23-typed-external-host-boundary.md
specs/24-operational-alpha05-alpha08-readiness.md
plan/45-operational-alpha05-roadmap.md
plan/46-operational-alpha08-roadmap.md
plan/47-operational-alpha09-devtools-roadmap.md
plan/48-theory-freeze-proof-obligations.md
plan/49-host-io-and-session-runtime-roadmap.md
```

## essential principle

今後、`100%` は実用可能性を意味する。current-scope evidence closeout や first-floor closeout は別 category として明示する。

