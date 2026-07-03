---
id: theory/07-observation
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, theory/02-types-effects-failures]
summary: 観測=型付き情報効果。label / authority / redaction 単調 / retention と THM-005。
open_items: [OPEN-020]
---

# 07 — Observation as a typed effect

Observation is information-bearing and therefore an effect — never a
helper-local leak. Carrier:

```text
ObservationEvent = { subject_event, observer_principal, view_label,
  redaction_level, retention_scope, export_surface, proof_or_reason_refs }
```

Pipeline: runtime occurrence → typed telemetry row → authority check →
redaction → retention decision → export → viewer surface. Every exported row
derives from the occurrence DAG or a declared telemetry effect.

**Redaction is monotone**: `admin_full ≥ redacted_admin ≥ observer_safe ≥
public_summary`; layers may strengthen redaction, never weaken it
transparently. **Retention** ∈ {none, ephemeral, report_local, session_local,
durable_audit} and must be explicit. Observer-safe views never expose raw
witness payloads, raw auth evidence, high-label state, private grants, or
secrets. On-demand trace: disabled ⇒ no high-volume materialization; enabling
is itself audited and starts only after an activation cut.

Minimum devtools panel family (binding for PHASE-I5): occurrence DAG, route
trace, membership timeline, witness relation, hot-plug lifecycle, fallback
degradation, save/load timeline, observer-safe redacted view. Every panel row
links back to source spans (with theory/03).

```text
THM-005 (noninterference of observer-safe export): observer-safe exports are
invariant under variation of high-label state and raw witness/auth payloads;
i.e. two configurations agreeing on low-label state produce identical
observer-safe rows.
```

(OBL-017 statement, OBL-018 proof — IFC-style, first explicit-flow fragment.)

OPEN-020: label lattice finalization (currently the 4-level redaction chain +
declared labels; a general finite lattice is the intended widening).
