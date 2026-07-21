# OBL-021 conditional outcome relation

This LAB-only Lean file directly imports `ElabDeterminismStatementDraft.lean`.
It introduces an experiment-local tagged `Outcome` and makes outcome existence
an explicit `OutcomeTotal` premise. The conditional theorem case-splits every
constructor combination in one fixed input's actual-outcome fiber: values
satisfying `OutcomeOf`. Homogeneous pairs use the draft's success-success or
reject-reject clauses; mixed pairs are impossible through its success/reject
exclusion. Pairwise relatedness of actual outcomes comes from the draft and
well-scopedness; `OutcomeTotal` supplies only an inhabitance witness.

The relation is not claimed to have global laws over every tagged value. On the
subtype of actual outcomes, its all-pairs theorem does entail local reflexive,
symmetric, and transitive closure, but this file neither selects a public
equivalence nor constructs a Canon quotient.

The conclusion is not native equality, a global equivalence/setoid law, a
Canon quotient, a final Result relation, or a Canon statement. It is a
conditional LAB relation only.

In the rejection branch, the relation is only the supplied
`P.EquivalentDiagnostic` predicate between actual diagnostics. This file does
not connect it to the canonical Diagnostic fields or explanation properties in
`theory/10-diagnostics.md`.

The governing research record and reproducible commands are in
`mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md` and
`plan/wrk-0005-conditional-outcome-relation.md`.
