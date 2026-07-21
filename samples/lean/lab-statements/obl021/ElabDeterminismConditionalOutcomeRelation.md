# OBL-021 conditional outcome relation

This LAB-only Lean file directly imports `ElabDeterminismStatementDraft.lean`.
It introduces an experiment-local tagged `Outcome` and makes outcome existence
an explicit `OutcomeTotal` premise. The conditional theorem then relates every
outcome pair through the draft's existing success-success, reject-reject, and
mixed-case clauses.

The conclusion is not native equality, an equivalence law, a quotient, a final
Result relation, or a Canon statement. It is a conditional LAB relation only.

The governing research record and reproducible commands are in
`mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md` and
`plan/wrk-0005-conditional-outcome-relation.md`.
