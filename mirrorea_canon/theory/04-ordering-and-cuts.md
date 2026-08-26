---
id: theory/04-ordering-and-cuts
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, adr/ADR-0007, adr/ADR-0028]
summary: 因果関係族、SYS-2 ST/OW1 bounded refinement、atomic_cut、consistent cut、SaveObject、load許容性、Z-cycle、durable_cut(all_of)。
open_items: [OPEN-015, OPEN-016]
---

# 04 — Ordering, cuts, save/load

## Causal order

History is `(E, ≺)`, ≺ = transitive closure of the generating family:

```text
program_order            state_dependency_order
send → receive           publish → observe
witness_create → witness_use          capability_grant → capability_use
auth_evidence_create → use            membership_update → dependent dispatch
admit_request → verdict → activation_cut
fallback_degrade → later access on same lineage
cut(ℓ) → later local transition at ℓ
```

Acyclicity is a well-formedness invariant. The source-level principal is this
high-level family (ADR-0007); `memory_order_*` profiles belong to the
model-check line and to implementation mappings only.

## SYS-2 bounded backend refinement

For the selected SYS-1 operation fragment, an implementation trace refines
the Mir history only when each observed lifecycle and successful semantic
mutation embeds into the high-level order above while preserving its owner,
typed result/failure, source/Core provenance, and authority generation.
ADR-0028 fixes two finite profiles:

```text
ST  = deterministic single-thread reference
OW1 = one coordinator + one acknowledged mailbox + one worker-exclusive
      owner store for exactly one combined owner/source-owner locus
```

The selected successful owner mutation has one linearization point at its
actual owner-store write. Its same-owner read records either the admitted
initial version or the preceding successful write; per-key version advance
supplies the bounded modification/coherence order. A failure has no write
linearization or version advance. Remote input preserves request → source-
owner serve/read → reply → receive → evaluator consume and derives its value
from that source-owner read.

Live authority change follows:

```text
M9 revoke and complete retranslation
  -> owner runtime refresh acknowledgement
  -> immutable generation publish
  -> later serve/use observes the generation
```

A queued earlier-generation use served after this publish rejects without
mutation. A write linearized before the publish remains complete; its later
reply/receipt does not transfer authority. This is the selected finite
revocation-visibility rule, not a general memory model.

The SYS-2 bounded model removes one of ten required edge families at a time:
request/serve, publish/observe, witness/create-use, capability/grant-use,
revocation/use, patch-activation/request, save-cut/mutation, relation-epoch/
sample, same-owner reads-from/coherence, and presentation-gap/nonmutation.
Each removal has a replayable typed bad-state witness at the declared bound.
Store buffering is a separate weak-memory calibration; it neither defines Mir
semantics nor imports low-level ordering syntax into Surface.

## atomic_cut

`cut(ℓ)` fixes ℓ's rollback frontier. Property:

```text
If cut(ℓ) ∈ H, rollback at ℓ cannot remove occurrences causally before it
within ℓ.
```

It is not a distributed commit, durable checkpoint, global sync point, or
fence. Post-cut failures are handled by compensation / fallback / explicit
failure, never by implicit rollback across the cut.

## Consistent cut

```text
Consistent(Kc) := ∀e ∈ Kc. ∀e′. e′ ≺ e ⇒ e′ ∈ Kc      (prefix closure)
```

Consequences: receive∈Kc ⇒ send∈Kc (or channel state carries it);
observe ⇒ publish; witness_use ⇒ witness_create; capability_use ⇒ its grant
frontier; activation_cut ⇒ its request and verdict; membership-dependent
dispatch ⇒ its membership frontier.

## SaveObject

Saving is cut-backed, never a byte copy:

```text
SaveObject = { cut, causal_frontier, place_states, queues,
  in_flight_messages, membership_registry, place_catalog, capability_state,
  auth_evidence_store, witness_store, lease_store, fallback_positions,
  hotplug_lifecycle_state, package_versions, adapter_state,
  external_effect_obligations, provenance }
```

M4 adds every live owner-held `RelationDef`/`BindingState`, selected fallback
position, relation lineage, binding/witness/anchor epochs, and activation
frontier to this provenance closure. M5 must integrate those carriers with
designated-result versioning and receipt/consumption in the shared model. This
document deliberately selects neither field names nor an internal
representation or serialization. Temporary consumer presentation samples and
local no-frame decisions are not semantic binding state and do not imply
per-sample persistence. Whatever carrier M4/M5 selects belongs inside the
SaveObject consistent cut and its provenance closure, never in untracked side
state. This is a required semantic closure, not a general M4 load/restore proof
or implementation claim.

Widening candidates (two-layer time, theory/09): computational_state,
anchor_graph, pose_snapshot_frontier, pose_versions, anchor_switch_state — if
admitted they live *inside* SaveObject under Consistent(cut), never as side
state. M4's finite relation model does not select or implement these widening
candidates.

## Load admissibility and THM-003

Load may succeed only if: Consistent(cut) ∧ no rollback across atomic_cut ∧
no stale membership / witness / lease resurrection ∧ capability & auth
provenance connected ∧ every M4/M5 relation/result/consumption carrier's
lineage connected ∧ package versions compatible ∧ external irreversible
effects compensated or isolated.

```text
THM-003: Load(SaveObject) succeeds ⇒ the restored configuration is well-formed,
its history prefix is Consistent, and no expired lease, stale witness, stale
membership epoch/incarnation, retired indexed entry, or severed provenance is
live in the restored state.
```

Reacquisition after load is a new occurrence / epoch / witness — never hidden
repair. (OBL-010..013.)

## Z-cycles

In a distributed checkpoint graph, a checkpoint on a zigzag cycle (Netzer–Xu)
can belong to no recoverable global cut:

```text
ZCycle(c) ⇒ c is inadmissible for a recoverable global cut.
```

The checker's structural reject (OBL-014: equivalence with the Netzer–Xu
useless-checkpoint characterization) is a guard, not a recovery protocol.

## durable_cut (Mir-1 vocabulary, L2)

Minimal meaning: a successful pre-cut prefix does not return to an
undetermined state after local rollback, process restart, or route rebinding.
Failure default is `Reject` (Compensate only to unwind externalized
obligations; Approximate only if the contract explicitly weakens durability).
Cross-locus scope profile: **all_of only** — aggregate success requires
counted success-side local observation + persistence evidence for every
participating locus; coverage shortfall alone does not justify aggregate
failure (an explicit failure justification is required: a required member's
local failure, or an explicit failed closure). Per-member audit must
distinguish {covered, impossible-by-local-failure, unfulfilled} at the
decision point. `quorum-like` is a future profile; `implementation-defined` is
not a profile. Realization (storage/replication/consensus) is fabric-side.

Local-only save/load vs distributed durable save/load remain distinct
readiness classes (plan/01: R2 vs R3/R4).

OPEN-015: `barrier` as an independent ordering primitive — keep deferred.
OPEN-016: communication-induced checkpointing repair strategies (later gate).
