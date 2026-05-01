# 04 — Theory freeze: lifetime, fallback, and guarded references

This file defines the intended theory direction for lifetime/fallback. Mirror the normative parts into `specs/13-type-system-lifetime-fallback.md`.

## 1. Core problem

The language should allow references to values in other Places, processes, or networked machines, but these references must not be raw pointers. They are generally read-only or observed references guarded by lifetime/freshness/membership/capability conditions.

The original motivating problem:

```text
lifetime: a > b > c
b = c
```

is unsafe because `b` may outlive `c`.

With fallback:

```text
b = c fallback a
```

`b` can remain usable after `c` expires by degrading to `a`.

The correct meaning is not that `c`'s lifetime is extended. The correct meaning is that `b` stores a guarded logical access path with two options.

## 2. Key definition: guarded access path

A guarded reference is conceptually:

```text
GuardedRef<T, Role, Cap, Label> = ordered finite list of Option

Option = {
  target: AccessTarget<T>,
  role: LogicalAccessRole,
  lease: LifetimeOrFreshnessGuard,
  contract: ContractSurface,
  capability: CapabilitySurface,
  label: ObservationLabel,
  lineage: optional edge-local lineage evidence
}
```

A chain:

```text
FriendHead > SelfShoulder > WorldOrigin
```

means leftmost viable option under monotone degradation. It does not mean the targets have the same identity or lifetime.

## 3. What fallback guarantees

Fallback guarantees, when well-formed:

- no successful access selects an expired option
- access may degrade to a later option when an earlier option expires, fails, or is no longer admissible
- degradation is monotone within the same semantic lineage
- later options do not require stronger capabilities or stronger guarantees than earlier options
- failure is explicit if no admissible option remains
- fallback decisions are visible as events/telemetry, not hidden repairs

Fallback does not guarantee:

- target lifetime extension
- object identity preservation
- write capability preservation
- automatic reacquire after an earlier target returns
- hidden buffering of writes
- hidden transport recovery

## 4. Raw reference vs guarded reference vs inherited chain

Three operations must remain distinct.

### 4.1 Plain reference to object/cell

```text
c = ref(d) fallback a
```

If `d` expires, `c` falls back to `a`. `c` does not automatically inherit fallback options inside `d`.

### 4.2 Inherit/splice a chain

```text
c = inherit_chain(d) fallback a
```

If `d` denotes a guarded chain `e > b`, and static evidence proves same logical access path / semantic lineage, then `c` canonicalizes to:

```text
e > b > a
```

This must be explicit.

### 4.3 Snapshot selected target

```text
c = snapshot_selected(d) fallback a
```

If `d` currently selects `e`, then `c` may become:

```text
e > a
```

without inheriting `d`'s other fallback options.

## 5. Canonicalization rule

Canonicalization is left-to-right flattening only for same logical access path / semantic lineage.

```text
canon(option) = [option]
canon(fallback(x, y)) = canon(x) ++ canon(y)
```

This rule applies only if:

1. all options share the required logical access role/path,
2. edge-local lineage evidence exists for inherited/spliced edges,
3. later options are contract-compatible successors,
4. capability/guarantee does not strengthen,
5. declared access target and contract/capability surfaces are present.

If these are not met, do not silently flatten.

## 6. Static rejection vs dynamic Reject

Static errors/rejections:

- missing declared access target
- missing documented lineage annotation where inheritance is claimed
- mismatched lineage edge
- different logical access role/path
- successor requires stronger capability
- successor contract is statically incompatible
- mutable covariance or write capability promotion
- underdeclared branch where current L2 requires evidence

Dynamic `Reject`:

- all options well-formed, but current leases/guards fail
- write through selected option not permitted
- witness/membership/capability absent at runtime when not statically decidable
- runtime freshness failure

## 7. Lifetime/fallback typing obligations

A future checker should prove or enforce:

1. No successful dangling read.
2. No successful write through expired or read-only option.
3. No implicit chain propagation.
4. Inherited chain has lineage evidence.
5. Fallback successor capability is monotone non-increasing.
6. Contract surface admits fallback successor.
7. Expired lease is never resurrected by rollback, `atomic_cut`, or load.
8. Canonicalization preserves observable outcome for well-formed chains.
9. Underdeclared fallback is static error.
10. Reacquire is explicit and creates new evidence/epoch; it is not no-repromotion violation.

## 8. Bird/sparkle motivating sample

The example:

```text
WorldOrigin > SelfShoulder > Sparkle > Bird
FriendHead incomparable with Bird

BirdAnchor = FriendHead fallback SelfShoulder
SparkleAnchor = Bird fallback WorldOrigin
```

is ambiguous if `SparkleAnchor` refers to `Bird` as an object.

Desired behavior:

- Bird disappears.
- Sparkle remains at FriendHead if FriendHead is still valid.
- If Friend logs out, Sparkle may degrade to SelfShoulder or fade out according to policy.

Correct modeling:

```text
BirdAnchor = FriendHead > SelfShoulder
SparkleAnchor = inherit_chain(BirdAnchor) > WorldOrigin
```

or, for a different behavior:

```text
SparkleAnchor = snapshot_selected(BirdAnchor) > WorldOrigin
```

Do not get this behavior by implicit propagation through `Bird` object lifetime.

## 9. Remote reference model

Remote observed references should include:

- Place identity
- target identity
- membership epoch / participant incarnation
- freshness guard
- visibility/observation frontier
- read/observe capability
- optional write capability if explicitly granted
- witness refs where needed
- fallback chain
- label/redaction policy for debug output

## 10. Extension points

Open or later items:

- final surface syntax for `inherit_chain`, `snapshot_selected`, or lineage annotations
- richer reacquire semantics
- dynamic vs static contract-surface comparison granularity
- region polymorphism
- dependent lifetime reasoning beyond finite decidable fragment
- full proof mechanization

## 11. Minimal sample list

See `08-sample-matrix.md` for full IDs. The lifetime/fallback set must include at least:

- raw dangling reject
- fallback availability extension
- nested inherit valid
- plain ref non-inheritance
- underdeclared static error
- incompatible access target reject
- capability promotion reject
- write-after-read-only fallback reject
- rollback no lease resurrection
- atomic_cut no re-promotion
- bird/sparkle anchor inheritance
