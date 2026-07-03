---
id: spec/05-runtime-semantics
status: L2-working
maturity: draft
depends_on: [theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority]
summary: 参照実装が満たすべき観測可能挙動と、適合試験用の決定的スケジューリング profile。
open_items: [OPEN-027]
---

# 05 — Runtime semantics (observable behavior)

An implementation conforms if its observable behavior (verdicts, occurrence
rows, store states at cuts, diagnostics) matches the calculus under the
conformance profile.

- **Request lifecycle**: emitted → enqueued at owner → validated (epoch,
  incarnation, capability lineage, witnesses, visibility) → served | explicit
  failure ∈ declared row. Fail-closed: validation failure changes no store.
- **Owner seriality**: one owner's store mutations are totally ordered by its
  serve loop (ADR-0003). Cross-owner interleaving is otherwise free.
- **Membership**: join/leave bump epoch; leave retires the incarnation and
  tombstones entries; rejoin creates a fresh incarnation; stale-epoch
  messages are rejected with StaleMembership.
- **Cuts and save/load**: `atomic_cut` per theory/04; save produces a
  SaveObject at a consistent cut; load applies the admissibility checks and
  refuses stale resurrection (SCN-10 binds this).
- **Patching**: pipeline verdict surfaces (accepted/rejected/deferred) and
  no-mutation on reject are observable (SCN-09).
- **Observation**: exports pass authority/redaction/retention; observer_safe
  never contains raw witness/auth payloads.

## Deterministic conformance profile (testing only, not semantics)

Single process; loci stepped round-robin in declaration order; each owner
serves its queue FIFO, one request per turn; handler invocations from the
scenario script are injected between turns; RNG is a named provider seeded by
the scenario; timestamps are logical (turn counter). This profile exists so
SCN expectations are exact; real deployments are nondeterministic within the
calculus.

OPEN-027: reply/receipt observability for read-requests (with OPEN-011).
