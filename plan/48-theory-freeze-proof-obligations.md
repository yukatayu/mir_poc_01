# plan/48 — theory freeze proof obligations

## purpose

この文書は、
`specs/19..23`
で fixed した bounded theory freeze の
**residual proof / model / checker obligation inventory**
を repository memory として整理する。

## current reading

P-A1-18 で fixed するのは、
proof complete state ではなく obligation boundary である。

したがって、この文書は

- discharge 済みの事実
- current checker floor で実装済みの事実
- still residual な obligation

を分けて保持する。

## obligation families

### static-checker soundness side

- fallback monotonicity
- no implicit chain propagation
- mutable/read-write invariance
- contract pre/post comparison soundness
- effect / failure containment soundness
- package admission structural soundness

### model-check side

- membership frontier race abstraction
- reset / handoff race abstraction
- small checkpoint / cut graph abstraction
- weak-memory profile abstraction

### proof-side semantic lemmas

- no stale membership resurrection
- no stale witness resurrection
- no expired lease resurrection
- cut prefix-closure
- no rollback across `atomic_cut`
- redaction / noninterference
- auth-layer composition and contract-update law

## current discharge status

already actualized as current executable or checker floor:

- selected `CHK-*` rows
- selected `CUT-*` rows
- selected `VIS-A1-*` rows
- selected `HP-A1-*` rows

still residual / not fully discharged:

- full static checker soundness proof
- model extraction soundness proof for broader runtime
- full no-stale-witness / no-stale-lease proofs on saved carrier
- auth-stack composition proof beyond first-floor explicit rows
- session-bound observability derivation proof
- typed external direct execution lane proof obligations

## export targets

current allowed export targets:

- Lean
- Isabelle
- model checker / TLA+-style tool
- explicit kept-later record

Mir itself is not the proof language.

## proof-side stop lines

- model-check pass ≠ general proof
- exact expected JSON ≠ discharged theorem
- report-local argument ≠ trusted prover discharge
- signature / provenance evidence ≠ semantic safety proof

## next reopen point

`P-A1-18` does not promote a proof package by itself.
Next executable package should only reopen obligation families needed by
`P-A1-19` / `P-A1-20` / `P-A1-21`.
