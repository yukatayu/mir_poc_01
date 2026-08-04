---
id: theory/02-types-effects-failures
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, adr/ADR-0010]
summary: 型・mode・effect row・failure row・contract の語彙。有限 index 断片の範囲。
open_items: [OPEN-013]
---

# 02 — Types, effects, failures

## Types

```text
A ::= Int64 | Float64 | Bool | Text
    | Record{f₁:A₁, ..., fₙ:Aₙ}
    | Key(K)                          keyspace member (e.g. Participant)
    | Ref⟨A, target, cap, ℓ⟩          guarded reference (chain-backed)
    | Cap⟨op, scope⟩                  capability (linear-ish; lives in Δ)
    | Wit⟨kind⟩                       witness reference
```

Indexed/refinement discipline (ADR-0010): the checker admits only the finite
decidable index families — finite poset / lattice / powerset lattice /
region-lifetime preorder / finite capture-set inclusion / simple numeric
resource bounds. Examples of admissible indexed shapes:

```text
Ref⟨T, region=r, cap=Read, label=ℓ⟩     Message⟨epoch=e, incarnation=i⟩
Package⟨effects=E, failures=F⟩          Cut⟨closed_under=causal_order⟩
```

Not admitted in v0: arbitrary source-term dependency, proof terms in source,
type-level computation.

## Modes

`μ ::= local | remote(ℓ)` — where the value/action is anchored. `remote`
results are always mediated by request/observe; there is no transparent remote
lvalue. This residency mode is not an evaluation plan: theory/13 records
evaluation site, trigger/clock, authority origin, and materialization as
independent typed coordinates.

## Effect rows

`ε` is a finite set of declared operation names (state write classes, external
adapter ops, publish/observe classes, patch ops). Containment `ε ⊆ ε_allowed`
is a Line-1 check. Undeclared effects are static errors (E-EFF-001), never
silently widened.

## Failure rows

```text
F ∈ { Reject, Approximate, Compensate }                (outcome classes)
  ∪ { StaleMembership, MissingCapability, MissingWitness,
      RouteUnavailable, VisibilityDenied, TypeMismatch }   (generated set)
  ∪ declared domain failures
```

Laws (settled): declared row must contain every generated failure
(containment, E-ROW-001); failure is structured and explicit — no generic
exception bucket; `Reject` is the terminal dynamic outcome when admissible
options are exhausted; `Approximate` is admissible only where the contract
explicitly weakens the guarantee; `Compensate` marks explicit unwinding of
externalized obligations. Static errors (malformed / underdeclared) are a
*phase before* dynamic failure and never get folded into `Reject` (theory/06).

## Contracts and layers

```text
Contract = { input_type, output_type, precondition, postcondition,
             effect_row, failure_row, required_capabilities,
             provided_surface, observation_policy, redaction_policy,
             retention_policy, cost_bound }
Layer : Contract → Contract
```

Transparent-overlay condition (substitutability; all must hold):
no input narrowing, no output weakening, no precondition strengthening, no
postcondition weakening, no undeclared effect/failure widening, no ordinary-
path capability strengthening, no provided-surface shrink, no observation
widening, no redaction weakening, no retention widening. Otherwise the layer
requires an explicit `ContractUpdate` (old_ref, new_ref, reason, activation
cut, admitted_by, observation_delta). Auth and rate-limit layers are typically
non-transparent (they add failures / strengthen preconditions) — that is a
feature, not a bug: it forces the update to be visible.

Composition: `all_of(A, B)` requires both; `any_of` only when explicitly
declared with per-branch failure/audit surfaces and capability-union bounds
(hidden policy weakening is forbidden).

Runtime layers and verification modules are intentionally distinct. Runtime
layers transform `Contract`; a verifier maps a `Judgment` or
`ResidualObligation` to `Evidence | Diagnostic | ResidualObligation`. A
verifier may reject more, strengthen a contract, discharge an obligation, or
add evidence, but may not mint authority, permit an undeclared effect, erase a
failure, redefine a Core operation, or change projection semantics.

OPEN-013: cost_bound algebra (currently an opaque bound checked by simple
numeric comparison; runtime cost semantics deferred to Gate 4 / plan/03 R-09).
