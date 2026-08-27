---
id: spec/12-sys3-per-locus-projection
status: L1-fixed
maturity: draft
depends_on: [spec/04-core-ir, spec/08-m7-checked-elaboration, arch/03-toolchain, arch/04-runtime-carriers, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, adr/ADR-0029, adr/ADR-0030]
summary: cut 3013e7feで受理したSYS-3 bounded source/Core projectionと、SYS-4が実現したgenerated endpoint refinementへのstatic handoff。
open_items: []
---

# 12 — SYS-3 per-locus projection

## Input and output boundary

The selected finite compiler route is:

```text
CheckedSurfaceV0 + DeclaredLogicalTopology
  -> project_checked_core
  -> GlobalProjectionResult | ProjectionDiagnostics
```

The checked artifact is the only source-program input. The topology is bound
to its exact `CheckedProgramIdentity` and supplies only a set of logical loci.
That set must equal the union of checked static-environment loci and every
owner/origin/consumer/evaluator/source-owner locus referenced by checked Core.
Duplicate, missing, extra, or identity-mismatched topology fails without a
partial result. Topology never supplies an interface, edge, schema, authority,
failure row, handler, host, or expected result.

The output owns its checked fragments and is deterministic under topology and
input iteration order. It has no AST dependency and no runtime/conformance
facade dependency. Internal Rust names and the provisional profile string are
not a public API, ABI, JSON schema, wire contract, or compatibility promise.

The projector's no-AST dependency does not mean SYS-3 may omit a source fact.
The bounded non-final internal Surface-v0 clause
`designated consume E.result at C` must first create a distinct AST item, M6
template/source map, and M7 `DesignatedResultConsume` checked Core edge. Only
that checked edge may generate the evaluator-to-consumer artifact/communication
path. Topology, schedule, relation, or deployment metadata cannot supply or
infer `C`. This is not final/public grammar or a public Core/API/ABI/wire
contract.

## Required result

For this finite profile, `GlobalProjectionResult` contains:

| Component | Required content | Required non-effect |
|---|---|---|
| identity | checked program identity, exact topology, internal projection profile | no M10 release/profile identity reuse |
| `LocusProgram` | locus tag; placement-specific typed checked fragments; source/Core/artifact identity; exact failure rows; semantic and runtime-seam obligations | no AST/source reconstruction; no remote store handle |
| `CommunicationPlan` | every checked cross-locus owner, relation, and designated edge; source and target fragment refs; typed lifecycle contract | no handwritten edge; no authority transfer |
| `EffectHandlerPlan` | source/Core-bound owner service, designated source service, designated evaluator | no generic provider registry |
| `ProjectionRelationGraph` | each checked two-anchor relation and its primary -> fallback edge | no production nested-relation dependency |
| `ObservationPlan` | source/Core/fragment/edge-bound required future occurrence rows with reference-only redaction | no claim that an occurrence happened |
| `PersistencePlan` | per-locus, per-relation/designated, and whole-fabric responsibility assignment | no save, restore, or durable format |
| `ProjectedSourceMap` | source -> Core -> fragment/artifact -> edge/plan correspondence | no report/manifest lookup as semantic source |
| readiness/backend | static readiness, separate runtime admission status, ST/OW1 semantic requirements | no runtime admission or worker/channel ABI |

An empty residual row yields `AwaitingRuntimeSeam`, not admitted execution. A
nonempty residual row yields `BlockedByResidual`. ST is supported for every
successful finite projection. OW1 is eligible only for exactly one combined
semantic owner/source-owner locus, as required by ADR-0028.

## Placement rules

For an accepted owner RMW:

```text
authority-origin locus: OwnerRequestInvocation
checked owner locus:    OwnerRmwExecution + local checked state schemas
generated edges:        OwnerRequest + linked typed OwnerReplyReceipt
```

The requester receives no private read or direct mutation fragment. The reply
keeps typed success or declared failure and explicit request/serve/reply/
receive lifecycle slots, but transfers no authority.

For an accepted maintained relation, the owner retains `RelationPublication`
and its exact checked two-anchor/fallback/residual lineage. An explicitly
checked consumer receives `ConsumerLocalRelationProjection`; the generated
edge is `RelationProjectionPublication`, with publish at the owner fragment
and observe at the consumer fragment. `AbsoluteValueStream` is never generated
by this profile.

For an accepted designated computation, every checked remote-input dependency
places a `DesignatedRemoteInputService` at its source owner, generates a typed
request and receipt to the designated evaluator, and preserves the dependency
ordinal/state-read source span separately from the evaluator artifact span.
The `DesignatedEvaluation` fragment remains only at the checked evaluator.
Producer-release authority and evaluator-decision authority are distinct
runtime-seam requirements.

For an explicitly checked designated consume:

```text
evaluator locus:       existing DesignatedEvaluation result-delivery source
named consumer locus: DesignatedResultConsumer, no expression/raw input
generated edge:       DesignatedResultDelivery(E -> C)
```

The consumer fragment and carrier preserve the consume source/Core ref,
designated result identity, input/result frontiers, result version,
observation policy, policy stamp, and
`ReturnExistingNoNewConsumption`. Exactly one consumer is permitted for a
designated result in this finite profile. This field is a static
source/Core-bound requirement: a future retry by the same named consumer must
return the existing decided value without a new semantic consume, and an
undeclared or competing consumer is a typed conflict. Consumer membership,
capability, and witness rows remain sealed-runtime-seam requirements and are
not grants.

The static field is not evidence that current M8 implements this return
behavior. Legacy M8 direct consumption records delivery ids, rejects a repeat
of the same id as `AlreadyConsumed`, and can consume a different id again.
M10's accepted duplicate-delivery pressure row retains that exact rejection;
SYS-3 neither changes nor reinterprets it. SYS-4 must refine the generated
endpoint with a source/Core-bound semantic-consumption identity and a
carrier-side idempotent return or compatible wrapper. On the accepted path it
calls legacy M8 consumption exactly once for that identity; the same named
consumer's retry returns the retained decision before another M8 call. Actual
positive, retry, and competing-consumer endpoint evidence is required in
SYS-4, not claimed by SYS-3.

## Carrier and correspondence rules

Every generated edge directly names existing source and target fragment refs
and one `CheckedCoreIdentity`. Its carrier contract retains the applicable
operation/request identities, source/Core reference, origin/target loci,
failure/effect rows, required occurrence slots and frontiers, runtime-seam
authority requirements, typed result, and receipt-consumption state. Those
requirements are source/Core-derived slots and cannot mint membership,
capability, witness, producer release, evaluator authority, state, or Core.

`DesignatedResultDelivery` exists only for a distinct designated-consume Core.
It binds the evaluator source fragment to the named consumer fragment, retains
publish/receive/consume occurrence requirements, and transfers no authority.
The observation plan and persistence plan must respectively include those
future occurrence refs and the consumption identity/in-flight/receipt-
consumption responsibilities. None records an actual delivery or consume.

The observation plan derives rows only from operation fragments and generated
carrier occurrence requirements. Re-running finalization must be idempotent.
Deduplication compares complete `ObservationRow` semantic equality; it cannot
collapse rows that differ in source/Core provenance, edge, fragment,
occurrence, or redaction. A local owner-request invocation has only the
request-side occurrence it can actually supply; serve belongs to the owner
fragment. Relation publication uses owner publish and consumer observe.

`verify_projection` recomputes the pure result and checks exact identity,
backend requirements, persistence assignment, effect-handler provenance,
source map, owner placement, derived-edge completeness, and final structure.
It never repairs, enriches, or admits the candidate.

## Relation extension pressure

Production lowering adds only one checked primary -> fallback edge for each
current two-anchor relation. A separate test-only constructor may connect
anchors already derived from the same checked program to pressure a finite
deeper/shared acyclic graph. It rejects cycles and foreign checked-program
source references and is marked test-only. This is a conservative
representation boundary, not ordinary-source nested-relation semantics,
arbitrary-DAG checking, or a general projection-coherence theorem.

The accepted current two-anchor relation also passes one direct finite
`project then evaluate == existing M8 evaluate` runtime test. That is bounded
correspondence for the fixture only and does not change OBL-035 or prove a new
general law.

## Diagnostics and evidence boundary

The finite diagnostic kinds cover duplicate/missing/unknown loci, checked
identity mismatch, missing/extra derived edge, moved owner operation, source
map mismatch, relation cycle or foreign-program dependency, backend or
persistence mismatch, handler provenance mismatch, and other structural
mismatch. The reopened boundary additionally rejects a missing/moved
designated-consumer fragment, missing/extra delivery edge, consumer expression
or raw-input leakage, undeclared consumer, competing consumer, and any
topology-invented consumer. A diagnostic carries no partial output.

The former cut `ded622fef91bab2cadc571ba944e5ee2c69a7b63` passed 25 focused
tests for its projected fragment, but close review found the absent E-CONSUME
source/Core/artifact path. The correction commits `b39f3e76`, `f37be73c`,
`27e42658`, and `30be30bb` led to accepted cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`. The final source-bound profile
passes AST Surface M6 9/9, M6 classification 13/13, M7 pipeline 25/25, M9 8/8,
SYS-3 27/27, M8 admission 7/7, M10 source 2/2, M10 conformance 67/67, full
runtime/workspace tests, format, scoped Clippy, and diff validation; final
semantic and code-quality reviews are ACCEPT.

OBL-060 is `runtime-monitored` only for the static finite projection contract.
These results establish the source/Core semantic identity and refinement
requirement, not SYS-4 endpoint return behavior. This specification does not
claim a Lean proof, arbitrary projection determinism/completeness, general
owner preservation, arbitrary relation-DAG semantics/coherence, runtime
dispatch, actual endpoint/occurrence execution, public compatibility, real
transport, multi-consumer semantics, or lifecycle acceptance.

## SYS-4 realization note

ADR-0030 / spec/13 accepts the direct runtime consumer of this static output at
cut `22196f93b0112b8fd2987ec078021c8865b71651`. It iterates the owned
`LocusProgram` values and `CommunicationPlan` only, instantiates locus-tagged
endpoints, and binds actual send/receive/serve/publication/consume occurrences
back to this specification's exact source/Core/fragment/edge correspondence.
It does not reparse source, infer a consumer, or add a manual route.

The realized designated endpoint satisfies this chapter's static
`ReturnExistingNoNewConsumption` handoff: first accepted delivery reaches M8
semantic consumption once; the exact same-consumer semantic-identity retry
returns the retained typed decision before another M8 consume; any changed
binding fails closed. OBL-061 records that runtime evidence separately from
OBL-060. This note does not retroactively turn SYS-3 projection into dispatch,
change legacy M8/M10 duplicate behavior, or claim public compatibility,
transport, exactly-once, multi-consumer semantics, or a general theorem.
