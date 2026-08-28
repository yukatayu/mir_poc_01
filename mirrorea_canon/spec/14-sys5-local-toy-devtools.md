---
id: spec/14-sys5-local-toy-devtools
status: L1-fixed
maturity: reviewed
depends_on: [spec/02-surface-grammar, spec/03-static-semantics, spec/04-core-ir, spec/08-m7-checked-elaboration, spec/12-sys3-per-locus-projection, spec/13-sys4-in-process-generated-dispatch, adr/ADR-0031]
summary: SYS-5 finite four-locus local workflow、source-bound leave/fresh lifecycle、failure-atomic ST boundary、observer-safe joined devtools contract。
open_items: []
---

# 14 — SYS-5 local toy fabric and typed devtools

## Finite source-first workflow

The SYS-5 profile has one semantic input path:

```text
ordinary .mir source
  -> M6 parse/classify
  -> M7 checked elaboration
  -> SYS-3 GlobalProjectionResult
  -> source-derived complete M9 admission
  -> one SYS-4 LocalFabric
  -> deterministic observer-safe workflow report
```

The exact logical loci are `WorldAuthority`, `ParticipantA`, `ParticipantB`,
and `ViewerC`. The accepted sample names are not Core vocabulary. Project and
runtime code may consume checked/projected objects only; they may not reparse
source, select a plan from a filename, consult expected output, or accept a
manual edge, Core, authority, semantic state, or result.

`mir project-loci`, `mir run-local`, and `mir inspect` are provisional finite
facades over this path. Their names and JSON fields are not public/final
compatibility surfaces.

## Provisional explicit anchor locus

The bounded grammar refines each relation anchor with an optional internal
explicit locus:

```ebnf
PrimaryAnchor  ::= "primary" AnchorName [ "at" LocusName ]
                   "epoch" EpochName "transform" Transform
FallbackAnchor ::= "fallback" AnchorName [ "at" LocusName ]
                   "epoch" EpochName "transform" Transform
```

When present, `at L` is checked as a declared locus and retained exactly in the
AST, Core relation anchor, projection relation plan, required locus inventory,
and source/Core/artifact map. An unknown explicit locus is a typed
`UndefinedRelationAnchorLocus` diagnostic at that anchor locus span and emits
no checked executable Core.

When omitted, the legacy bounded form retains `None`; M7, topology, projector,
runtime, relation owner, and consumer must not infer a replacement. The SYS-5
leave/fresh lifecycle below admits only a relation with an explicit primary
anchor locus. The clause is provisional internal Surface v0, not final/public
grammar or a compatibility guarantee.

## Source-bound leave and fallback

The accepted external lifecycle action names a checked relation only. It is
not a new ordinary Surface form. The runtime resolves:

```text
checked relation
  -> exact explicit primary anchor locus
  -> exact sealed M9 participant membership/incarnation
  -> all checked capability/witness lineages scoped to that participant
```

A successful leave has this required order:

```text
external request/enqueue
  -> exact M9 membership/capability/witness retirement
  -> monotone successor/tombstone publication
  -> relation-owner primary invalidation
  -> generated fallback publication
  -> observer-safe lifecycle receipt
```

The relation owner remains the declared relation owner (`ParticipantB` in the
canonical sample). Retiring `ParticipantA` does not transfer relation authority
and does not let `ViewerC` mutate semantic relation state. M8 semantic state
does not change before the successful M9 retirement. Request, M9 retirement,
relation publication, and receipt have distinct typed occurrences and explicit
causal links.

A duplicate/stale leave or missing explicit primary anchor rejects without a
partial successor, capability/witness retirement, M8 state/relation/result
mutation, or fabricated success.

## Exact fresh reacquire

Fresh reacquire also names only the checked relation. It requires the completed
leave retained for that exact relation and consumes its exact tombstone:

```text
fresh.retired_membership_ref
  = leave.successor_tombstone_ref
fresh.retired_membership_epoch_ref
  = leave.membership_epoch_after_ref
```

M9 creates a distinct membership epoch and incarnation and fresh checked
capability/witness lineages. Only after the M9 fresh-membership occurrence may
the relation owner republish the primary through the generated endpoint. The
caller supplies none of the epoch, incarnation, membership, grant, witness, or
authority fields. A same-shaped but unjoined admission is not fresh reacquire.

## Failure atomicity and local cut

For the accepted ST profile, relation invalidation, ParticipantA leave, and
fresh reacquire are failure-atomic candidates:

1. identifier capacity and exact checked relation/anchor/endpoint inputs are
   validated before semantic mutation;
2. M9 and M8 changes are prepared on a detached candidate;
3. the live fabric is replaced only after the complete endpoint/causal result
   is valid; and
4. any failure preserves the prior live M9 generation, M8 state/relation/
   designated result, queues, counters, causality, and observer state.

The representative falsifier injects a relation-route failure; exact before/
after snapshots agree and the same source-bound action succeeds after the
fault is cleared. This finite candidate discipline is not a general
transaction theorem and creates no hidden multi-owner transaction.

A complete ST local cut includes completed participant leaves, their exact
retired lineage, relation shadows, M9 lifecycle/live floor, and the next
lifecycle occurrence floor. A cut made after leave and before fresh reacquire
restores that exact lineage and permits fresh reacquire. Altered leave evidence
or inconsistent cross-record lineage rejects before partial restore. No
durability or OW1 cut/lifecycle behavior is specified.

## Canonical action set

The selected workflow executes, in one deterministic local process:

1. ParticipantA attack as WorldAuthority owner-side RMW;
2. designated WorldAuthority tick/value publication and ViewerC named consume;
3. ParticipantB-owned maintained relation with ParticipantA primary,
   ParticipantB fallback, and ViewerC consumer-local projection;
4. local save, one discarded post-cut tick, and restore;
5. an accepted designated-only patch and rejected owner-RMW-changing patch;
6. ParticipantA leave, typed duplicate-leave failure, and fallback publication;
7. a ViewerC presentation gap that changes no semantic lineage or endpoint;
8. exact fresh reacquire and primary republication;
9. ViewerC consumer-capability revocation and typed failed consume; and
10. one finite optional-verification residual/discharge.

Patch activation precedes leave/fresh in this profile. Patch activation after
an already advanced fresh membership lineage, arbitrary lifecycle/patch
commutation, and general patch compatibility are not specified.

## Joined observer-safe report

One ordered document must provide enough typed fields to follow:

```text
source span
  -> checked Core operation
  -> source/target LocusProgram fragments
  -> generated CommunicationEdge
  -> request identity
  -> enqueue / dispatch / receive / observe / serve occurrence
  -> mutation, result, failure, relation, cut, or patch lifecycle evidence
```

Request identity is distinct from queue position and every occurrence.
Source/Core/artifact/edge fields are retained from actual checked/projected
rows; the report cannot synthesize a route or causal occurrence. Save/restore
rows distinguish `active_prefix`, `discarded_post_cut`, and
`active_restored`. Patch rows join safe logical source provenance and checked
program identity to the exact actual patch occurrence and verdict.

Relation rows show owner, selected anchor/floor, semantic digest, lineage,
leave→fallback, presentation-gap nonmutation, and exact leave→fresh lineage.
Designated rows show result version, publication/delivery/cache binding, and
typed post-revocation failure. Verification rows distinguish residual from
the finite discharge.

Observer-safe export contains only opaque references and declared safe values.
It must not expose raw source text, credential, capability/witness material,
private state names/values, private payload, or raw M8/M9 identity. Observation
is evidence and cannot mint authority or semantic state.

## Evidence boundary and non-claims

OBL-062 classifies the exact cut
`53a21e64b5a17e24b522f720db10b6e539c058e0` as `runtime-monitored` for this
finite source-first workflow, lifecycle, failure-atomic candidate, post-leave
cut continuity, and observer-safe joined report. It is not Lean-proved or
generally model-checked.

This specification does not define public CLI/API/ABI/wire/JSON, final
grammar, browser/View/renderer, real transport, multi-process execution,
durable persistence, OW1 lifecycle/cut/patch, arbitrary relation DAG,
arbitrary patch/lifecycle commutation, a general projection/authority/
failure-atomicity/save/reacquire/noninterference theorem, broad PHASE-I1, or
official I2 entry/exit. SYS-6 is the direct consumer.
