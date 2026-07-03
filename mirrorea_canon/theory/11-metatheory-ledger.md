---
id: theory/11-metatheory-ledger
status: L1-fixed
maturity: draft
depends_on: [theory/03-elaboration, theory/04-ordering-and-cuts, theory/05-authority, theory/06-existence-fallback, theory/07-observation, theory/08-patch-hotplug]
summary: 定理 THM と義務 OBL の単一台帳。証明状態はここだけが語る。
open_items: []
---

# 11 — Metatheory ledger (THM / OBL)

**This file is the only place that states proof status.** Chapters state
theorems; this ledger tracks them. Status ∈ {open, stated, lean-stated,
lean-proved, external}. All entries are `open` at v0.1.0 unless noted.

## Theorems

| ID | Statement (short) | Chapter | Gate |
|---|---|---|---|
| THM-001 | Assignment elaboration soundness | 03 | GATE-1 exit |
| THM-002 | Fallback monotonicity / no re-promotion | 06 | GATE-2 |
| THM-003 | Load ⇒ consistency ∧ no stale resurrection | 04 | GATE-5 |
| THM-004 | Authority soundness (grant-lineage justification) | 05 | GATE-3 |
| THM-005 | Observer-safe noninterference | 07 | GATE-4 |
| THM-006 | Patch rejection no-mutation | 08 | GATE-7 |

## Obligations

| ID | Obligation | For | Lean target |
|---|---|---|---|
| OBL-001 | THM-001 Lean statement | THM-001 | MirCore.Elab.Soundness (stmt) |
| OBL-002 | THM-001 proof | THM-001 | same |
| OBL-003 | Line-1 decidability on declared fragment | judgment | MirCore.Check.Decidable |
| OBL-004 | No-undeclared-communication corollary | THM-001 | MirCore.Elab.NoHidden |
| OBL-005 | Canonical flattening laws (assoc/unit) | chains | MirCore.Chain.Canon |
| OBL-006 | Canon uniqueness / confluence | chains | same |
| OBL-007 | THM-002 Lean statement | THM-002 | MirCore.Chain.Monotone |
| OBL-008 | THM-002 proof | THM-002 | same |
| OBL-009 | THM-003 Lean statement | THM-003 | MirCore.Cut.Load |
| OBL-010 | Consistent(K) checker soundness | 04 | MirCore.Cut.Consistent |
| OBL-011 | No stale membership resurrection | THM-003 | lemma |
| OBL-012 | No stale witness resurrection | THM-003 | lemma |
| OBL-013 | No expired lease resurrection | THM-003 | lemma |
| OBL-014 | Z-cycle reject ≙ Netzer–Xu useless checkpoint | 04 | MirCore.Cut.ZCycle |
| OBL-015 | THM-004 Lean statement | THM-004 | MirCore.Auth.Sound |
| OBL-016 | THM-004 proof | THM-004 | same |
| OBL-017 | THM-005 Lean statement | THM-005 | MirCore.Obs.NI |
| OBL-018 | THM-005 proof (explicit-flow fragment) | THM-005 | same |
| OBL-019 | THM-006 statement+proof | THM-006 | MirCore.Patch.NoMut |
| OBL-020 | Well-formedness preservation of step rules | 01 | MirCore.Step.WF |
| OBL-021 | Elaboration determinism | 03 | MirCore.Elab.Det |
| OBL-022 | Stream non-influence on discrete state | 09 | MirCore.Time.ReadSide |
| OBL-023 | Temporal coherence from frontier admissibility | 09 | MirCore.Time.Coherent |
| OBL-024 | Explanation soundness | 10 | MirCore.Diag.Sound |
| OBL-025 | Explanation completeness (Line-1, single-edit) | 10 | MirCore.Diag.Complete |
| OBL-026 | Overlay substitutability composes over stacks | 02 | MirCore.Layer.Compose |
| OBL-027 | Rollback cannot cross atomic_cut | 04 | MirCore.Cut.NoCross |
| OBL-028 | Revocation monotone absent new epoch/evidence | 05 | MirCore.Auth.Revoke |

Discharge order recommendation: 020 → 021 → 001/002 → 005–008 → 009–014 →
015/016 → 017/018 → 019 (matches Gate order, plan/00).
