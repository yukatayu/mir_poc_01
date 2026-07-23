# WRK-0021 - CostBudget scalar-projection falsifier

## status

This LAB memo records the registered Lean falsifier for
`mirrorea_canon/working/WRK-0021-costbudget-scalar-projection.md`. The Canon
working record is `L3-open, frozen`; this memo is not a CostBudget algebra,
Contract rule, or repaired countermodel.

## registered execution

The registration at `4ac08f77f0ef718803ab0628abce7ac85eebff43` required a
pre-source marker check after push. That check found no prior
`scalar_total_does_not_reflect_pointwise` marker. The allowed transient source
then introduced `ThreeCounterBudget`, Prop aliases for pointwise and scalar
comparison, and the fixed pair `(0, 1, 0)` / `(1, 0, 0)` in the existing Lean
foundation only.

## observed falsifier

The exact registered Lean command emitted:

```text
samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean:152:2: error: failed to synthesize
  Decidable (scalarTotalLeq scalarCandidate scalarReference)

samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean:156:2: error: failed to synthesize
  Decidable ¬pointwiseBudgetLeq scalarCandidate scalarReference
```

This is the record's explicit `Lean cannot establish both` falsifier. The
registered wrapper used semicolon separators, so the clean-near-end baseline
and Lean manifest sync continued after the failed Lean invocation. Their output
does not establish the proposed countermodel and is not retained as result
evidence. The temporary foundation and explanation edits, plus the generated
manifest failure update, were restored before this memo was committed.

## root-cause boundary

The fixed source expressed the two comparisons as `Prop` aliases but supplied
no `Decidable` instances that would let `decide` construct either proof. Adding
the missing instances, unfolding relations differently, changing proof tactics,
or otherwise making the theorem compile would repair the frozen registration;
WRK-0021 prohibits that.

## permitted follow-up

A future record may independently define a different bounded question,
comparison representation, source procedure, and falsifier after checking
duplication and scope. It must not claim to repair, validate, or promote
WRK-0021, and it must not select a pointwise relation, scalar policy,
CostBudget algebra, Contract direction, runtime accounting model, or
PROPOSAL-011 disposition from this failure.

## non-claims

No finite countermodel, theorem, CostBudget semantics, Contract behavior,
transparent-overlay classification, OBL premise, runtime behavior, sample
workflow, Gate/Phase, or public status follows from this failed proof.
