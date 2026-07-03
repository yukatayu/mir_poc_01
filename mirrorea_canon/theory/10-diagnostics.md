---
id: theory/10-diagnostics
status: L2-working
maturity: draft
depends_on: [theory/03-elaboration, theory/06-existence-fallback]
summary: 差し戻しの理論。Diagnostic carrier、blame=失敗した前提、説明健全性・説明完全性。
open_items: [OPEN-024]
---

# 10 — Diagnostics as a first-class object (working)

The axis promises: "if ordinary code cannot be given the intended meaning, it
comes back with a clear reason". That promise is a theorem surface, not UI
polish.

## Carrier

```text
Diagnostic = { id,                  spec/07 error id (E-XXXX-###)
               span,                source span (file, range)
               rule_instance,       the judgment rule whose premise failed
               failed_premise,      which premise, with bindings
               missing_evidence,    declared item(s) whose absence caused it
               suggested_repair,    machine-readable edit suggestion(s)
               refs }               ADR/THM/CON links for the human
```

## Blame principle (settled direction)

Every rejection is the failure of a *named premise* of a *named rule instance*
of the unified judgment, with bindings and span. There is no anonymous
"type error". Underdeclared cases (theory/06) blame the missing declaration
site, not the use site alone.

## Two target properties

- **Explanation soundness** (OBL-024): every emitted Diagnostic's
  rule_instance and failed_premise are actual — replaying the judgment with
  the reported bindings fails exactly there.
- **Explanation completeness for Line-1** (OBL-025): every Line-1 rejection
  emits a Diagnostic with a non-empty suggested_repair whenever a single-edit
  repair exists in the declared fragment (e.g. add F to a `fails` row, add a
  lineage annotation, request a capability).

## Repair taxonomy (initial)

add-to-fails-row / declare-visibility / add-lineage-annotation /
declare-access-target / request-capability / move-into-owner-block /
use-chain-instead-of-expired-ref / split-ambiguous-brace / rename-collision.

Human-facing wording rules live in spec/07 and must reuse mental-model/03's
vocabulary for the three known intuition gaps.

OPEN-024: repair ranking and multi-edit repairs — post-GATE-1.
