# OBL-021 projection-vacuity countermodel

This LAB-only Lean file directly imports `ElabDeterminismStatementDraft.lean`.
It constructs two distinct successful results for one well-scoped input while
all result-projection predicates are empty.

The checked conclusion is deliberately narrow: the current
`OBL021StatementDraft` can hold in that model because its result comparison only
requires equivalence after both projection witnesses are supplied.  It does not
select result equality, require a projection-totality law, define diagnostics,
or change any OBL or canon status.

The governing research record and reproducible commands are in
`mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md` and
`plan/wrk-0002-projection-vacuity-countermodel.md`.
