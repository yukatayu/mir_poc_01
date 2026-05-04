# 04 — verification stratification

## problem

Mir / Mirrorea の型システムと検証は、すべてを 1 本の静的型検査へ押し込むと歪む。一方で、何でも residual obligation に逃がすと実用にならない。

したがって、検証を 3 line に分ける。

```text
Line 1: decidable static checker
Line 2: model-check second line
Line 3: proof side line
```

## Line 1 — decidable static checker

対象:

- finite preorder / lattice / subset
- region / lifetime / lease
- fallback well-formedness
- capability monotonicity
- contract subtyping
- effect row containment
- failure row containment
- observation label / redaction / retention containment
- package manifest admission
- invalid cut structural reject

判定形式:

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

意味:

- `Σ`: user-defined finite theory
- `Ψ`: place / phase / frontier context
- `Γ`: unrestricted context
- `Δ`: linear / affine / capability context
- `A`: value type
- `μ`: mode
- `ε`: effect row
- `C`: decidable constraints
- `O`: residual obligations

重要:

- `C` は checker が判定する。
- `O` は hidden acceptance ではなく、明示的に残る。
- `O` が未解決なら public/product claim しない。

## Indexed / refinement types, not full dependent type theory

α-0.5 / α-0.8 では、full dependent type theory は採用しない。

採用するもの:

```text
Ref<T, region=r, cap=Read, label=L>
Message<epoch=e, incarnation=i>
Package<requires=C, effects=E, failures=F>
Layer<pre=P, post=Q, effects=E, failures=F>
Cut<closed_under=causal_order>
```

採用しないもの:

- 任意 term への依存
- source language 内の一般証明 term
- Mir を Isabelle / Lean 相当の proof language にすること

Mir は Isabelle ではない。Mir semantic core の横に proof spine / mechanization side line を置く。

## Line 2 — model-check second line

対象:

- finite transition system
- interleaving
- membership epoch races
- reset / handoff race
- small hot-plug activation order
- weak-memory profile
- checkpoint/cut small graph

model obligation carrier:

```text
ModelObligation = {
  variables,
  transition_relation,
  initial_states,
  safety_properties,
  liveness_properties?,
  fairness_assumptions?,
  abstraction_relation,
  expected_result
}
```

Line 2 の completeness は、抽出された finite transition system に対してだけ言える。現実の distributed runtime 全体への完全性は抽象化の soundness が必要。

## Line 3 — proof side line

対象:

- checker soundness lemmas
- fallback monotonicity
- no re-promotion
- contract subtyping composition
- cut prefix-closure lemmas
- no stale resurrection lemmas
- noninterference / redaction lemmas

proof side は Lean / Isabelle / other prover へ渡せるが、Mir 本体と一体化しない。

## soundness targets

### Static checker soundness

```text
If checker accepts and all residual obligations are discharged,
then execution cannot violate the checked invariant class.
```

### Model-check soundness

```text
If extracted finite model satisfies property and abstraction is sound,
then source scenario satisfies the modeled property.
```

### Proof side soundness

```text
If proof obligation is discharged in trusted prover,
then the corresponding lemma may be used by checker/runtime docs.
```

## bounded completeness

完全性は限定する。

- static checker: declared finite constraints について complete
- model checker: extracted finite model について complete
- proof line: algorithmic completeness は claim しない

## required samples

- raw dangling reference reject
- fallback chain accept
- inherited chain with lineage accept
- mutable covariance reject
- precondition strengthening reject
- invalid distributed cut reject
- local runtime accepted dispatch
- stale membership reject
- model-check pass and counterexample pair

## stop line

- full dependent type theory 完成と書かない
- Isabelle/Lean 相当の integrated proof language と書かない
- model-check result を production proof と書かない
- residual obligation を hidden acceptance として扱わない

