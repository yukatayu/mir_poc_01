# 05 — Computational Core Theory

## Judgment split

Pure computation:

```text
Σ ; Γ ⊢ e : A ⇝ C
```

Effectful computation:

```text
Σ ; Ψ ; Γ ; Δ ⊢ t : A @ μ ! ε ? ρ ⇝ C ; O
```

Where:

- `Σ`: type/module/package environment
- `Ψ`: place/observation/projection context
- `Γ`: lexical variable environment
- `Δ`: capability/resource/effect precondition environment
- `A`: result type
- `μ`: execution locus/mode
- `ε`: effect row
- `ρ`: failure row
- `C`: discharged constraints
- `O`: residual obligations / observation obligations

## Soundness goals

### Type preservation

If a typed expression steps, its type is preserved.

```text
Σ ; Γ ⊢ e : A
and e -> e'
then Σ ; Γ ⊢ e' : A
```

For effectful terms, preservation includes effect/failure row containment.

### Progress, bounded

If a closed term is well-typed, then either:

- it is a value;
- it can step;
- it produces a declared rejection in `ρ`;
- it emits an explicit residual obligation.

No hidden stuck states.

### Boundary preservation

Host I/O may only occur through declared `perform` boundary effects.

### Save/load preservation

Runtime-visible computational state must be included in save/load state when it crosses runtime frontier.

## Runtime rejection vs static rejection

Static rejection:

- undeclared effect
- undeclared failure
- missing capability declaration
- impossible type mismatch
- malformed import
- statically provable array out of bounds

Runtime rejection:

- dynamic array index out of bounds
- runtime type mismatch from boundary
- missing live witness/capability at runtime
- stale membership
- adapter unavailable

## Samples required

### Positive

- AddOne
- variables / scope
- arrays / bounds in range
- records Vec3
- if/else control-flow
- imports/functions
- host read -> Mir compute -> host write

### Negative

- variable out of scope
- array index out of bounds
- record missing field
- non-exhaustive or invalid control case
- unresolved import
- undeclared effect
- undeclared failure
- missing capability

## Implementation notes

A simple interpreter is enough for first phase.

Do not implement native codegen before:

- typed IR is stable
- boundary schemas are defined
- negative samples exist
- devtools can inspect compute trace

## Compute trace

Every runtime row should produce:

```text
ComputeTrace {
  function_id,
  inputs,
  local_bindings_summary,
  branch_taken,
  outputs,
  rejected_reason?
}
```

Observer-safe trace must redact secrets.
