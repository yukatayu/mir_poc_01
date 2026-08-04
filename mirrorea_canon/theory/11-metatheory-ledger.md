---
id: theory/11-metatheory-ledger
status: L1-fixed
maturity: draft
depends_on: [theory/03-elaboration, theory/04-ordering-and-cuts, theory/05-authority, theory/06-existence-fallback, theory/07-observation, theory/08-patch-hotplug, theory/13-evaluation-materialization, theory/14-maintained-relation-projection]
summary: 定理 THM と義務 OBL の単一台帳。証明状態はここだけが語る。
open_items: []
---

# 11 — Metatheory ledger (THM / OBL)

**This file is the only place that states proof status.** Chapters state
theorems; this ledger tracks them. Status ∈ {`lean-proved`, `lean-stated`,
`model-checked-bounded`, `runtime-monitored`, `intentionally-deferred`}.
Each status names evidence for the exact stated scope only. In particular, a
finite model is not a general proof, and `intentionally-deferred` names the
milestone that must supply the missing model/evidence; it is not a hidden
success or a generic `open` bucket.

## Theorems

| ID | Statement (short) | Chapter | Gate |
|---|---|---|---|
| THM-001 | Assignment elaboration soundness | 03 | GATE-1 exit |
| THM-002 | Fallback monotonicity / no re-promotion | 06 | GATE-2 |
| THM-003 | Load ⇒ consistency ∧ no stale resurrection | 04 | GATE-5 |
| THM-004 | Authority soundness (grant-lineage justification) | 05 | GATE-3 |
| THM-005 | Observer-safe noninterference | 07 | GATE-4 |
| THM-006 | Patch rejection no-mutation | 08 | GATE-7 |
| THM-007 | Finite EvalPlan elaboration determinism | 13 | M3 |
| THM-008 | Finite owner RMW seriality / no stale blind write | 13 | M3 |
| THM-009 | Finite cross-owner dependency requires typed receipt | 13 | M3 |
| THM-010 | Finite designated-result duplicate stability | 13 | M3 |

## Obligations

| ID | Obligation | For | Evidence target | Status | Boundary / next owner |
|---|---|---|---|---|---|
| OBL-001 | THM-001 general Lean statement | THM-001 | MirCore.Elab.Soundness | intentionally-deferred | M5 shared model; M3 finite support is OBL-029--031, not this general statement |
| OBL-002 | THM-001 general proof | THM-001 | same | intentionally-deferred | M5 after exact Core/source correspondence |
| OBL-003 | Line-1 decidability on declared fragment | judgment | MirCore.Check.Decidable | intentionally-deferred | M7 checker; OBL-033 covers only the M3 finite target set |
| OBL-004 | General no-undeclared-communication corollary | THM-001 | MirCore.Elab.NoHidden | intentionally-deferred | M5; OBL-031 is a finite M3 receipt boundary |
| OBL-005 | Canonical flattening laws (assoc/unit) | chains | MirCore.Chain.Canon | intentionally-deferred | M4 relation/fallback calculus |
| OBL-006 | Canon uniqueness / confluence | chains | same | intentionally-deferred | M4 relation/fallback calculus |
| OBL-007 | THM-002 Lean statement | THM-002 | MirCore.Chain.Monotone | intentionally-deferred | M4 fallback model |
| OBL-008 | THM-002 proof | THM-002 | same | intentionally-deferred | M4 fallback model |
| OBL-009 | THM-003 Lean statement | THM-003 | MirCore.Cut.Load | intentionally-deferred | M5/M8 cut-save shared model |
| OBL-010 | Consistent(K) checker soundness | 04 | MirCore.Cut.Consistent | intentionally-deferred | M5/M7 |
| OBL-011 | No stale membership resurrection | THM-003 | lemma | intentionally-deferred | M5/M8 |
| OBL-012 | No stale witness resurrection | THM-003 | lemma | intentionally-deferred | M5/M8 |
| OBL-013 | No expired lease resurrection | THM-003 | lemma | intentionally-deferred | M5/M8 |
| OBL-014 | Z-cycle reject ≙ Netzer–Xu useless checkpoint | 04 | MirCore.Cut.ZCycle | intentionally-deferred | M5 finite cut model |
| OBL-015 | THM-004 Lean statement | THM-004 | MirCore.Auth.Sound | intentionally-deferred | M5 authority extension |
| OBL-016 | THM-004 proof | THM-004 | same | intentionally-deferred | M5 authority extension |
| OBL-017 | THM-005 Lean statement | THM-005 | MirCore.Obs.NI | intentionally-deferred | M5/M9 observation model |
| OBL-018 | THM-005 proof (explicit-flow fragment) | THM-005 | same | intentionally-deferred | M5/M9 |
| OBL-019 | THM-006 statement+proof | THM-006 | MirCore.Patch.NoMut | intentionally-deferred | M5/M8 patch model |
| OBL-020 | General well-formedness preservation of step rules | 01 | MirCore.Step.WF | intentionally-deferred | M5; finite M3 owner step is OBL-030 |
| OBL-021 | General elaboration determinism | 03 | MirCore.Elab.Det | intentionally-deferred | M5; finite M3 elaborator is OBL-029 |
| OBL-022 | Stream non-influence on discrete state | 09 | MirCore.Time.ReadSide | intentionally-deferred | M4/M5 time model |
| OBL-023 | Temporal coherence from frontier admissibility | 09 | MirCore.Time.Coherent | intentionally-deferred | M4 relation/presentation model |
| OBL-024 | Explanation soundness | 10 | MirCore.Diag.Sound | intentionally-deferred | M5/M7 diagnostics |
| OBL-025 | Explanation completeness (Line-1, single-edit) | 10 | MirCore.Diag.Complete | intentionally-deferred | M5/M7 diagnostics |
| OBL-026 | Overlay substitutability composes over stacks | 02 | MirCore.Layer.Compose | intentionally-deferred | M9 auth/layer extension |
| OBL-027 | Rollback cannot cross atomic_cut | 04 | MirCore.Cut.NoCross | intentionally-deferred | M5/M8 |
| OBL-028 | Revocation monotone absent new epoch/evidence | 05 | MirCore.Auth.Revoke | intentionally-deferred | M5/M9 |
| OBL-029 | Finite M3 EvalPlan elaboration functional | THM-007 | `elaboration_deterministic` in M3 Lean foundation | lean-proved | finite `Input` and `EvalPlan` only; `lean --trust=0`, no `sorry`/`admit`/axiom |
| OBL-030 | Finite M3 owner RMW / failure / WF fragment | THM-008 | `two_attacks_are_serial_owner_rmw`, no-mutation and WF lemmas | lean-proved | one owner cell, `-10` request, non-negative hp; no general Core preservation claim |
| OBL-031 | Finite M3 receipt boundary | THM-009 | explicit/missing/failed/wrong-target/release-unadmitted/incomplete-causal-chain receipt lemmas | lean-proved | one typed `Int` receipt, admitted release tuple, and finite request≺serve≺reply≺receive order; no transport/exactly-once/snapshot theorem |
| OBL-032 | Finite M3 designated duplicate stability | THM-010 | duplicate decision and idempotent consumption lemmas | lean-proved | one evaluator/key lineage; no save/load/stale-result theorem |
| OBL-033 | Finite M3 materialization target checker | judgment | exhaustive six-target model enumeration | model-checked-bounded | exact 64 target subsets only; no general checker claim |
| OBL-034 | M3 typed reference trace invariants | M3 reference | Rust focused trace tests | runtime-monitored | parser-free deterministic reference, not M8 runtime conformance |
| OBL-035 | Finite M4 projection coherence / relative-offset preservation | M4 relation | `project_then_evaluate_equals_evaluate_relation`, `relative_offset_is_preserved_by_projection` in M4 Lean foundation | lean-proved | one owner, consumer, subject, two anchors, exact `Int` offset and coherent finite context; no arbitrary DAG or renderer theorem |
| OBL-036 | Finite M4 semantic fallback monotonicity / fresh reacquire | THM-002 fragment | `semantic_fallback_is_monotone`, no-auto-repromotion, and fresh/nonfresh reacquire lemmas in M4 Lean foundation | lean-proved | exactly primary → fallback; fresh epochs begin one new finite lineage; no general chain proof |
| OBL-037 | Finite M4 presentation-context coherence / gap nonmutation | OBL-023 fragment | coherence, stale/split reject, and `presentation_gap_does_not_mutate_semantic_binding` in M4 Lean foundation | lean-proved | two-anchor one-frontier profile only; no general clock, latency, or stream theorem |
| OBL-038 | Finite M4 greatest-restriction label propagation | M4 observation fragment | three-label order / dominance and private-to-public reject lemmas in M4 Lean foundation | lean-proved | exact `public < restricted < private` chain only; no general label lattice/noninterference proof |
| OBL-039 | Finite M4 relation admission rejects | M4 relation | cycle and consumer-mutation reject lemmas in M4 Lean foundation | lean-proved | direct self-cycle and one non-owner consumer only; no arbitrary cycle detection or authority theorem |

Discharge order recommendation: 020 → 021 → 001/002 → 005–008 → 009–014 →
015/016 → 017/018 → 019 (matches Gate order, plan/00).
