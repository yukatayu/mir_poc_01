---
id: arch/04-runtime-carriers
status: L2-working
maturity: reviewed
depends_on: [theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, theory/08-patch-hotplug, theory/13-evaluation-materialization, theory/18-m9-auth-verification, adr/ADR-0027, adr/ADR-0028, adr/ADR-0029, adr/ADR-0030, adr/ADR-0031, adr/ADR-0032, adr/ADR-0038, adr/ADR-0039, arch/09-i3-private-adapter, arch/10-i3-multi-process-runtime, spec/15-sys6-i2-conformance]
summary: broad runtime carrier catalogとSYS-1--6 internal lifecycle/execution/projection/dispatch/devtools/assurance requirements。public API/ABI/wireは未凍結。
open_items: [OPEN-026, OPEN-027]
---

# 04 — Runtime carriers(正本)

下の broad catalog は L2 working であり、public API / ABI / wire 又は現在の
Rust type layout ではない。theory 各章の carrier requirements を集約し、広い
PHASE-I1 exit で初めて full internal freeze の可否を判定する。

```text
MessageEnvelope = { envelope_id, transport_medium, transport_seam,
  emitter_principal, target_locus, op, payload_ref, membership_epoch,
  member_incarnation, capability_refs[], witness_refs[], freshness_checks,
  authorization_checks, declared_failure_row[], source_span }

AdmissionRequest / AdmissionVerdict           … theory/05 §Claims
CapabilityRef(lineage record)                 … theory/05 §Validation
Witness = { witness_id, kind, subject_refs[], issuer, bindings{}, created_at }
SaveObject                                    … theory/04 §SaveObject
HotPlugRequest / HotPlugVerdict = { patch_id, compat_carrier,
  checked_epoch, checked_incarnations[], required_witness_refs[],
  verdict, reason_refs[], activation_cut_ref? }
ObservationEvent                              … theory/07
ResidualObligation = { obligation_id, obligation_kind, source_refs[],
  required_context, suggested_target, current_status }
ModelObligation = { variables, transition_relation, initial_states,
  safety_properties, liveness_properties, fairness_assumptions,
  abstraction_relation, expected_result }
ProofObligation = { lemma_id, lemma_kind, statement_summary, source_refs[],
  assumed_invariants, export_target, current_status }
Diagnostic                                    … theory/10 + spec/07
```

Broad catalog の規則: externalized carrier は直列化可能で、span/参照は ID
ベース、秘匿 payload は ref 化して observer_safe 面に生値を出さない。これは
JSON field 名を固定しない。

## SYS-1 selected internal kernel boundary

ADR-0027 fixes one narrower, crate-private I2 contract below the M10 facade:

```text
ordinary source / generic checked OwnerEvent
  -> checked Core + sealed M9 execution seam
  -> SemanticRuntimeKernel
  -> owned M8 runtime

M10 profile / evidence generator / verifier / release orchestration / CLI
  -> may invoke or observe that path
  -> never enters the kernel as semantic input
```

Specialized historical SCN-04, SCN-09, SCN-10, and route-patch runners are
preserved M10 regression-only paths. They are outside the SYS-1 kernel
acceptance claim.

The selected lifecycle families are:

```text
OwnerRequestCarrier
  = { request_identity, operation_identity,
      origin_principal, origin_locus, target_owner,
      request_occurrence, membership_ref, membership_epoch,
      membership_incarnation, capability_refs[], witness_refs[],
      checked_source_ref, checked_core_ref,
      effect_row, failure_row, visibility, redaction, arguments }

OwnerReply / OwnerReceipt
  = { same lineage/provenance, typed_success | declared_failure,
      request_occurrence, serve_occurrence,
      reply_occurrence, receive_occurrence }

DesignatedRemoteInputCarrier
  = { request_identity, checked_dependency_identity,
      origin_principal, source_owner, target_evaluator,
      input_frontier, producer_release_tuple,
      request_occurrence, membership_ref, membership_epoch,
      membership_incarnation, capability_refs[], witness_refs[],
      checked_source_ref, checked_core_ref,
      effect_row, failure_row, visibility, redaction }

DesignatedRemoteInputReceipt / Consume
  = { same lineage/provenance, receipt_identity,
      typed_success | declared_failure,
      request_occurrence, serve_occurrence,
      reply_occurrence, receive_occurrence,
      evaluator_consumption_key, consumption_state }
```

The implementation may use opaque typed references and may split these
logical records into request/reply/receipt structs. It must preserve the
listed information losslessly at the lifecycle point where it applies.
Request identity is kernel-issued and is never queue position. Occurrence
order is `request < serve < reply < receive`; designated input additionally
has explicit evaluator consumption. A receipt transfers neither authority nor
ownership.

Authority, target, release tuple, checked source/Core provenance, effect and
failure rows, visibility/redaction, and freshness derive from checked Core and
the sealed M9 seam. A source/schedule cannot supply or mint them. Validation
failure before admission creates no semantic occurrence, enqueues nothing in
M8, and mutates no semantic store. Duplicate receipt/reply rejection is
single-assignment checking, not hidden retry or exactly-once delivery.

The designated remote-input carrier is a bounded typed effect request/result
instance at the admitted source owner. It is not a generic provider registry,
and transport, auth, projection, persistence, and semantic ownership remain
separate subsystems.

## SYS-2 selected execution evidence boundary

ADR-0028 fixes two internal execution profiles for the preceding lifecycle.
ST is the deterministic single-thread reference. OW1 is deliberately bounded
to exactly one combined semantic owner/source-owner locus: one dedicated
worker exclusively owns `M8LocalRuntime`, while a coordinator communicates
through an acknowledged zero-capacity synchronous mailbox. The coordinator
may retain checked Core, carriers, queues/receipts, and typed ordering
evidence, but exposes no public shared mutable M8 store. A different combined-
locus count fails typed; it does not silently share or duplicate owner state.

The evidence carrier is logical and provisional rather than a Rust/public
schema. It retains, where applicable:

```text
profile and admitted owner/source-owner locus
request/serve/reply/receive/consume occurrences
actual M8 enqueue occurrence
actual owner-read and owner-write trace nodes
per-key written version and preceding writer/request
M9 authority generation and acknowledged publish occurrence
source-owner read value/version and producer/evaluator release lineage
worker-execution observation (debug evidence only; never authority)
```

A successful owner write linearizes at the actual acknowledged M8
`OwnerWrite` node. A failed/revoked serve has no write linearization,
reads-from, or version-advance row. A designated-input reply derives from the
acknowledged source-owner read; caller-supplied mismatch rejects before reply,
receipt, or mutation. Worker identity, mailbox position, and authority
generation are evidence/correlation, not authority.

An M9 successor is published only after complete retranslation and the sole
M8 owner refresh acknowledgement. It must preserve checked program identity,
advance generation strictly, retain prior tombstones monotonically, and keep
unrelated admitted owner/designated-release lineages. This boundary does not
freeze concrete channel types, field names, or an artifact/backend ABI. SYS-3
may consume its semantic requirements, not its Rust layout.

## SYS-3 selected generated carrier-plan boundary

ADR-0029 consumes the preceding semantic requirements as static, crate-private
carrier contracts attached to checked-Core-derived communication edges. Each
generated edge directly names its checked operation/dependency identity, exact
source/Core reference, source and target locus, and real source/target artifact
fragment refs. The carrier plan retains the applicable subset of:

```text
operation/request identity slots
origin principal/locus and target owner/source-owner/evaluator locus
declared failure and effect rows
request/serve/reply/receive or publish/observe occurrence slots
input/result frontier and evaluator consumption state
membership epoch/incarnation, capability/witness, producer-release,
  and evaluator-authority requirements from the sealed runtime seam
reference-only visibility/redaction policy
```

These are requirements for SYS-4 materialization, not concrete messages,
runtime occurrences, grants, or admission evidence. `OwnerRequest`, linked
`OwnerReplyReceipt`, `DesignatedInputRequest`, linked
`DesignatedInputReceipt`, and `RelationProjectionPublication` are the
previously implemented finite lifecycle kinds. The accepted SYS-3 boundary
additionally requires `DesignatedResultDelivery` from the evaluator fragment
to the consumer fragment, but only when the distinct checked source/Core clause
`designated consume E.result at C` exists. Logical topology cannot invent that
edge. The carrier preserves result identity, version, input/result frontiers,
policy, reference-only visibility, same-consumer no-new-consume retry, and
sealed consumer authority requirements; it records publish/receive/consume
slots without claiming actual dispatch or transferring authority. The retry
field is a new required SYS-4 endpoint refinement contract derived from
theory/13, not current M8 evidence: legacy M8 rejects the same delivery id as
`AlreadyConsumed` and may consume a different id. SYS-4 must implement a
source/Core-bound idempotent return or compatible wrapper before a retry can
avoid another M8 call. M10 duplicate-delivery rejection remains unchanged.
`AbsoluteValueStream` remains an explicit non-generated falsifier.

The projection observation plan records required future occurrence bindings.
It does not claim that request/serve/reply/receive/publish/observe happened.
Full-row semantic equality is required for idempotent deduplication; rows with
different provenance, edge, fragment, occurrence, or redaction remain
distinct. The persistence plan assigns responsibilities but is not a
SaveObject or restore algorithm.

This generated plan does not serialize the broad catalog above and does not
resolve OPEN-026/027. Field names, Rust layout, JSON, ABI, wire encoding,
transport retries, and external delivery observation remain unfrozen.

## SYS-4 selected carrier materialization boundary

ADR-0030 materializes the preceding plan only inside one process. Every live
endpoint is generated from one `CommunicationEdge`; no external schedule,
fixture, topology supplement, or runtime fallback may add an endpoint. A staged
envelope preserves the plan's exact checked-program, source/Core, source and
target fragment, edge, route, effect/failure, visibility/redaction, frontier,
and sealed authority-requirement binding. Send, transport, receive, dequeue,
serve/evaluate/consume, reply/publication, receipt, and quarantine have distinct
occurrence identities and causal edges.

The carrier is never authority. Target-side service revalidates the current M9
generation and the exact membership/capability/witness/producer/evaluator/
consumer lineage required by the projected edge. Wrong route/target, stale or
missing authority, duplicate/stale receipt/result, split frontier, missing
payload, and provenance/policy/visibility/redaction mismatch fail typed without
semantic mutation, authority creation, or fabricated M8 success.

For the finite `DesignatedResultDelivery`, the cache key is the exact source/
Core-bound semantic-consumption identity plus named consumer, publication,
frontier/version, policy, visibility, redaction, and binding digest. The first
accepted delivery reaches M8 consume once. An exact same-consumer retry returns
the stored typed decision without another semantic consume. Any changed member
is a conflict, not a retry. This remains distinct from transport exactly-once
and from the accepted M10 direct duplicate-delivery rejection.

Observer evidence carries typed label/redaction and reference-only provenance,
not raw credential, capability, witness, private payload, or raw M8 identity.
An OW1 observer-snapshot failure is typed and distinct from absent state. It
cannot rewrite the already committed semantic outcome, serve stale evidence,
or trigger replay.

The accepted ST `Sys4LocalCut` retains per-locus M8 cuts, stores/traces,
mailboxes and symmetric endpoint records, pending carriers/faults, receipts,
cache/publication/consumption state, counters, causal dependencies, exact M9
authority lifecycle/live floor, admitted-validation counters and observer-safe
audit maps, and patch lifecycle/frontier. Restore validates this entire
relationship before mutation. It is an internal local cut, not a durable or
public carrier. OW1 cut remains `BackendIneligible`.

`Sys4CheckedPatchCandidate` is an internal boundary from an already checked,
projected, complete M9-admitted pipeline. It binds an exact base frontier and
authority lineage and contains no raw source/AST/manual edge/grant. The finite
profile admits only a quiescent ST designated-material delta while topology,
schema, owner routes/RMW, relation and every non-designated fragment/edge/
handler remain fixed. Rejection changes lifecycle evidence only; OW1 patch and
general compatibility remain `BackendIneligible`/unselected.

None of these selected Rust/logical names freezes a public field name, schema,
artifact ABI, JSON representation, wire encoding, deployment or transport.

## SYS-5 selected lifecycle and joined-observation boundary

ADR-0031 adds one narrower ST lifecycle pressure path. A relation primary
anchor may retain an explicit checked locus. The external action names only
the checked relation; the runtime derives the primary participant and sealed
M9 lineage. The resulting internal leave/fresh carriers retain distinct
request/enqueue, M9 transition, generated relation dispatch/receive/serve, and
receipt occurrences together with source/Core/relation/anchor provenance.
They accept no caller-supplied membership, epoch, incarnation, grant, witness,
authority, target, or route.

Successful leave first installs the exact monotone M9 retirement/tombstone and
then lets the independent relation owner publish fallback. Fresh reacquire
must cite that exact tombstone and successor epoch, create a distinct epoch/
incarnation and fresh capability/witness lineages, and then publish primary.
Duplicate/stale/missing-anchor/route/capacity/endpoint failure is
failure-atomic in the selected clone-prepared ST candidate. No partial M9 or M8
state becomes live. The same relationship is retained across an exact
post-leave local cut/restore; corrupt relationship evidence rejects before
mutation.

The joined observer carrier correlates exact source span, Core, fragments,
edge, request identity, distinct runtime occurrences, owner/relation/
designated outcomes, failures, and cut/patch lifecycle. It labels the active
prefix, discarded post-cut branch, and restored branch. Correlation fields are
evidence, not authority. Raw source text, credentials, capability/witness
material, private state/payload, and raw M8/M9 identity remain outside the
observer-safe partition.

These are internal finite SYS-5 carriers only. They neither serialize the
broad catalog nor freeze public CLI/JSON/API/ABI/wire fields, and do not select
OW1 lifecycle/cut, durable persistence, browser/View, or real transport.

## SYS-6 selected conformance evidence carrier

ADR-0032 / spec/15 adds a downstream evidence carrier, not another semantic
runtime envelope. The producer reads actual checked/projected/runtime/model
objects and records inventories of checked-program, Core, artifact, generated
edge, request, occurrence, lifecycle/model anchor, and executed control
references. Each conformance row joins those existing facts and carries its
bounded scope, evidence class, positive/falsifier references, controls, and
property-specific provenance anchor.

The verifier cannot supply or amend any inventory member. Missing or failed
evidence, a wrong diagnostic, a missing anchor, or a row mismatch rejects. The
observer serializer redacts immediately before materialization and exposes no
host path, source text, raw credential/capability/witness, private value, or raw
M8/M9 identity. The I2 manifest is content-bound reproducibility metadata, not
authority, runtime identity, public schema, or the M10 release identity.

Lifecycle bits in this carrier are typed non-authorizing candidates. They
prove that the producer/verifier cannot self-activate I2/I3 or transport;
ADR-0032 and plan/01 separately record the accepted I2 lifecycle state.

## Resolution and remaining L2 boundary

OPEN-030 is resolved for the preceding I2-internal lifecycle only. ADR-0028
closes its selected ST/OW1 ordering and live M9-generation visibility residual,
but not its public encoding, real transport mapping, retry policy, or
compatibility. ADR-0031 adds finite ST lifecycle/devtools evidence and ADR-0032
adds downstream I2 conformance evidence without freezing those external
contracts. This file stays L2-working because OPEN-026 field-name/IR exchange,
OPEN-027 external delivery observability, and the broader carrier catalog/full
internal freeze remain unresolved. Therefore broad PHASE-I1 remains
unaccepted even though ADR-0032 separately accepts the bounded official I2
entry and exit criteria.

## I3-1 bounded private adapter mapping

The accepted bounded mapping is maintained separately in
[`architecture/09-i3-private-adapter.md`](09-i3-private-adapter.md) and is
consumed directly by I3-2. This broad carrier catalog remains L2-working and
does not become a public schema.

## I3-2 bounded multi-process mapping

The accepted two-process execution/lifecycle mapping is maintained separately
in [`architecture/10-i3-multi-process-runtime.md`](10-i3-multi-process-runtime.md).
It consumes, but does not freeze or broaden, this L2-working carrier catalog.
