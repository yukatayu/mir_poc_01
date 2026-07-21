# plan/163 - Foundation integrity and elaboration-outcome audit

## Role and authority

This is LAB repository memory. `mirrorea_canon/` remains authoritative for
theory, decision levels, obligations, Gates, Phases, and process. This audit
does not select a carrier, alter an OBL, settle BND-001, prove a theorem, or
open a new WRK record.

## Question

After the narrow post-WRK-0006 selection pause, does the whole foundational
theory contain a structural mismatch, accidental proof claim, or reproducibility
gap that changes what autonomous research may validly do?

## Findings

### 1. The core direction is coherent; the proof boundary is still open

The S0--S5 separation, four graph families, explicit cross-locus effects,
capability/witness lineage, failure-row containment, typed observation,
fallback monotonicity, cut, and frontier-bound patch lifecycle all support the
project axis without importing `World`/`Game`, standard I/O, transport identity,
or untyped debug output into Mir Core. No new core primitive is justified by
this audit.

The theory is not proof-ready: `theory/11` keeps every OBL open, and the prior
source-adequacy audit recorded that full OBL-020 preservation needs explicit
transition, history, frame, freshness, and record premises. This is an open
formalization boundary, not evidence that the direction is false.

### 2. BND-001 has an unassigned outcome-totality surface

`theory/03` says that a well-scoped Surface item either produces the stated
elaboration tuple or a Diagnostic, while its only named proof target for that
paragraph is OBL-021 determinism. The current LAB OBL-021 draft supplies
pairwise actual-outcome coherence and success/rejection exclusion, but
WRK-0004's checked countermodel permits no outcome at all. `plan/160` already
states the narrower LAB conclusion correctly: totality is a separate premise
there.

This is a source-to-ledger traceability mismatch, not a completed inference
about Canon semantics. `mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md`
asks the owner whether the canonical sentence is total, and, if it is, whether
its eventual obligation belongs beside or inside OBL-021. No autonomous Lean
artifact can answer that question without choosing a reserved boundary.

### 3. Existence-DAG and patch-DAG preservation remain intentionally unassigned

The occurrence DAG is in the current `WellFormed` wording and OBL-020's
direction. The existence DAG and patch dependency DAG are distinct project
invariants, but neither has a selected runtime carrier or a current preservation
obligation. This is not new: T-RESEARCH-032 recorded exactly this distinction.
It must remain visible so that a later safe-hot-plug or lifetime argument does
not cite OBL-020 as if it covered all four graph families.

### 4. Import-bearing Lean evidence is reproducible but not manifest-attested

In a fresh external temporary directory, the current source drafts compiled to
external `.olean` files and `LEAN_PATH` then compiled all five import-bearing
OBL-020/OBL-021 L3 sources: WRK-0006's familywise boundary, the three OBL-021
countermodels, and the conditional outcome relation. The external build output
is 388 KiB and is not a repository artifact.

The committed synchronizer/manifest intentionally verifies only the five
standalone statement drafts. It therefore must not be cited as a fresh
attestation of import-bearing evidence. This audit does not add a runner,
helper, CI surface, or manifest category; a future operational change requires
its own scope review.

### 5. The candidate pause is a priority heuristic, not a new eligibility rule

`plan/162` correctly records a LAB priority disposition: no new
non-duplicative micro-theorem candidate was selected from the then-known
existing vocabulary. ADR-0014 still independently permits any future L3
candidate satisfying its standing predicate. The snapshot wording is corrected
to avoid treating the heuristic as a Canon-level prohibition.

The temporary Oracle suggested an OBL-020 premise-inventory record. Local
comparison rejects it as a new candidate because T-RESEARCH-006 has already
performed the selected 13-rule by five-clause source-adequacy inventory and
recorded all 65 cells as lacking derivation-complete premises. Re-running it
under a different name would be duplicate theorem-shaped activity, not a new
research branch.

## Current disposition

1. Do not open WRK-0007 from this audit.
2. Keep autonomous research available under ADR-0014. `plan/162`'s priority
   heuristic favors a genuinely new, pre-registered existing-lane question
   with distinct positive and falsifying outcomes, but it is not an additional
   standing-eligibility condition.
3. Treat PROPOSAL-008 as an owner-reserved decision point before a later
   proof-facing package assumes elaboration outcome totality.
4. Keep existence-DAG and patch-DAG coverage unassigned until a later package
   selects their carrier and proof boundary.

## Non-claims

This audit does not establish BND-001 totality, change OBL-021 or any other
ledger entry, complete OBL-020, prove a Core theorem, authorize a new
formalization interface, change a Gate/Phase, or claim runtime/conformance or
public readiness.
