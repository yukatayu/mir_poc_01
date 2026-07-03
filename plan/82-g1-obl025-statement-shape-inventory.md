# plan/82 - G1 OBL-025 statement-shape inventory

## Purpose

This file inventories the statement shape needed for OBL-025 explanation
completeness before writing a Lean statement, generating repairs, or changing
diagnostic code.

This is LAB repository memory. It does not state OBL-025 formally, does not
prove OBL-025, does not implement repair generation, does not implement a final
Diagnostic ABI, does not claim explanation completeness, does not claim
conformance, and does not edit canon.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB diagnostic carrier inventory:
  `plan/80-g1-diagnostic-carrier-inventory.md`
- LAB OBL-024 statement-shape inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- LAB E-ROW alignment:
  `plan/79-g1-erow-diagnostic-alignment.md`

If this LAB inventory conflicts with canon, canon wins.

## Relation inventory, not repair implementation

`plan/80` inventories carrier gaps, including absent `suggested_repair[]`.
`plan/81` inventories explanation-soundness replay relations. This file
records the additional completeness relations needed before a future OBL-025
statement can say that a repair suggestion is present when a single-edit repair
exists.

It does not choose final JSON shape, final edit syntax, final ranking, or
implementation strategy.

## Canon target reading

Canon `theory/10` reads OBL-025 as:

```text
Every Line-1 rejection emits a Diagnostic with a non-empty suggested_repair
whenever a single-edit repair exists in the declared fragment.
```

The immediate G1 pressure case is E-ROW-shaped failure-row underdeclaration,
where the obvious repair family is `add-to-fails-row`. This package keeps the
scope as statement-shape inventory, not repair generation.

## Minimum statement vocabulary

An eventual OBL-025 statement needs at least these abstract roles. The names
below are inventory names, not final Lean names.

| Role | Meaning | Current status |
|---|---|---|
| `Line1JudgmentInput` | the Line-1 judgment input that rejected | OPEN carrier |
| `Line1Rejection` | relation saying the judgment rejected in the Line-1 fragment | OPEN |
| `Diagnostic` | emitted diagnostic carrier | LAB carrier gap inventoried in `plan/80` |
| `DiagnosticFor` / `EmittedBy` | relation connecting one emitted diagnostic to the rejection | inherited from OBL-024 context, not implemented |
| `DeclaredFragment` | fragment in which single-edit repair search is allowed | OPEN boundary |
| `RepairEdit` | abstract machine-readable repair edit | OPEN vocabulary |
| `SingleEdit` | predicate saying the repair is one edit in the declared fragment | OPEN |
| `RepairInDeclaredFragment` | relation saying applying the edit keeps the candidate in the declared fragment | OPEN |
| `RepairExists` | relation saying at least one single-edit repair exists | OPEN |
| `SuggestedRepairValid` | relation saying a suggested repair is valid for the rejected premise family | OPEN; not repair-ranking proof |
| `SuggestedRepairNonEmpty` | diagnostic carries at least one suggested repair | not implemented in LAB carrier |
| `SuggestedRepairRealizes` | relation saying an emitted repair suggestion corresponds to an actual repair witness | OPEN |
| `RepairMatchesFailure` | suggested repair corresponds to the missing evidence / failed family | OPEN |
| `RepairDischargesLocalPremise` | applying the repair discharges the local reported premise, without claiming whole-program acceptance | OPEN |
| `RepairBlameTarget` | repair target points at the declaration/evidence site, not only the use site | OPEN multi-span policy |
| `CompletenessCoverage` | if a single-edit repair exists, some diagnostic for the rejection has non-empty repair suggestions | not stated/proved |

## Candidate statement skeleton

The future formal shape should be closer to:

```text
For every Line-1 judgment input I,
if checking/elaboration rejects I,
and there exists a repair edit r such that
  r is a single edit in the declared fragment
  and r addresses the rejected premise family,
then the emitted diagnostics for I include at least one Diagnostic D whose
suggested_repair list is non-empty and contains a repair compatible with r.
```

The guarantee is intentionally narrow. It does not require all possible repairs
to be listed, does not rank repairs, and does not assert global minimality.
It also does not claim whole-program acceptance after applying the repair; the
local target is that the suggestion addresses the reported premise family.

This is not yet a Lean statement because the project still needs shared
carriers for Line-1 input, rejection, repair edit, declared fragment,
single-edit repair existence, and Diagnostic repair projection.

## E-ROW instantiation target

For current G1 E-ROW evidence, the candidate instantiation is:

| OBL-025 role | E-ROW-shaped candidate |
|---|---|
| Line-1 rejection | Surface elaboration rejects because generated failures are not contained in declared `fails` |
| repair family | `add-to-fails-row` |
| single-edit candidate | add the missing generated failure family to the nearest relevant `when ... fails` row |
| repair existence input | `missing_failures = required_failures - declared_failures` is non-empty and can be added in one source edit; the clean single-edit case is exactly one missing failure family unless later atomicity rules say otherwise |
| repair target evidence | generated failures, declared `fails`, missing failures, nearest target `when ... fails` row, use span, declaration span if available |
| suggested repair payload | machine-readable edit family plus target failure row and concrete missing failure family, such as `VisibilityDenied` |
| current LAB gap | `TextualMirDiagnostic` has no `suggested_repair[]`; helper expected JSON has no repair rows |

Evidence classification:

| LAB row | Reading for OBL-025 inventory |
|---|---|
| `ELAB-07` | clean E-ROW-001-shaped add-to-fails-row pressure case |
| `ELAB-10` | clean E-ROW-002 pressure case for adding `VisibilityDenied` or declaring visibility/observe authority; exact repair family remains OPEN |
| `ELAB-04` | mixed E-ROW-shaped case; useful pressure but not a clean single-repair witness |

## Dependencies before a Lean statement

| Dependency | Needed for | Current state |
|---|---|---|
| Diagnostic carrier vocabulary | quantify over `suggested_repair[]` | inventoried in `plan/80`; not implemented |
| OBL-024 association context | connect diagnostics to the rejecting judgment | inventoried in `plan/81`; not proved |
| Line-1 fragment boundary | decide which rejections OBL-025 covers first | OPEN |
| repair edit vocabulary | say what a machine-readable edit is | OPEN |
| single-edit predicate | separate single-edit from multi-edit repairs | OPEN |
| repair existence relation | avoid claiming completeness when no single-edit repair exists | OPEN |
| repair/failure matching | ensure suggestion addresses missing evidence, not arbitrary text | OPEN |
| suggested repair validity | require at least one suggested repair that is valid for the rejection family | OPEN |
| suggested repair realization | prevent non-empty placeholder arrays from satisfying the obligation | OPEN |
| local-premise discharge | distinguish addressing the reported premise from whole-program success | OPEN |
| repair ranking | choose among multiple possible repairs | OPEN-024, later |

## What remains OPEN

- Final Diagnostic ABI and JSON field names for `suggested_repair[]`.
- Final repair edit payload format.
- Whether `add-to-fails-row` can add a set of missing failures as one edit or
  must be one failure family per edit.
- How E-ROW-002 chooses between adding `VisibilityDenied` to `fails` and
  changing visibility / observe declarations.
- Repair ranking and multi-edit repairs, which canon explicitly leaves
  post-GATE-1.
- Whether the first OBL-025 statement covers all Line-1 rejection families or
  an E-ROW fragment with a later-generalization boundary.
- Repair validity / preservation after applying the edit; this inventory only
  records non-empty repair coverage.
- Patch application semantics for edit scripts.
- Diagnostic ordering/equality when multiple diagnostics are emitted; the safe
  shape is "some associated diagnostic carries an applicable repair" until
  ordering is formalized.
- Interaction with OBL-003 Line-1 decidability; this is prerequisite context,
  not discharged here.
- Localization and human-facing wording.
- OBL-024 explanation soundness, which remains separate.

## Overclaim guards

- Do not read current helper diagnostics as having `suggested_repair[]`.
- Do not claim a non-empty `suggested_repair[]` placeholder is OBL-025 unless
  it realizes an actual single-edit repair witness.
- Do not treat a human-inferable add-to-fails-row repair as emitted evidence.
- Do not claim OBL-025 completeness from `failure_row_complete: false`.
- Do not collapse OBL-025 into OBL-024 explanation soundness.
- Do not claim repair ranking, multi-edit handling, or repair validity.
- Do not read a non-empty repair suggestion as proof that all single-edit
  repairs are covered.
- Do not imply that adding a generated failure to `fails` makes runtime
  execution safe or successful; it only declares an explicit failure surface.
- Do not freeze final repair payload JSON or message wording.
- Do not treat `ELAB-04` as a clean single-repair witness.
- Do not let E-ROW become the whole diagnostic theorem.
- Do not edit canon or move `theory/11` proof status from this LAB inventory.

## Suggested next packages

1. Additive E-ROW diagnostic carrier prototype, preserving legacy helper code,
   now constrained by OBL-024 replay-relation and OBL-025 repair-coverage
   inventories.
2. Focused E-ROW repair payload inventory if the prototype should include
   `suggested_repair[]` immediately.
3. Carrier-only E-ROW prototype without repair rows, if the next implementation
   deliberately does not advance OBL-025.
4. OBL-025 Lean statement draft only after repair edit vocabulary and
   single-edit predicate are stable enough in LAB.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation implementation.
- No OBL-024 statement.
- No OBL-024 proof.
- No OBL-025 formal statement.
- No OBL-025 proof.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
