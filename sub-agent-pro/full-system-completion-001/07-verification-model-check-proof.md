# 07 — Verification / Model Check / Proof

## Three verification lines

### Line 1: Static checker

Handles decidable finite facts.

- types
- imports
- scope
- effect/failure row containment
- capability declarations
- fallback evidence
- package schema
- cut structural rejection

### Line 2: Model-check second line

Handles finite transition systems.

- interleavings
- stale membership
- reset/handoff races
- small cut graphs
- small hot-plug activation order
- weak-memory profiles
- two-shard handoff cases

### Line 3: Proof side

Handles semantic lemmas.

- type soundness
- fallback monotonicity
- no re-promotion
- no split-frame theorem
- no double authoritative owner
- contract-subtyping composition
- no stale fact resurrection

## Residual obligations

Every unproven claim must be explicit.

```json
{
  "obligation_id": "...",
  "obligation_kind": "model_check|proof|kept_later",
  "source_refs": [],
  "required_context": {},
  "current_status": "undischarged|discharged_elsewhere|deferred"
}
```

Do not treat residual obligations as hidden success.

## Isabelle / Lean relation

Mir is not Isabelle or Lean.

Mir source does not become a proof assistant language.

But Mir may export obligations to Lean / Isabelle / TLA+ / model-check tools.

## Required proof sketches

1. Pure computational preservation
2. Effect row containment
3. Failure row containment
4. Fallback monotone degradation
5. No hidden fallback inheritance
6. Atomic cut rollback boundary
7. Consistent cut prefix closure
8. No stale membership/witness/lease resurrection
9. No split-frame for PoseGraph snapshot
10. Single-owner shard handoff invariant
