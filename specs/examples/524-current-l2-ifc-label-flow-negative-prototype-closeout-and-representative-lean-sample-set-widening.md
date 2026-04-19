# 524 — current L2 IFC label-flow negative prototype closeout and representative Lean sample-set widening

## 目的

`specs/examples/522` と `523` により、

- Lean-side IFC first fragment
- source-side explicit authority success/failure pair

は source-backed に揃った。
この文書は、その次段として
**authority 欠如とは別の source-side label-flow negative を `p12` に actualize し、
Package 56 を actual adoption package として close する current cut**
を整理する。

これは final typed source principal や final IFC syntax や final public verifier contract の adoption ではない。
checker-adjacent principal を保ったまま、
source-side IFC trio と representative Lean sample set widening を current package floor として固定する。

## 1. current question

Layered strong typing / IFC first-fragment を

- authority があると release できる
- authority がないと止まる

だけでなく、

- authority はあっても classified payload を public board へそのまま流そうとすると止まる

まで source-side corrected prototype へ押し込んでよいか。

## 2. current first line

current recommendation は次である。

1. checker-adjacent principal を維持する
2. stronger typed surface early principal promotion はしない
3. `p10` を authority-sensitive success prototype に置く
4. `p11` を authority-miss negative prototype に置く
5. `p12` を label-flow negative prototype に置く
6. representative Lean sample set を `e5 / p06 / p10 / p11 / p12 / p07 / p08` に widen する
7. Package 56 は first actual adoption package として close してよい

## 3. actualized source-side IFC trio

| sample | role | runtime outcome | distinct reading |
|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | explicit authority declassification success | `Success` | authority-sensitive success |
| `p11-typed-unauthorized-fingerprint-release` | authority なし release negative | `Reject` after `atomic_cut` | authority-miss negative |
| `p12-typed-classified-fingerprint-publication-block` | classified payload を public へ流そうとして止まる negative | `Reject` after `atomic_cut` | label-flow negative |

`p11` と `p12` は同じ negative ではない。
`p11` は request そのものが authority を欠く line であり、
`p12` は request-require は通るが request-ensure が classified-to-public publication を拒否する line として読む。

## 4. why this closes Package 56

- Lean-side first fragment:
  `CurrentL2LabelModel.lean` と `CurrentL2IfcSecretExamples.lean`
  により label / authority semantics の first fragment は already available である。
- source-side trio:
  `p10 / p11 / p12` により source-side IFC success / authority-miss negative / label-flow negative が corrected prototype として揃った。
- helper-local formal routes:
  verifier preview、model-check projection prefloor、theorem Lean actualization representative set がこの trio を accepted した。

したがって、Package 56 は
「Lean-side IFC first fragment を source-side corrected prototype trio まで押し込み、current checker-adjacent IFC first-fragment を close する」
ところまでは close してよい。

## 5. evidence

- source sample
  - `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- host plan
  - `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.host-plan.json`
- runtime / preview / model-check / theorem tests
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  - `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
  - `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
  - `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- Lean export / committed corpus
  - `samples/lean/current-l2/p12-typed-classified-fingerprint-publication-block/`
  - `scripts/current_l2_lean_sample_sync.py`

## 6. retained later

- stronger typed source principal
- checker-hint / diagnostics exhaustive integration
- final typed calculus
- final IFC syntax
- final public label API
- final public verifier contract
- concrete theorem/model-check production binding

## 7. stop line

この package の stop line は、

- source-side IFC trio が runnable である
- representative Lean sample set がこの trio を含んで regenerate / verify できる
- docs / snapshot / plan が `p10 / p11 / p12` を current IFC source-side floor として追える

の 3 点である。

その先の

- final typed source principal
- checker-hint / diagnostics exhaustive integration
- final public theorem/model-check contract

は still later に残す。
