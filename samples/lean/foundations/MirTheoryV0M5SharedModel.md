# MirTheoryV0M5SharedModel.lean

## Summary

This is the M5 trusted finite shared model.  It is a fresh, self-contained
Lean universe, not an import or adapter layer over the M3 or M4 foundations.
One finite carrier supplies semantic `SurfaceFragment` tags and `SourceRef`,
deterministic elaboration to `Core` or `StaticDiagnostic`, `Config`,
`SemanticStep`, `WellFormed`, occurrence history, typed observation output,
maintained-relation projection, and the cut/save/restore interface.

## Concrete boundaries

- `ResultFrontier`, `ResultVersion`, `RelationFrontier`, `Epoch`,
  `PresentationContext`, and `AtomicCut` are nominally distinct Lean types.
  In particular, an M3 designated-result producer frontier cannot be used as
  an M4 relation activation frontier.
- `Config` concretely carries a finite history, pending request tags, owner
  state, membership epoch, capability/witness lineage, lease state, receipt
  store, designated-result store, relation/binding store, an optional
  published-projection carrier, and an explicitly inactive patch slot.
- `PresentationContext` is read-side input to projection only.  It is neither
  a `Config` field nor representable in `SaveObject`; samples and local gaps
  are therefore not saved semantic binding state.
- Relation mutation validates the owner principal plus the exact relation,
  binding epoch, membership epoch, capability, and witness records.  Each
  primary/fallback sample separately validates the consumer release, anchor,
  epoch, and activation frontier.
- Coordinates and transforms use a closed four-value domain.  Out-of-domain
  derived coordinates reject; no wrapping, saturation, or fallback is used.

## Finite theorem evidence

The named theorems cover deterministic elaboration, explicit receipt use,
designated-result duplicate stability, owner credential rejection, finite
fallback/reacquire, project/evaluate correspondence, per-sample admission,
label propagation, consumer materialization/J-mutation rejection, the mixed
actual owner-bind → owner publication → `consumerProject` trace (which reads
the published relation/binding carrier), finite step preservation, and
cut-backed save/restore stale-witness rejection.

The file also prints Lean axiom dependencies for the selected theorems.  It
declares no user `axiom`, uses no `sorry`/`admit`, and has no opaque predicate
or propositionally vacuous placeholder.

## Compile

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M5SharedModel.lean
```

## Inventory sync

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## Boundary

This is finite evidence for the exact profile recorded in
`mirrorea_canon/theory/15-shared-formal-model.md` and
`theory/11-metatheory-ledger.md`.  It is not a proof of general Surface
grammar, arbitrary Core programs, arbitrary relation DAGs or label lattices,
general save/load or cuts, patch semantics, transport/distributed execution,
runtime conformance, or a public API/wire contract.
