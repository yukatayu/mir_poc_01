# 10 — Fallback / Guarded Reference

## Core reading

Fallback reference is a guarded option chain over one logical access path.

```text
Ref = [option1, option2, ...]
```

Each option carries:

- target
- lifetime/lease guard
- contract surface
- capability surface
- lineage evidence

## It does not extend target lifetime

Fallback extends access-path availability, not the lifetime of the short-lived target.

## Default non-propagating

Given:

```text
d = e fallback b
c = d fallback a
```

Default reading:

```text
c = ref(d) fallback a
```

If `d` expires, `c` falls to `a`.

It does not automatically become `e fallback b fallback a`.

## Explicit inheritance

If intended:

```text
c = inherit_chain(d) fallback a
```

Then canonical chain:

```text
e fallback b fallback a
```

only if same logical access path / semantic lineage evidence holds.

## Snapshot selected

```text
c = snapshot_selected(d) fallback a
```

captures current selected target, not whole chain.

## Optional seal sugar

A seal operator may exist later:

```text
seal(e fallback b)
```

but default must remain non-propagating.

## Static rejection

Reject before runtime if:

- missing target declaration
- missing lineage annotation when inheritance is requested
- successor capability strengthens
- contract surfaces contradict
- access path is not same lineage

## Samples

- raw dangling reject
- fallback extends access path
- inherited chain splice
- plain ref boundary preserved
- snapshot selected
- underdeclared fallback reject
- capability promotion reject
