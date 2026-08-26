---
id: spec/05-runtime-semantics
status: L2-working
maturity: draft
depends_on: [theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, theory/13-evaluation-materialization, adr/ADR-0027]
summary: 参照実装の観測可能挙動、SYS-1 internal kernel lifecycle、適合試験用の決定的 scheduling profile。
open_items: [OPEN-027]
---

# 05 — Runtime semantics (observable behavior)

An implementation conforms if its observable behavior (verdicts, occurrence
rows, store states at cuts, diagnostics) matches the calculus under the
conformance profile.

- **Request lifecycle**: carrier pre-admission validates exact checked
  source/Core, target/origin, epoch, incarnation, capability/witness lineage,
  and visibility before creating a semantic request occurrence or enqueuing
  owner work. An admitted request follows request → serve → reply →
  receive/receipt and yields typed success or an explicit failure in its
  declared row. Fail-closed: either rejection changes no semantic store.
- **Owner seriality**: one owner's store mutations are totally ordered by its
  serve loop (ADR-0003). Cross-owner interleaving is otherwise free.
- **Evaluation/materialization**: an owner transition evaluates its same-owner
  reads and RHS only at validated owner service; the requester does not obtain
  private operands or blind-write a stale value. An other-owner operand needs
  the explicit receipt boundary in theory/13 or a static Diagnostic. A
  designated evaluator decides and publishes one versioned value per
  evaluator/key/canonical-frontier; an explicit consumer occurrence consumes
  that value without re-deciding it.
- **Maintained relation / late projection**: an owner holds relation definition
  and binding state, and may publish only an admitted `publish-relation`
  projection. A consumer evaluates it locally at presentation-frame from a
  coherent one-frontier presentation context; it does not receive a derived
  absolute value or adapter stream and cannot mutate the binding. Semantic
  invalidation advances the owner-recorded selected relation option
  monotonically; fresh witness+binding-epoch reacquire begins a new lineage.
  A missing, stale, or split-frame presentation sample yields a local gap or
  reject without semantic binding mutation. Derived release respects the
  greatest restriction of relation/input labels.
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

## SYS-1 bounded kernel profile

The crate-private production kernel profile is admitted only from an exact
checked program and the sealed M9 execution seam. It owns the admitted M8
runtime while serving ordinary `run_source` and generic checked `OwnerEvent`
operations. Kernel behavior does not depend on M10 profile predicates,
correspondence verification, release manifests/anchors, or CLI rendering.
Specialized historical SCN-04/09/10 and route-patch runners are retained M10
regression paths and do not witness this profile.

For an owner request `r`, the observable internal state machine is:

```text
Unissued
  -- checked/M9 pre-admit --> Queued(request-id, request-occurrence)
  -- owner FIFO serve --> Served(serve-occurrence, outcome)
  -- reply --> Replied(reply-occurrence, typed-success | declared-failure)
  -- receive --> Receipted(receive-occurrence)
```

For a checked designated remote-input dependency `d`, it is:

```text
Unissued
  -- checked producer-release pre-admit --> QueuedAtSourceOwner
  -- source-owner read --> Served
  -- reply --> Replied(typed-success | declared-failure)
  -- receive --> Receipted(input-frontier, release-tuple)
  -- exact designated evaluator --> ConsumedOnce
```

Queue position is not request identity. Reply/receipt is single-assignment;
unknown, duplicate, mismatched, stale, wrong-target, wrong-origin, wrong-
source/Core, source-free, or visibility-invalid input rejects without
unintended mutation. The producer's remote-input release lineage cannot be
substituted by evaluator decision authority. A received receipt transfers no
authority and provides no direct store-write rule.

The remote-input lifecycle is the selected bounded typed effect request/result
boundary. It is not a generic handler/provider registry. The runtime neither
retries nor promises exactly-once delivery implicitly. M9 admission is an
immutable snapshot in this profile; revoke-after-enqueue/serve visibility is
deferred to the SYS-2 ST/OW happens-before contract.

## Deterministic conformance profile (testing only, not semantics)

Single process; loci stepped round-robin in declaration order; each owner
serves its queue FIFO, one request per turn; handler invocations from the
scenario script are injected between turns; RNG is a named provider seeded by
the scenario; timestamps are logical (turn counter). This profile exists so
SCN expectations are exact; real deployments are nondeterministic within the
calculus.

OPEN-027 remains: external/runtime receipt delivery observability beyond this
single-process internal typed occurrence boundary. OPEN-030 is resolved only
for the ADR-0027 I2-internal carrier; public API/ABI/wire and the broader
carrier freeze remain unset.
