# 35 — Mir Typed IR and Interpreter

## role

This document fixes the Full System V1 boundary for textual Mir lowering, typed IR, checker, and the first safe C-like interpreter.

It extends `specs/28` without replacing it. The normative split between pure computation and effectful terms remains:

```text
Σ ; Γ ⊢ e : A ⇝ C
Σ ; Ψ ; Γ ; Δ ⊢ t : A @ μ ! ε ? ρ ⇝ C ; O
```

## decision level

- `L1`
  - Failure rows remain explicit.
  - Host I/O remains a typed external boundary, not a Mir core primitive.
  - Runtime rejection and static rejection must be distinguishable.
- `L2`
  - The first typed IR is a narrow, inspectable representation for alpha grammar samples.
  - The first interpreter executes the safe C-like subset before broad effectful runtime integration.

## typed IR minimum

Required IR nodes:

- `Module`.
- `Import`.
- `Function`.
- `RecordType`.
- `CapabilityDecl`.
- `CapabilityRequirement`.
- `EffectDecl`.
- `EffectMember`.
- `Block`.
- `Let`.
- `Assign`.
- `If`.
- `While`.
- `For`.
- `Return`.
- `Call`.
- `RecordConstruct`.
- `FieldAccess`.
- `ArrayConstruct`.
- `Index`.
- `Perform`.
- `BoundaryRef`.
- `Publish`.
- `Observe`.
- `Witness`.
- `Handoff`.
- `AtomicCut`.

`Perform`, `Publish`, `Observe`, `Witness`, `Handoff`, and `AtomicCut` are effectful IR nodes. They must not be represented as pure expressions.

Capabilities in the first typed IR are symbolic authority requirements carried by source declarations. They are not transport authentication and must not be silently imported from package/provider metadata without a source or generated-source reference.

## checker minimum

The checker must validate:

- primitive and record types.
- lexical variable scope.
- function arity and return type.
- import resolution.
- fixed array length and statically obvious bounds.
- effect row containment.
- failure row containment.
- required capability declarations.
- basic `require` / `ensure` shape.
- source span and diagnostic attachment.

Accepted evidence must include explicit accepted obligations. Absence of diagnostics is not sufficient.

## interpreter minimum

The first interpreter must execute:

- pure `add_one`.
- variables and lexical scope.
- arrays with in-range and out-of-range rows.
- record construction and field access.
- `if` / `else`.
- minimal loop rows, bounded where needed.
- imports and function calls.

Runtime rows must produce a compute trace:

```text
ComputeTrace = {
  function_id,
  inputs,
  local_bindings_summary,
  branch_taken,
  outputs,
  rejected_reason
}
```

Observer-safe traces must redact secrets and must not expose raw witness or auth payloads.

## rejection split

Static rejection examples:

- undeclared effect.
- undeclared failure.
- missing capability declaration.
- impossible type mismatch.
- malformed import.
- statically provable array out of bounds.

Runtime rejection examples:

- dynamic array index out of bounds.
- adapter unavailable.
- runtime boundary type mismatch.
- missing live witness/capability.
- stale membership.

## save/load relation

Runtime-visible computational state must enter the existing cut/save/load carrier if it crosses a runtime frontier.

Any executable `AtomicCut` / save-load row must inherit the rules from `specs/20` and `specs/28`:

- computational state stays inside `SaveObject` or runtime session state.
- `R2` still requires `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`.
- rollback across an accepted cut must be rejected unless the cut semantics explicitly admit it.
- load must not resurrect stale computational state, stale capability/witness state, or hidden interpreter-local state.

`P-MIR-04` cannot close on merely visible cut syntax. It needs positive runtime rows and negative rows for rollback-across-cut rejection, stale-state non-resurrection, and violated `R2` preconditions where the package claims executable cut/save behavior.

The interpreter must not keep hidden computational state outside:

- event DAG.
- runtime session state.
- save object.
- devtools/report evidence.

## stop line

- Do not claim Rust-level language completion.
- Do not claim final parser/checker/runtime public API.
- Do not claim broad effectful semantics before `FS-04`.
- Do not implement native codegen before typed IR and boundary preservation tests exist.
