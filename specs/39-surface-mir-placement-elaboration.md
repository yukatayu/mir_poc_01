# 39 — Surface Mir Placement / Elaboration

## role

This document fixes the Surface Mir alpha placement syntax and its lowering
boundary to Core Mir.

Surface Mir is the user-facing source layer. Core Mir remains the explicit
checker/runtime/devtools target. The semantic source authority is `.mir` source
files; `package.mir.json` is an alpha compatibility / generated package artifact.

## decision level

- `L1`
  - canonical Surface Mir place-scope syntax is `S { ... }`.
  - `S[ ... ]` is not supported and is not accepted as sugar.
  - `[]` remains value-level indexing for arrays, maps, role instances, and
    indexed state access.
  - generated communication / publish / observe edges must be explicit in Core
    IR and devtools.
- `L2`
  - alpha namespace disambiguation between place blocks, ordinary blocks, and
    record literals.
  - alpha Surface-to-Core elaboration judgment and obligation carrier shape.

## canonical surface placement syntax

Canonical place / location block:

```mir
S {
  state player[p: Participant]: Player
}
```

Canonical role-instance block:

```mir
Participant[self] {
  when start fails MissingCapability {
    join World as BrowserClient via WorldAdmission
  }
}
```

Rejected place-scope syntax:

```mir
S[
  state player[p: Participant]: Player
]
```

The required diagnostic is:

```text
bracket_place_scope_not_supported:
  use `S { ... }`; `[]` is reserved for indexing.
```

## brace disambiguation

`{}` is shared by place blocks, ordinary blocks, and record literals. Alpha
disambiguation uses namespace and syntactic context.

Rules:

1. `S { ... }` is a place block only when `S` resolves to a declared place path
   in item / statement context.
2. `Participant[self] { ... }` is a role-instance block only when
   `Participant` resolves to a declared role path and the bracket head is a
   role-instance binder. Bare `Participant { ... }` is not a role-instance
   block in alpha.
3. `Player { hp: 100 }` is a record literal only when `Player` resolves to a
   type path in expression context.
4. Alpha source disallows colliding place / role / type / value names in the
   same namespace frame when the collision would make a brace construct
   ambiguous.
5. Place-block body syntax is `BlockItems`; record literal body syntax is a
   field list.
6. Ambiguous source is rejected with `ambiguous_brace_construct`.

Dynamic place expressions are not represented by `owner { ... }` in alpha
unless `owner` is statically a place path. Dynamic role-instance expressions are
not block heads except through the explicit `Role[instance] { ... }` form. If
later source needs dynamic locus selection, it must use an explicit construct
such as `at expr { ... }` after a separate spec decision.

## Surface-to-Core elaboration judgment

Surface elaboration is explicit:

```text
Σ ; Γ ; Π ; current_locus = L ⊢ surface_item
  ⇝ core_items ; obligations ; generated_edges
```

where:

- `Σ` is the module / type / place / role / effect environment.
- `Γ` is the lexical value environment.
- `Π` is placement / projection context.
- `L` is the current Surface locus.
- `core_items` are explicit Core Mir transitions, effects, and metadata.
- `obligations` include capability, visibility, failure, freshness, and
  residual proof / model-check carriers.
- `generated_edges` include MessageEnvelope, publish, observe, and witness /
  audit edges.

## location block elaboration

Surface:

```mir
S {
  body
}
```

Owner-local declaration / execution elaboration:

```text
save current_locus
current_locus := S
elaborate body
restore current_locus
```

The lowering must preserve source spans so diagnostics and devtools can point
from generated Core IR back to Surface source.

Foreign nested place blocks do not grant owner-local authority. If an executable
place block targeting owner `O` appears inside an actor/current locus `L` where
`L != O`, the block elaborates to an explicit owner-directed request/effect at
`O`. The original actor locus, principal, membership epoch, incarnation,
capability refs, witness refs, and source span remain attached to the generated
request. The body is checked under `O` for owner invariants, but the request is
authorized from `L`; missing capability, freshness, witness, route, visibility,
or failure-row containment rejects or defers the request.

This rule preserves the readable Surface idiom:

```mir
Participant[self] {
  when attack(target: Participant)
    fails StaleMembership, MissingCapability, MissingWitness, RouteUnavailable {
    S {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}
```

The nested `S { ... }` is not an ambient authority switch for
`Participant[self]`.

## cross-locus read / write

If a Surface expression reads state owned by `O` while current locus is `L`:

```text
if L == O:
  local_read
else:
  generate read request or observe edge
  require declared observe / visibility authority
  add declared failure possibilities
```

If a Surface statement writes state owned by `O` while current locus is `L`:

```text
if L == O:
  local_write
else:
  generate write/effect request to O
  require write capability or owner-mediated effect
  add declared failure possibilities
```

The capability in the `L != O` branch authorizes the generated request/effect;
it does not create a direct remote store from `L` into `O`'s state.

Generated failure rows may include:

```text
StaleMembership
MissingCapability
MissingWitness
RouteUnavailable
VisibilityDenied
TypeMismatch
```

They must be contained in the declared failure row or rejected.

Surface behavior blocks declare generated failure containment with a `fails`
clause:

```mir
when attack(target: Participant)
  fails StaleMembership, MissingCapability, MissingWitness, RouteUnavailable {
  ...
}
```

Equivalent implementation syntax may be accepted only if it preserves an
explicit source-spanned failure row for generated communication.

## auto publish / observe

Visible state fields may generate publish / observe rows.

```mir
S {
  state score[p: Participant]: Int64
    visible observer_safe
}
```

Rules:

- only declared visible fields are auto-published.
- private fields are not auto-published.
- observer-safe output must preserve label / authority / redaction / retention
  policy from `specs/22-observability-devtools-semantics.md`.
- generated publish / observe rows are not hidden magic; they must appear in
  Core IR, event DAG, MessageEnvelope trace, and devtools source-link panels.

Contract-bearing witness is not silently created. Devtools / audit witness may
be generated for traceability, but authority-bearing witness requires explicit
annotation or explicit policy.

## source authority

The alpha source flow is:

```text
.mir source files
  -> parse
  -> typecheck
  -> elaborate to Core Mir
  -> optional generated package artifact / package.mir.json
  -> runtime / devtools / release evidence
```

The reverse direction may exist as a migration or compatibility helper. It is
not semantic source authority.

## soundness target

Target theorem:

```text
If a Surface Mir program elaborates to Core Mir and the Core program passes
its checks, every cross-locus Surface action is represented by explicit Core
communication and satisfies declared capability / visibility / failure
constraints.
```

This is a target obligation for the Surface line, not a discharged theorem in
this docs-only package.

## required alpha sample rows

- `SURF-01`: `S { state player[p: Participant]: Player }` accepted.
- `SURF-02`: `S[ ... ]` rejected as
  `bracket_place_scope_not_supported`.
- `SURF-03`: record literal `Player { hp: 1 }` accepted.
- `SURF-04`: ambiguous place/type brace construct rejected.
- `SURF-05`: `Participant[self] { when start fails MissingCapability { ... } }`
  accepted.
- `SURF-06`: undeclared `Unknown { ... }` place block head rejected.
- `SURF-07`: undeclared `Unknown[self] { ... }` role-instance head rejected.
- `SURF-08`: invalid role-instance binder such as `Role[self + other]`
  rejected.
- `SURF-09`: `S[self] { ... }` accepted when `S` resolves to a declared
  role, not a place scope.
- `ELAB-01`: cross-place read generates explicit Core edge.
- `ELAB-02`: cross-place write generates explicit Core edge.
- `ELAB-03`: private field auto-publish rejected or blocked.
- `ELAB-04`: undeclared generated failure row rejected.
- `ELAB-05`: generated Core IR preserves source spans.

## non-claims

This document does not claim:

- final public grammar.
- completed parser / checker / runtime implementation for Surface Mir.
- hidden automatic communication without Core/devtools evidence.
- final packet / FFI transport semantics.
- final public ABI / SDK.
