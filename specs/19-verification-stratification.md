# 19 — Verification Stratification

## role

この文書は、Mir / Mirrorea の verification を
**static checker first line / model-check second line / proof side line**
へ分ける bounded theory freeze を置く。

- ここで固定するのは α-0.5 / α-0.8 / α-0.9 operational readiness を支える verification stratification である
- Mir を integrated proof language にしない
- full dependent type theory、final prover binding、final public verifier API は still later に残す

## decision level

- `L1`
  - verification は 1 本の static checker に collapse しない
  - residual obligation は hidden acceptance ではなく explicit carrier として残す
  - Mir は Isabelle / Lean そのものではない
  - proof / model obligation は external tool へ外出しできる
- `L2`
  - current obligation carrier shape
  - soundness / bounded completeness の current wording
  - α-0.5 / α-0.8 / α-0.9 で required な verification split

## three-line overview

```text
Line 1: decidable static checker
Line 2: model-check second line
Line 3: proof side line
```

この split は
「何でも static checker に押し込む」
ことも、
「何でも residual へ逃がす」
ことも避けるためのものである。

## Line 1 — decidable static checker

current first line は、finite / declared / explicitly modeled fragment に対する
decidable checker である。

judgment の current reading:

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

- `Σ`
  user-defined finite theory
- `Ψ`
  place / phase / frontier / observation context
- `Γ`
  unrestricted context
- `Δ`
  linear / affine / capability context
- `A`
  value type
- `μ`
  mode
- `ε`
  effect row
- `C`
  checker が discharge する decidable constraints
- `O`
  residual obligations

Line 1 が少なくとも cover すべき current family:

- finite preorder / lattice / subset check
- lifetime / lease / freshness floor
- fallback well-formedness
- mutable / read-write invariance
- contract pre/post comparison
- effect row containment
- failure row containment
- capability monotonicity
- observation label / redaction / retention containment
- package manifest admission
- invalid distributed cut structural reject

## residual obligation carrier

Line 1 は、discharge しない obligation を success 扱いしない。

minimum carrier:

```text
ResidualObligation = {
  obligation_id,
  obligation_kind,
  source_refs,
  required_context,
  suggested_target,
  current_status
}
```

- `suggested_target`
  `model_check` / `proof` / `kept_later` など
- `current_status`
  `undischarged` / `discharged_elsewhere` / `intentionally_deferred`

accepted evidence は
negative reason code の不在ではなく、
accepted row / accepted obligation / residual obligation inventory を
explicit に持つべきである。

## indexed / refinement types, not full dependent type theory

current line で採るのは、
finite-index / refinement-oriented current cut である。

admissible examples:

```text
Ref<T, region=r, cap=Read, label=L>
Message<epoch=e, incarnation=i>
Package<effects=E, failures=F>
Layer<pre=P, post=Q, effects=E, failures=F>
Cut<closed_under=causal_order>
```

current line で採らないもの:

- arbitrary source-term dependency
- general proof term in Mir source
- Mir source itself as full theorem prover language
- Isabelle / Lean 相当の integrated proof surface

Mir は theorem prover ではなく semantic core である。
proof spine は side line として接続する。

## Line 2 — model-check second line

Line 2 は、finite transition system へ落とせる scenario を
model-check second line として扱う。

targets:

- interleaving / race
- membership epoch race
- reset / handoff race
- weak-memory profile
- small hot-plug activation order
- checkpoint / cut small graph

minimum carrier:

```text
ModelObligation = {
  variables,
  transition_relation,
  initial_states,
  safety_properties,
  liveness_properties,
  fairness_assumptions,
  abstraction_relation,
  expected_result
}
```

Line 2 の completeness は、
**extracted finite model に対してだけ**
bounded に読む。
distributed runtime 全体の completeness を直接 claim しない。

## Line 3 — proof side line

Line 3 は、checker soundness や semantic lemma を扱う proof side line である。

targets:

- checker soundness lemma
- fallback monotonicity
- no re-promotion
- contract-subtyping composition
- cut prefix-closure
- no stale membership / witness / lease resurrection
- noninterference / redaction lemma

minimum carrier:

```text
ProofObligation = {
  lemma_id,
  lemma_kind,
  statement_summary,
  source_refs,
  assumed_invariants,
  export_target,
  current_status
}
```

`export_target` は
`Lean` / `Isabelle` / `other_prover` / `kept_later`
のように explicit に持ってよい。

## externalization boundary

current line は次を許す。

- checker residual obligation を Lean / Isabelle / model checker / TLA+-style tool へ外出しする
- discharged result を docs / report / handoff artifact へ mirror する

current line は次をしない。

- Mir source を external prover そのものにする
- discharge 済みでない obligation を hidden success として扱う
- model-check result を general proof と言い換える

## soundness

### static checker soundness

```text
If Line 1 accepts and all referenced residual obligations are discharged,
then execution does not violate the checked invariant class.
```

### model-check soundness

```text
If the extracted finite model satisfies the property and the abstraction relation is sound,
then the source scenario satisfies the modeled property class.
```

### proof-side soundness

```text
If a proof-side obligation is discharged in a trusted prover,
the corresponding lemma may be cited by checker/runtime documentation and review.
```

## bounded completeness

current completeness claims are intentionally bounded.

- static checker:
  declared finite constraints に関して bounded complete
- model checker:
  extracted finite model に関して bounded complete
- proof side:
  algorithmic completeness を claim しない

したがって、
Line 1 / 2 / 3 のどれかが未整備でも、
他 line の bounded result を使って partial closeout はできる。
ただし operational/product claim は residual inventory を隠さない。

## operational alpha boundary

α-0.5 / α-0.8 / α-0.9 で必要なのは、
verification を全自動化することではなく、
workflow 中の verification authority を explicit に保つことである。

- α-0.5:
  package input から checker result / runtime plan / residual inventory が追える
- α-0.8:
  attach verdict や contract-update path が checker / model / proof obligation と対応づく
- α-0.9:
  devtools export が reason/proof refs を typed metadata として参照できる

## required sample / obligation anchors

current freeze で visible であるべき family:

- `LIF-01/02/03/04/05/07/08/11/13`
- `VAR-01/02/03/04/05/06/07/08/09/10/11/13/15`
- `CUT-04/05/07/08/09/11/14/15/17`
- `CHK-LIF-01..04`
- `CHK-VAR-01..03`
- `CHK-CUT-01`
- `CHK-PKG-01/02`
- model-check pass / counterexample pair

repository-memory sequencing は `plan/45` と `plan/48` を参照する。

## deferred

- full dependent type theory
- final public verifier handoff contract
- final public diagnostics schema
- proof-complete integrated language
- production theorem prover / model-checker binding

## stop line

- full dependent type theory 完成と書かない
- Mir を Isabelle / Lean 相当の integrated proof language と書かない
- residual obligation を hidden acceptance として扱わない
- model-check result を general proof と書かない
- proof-side placeholder を discharged lemma と書かない
