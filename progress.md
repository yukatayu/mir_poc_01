# progress

最終更新: 2026-04-21 01:36 JST

## この文書について

- この文書は repo 全体の **薄い進捗 snapshot** である。
- 規範判断の正本は `specs/`、長期参照は `plan/`、詳細経緯は `docs/reports/` に置く。
- 進捗率は **rough estimate** であり、current-L2 / fixed-subset / non-production / repo-local floor に scoped した読みである。
- 高い数字は final public completion を意味しない。

## current snapshot

- execution lane:
  `Macro 4 active on fixed authored/prototype floor`
  authored sixteen と corrected prototype set `p01 ... p16` は runnable
- theory-lab lane:
  `Macro 5 repo-local near-end`
  typed / theorem / model-check / order-handoff representative bundle と reserve summary index は揃っている
- reserve integration lane:
  `Macro 6 minimal working subset default / Macro 7 mixed`
  authoritative-room default と reserve package summary はあるが、final public contract と packaging は later

## practical reading

- **動くもの**
  - current-L2 sample runner / CLI
  - Problem 1 representative bundle
  - Problem 2 representative bundle
  - Lean foundation proof fragment
  - generated Lean stub corpus acceptance
- **まだ later のもの**
  - final public parser / checker / runtime API
  - full strong typed surface
  - concrete theorem/model-check production binding
  - final public verifier contract
  - low-level `memory_order` exact source surface
  - final witness/provider public contract

## 2026-04-21 に再確認した evidence

- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `problem1` passed
  - `problem2` passed
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p10...`
  - `terminal_outcome: success`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p11...`
  - `terminal_outcome: Reject`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p12...`
  - `terminal_outcome: Reject`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p15...`
  - `terminal_outcome: Reject`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p16...`
  - `terminal_outcome: Reject`
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - `p06 / p07 / p08` の Lean bundle JSON と `pilot-summary.md|json` を再生成
- `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
  - `p06 / p10 / p11 / p12 / p15 / p16` summary index を再生成
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p07...`
  - `terminal_outcome: success`
  - `surface_preview.status: reached`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p13...`
  - `static_gate: underdeclared`
  - `entered_evaluation: false`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p14...`
  - `static_gate: malformed`
  - `entered_evaluation: false`
- `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  - `p07 / p08` success、`p09` reserve、`p13 / p14` negative static-stop を同一 output dir に materialize
- `source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  - success
- `source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  - success
- `source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py`
  - foundation files success
  - current-l2 generated stubs success with `warning: declaration uses 'sorry'`
- `python3 -m unittest scripts/tests/test_current_l2_lean_sample_sync.py`
  - `Ran 9 tests ... OK`
- `docs/research_abstract/static_analysis_01.md`
  - Problem 1 の beginner guide を追加
- `docs/research_abstract/order_01.md`
  - Problem 2 の beginner guide を追加
- `docs/research_abstract/lean_01.md`
  - Lean foundation / generated stub / 入出力の読み方に絞った beginner guide を追加

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 重さ | 自走可否 |
|---|---|---:|---:|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 95% | 低 | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 84% | 中 | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 85% | 中 | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 81% | 中 | 着手可能 |
| `Macro 4` | executable fixed-subset sample floor | active on fixed floor | 94% | 重 | 着手可能 |
| `Macro 5` | typed / theorem / model-check / order-handoff | repo-local near-end | 96% | 重 | 自走可能（final public seam 以外） |
| `Macro 6` | shared-space minimal subset | default fixed + reserve split | 75% | 重 | 自走可能（catalog / final contract 以外） |
| `Macro 7` | toolchain / backend / host integration | mixed | 53% | 重 | repo-local まで |
| `Macro 8` | broader application realization | early | 18% | とても重い | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | stop line |
|---|---|---|---|
| current-L2 runner / CLI | `S6` | authored sixteen、prototype set `p01 ... p16`、pretty/json CLI、helper preview | final public runtime surface |
| strong typing / IFC | `S6` | `p10 / p11 / p12 / p15 / p16`、checker-adjacent summary、Lean foundation | stronger typed source principal / final calculus |
| theorem-side pilot | `S6` | `emit-theorem problem1`、Lean stub acceptance、pilot summary index | final public theorem contract / prover brand |
| model-check second line | `S6` repo-local | reserve summary index、rejection pair surfacing | settled property language / checker artifact |
| order / handoff | `S6` | `p07 / p08` representative pair、`surface_preview`、`emit-scenario problem2` | low-level `memory_order` exact surface / final wording |
| shared-space reserve | `S5-S6` | witness strengthening、delegated RNG reserve package | final witness/provider public contract |
| Lean foundations | `S6` | self-contained small proofs、reusable lemma groups、guide-aligned explanations | final mechanized public theory |
| backend / packaging | `S2-S3` | repo-local CLI / artifacts / helper loop | installed binary / FFI / engine adapter |

## layer / track progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手性 |
|---|---:|---:|---:|---|
| semantic kernel | 92% | 86% | 76% | 着手可能 |
| parser-free substrate | 89% | 79% | 87% | 着手可能 |
| compile-ready minimal actualization | 85% | 75% | 85% | 着手可能 |
| fixed-subset source sample line | 86% | 83% | 89% | 着手可能 |
| Problem 1 typed / theorem / model-check | 95% | 92% | 87% | final public seam 以外は自走可能 |
| Problem 2 order / handoff / room default | 89% | 88% | 73% | final public wording / contract 以外は自走可能 |
| Lean foundations / proof spine | 86% | 78% | 58% | repo-local mechanization までは自走可能 |
| backend / packaging | 37% | 32% | 31% | repo-local まで |

## twin peaks

### Problem 1 — 型システム / 定理証明 / モデル検査

- current first line:
  checker-adjacent first strong typing layer、notebook-first theorem line、row-local model-check carrier first
- 実行 evidence:
  `p06 / p10 / p11 / p12 / p15 / p16`、`emit-theorem problem1`、`emit-reserve model-check-second-line`、Lean foundation、generated stub acceptance
- まだ later:
  final public theorem result object、payload public contract、prover brand、settled property language、concrete model-check tool brand、final public checker artifact、final public verifier contract

### Problem 2 — order / handoff / authoritative-room

- current first line:
  relation decomposition principal、authoritative-room first default、reserve strengthening split、negative static-stop pair
- 実行 evidence:
  `p07 / p08 / p09 / p13 / p14`、`emit-scenario problem2`、`emit-reserve auditable-authority-witness`、`emit-reserve delegated-rng-service`
- まだ later:
  low-level `memory_order` exact source surface、final source wording、final emitted schema、final public witness/provider contract、exhaustive shared-space catalog

## next reading

- concise status: `Documentation.md`
- current task map: `tasks.md`
- long-lived status memory: `plan/01-status-at-a-glance.md`
- Problem 1 bundle: `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- Problem 2 bundle: `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## recent log

- 2026-04-21 00:48 JST — Problem 1 / Problem 2 representative bundle、typed rejection pair、order-handoff negative pair、Lean foundation、generated Lean stub corpus を再実行し、summary docs を `repo-local near-end` と `final public stop line` を分けた記述へ全面更新した。
- 2026-04-21 01:21 JST — Lean foundation に reusable lemma 群を追加して `python3 scripts/current_l2_lean_sample_sync.py` と unit test を通し、Problem 1 / Problem 2 の beginner 向け guide `docs/research_abstract/static_analysis_01.md` / `order_01.md` を追加した。
- 2026-04-21 01:36 JST — Lean beginner guide `docs/research_abstract/lean_01.md` を追加し、label-model / proof-skeleton の補題を少量補強したうえで Lean 実行・standalone success/error 例・unit test・docs validation を再確認した。
