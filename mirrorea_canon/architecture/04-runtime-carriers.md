---
id: arch/04-runtime-carriers
status: L2-working
maturity: draft
depends_on: [theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, theory/08-patch-hotplug]
summary: runtime carrier のフィールド正本(Envelope / Admission / CapRef / Witness / SaveObject / HotPlug / Observation / Obligation / Diagnostic)。
open_items: [OPEN-030]
---

# 04 — Runtime carriers(正本)

フィールド名は L2(凍結は PHASE-I1 出口)。theory 各章の carrier をここに集約する。実装は追加フィールドを足してよいが、削除・意味変更は不可。

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

規則: carrier は JSON で直列化可能・span/参照は ID ベース・秘匿 payload は ref 化して observer_safe 面に生値を出さない。OPEN-030: envelope の reply/receipt 形(OPEN-011/027 と連動)。
