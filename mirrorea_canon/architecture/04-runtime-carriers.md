---
id: arch/04-runtime-carriers
status: L2-working
maturity: reviewed
depends_on: [theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, theory/08-patch-hotplug, theory/13-evaluation-materialization, theory/18-m9-auth-verification, adr/ADR-0027]
summary: broad runtime carrier catalogと、SYS-1で固定したcrate-private owner/designated-input lifecycle carrier。public API/ABI/wireは未凍結。
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

## Resolution and remaining L2 boundary

OPEN-030 is resolved for the preceding I2-internal lifecycle only. Its public
encoding, real transport mapping, retry policy, and compatibility remain
unset. This file stays L2-working because OPEN-026 field-name/IR exchange,
OPEN-027 external delivery observability, and the broader carrier catalog/full
internal freeze remain unresolved. The immutable M9 admission snapshot also
leaves revoke-after-enqueue/serve visibility to SYS-2's ST/OW refinement.
