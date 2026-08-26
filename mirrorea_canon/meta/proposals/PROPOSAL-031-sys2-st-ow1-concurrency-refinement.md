---
id: meta/proposal-031
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0026, adr/ADR-0027, arch/04-runtime-carriers, theory/04-ordering-and-cuts, theory/05-authority, theory/13-evaluation-materialization, theory/18-m9-auth-verification, spec/05-runtime-semantics]
summary: SYS-2のdeterministic STとsingle-owner OW1 backend、M9 successor generation visibility、有限ordering modelを固定する提案。
open_items: []
---

# PROPOSAL-031 — SYS-2 ST/OW1 concurrency and visibility refinement

## Owner disposition and selected capability

Under ADR-0026, accept the smallest SYS-2 backend contract that lets SYS-3
place executable requirements in per-locus artifacts and lets SYS-4 run those
artifacts without importing low-level memory-order vocabulary into ordinary
Mir Surface.

Two internal execution profiles are selected for the admitted SYS-1 kernel
fragment:

```text
ST:
  deterministic single-thread reference execution

OW1:
  one coordinator + one dedicated owner worker
  exactly one combined semantic owner/source-owner locus per kernel
  worker-exclusive M8LocalRuntime
  zero-capacity synchronous mailbox and acknowledged commands
```

OW1 rejects a checked admission with zero or more than one combined owner/
source-owner locus as `ExecutionProfileUnsupported` before it can duplicate
or expose owner state. The coordinator retains checked Core, typed carriers,
kernel queues/receipts, and ordering evidence, but it has no public shared
mutable M8 store. This is a bounded one-owner-worker profile, not the final
multi-locus dispatch architecture.

## Abstract order and operation evidence

Backend execution must preserve the high-level Mir order of theory/04. For the
selected owner operation, successful mutation linearizes at the actual M8
`OwnerWrite` trace node acknowledged by the owner worker. The associated
`OwnerRead`, written per-key version, preceding writer/request, and M8 enqueue
occurrence form the bounded reads-from and per-location coherence evidence.
A declared or authority-rejected serve has no fabricated write
linearization point, reads-from edge, or version advance.

For designated remote input, the source-owned value is read from the same
worker-owned M8 state and the reply is derived from that acknowledged read.
The lifecycle preserves:

```text
request -> source-owner serve/read -> reply -> receive/receipt
        -> designated-evaluator consume
```

A caller-supplied result that differs from the acknowledged source-owner read
is rejected as `RemoteInputValueMismatch` before reply, receipt, or semantic
mutation. Producer release authority remains distinct from evaluator decision
authority. The lifecycle is a bounded typed effect request/result instance,
not a generic provider registry.

## Live M9 generation and revocation visibility

The admitted kernel retains the successor publisher from its own sealed M9
seam. A production revocation names only the checked owner operation; it does
not accept a caller-constructed generation or capability. M9 performs the
actual revoke, retranslates the complete admitted authority inventory, and
produces an immutable successor generation with:

```text
same checked program identity
strictly increasing generation
monotone retained revocation tombstones
unrelated owner and designated-release lineages preserved
```

ST installs the translated M8 authority inventory directly. OW1 sends it to
the sole owner worker and waits for acknowledgement. Only after successful
refresh is the new generation published to the kernel; failure leaves the
prior generation and publisher live. Consequently:

- a request queued at generation 0 and served after acknowledged generation 1
  revocation fails with typed `MissingCapability` and does not mutate owner
  state;
- a mutation whose owner write completed before generation 1 remains
  completed; a later reply/receipt carries outcome and causality but no
  authority; and
- a new owner use after generation 1 cannot be constructed from the revoked
  lineage.

Receipt arrival is never a grant or authority transfer. Transport metadata,
worker identity, mailbox order, and generation numbers are not authority.

## Bounded model and evidence boundary

Accept one executable finite transition model at bound 6 for the selected
action alphabet. Its required-edge families are:

```text
owner request -> serve
publish -> observe
witness create -> use
capability grant -> use
revocation publish -> later serve/use
patch activation -> later request
save cut/quiescence -> later mutation
relation epoch -> coherent sample
same-owner reads-from/coherence
presentation gap -> semantic nonmutation
```

Each missing-edge profile reaches a typed bad-state predicate with a replayable
state/transition trace. The full-edge ST and OW1 model runs have the same
selected observable result and deterministic case coverage. Store buffering
uses a separate `WeakMemoryCalibration` profile with explicit buffer/flush/
read state; its `0/0` outcome calibrates the missing publication edge and is
not silently called a Mir failure.

The model also records a non-vacuous rejected source-free authority attempt,
revoked-use no-mutation, separate stale-patch reject versus stale execution,
unique terminal states, exact state fingerprints, and bounded-search
completeness. This is `model-checked-bounded`, not a general proof of a
scheduler, hardware memory model, fairness, liveness, or all traces.

The actual ST/OW1 kernel runs are `runtime-monitored`. No Lean statement or
theorem is added, and no existing general OBL is discharged.

## Primary falsifiers and accepted evidence

The contract is falsified if ST and OW1 differ on the selected semantic
result; OW1 exposes/duplicates owner state; an M9 successor loses unrelated
lineage; a stale queued use mutates after acknowledged revocation; a failed
serve receives a fabricated commit/read/version edge; remote input is supplied
instead of derived from the source-owner read; or removing a required edge
cannot reach and replay the corresponding bad state.

The accepted source/evidence cut is
`920d3fe050b8b909253f8511d9ad897272323ced`. Evidence comprises 27/27 combined
SYS-2 tests, 13/13 SYS-1 regression tests, M10 source 2/2, CLI 4/4, conformance
67/67, the full `mir-runtime` suite, formatting, warnings-denied Clippy, diff
check, and independent semantic/specification, concurrency/code-quality,
finite-model, and test-contract reviews with no remaining P0/P1/P2 finding.

## Direct consumer and stop condition

```text
Direct consumer: SYS-3 per-locus projection and SYS-4 generated dispatch
Blocker reduced: threading and live authority publication lacked a bounded
  semantic refinement contract that generated artifacts could preserve
Acceptance use: SYS-3 artifact requirements, SYS-4 ST/OW1 execution, and the
  later finite SYS-6 correspondence profile
```

SYS-3 consumes the selected profile as an internal artifact requirement: a
future locus program may name ST or the provisional OW1 backend contract, but
must not expose concrete Rust channels, worker tokens, memory orders, or public
ABI. SYS-4 consumes both profiles over the same generated artifacts.

Close SYS-2 at this bounded contract. Reopen it only for an admitted selected-
fragment trace that violates the recorded high-level order, a stale authority
mutation past acknowledged generation publication, an unexplained ST/OW1
semantic difference, or a SYS-3/4 direct consumer that cannot conservatively
use the internal profile.

## Non-effects

This proposal does not add `memory_order_*` or worker/channel vocabulary to
Surface, reinterpret `atomic_cut` as a fence, admit more than one OW owner
locus, define multi-locus generated dispatch, select atomics or lock-free
algorithms, prove data-race freedom generally, select a scheduler/fairness
model, freeze public API/ABI/wire, define real transport, or change theory T1,
broad PHASE-I1 acceptance, or official I2 lifecycle state.
