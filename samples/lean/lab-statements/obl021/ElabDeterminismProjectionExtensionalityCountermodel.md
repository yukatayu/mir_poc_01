# OBL-021 projection-extensionality countermodel

This LAB-only Lean file directly imports `ElabDeterminismStatementDraft.lean`.
For every result, each listed projection has exactly one `Unit` witness and
each component-equivalence predicate is native equality.  Two distinct Result
constructors still successfully elaborate for the same input.

`total_unique_equality_projections_still_allow_distinct_results` packages all
nine total/unique projection clauses, all component-equality clauses, the
statement draft, and the distinct-success fact in one Lean theorem.

The checked conclusion is narrow: per-projection totality/uniqueness and
component equality alone do not make the current `OBL021StatementDraft`
functional on Result values.  It does not select the missing joint
extensionality law or a direct Result relation, and it changes no OBL or Canon
status.

The governing research record and reproducible commands are in
`mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md` and
`plan/wrk-0003-projection-extensionality-countermodel.md`.
