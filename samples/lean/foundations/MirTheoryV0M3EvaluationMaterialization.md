# MirTheoryV0M3EvaluationMaterialization.lean

## Summary

This is the actual Lean evidence for the finite M3 evaluation/materialization
fragment. It defines the five `EvalPlan` axes, canonical finite producer
frontiers, explicit success/failure receipt classification with a finite
request/serve/reply/receive order, a serial owner RMW service, and a
designated result store. Its parser-free trace rows carry deterministic finite
operation-origin keys, not M6 Surface spans.

## What is proved here

- elaboration is deterministic for the declared finite input syntax;
- unannotated, failed, target-mismatched, release-unadmitted, or causally
  incomplete cross-owner receipt forms are rejected;
- same-owner owner-service attacks retain caller authority, evaluate at the
  owner, and yield `100 → 90 → 80` when two requests are served;
- missing capability leaves the owner state unchanged, and the bounded owner
  service preserves its non-negative-state well-formedness condition; and
- duplicate designated decision is stable under frontier permutation, while
  the bounded result-consumption state is idempotent.

Compile with:

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M3EvaluationMaterialization.lean
```

## Boundary

The model uses one owner cell, one typed `Int` receipt, and one designated
result lineage. It is not a theorem for arbitrary Core programs, relation
projection, fallback, save/load, patching, transport, or distributed atomicity.
Canon proof-status classification remains in theory/11.
