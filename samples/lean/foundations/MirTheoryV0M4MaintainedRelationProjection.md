# MirTheoryV0M4MaintainedRelationProjection.lean

## Summary

This is the trusted finite M4 evidence model for a maintained relation and
consumer-local late projection. Its identifiers are intentionally domain
neutral: one relation owner, one consumer, one subject reference, and primary
and fallback anchor references. Application vocabulary is confined to
`SCN-12`.

## What is proved here

- a `publish-relation` projection evaluates to the same finite relative result
  as the owner-held relation in a coherent presentation context;
- semantic fallback is monotone from primary to fallback and never promotes
  automatically; a fresh binding epoch and witness epoch start a new lineage;
- a split-frame presentation gap returns the unchanged `BindingState`;
- stale anchor samples, split frames, relation cycles, consumer mutation, and
  private-to-public release are rejected in the finite model; and
- the explicit three-label order (`public < restricted < private`) computes a
  derived label that dominates every relation/input label.

## Compile

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M4MaintainedRelationProjection.lean
```

## Inventory sync

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## Boundary

This is a finite, self-contained relation/projection fragment. It is not a
general proof for arbitrary relation DAGs, Core programs, label lattices,
save/load, wire transport, renderer behavior, or the eventual runtime. Canon
proof status remains exclusively in `theory/11-metatheory-ledger.md`.
