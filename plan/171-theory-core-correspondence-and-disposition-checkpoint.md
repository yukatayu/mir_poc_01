# plan/171 - Theory core correspondence and disposition checkpoint

## Role and authority

This is LAB repository memory. `mirrorea_canon/` remains authoritative for
theory, Core vocabulary, obligations, proof status, Gates, Phases, and every
L0/L1 decision. This checkpoint does not open a WRK record, amend a Canon
statement, choose a final Lean representation, or change implementation or
public readiness.

## Question

After the post-WRK-0013 whole-portfolio no-candidate triage, can the existing
Lean and runtime evidence be reconciled with the canonical theory in a way that
identifies a non-duplicative autonomous next experiment?

## Reproduced evidence cut

At clean `d1e586af`, the current Lean synchronizer and its 21 unit tests passed.
The three standalone statement drafts for OBL-001, OBL-020, and OBL-021 compiled
directly. The three import-bearing countermodels were independently replayed by
compiling unchanged drafts under a fresh external import-relative `.olean` root
and supplying it through `LEAN_PATH`:

- OBL-001 Result/write coverage (WRK-0007);
- OBL-020 familywise/global boundary (WRK-0006); and
- OBL-021 no-outcome boundary (WRK-0004).

This confirms source reproducibility only. It does not make the LAB drafts the
Canon targets named in `theory/11-metatheory-ledger.md`.

## Correspondence reading

| Canon target | Exact LAB finding | Valid conclusion | Invalid conclusion |
| --- | --- | --- | --- |
| THM-001 / OBL-001 | Canon quantifies every write in elaborated Core `c`; the abstract LAB draft guards only `GeneratedWrite result write`. WRK-0007 has a successful experiment-local `Result` with an untracked write while that unchanged draft holds. | The draft alone does not establish an enumeration bridge from its opaque Result carrier to all relevant Core writes. A later proof-facing model must either state that bridge or formalize THM-001 directly over `c`. | THM-001 is false, Canon permits a hidden remote write, or the experiment-local Result is Canon Core. |
| OBL-020 | Canon states preservation for every step. WRK-0006 proves global preservation implies a familywise wrapper; its finite model proves a familywise wrapper can hold while an unclassified actual non-preserving step falsifies the global draft. A coverage premise supplies the converse only conditionally. | The direct global preservation target remains the safe proof target. Familywise reasoning is an optional proof organization and cannot stand in for the global conclusion unless its coverage of the quantified step domain is demonstrated. | Canon has selected a step taxonomy, a family partition, a coverage premise, or a completed preservation proof. |
| OBL-021 / BND-001 | Canon says elaboration is function-like and either produces a tuple or Diagnostic. WRK-0004 proves that the current LAB pairwise-coherence clauses permit a well-scoped input with no outcome. | Pairwise coherence is not an outcome-existence proof. Until an owner response and any required Canon process settle PROPOSAL-008, a proof-facing artifact must carry an explicit existence premise or avoid claiming an outcome exists. | Canon totality is false, totality belongs in OBL-021, or the LAB countermodel changes BND-001. |
| cuts, authority, observation, patching | Current LAB implementations execute bounded checks, but no attached Lean model establishes their correspondence to Canon configurations and steps. | They are implementation evidence with explicitly limited interpretation. | Their current behavior discharges Canon OBLs or fixes theorem interfaces. |

## Research disposition

No new L3 record is selected from this checkpoint. A new micro-theorem would
repeat the three already manifested logical boundaries while leaving the
Core/result correspondence, the actual global step domain, or BND-001 totality
placement unselected.

The recommended technical posture for any later proof-facing package is:

1. model OBL-001 directly over the Canon elaborated `c`, rather than treating
   an opaque Result predicate as a substitute for Core write enumeration;
2. retain OBL-020 as a global step-preservation goal, using familywise lemmas
   only together with a proved coverage relation; and
3. keep OBL-021 coherence separate from outcome totality until an owner response
   and the required Canon process settle PROPOSAL-008.

These are proof-hygiene constraints derived from the evidence, not new Canon
semantics or a selected formal interface.

## Decision and reopen boundary

- **PROPOSAL-008 remains an active owner-decision request:** it asks whether
  BND-001 promises total outcome production and where that obligation belongs;
  it records no owner answer and has no automatic Canon effect.
- **OBL-001 correspondence is a future proof-facing escalation:** a package
  that must use a result carrier needs an explicit Core/write enumeration or
  direct-`c` representation; no such representation is selected here.
- **OBL-020 does not need a familywise organization decision to remain sound:**
  direct global preservation is the canonical target. PROPOSAL-003 remains an
  owner-only organizational choice for later proof packages, not a prerequisite
  for bounded research.

Reopen autonomous candidate selection only on a new source-grounded structural
mismatch with distinct positive and adverse outcomes, or after an owner/canon
action selects one of these proof-facing boundaries.

## Non-claims

This checkpoint does not change `theory/03`, `theory/11`, PROPOSAL-003,
PROPOSAL-008, an OBL/THM status, a Gate or Phase, a Core constructor, a
configuration/step carrier, runtime behavior, conformance, or public readiness.
