# 523 — current L2 source-side IFC authority prototype pair and representative Lean sample-set widening

`specs/examples/520`、`521`、`522` により、
Lean-side first fragment と secret valid/invalid concrete example は source-backed に揃った。
この文書は、その次段として
**source-side explicit authority pair を `p10 / p11` に actualize し、
representative Lean sample set を widen する current cut**
を整理する。

これは final typed source principal や final IFC syntax や final public verifier contract の adoption ではない。
current checker-adjacent principal を保ったまま、
source-side の success / failure pair を corrected prototype と helper-local emitted route に乗せる package である。

## 1. current question

Layered strong typing / IFC first-fragment を
Lean-side example だけで止めず、
source-side でも

- authority があると release できること
- authority がないと release が止まること

を runnable pair として固定してよいか。

## 2. current first line

current recommendation は次である。

1. checker-adjacent principal を維持する
2. stronger typed surface early principal promotion はしない
3. `p10` を authority-sensitive success prototype に置く
4. `p11` を authority-sensitive failure prototype に置く
5. `verification_preview` / `model_check_projection_prefloor` / `theorem Lean actualization` を representative set へ widen する

## 3. actualized pair

| sample | role | runtime outcome | preview / theorem / model-check reading |
|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | explicit authority declassification success | `Success` | `runtime_try_cut_cluster` reached |
| `p11-typed-unauthorized-fingerprint-release` | authority なし release negative | `Reject` after `atomic_cut` | `runtime_try_cut_cluster` reached |

current cut では、
`p10` は authority-sensitive success、
`p11` は holder と release authority を分けた failure pair として読む。

## 4. why this is enough for the current package

- Lean-side first fragment:
  `CurrentL2LabelModel.lean` と `CurrentL2IfcSecretExamples.lean`
  により label / authority semantics の first fragment は already available である。
- source-side pair:
  `p10 / p11` により authority-sensitive success/failure pair が current-L2 runnable source に actualize された。
- helper-local formal routes:
  verifier preview、model-check projection prefloor、theorem Lean actualization representative set がこの pair を accepted した。

したがって、Package 56 は
「authority-sensitive IFC line を Lean-side explanation だけでなく source-side corrected prototype pair まで押し込む」
ところまでは close してよい。

## 5. evidence

- source samples
  - `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  - `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
- host plans
  - `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.host-plan.json`
  - `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.host-plan.json`
- runtime / preview / model-check / theorem tests
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  - `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
  - `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
  - `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- Lean export / committed corpus
  - `samples/lean/current-l2/p10-typed-authorized-fingerprint-declassification/`
  - `samples/lean/current-l2/p11-typed-unauthorized-fingerprint-release/`
  - `scripts/current_l2_lean_sample_sync.py`

## 6. retained later

- label-flow negative exhaustive corpus
- stronger typed source principal
- final typed calculus
- final IFC syntax
- final public verifier contract
- concrete theorem/model-check production binding

## 7. stop line

この package の stop line は、

- source-side explicit authority success/failure pair が runnable である
- preview / model-check / theorem Lean actualization representative set がこの pair を受理する
- docs / snapshot / plan が `p10 / p11` を current IFC evidence pair として追える

の 3 点である。

その先の

- final typed source principal
- label-flow exhaustive checker corpus
- final public theorem/model-check contract

は still later に残す。
