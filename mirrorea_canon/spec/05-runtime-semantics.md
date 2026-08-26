---
id: spec/05-runtime-semantics
status: L2-working
maturity: draft
depends_on: [theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, theory/13-evaluation-materialization, adr/ADR-0027, adr/ADR-0028]
summary: 参照実装の観測可能挙動、SYS-1 lifecycle、SYS-2 ST/OW1 profile、適合試験用deterministic scheduling。
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
retries nor promises exactly-once delivery implicitly. Initial M9 admission is
an immutable generation; ADR-0028 defines the bounded successor visibility
contract below.

## SYS-2 ST/OW1 runtime profile

ST preserves the deterministic single-thread event loop. OW1 admits exactly
one combined owner/source-owner locus, whose dedicated worker exclusively owns
the M8 runtime. Coordinator/worker communication is acknowledged; there is no
public shared mutable M8 store. Any other combined-locus count rejects with
`ExecutionProfileUnsupported` before executing owner work.

For an owner request:

```text
kernel pre-admit
  -> owner-runtime enqueue acknowledgement
  -> owner read
  -> successful owner write (linearization point) | declared/authority failure
  -> kernel reply -> receipt
```

The successful record names the actual M8 enqueue, `OwnerRead`, and
`OwnerWrite` trace nodes plus per-key version/preceding-writer information.
Failure records no successful linearization, reads-from, or version advance.
Two same-owner RMW requests therefore observe the serialized versions in both
ST and OW1.

For designated remote input, source-owner service reads worker-owned state and
the kernel derives the reply from that read. A mismatched explicit result
returns `RemoteInputValueMismatch` before reply/receipt/mutation. The result
then follows the SYS-1 reply/receive/consume order; acknowledgement and worker
identity do not confer authority.

Live revocation is:

```text
same-seam M9 revoke + complete retranslation
  -> ST install | OW1 owner-worker install+ack
  -> kernel generation publish
```

Only a same-program strictly newer generation with monotone tombstones is
accepted. Unrelated admitted owner and designated-release lineages remain
available. A queued old-generation use served after publish fails typed with
no mutation; a write completed before publish remains completed, and its later
receipt is non-authority. Failed installation retains the prior generation.

The bounded executable model covers the ten high-level edge families in
theory/04 at bound 6 and emits replayable missing-edge counterexamples. Its ST
and OW1 full-edge selected observations agree. `WeakMemoryCalibration` is a
separate store-buffer/flush/read model and is not a third Mir execution
profile. These are finite internal requirements for SYS-3/4, not public
backend/API/ABI or general concurrency guarantees.

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
