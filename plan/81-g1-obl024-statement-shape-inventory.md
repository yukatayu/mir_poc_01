# plan/81 - G1 OBL-024 statement-shape inventory

## Purpose

This file inventories the statement shape needed for OBL-024 explanation
soundness before writing a Lean statement or changing diagnostic code. The
later LAB-only Lean statement draft is now recorded in `plan/109`, and the
later executable LAB projection carrier is recorded in `plan/110`; this file
remains the pre-draft relation inventory.

This is LAB repository memory. It does not state OBL-024 formally, does not
prove OBL-024, does not implement a final Diagnostic ABI, does not claim
explanation soundness, does not claim conformance, and does not edit canon.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration contract:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB E-ROW alignment:
  `plan/79-g1-erow-diagnostic-alignment.md`
- LAB diagnostic carrier inventory:
  `plan/80-g1-diagnostic-carrier-inventory.md`

If this LAB inventory conflicts with canon, canon wins.

## Relation inventory, not carrier inventory

`plan/80` already inventories Diagnostic carrier fields and current LAB carrier
gaps. This file does not repeat that as a schema decision. Its job is to list
the relations that must exist before a future OBL-024 statement can say a
Diagnostic's reported rule instance and failed premise are actual.

## Canon target reading

Canon `theory/10` reads OBL-024 as explanation soundness:

```text
Every emitted Diagnostic's rule_instance and failed_premise are actual:
replaying the judgment with the reported bindings fails exactly there.
```

For G1 work, the immediate concrete pressure is E-ROW diagnostics produced from
BND-001 row containment, but the statement shape must not hard-code only
E-ROW.

This package's concrete inventory scope is intentionally narrower:
E-ROW-shaped Surface elaboration diagnostics only. That narrow scope is a
working prerequisite, not a claim that OBL-024 itself is only about E-ROW.

## Minimum statement vocabulary

An eventual OBL-024 statement needs at least these abstract roles. The names
below are inventory names, not final Lean names.

| Role | Meaning | Current status |
|---|---|---|
| `JudgmentInput` | the scoped input to the judgment that rejected | not yet a shared Lean carrier |
| `Diagnostic` | emitted diagnostic carrier | LAB carrier gap inventoried in `plan/80` |
| `DiagnosticFor` / `EmittedBy` | relation connecting one emitted diagnostic to the rejecting judgment attempt | not implemented |
| `RuleInstance` | named rule instance reported by the diagnostic | OPEN vocabulary |
| `Premise` | named failed premise inside the rule instance | OPEN vocabulary |
| `ActualRuleInstance` | relation saying the named rule schema exists and is instantiated under the same judgment context | OPEN |
| `PremiseOf` | relation saying the failed premise belongs to the reported rule instance | OPEN |
| `Bindings` | variable / entity bindings used by the failed premise | OPEN representation |
| `BindingReconstructable` | relation saying reported bindings are sufficient to reconstruct the rule instance and failed premise | OPEN |
| `Replay` | relation that reruns the relevant judgment slice | not implemented |
| `FailsExactlyAt` | relation saying replay fails at the reported premise, not elsewhere | not implemented |
| `CarrierReports` | projection from diagnostic carrier to rule, premise, bindings, span, refs | depends on future Diagnostic ABI |
| `IdCompatible` | relation between diagnostic ID family and failed premise family | OPEN; no helper ABI freeze |
| `SpanBlame` | relation between failed premise, missing declaration, and reported span(s) | OPEN multi-span policy |
| `SameDiagnostic` / equality context | relation to OBL-021 diagnostic determinism and diagnostic comparison | context only; not discharged here |

## Candidate statement skeleton

The future formal shape should be closer to:

```text
For every judgment input I and emitted diagnostic D,
if the checker/elaborator emits D while rejecting I,
and D reports rule instance R, failed premise P, and bindings B,
then R is an actual rule instance of the judgment for I,
P is a premise of R under B,
and replaying the judgment slice for I/R/B fails exactly at P.
```

Here "exactly at P" should be read trace-locally: replay follows the same
named derivation path and the reported premise is the failing premise on that
path. This inventory does not add global root-cause uniqueness, ranking, or
minimality.

`plan/109` adds a compile-check-only Lean statement draft using abstract
carriers for `JudgmentInput`, `Diagnostic`, `RuleInstance`, `Premise`,
`Bindings`, and `Replay`. That draft is LAB evidence only and does not prove
OBL-024 or settle final carriers.

## E-ROW instantiation target

For current G1 E-ROW evidence, the candidate instantiation is:

| OBL-024 role | E-ROW-shaped candidate |
|---|---|
| `JudgmentInput` | Surface elaboration input under `(Sigma, Psi, Gamma, Delta, L)` |
| `RuleInstance` | BND-001 row-containment clause for generated remote requests |
| `Premise` | generated failure set is contained in declared `fails` row |
| `Bindings` | request kind, requester locus, owner locus, state name, key expression, generated source, generated failures, declared failures, missing failures, source item/request row, source span |
| replay failure | recompute generated request failures and observe `required_failures ⊄ declared_failures` |
| missing evidence | `missing_failures = required_failures - declared_failures` |
| ID compatibility | E-ROW-001 for generated failures not contained in declared `fails`; E-ROW-002 for undeclared `VisibilityDenied` |
| span/blame | failure row / generated request source span; declaration-site versus use-site split remains OPEN |
| current LAB diagnostic | helper-local `generated_failure_not_declared`, not canon carrier |

The current LAB rows support this inventory by carrying
`required_failures`, `declared_failures`, `failure_row_complete`, and
`source_span` on remote request evidence. They do not yet emit those as a
Diagnostic carrier and do not prove replay.

Evidence classification:

| LAB row | Reading |
|---|---|
| `ELAB-07` | clean E-ROW-001-shaped write failure-row evidence |
| `ELAB-10` | clean E-ROW-002 pressure evidence for `VisibilityDenied`, still using the same helper-local diagnostic family |
| `ELAB-04` | mixed E-ROW-shaped evidence, not the clean E-ROW-002 case |

## Dependencies before a Lean statement

| Dependency | Needed for | Current state |
|---|---|---|
| Diagnostic carrier vocabulary | quantify over emitted diagnostics and reported fields | inventoried in `plan/80`; ABI not final |
| diagnostic-to-judgment association | connect emitted diagnostics to the rejecting judgment attempt | OPEN |
| rule / premise ID vocabulary | say what `rule_instance` and `failed_premise` mean | OPEN |
| bindings representation | replay reported premise with concrete variables/entities | OPEN |
| replay relation | connect emitted diagnostic to actual failed judgment slice | OPEN |
| trace-local exactly-there relation | avoid overclaiming global root-cause uniqueness | OPEN |
| span/blame relation | enforce declaration-site blame for underdeclared cases | OPEN |
| diagnostic family coverage | avoid overfitting OBL-024 to E-ROW only | OPEN |
| diagnostic equivalence / ordering | handle multiple diagnostics without accidental ABI freeze | OPEN |
| OBL-021 determinism context | ensure diagnostic comparison does not conflict with determinism statement shape | prerequisite context only, not discharged |

## What remains OPEN

- Final Diagnostic ABI and JSON field names.
- Whether helper output keeps both `legacy_code` and `canon_id` during
  migration.
- Canonical names for rule instances and failed premises.
- Binding payload shape.
- Replay granularity: whole judgment replay versus rule-local replay witness.
- "Exactly there" semantics beyond trace-local replay; this inventory does not
  claim global uniqueness or repair ranking.
- Multi-span declaration/use-site representation.
- Warning diagnostics: this package scopes the concrete G1 pressure case to
  rejection/error diagnostics.
- Whether OBL-024 first statement is all diagnostic families or an E-ROW
  fragment plus later generalization.
- How parse diagnostics and elaboration diagnostics share, or do not share, the
  same proof statement.
- Diagnostic ordering and equality when more than one diagnostic is emitted.
- OBL-021 diagnostic equality granularity as prerequisite context.
- Non-E-ROW diagnostic families such as E-AUTH, E-IDX, and E-PATCH.
- OBL-025 repair completeness; this inventory only touches OBL-025 where it
  shares carrier fields.

## Overclaim guards

- Do not read current `generated_failure_not_declared` as canon E-ROW ABI.
- Do not read expected JSON projections as raw or final Diagnostic carriers.
- Do not read `failure_row_complete: false` as a proof of OBL-024.
- Do not claim explanation soundness before replay relation and carrier
  projection exist.
- Do not claim OBL-024 ledger movement; `theory/11` remains the proof-status
  source.
- Do not smuggle in OBL-020, OBL-021, THM-001, conformance, or G1 exit.
- Do not turn trace-local "fails exactly at this premise" into global
  root-cause uniqueness or ranking.
- Do not collapse OBL-024 into OBL-025 repair completeness.
- Do not make E-ROW-specific helper evidence the whole diagnostic theorem.
- Do not use `ELAB-04` as the clean E-ROW-002 case.
- Do not edit canon from this LAB inventory.

## Suggested next packages

1. Additive E-ROW diagnostic carrier prototype, preserving legacy helper code,
   if executable carrier evidence is desired next.
2. OBL-025 statement-shape inventory, still docs-only, to separate repair
   completeness from soundness.
3. OBL-024 Lean statement draft was added later as `plan/109` using abstract
   diagnostic projection and replay vocabulary. It remains compile-check-only.
4. OBL-024 executable projection carrier evidence was added later as `plan/110`
   inside current E-ROW `lab_diagnostic_details`.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No OBL-024 formal statement.
- No OBL-024 proof.
- No OBL-025 statement.
- No OBL-025 proof.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
