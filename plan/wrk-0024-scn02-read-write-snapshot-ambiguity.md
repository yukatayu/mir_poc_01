# WRK-0024 - SCN-02 read/write snapshot ambiguity evidence

## Role and authority

This is LAB evidence for `working/WRK-0024` at the pinned source cut
`fcf5ea613c2153667e1c4a887589fb939692c7a5`. It is a finite countermodel of
the implication from **owner-serial submitted writes alone** to an atomic
read-dependent update. It is not a Canon execution trace and does not choose
a snapshot, evaluation locus, request/reply/pending carrier, Core rule,
concurrency model, scenario result, theorem, or implementation.

## Source anchors

| Anchor | Literal input used |
| --- | --- |
| theory/01 | Cross-locus reads may become request/observe; service may do `read+reply`; owner queues serialize store mutation; reply/receipt remains OPEN-011. |
| theory/03 | SCN-02 worked shape emits a computed cross-owner write request, but does not supply a result-binding or evaluation/snapshot relation. |
| spec/05 | Conformance queues serve one request per owner turn; real calculus scheduling remains nondeterministic. |
| SCN-02 | Both `player[target].hp` and `player[self].atk` are dependencies of the computed assignment. |
| P012 | V1/R1 is a recorded direction, not an amended operational rule. |

## Finite model

The scratch model has initial HP `10`, two independently delivered target-HP
read values `10`, and two serially served submitted writes `7` and `6`.

```text
ownerSerialFinal(10, [7, 6]) = 6
atomicDamageFinal(10, [3, 4]) = 3
```

Both lists are processed sequentially at the owner. The difference is solely
whether subtraction occurs from an earlier external snapshot or at the owner
against its then-current state. The model deliberately contains no Mir request,
authority, membership, queue, event, or reply representation.

## Reproduction

Scratch file (not committed):
`/tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean`

```bash
lean --trust=0 /tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean
python3 -c "from pathlib import Path; text = Path('/tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean').read_text(); required = ('stale_final_is_six', 'atomic_final_is_three', 'owner_seriality_alone_does_not_imply_atomic_result'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by', 'Classical', 'Choice'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"
```

Observed tool: Lean 4.29.1. Both commands passed. Scratch SHA-256:
`9c02e90a8accaf156dffd4ee14c9fc10052a8d6f16b2ec6e82fca85b99b15cac`.

## Result

The registered finite model proves the limited non-implication:

> Per-owner serialization of already-computed writes does not by itself imply
> serializable/atomic read-dependent updates.

It therefore blocks a later shared model from treating owner seriality as an
unstated solution for SCN-02. The result does **not** say that current Canon
selects the stale schedule, that SCN-02 is invalid, or that Mir needs any
particular repair.

## Required next decision boundary

Any design that rules out the model must explicitly compare at least these
families, then stop for the ordinary Canon process when selecting one:

1. evaluate the relevant read and mutation at the owner as one served
   operation;
2. define an explicit snapshot/read-version validation relation; or
3. reject/serialize a dependent update whose source snapshot is no longer
   admissible.

The present record does not rank these families. Each interacts with the still
open V1/R1 pending-control, M1 request identity/replay, and SW1 occurrence
facets; no implementation may silently fill those gaps.

## Non-claims

No Core constructor, grammar production, `+=`/`-=` equivalence, request field,
read receipt, pending state, occurrence edge, snapshot isolation level, queue
policy, SCN expectation, OBL, `theory/11` status, Gate/Phase, conformance,
runtime, transport, persistence, public API, or product claim changes.
