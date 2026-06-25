# 05 — Surface To Core Elaboration Theory

## 1. Purpose

Surface Mir allows users to write system-wide meaning without manually writing all communication.
Core Mir makes the generated communication, effects, publications, witnesses, and failures explicit.

## 2. Judgment

Surface elaboration judgment:

```text
Σ ; Γ ; Π ; current_locus = L ⊢ surface_item ⇝ core_items ; obligations ; generated_edges
```

Where:

- `Σ`: module/type/place/effect environment.
- `Γ`: lexical variables.
- `Π`: placement / projection context.
- `L`: current evaluation locus.
- `core_items`: explicit Core Mir.
- `obligations`: capability, visibility, failure, freshness obligations.
- `generated_edges`: communication / publish / observe edges.

## 3. Location block

Surface:

```mir
S {
  body
}
```

Elaboration:

```text
current_locus := S
elaborate(body)
restore prior locus
```

## 4. Indexed state declaration

Surface:

```mir
S {
  state player[p: Participant]: Player
}
```

Core metadata:

```text
IndexedStateDecl {
  owner: S,
  name: player,
  keyspace: Participant,
  value_type: Player,
  lifecycle: active_key_partial_map,
}
```

## 5. Cross-locus read

If expression reads state `x` owned by `O` from locus `L`:

```text
if L == O:
  local_read(x)
else:
  generated_read_request or observe edge
  require Observe capability / visibility
  add failure possibilities
```

## 6. Cross-locus write

If expression writes state `x` owned by `O` from locus `L`:

```text
if L == O:
  local_write(x)
else:
  generated_write_request to O
  require Write capability or owner-mediated effect
  add failure possibilities
```

## 7. Auto publish / observe

For a write to visible state:

```text
write x
if visible(x.field, observer_class):
  generate publish(x_changed)
  generate observe edges to interested observers
```

No publish for private fields.

## 8. Witness

Two kinds:

```text
Devtools/Audit witness:
  may be auto generated for traceability.

Contract witness:
  must be explicit or generated from explicit policy.
```

Do not silently create authority-bearing witness.

## 9. Failure row completion

Every generated communication adds possible failures:

```text
StaleMembership
MissingCapability
MissingWitness
RouteUnavailable
VisibilityDenied
TypeMismatch
```

These must be contained in the declared failure row or rejected with diagnostic.

## 10. Devtools transparency

Generated Core IR must be inspectable.

Devtools must show:

- source span.
- generated core transition.
- generated MessageEnvelope.
- generated publish/observe.
- capability check.
- failure row contribution.

## 11. Soundness target

```text
If Surface program elaborates to Core program and Core program passes checks,
then every cross-locus action is represented by explicit Core communication and satisfies declared capability/visibility/failure constraints.
```

This is the central theorem for the Surface Mir line.
