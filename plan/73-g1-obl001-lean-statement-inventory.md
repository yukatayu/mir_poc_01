# plan/73 - G1 OBL-001 Lean statement inventory for THM-001

## Purpose

This file is a non-normative LAB planning inventory. It inventories the minimum
Lean-facing datatypes, predicates, and statement-shape decisions needed before
writing the THM-001 Lean statement, after `plan/71` and `plan/72`.

This file does not create the Lean statement, does not edit canon, does not
change `mirrorea_canon/theory/11-metatheory-ledger.md`, and does not claim
`lean-stated`, proof discharge, G0/G1/T1 exit, C-static conformance, runtime
dispatch, or final public API / grammar freeze.

In this file, "Lean statement inventory" means a pre-statement design checklist:
the datatypes, relations, predicates, theorem shape, and non-claims that should
be present before writing a repo-local Lean file for THM-001.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB evidence / history: legacy `specs/`, `plan/`, samples, helpers, reports,
  and Rust code outside `mirrorea_canon/`
- Advisory input: sub-agent and Oracle findings after local source review

LAB evidence is cited only as `LAB:` support. If LAB conflicts with canon,
canon wins.

## Canon anchors

| Anchor | Reading for this inventory |
|---|---|
| `mirrorea_canon/plan/00-gates.md` | G1 is ordinary assignment. G1 exit requires OBL-001 plus OBL-020/021; this inventory closes none of them. |
| `mirrorea_canon/plan/01-phases.md` | Canon implementation position remains T0. T1 is paper / Lean statement work after G1 exit conditions are satisfied. |
| `mirrorea_canon/theory/00-overview.md` | S0 stays ordinary while S1-S4 make hidden consequences explicit. Reads are dependencies; writes are occurrences. |
| `mirrorea_canon/theory/01-mircore-v0.md` | The statement must distinguish `read`, owner-local `write`, owner-directed `request`, `publish`, `observe`, obligations, generated edges, and source spans. |
| `mirrorea_canon/theory/02-types-effects-failures.md` | Failure rows are explicit finite rows; generated failures must be contained in declared failures. |
| `mirrorea_canon/theory/03-elaboration.md` | BND-001 and THM-001 are the immediate statement target: no hidden edges, span preservation, row containment, authority obligations, determinism, no authority creation. |
| `mirrorea_canon/theory/05-authority.md` | Authority validity is a later theorem family; OBL-001 should require obligation representation, not prove grant-lineage soundness. |
| `mirrorea_canon/theory/07-observation.md` | Publish / observe consequences are information effects; OBL-001 must not reduce them to helper-local debug output. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | OBL-001 remains open until an actual Lean statement is accepted through the ordinary process. OBL-020 and OBL-021 remain separate obligations. |
| `mirrorea_canon/spec/04-core-ir.md` | JSON field names are L2-working; the Lean statement should model semantic relations, not freeze exchange-form field names. |
| `mirrorea_canon/spec/06-conformance.md` | SCN-01/02 are conformance anchors, but this file does not claim a conformance pass. |
| `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md` | The statement must cover owner-directed write, same-field RHS dependency, visible publish, failure containment, spans, and write-capability obligation. |
| `mirrorea_canon/scenarios/SCN-02-attack.md` | The statement must cover owner-directed write, target/self RHS dependencies, failure containment, and nested-locus non-authority. |
| `mirrorea_canon/architecture/02-boundary-contracts.md` | BND-001 is the immediate boundary; runtime, transport, projection, provider, and devtools boundaries remain later. |

## Existing Lean evidence reading

Current Lean artifacts are useful as proof-engineering examples, not as MirCore
assignment semantics.

| Existing Lean artifact | Reusable pattern | Must not be reused as |
|---|---|---|
| `LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean` | Small finite inductives, decidable equality, and predicate-as-set containment such as `captureSubset`. | Final capability, lifetime, cost, or MirCore state semantics. |
| `LAB:samples/lean/foundations/CurrentL2LabelModel.lean` | Minimal lattice / relation proof style. | THM-001 authority, failure row, or assignment semantics. |
| `LAB:samples/lean/foundations/CurrentL2IfcSecretExamples.lean` | How to keep a tiny executable proof fragment self-contained. | THM-001 publish/observe, redaction, authority, or failure semantics. |
| `LAB:samples/lean/foundations/CurrentL2ProofSkeleton.lean` | Review-unit / emitted-stub identity preservation pattern. | OBL-001 itself, rollback/cut semantics, no re-promotion semantics, or theorem discharge. |
| `LAB:samples/lean/clean-near-end/` | Generated theorem stubs compile under Lean and preserve sample/stub naming. | Domain proof, SCN conformance, or completed theorem contract. |
| `LAB:samples/lean/manifest.json` | Machine-readable record that the current Lean files compile. | Proof status ledger for canon OBLs. |

## Statement boundary

The next Lean statement should be **assignment-local** and **elaboration-only**.

It should state:

> If a successful elaboration judgment for a simple Surface assignment produces
> a Core result, then every generated write consequence is either owner-local or
> an explicit owner-directed request; every required dependency, failure-row
> containment, authority obligation carrier, and source span relation is
> represented in the elaboration result.

It should not state:

- whole-program no undeclared communication;
- runtime request serving or fail-closed store mutation;
- grant-lineage authority soundness;
- observer-safe noninterference;
- determinism proof;
- well-formedness preservation proof;
- final exchange-form JSON field names;
- final materialization policy for transparent cross-locus reads.

## Minimum Lean vocabulary

The actual Lean file should prefer opaque or tiny finite datatypes until canon
requires more structure.

| Ingredient ID | Group | Candidate Lean ingredients | Reason |
|---|---|---|---|
| `LI-CORE-LOCUS` | Identity | `Locus`, `Principal`, `StateName`, `FieldName`, `Key`, `Span` | Enough to talk about ownership, indexed state references, and source mapping without importing domain names. |
| `LI-SURFACE-ASSIGN` | Assignment surface | `SurfaceAssign`, `SimpleAssign`, `AssignmentSpan`, `AssignedTarget`, `RhsReads` | Keeps OBL-001 scoped to simple assignment before compound assignment. |
| `LI-CORE-TERM` | Core consequence | `CoreOp`, `CoreWrite`, `OwnerLocalWrite`, `OwnerDirectedRequest`, `PublishRow`, `ObserveRow`, `DependencyRow`, `WitnessRow` | Mirrors BND-001 / THM-001 vocabulary without LAB `MessageEnvelope` overfit. |
| `LI-ELAB-RESULT` | Generated result | `ElabResult`, `GeneratedEdges`, `GeneratedFailures`, `GeneratedObligations`, `GeneratedSpans` | Lets the theorem talk about the judgment result as a carrier. |
| `LI-CORE-ROWS` | Rows and containment | `Failure`, `FailureSet`, `DeclaredFailures`, `subset` | Captures E-ROW obligations without importing helper diagnostic names. |
| `LI-AUTH-OBL` | Authority carrier | `CapabilityRef`, `WitnessRef`, `AuthorityObligation`, `AuthorityObligationPresent` | Requires representation of obligations, not authority proof. |
| `LI-OWNER` | Ownership | `OwnerOfState`, `WriteTargetOwner`, `CurrentLocus` | Distinguishes owner-local write from request. |
| `LI-CROSS-READ` | Dependency | `ReadRef`, `DependencyRecorded`, `AllRhsReadsRecorded` | Covers SCN-01 same-field read and SCN-02 target/self reads abstractly while keeping OPEN-014 materialization open. |
| `LI-SPAN-REL` | Span | `HasSpan`, `SpanPreservedForAssignment` | Captures BND-001 span preservation as a statement component. |
| `LI-DET-SEP` | Determinism separator | `ElabDeterministic` as a future signature need | Records that OBL-021 is separate and should not be smuggled into OBL-001. |
| `LI-WF-SEP` | Well-formedness separator | `WFElabInput` as input premise only, if needed | Records that OBL-020 is separate and should not become a hidden proof claim. |

## Predicate inventory

| Predicate | Intended reading | Boundary |
|---|---|---|
| `ElaboratesAssignment env ctx locus assign result` | The canon unified judgment succeeds for the assignment and returns an elaboration result. | Do not encode parser grammar or runtime step semantics here. |
| `AllGeneratedWritesSound env locus assign result` | Every generated write is owner-local at the current owner or an owner-directed request to the owner. | Does not prove request service or store update. |
| `OwnerDirectedRequestSound env locus request` | Request carries source locus, owner locus, target, capability/witness refs or obligations, failure row, and span. | Does not prove the refs are valid grants. |
| `AllRhsReadsRecorded assign result` | Every RHS read needed by the assignment is represented as a dependency/read consequence in `G_e`. | OPEN-014 keeps exact observe vs read-request materialization abstract. |
| `GeneratedFailuresContained assign result` | Generated failures are a subset of the declared `fails` row. | Static diagnostic naming remains outside the theorem statement. |
| `AuthorityObligationsRepresented result` | Required capability/witness obligations are in `C ∪ O` or the result's obligation carrier. | THM-004 / grant-lineage soundness is separate. |
| `SourceSpansPreserved assign result` | Generated Core operations and generated edges map back to the assignment span. | Does not prove final JSON `source_map` shape. |
| `VisibleWriteConsequencesExplicit env assign result` | Visible writes have explicit publish / observe consequences when canon requires them. | Does not claim runtime telemetry ABI or dispatch. |
| `NoAmbientAuthorityFromNestedLocus assign result` | Nested foreign locus blocks do not become local owner writes solely by syntax. | Does not prove all authority cases. |

## Candidate theorem shape

This is pseudocode for shape only. It is not Lean code and is not a Lean
statement.

```text
Inventory target shape, not a Lean statement:

Given:
  ElaboratesAssignment env ctx currentLocus assign result
  SimpleAssign assign
  ElaborationSucceeded result

Then:
  AllGeneratedWritesSound env currentLocus assign result
  and AllRhsReadsRecorded assign result
  and GeneratedFailuresContained assign result
  and AuthorityObligationsRepresented result
  and SourceSpansPreserved assign result
  and VisibleWriteConsequencesExplicit env assign result
  and NoAmbientAuthorityFromNestedLocus assign result
```

The final statement may split the conjunction into lemmas if Lean ergonomics
make a single theorem too broad. The split must remain traceable to THM-001.

## SCN row coverage required by the statement

| SCN row | Required in OBL-001 statement | Why |
|---|---|---|
| `SCN01-CROSS-WRITE-REQUEST` | yes | THM-001's owner-directed write alternative. |
| `SCN01-RHS-READ-DEPENDENCY` | yes, abstract dependency coverage | Current LAB JSON lacks exact same-field dependency evidence, but canon requires it. |
| `SCN01-VISIBLE-PUBLISH` | yes, consequence vocabulary | Visible-field publication must not be hidden or helper-local only. |
| `SCN01-FAILURE-CONTAINMENT` | yes | BND-001 row containment. |
| `SCN01-CAP-OBLIGATION` | yes, carrier only | Authority validity belongs to THM-004. |
| `SCN01-SOURCE-SPAN` | yes | BND-001 span preservation. |
| `SCN02-CROSS-WRITE-REQUEST` | yes | Canon attack example's write must become request, not direct remote store. |
| `SCN02-RHS-TARGET-READ` | yes, abstract dependency coverage | Required by canon even though not directly exposed by current LAB expected JSON. |
| `SCN02-RHS-SELF-READ` | yes, abstract dependency coverage | Same. |
| `SCN02-FAILURE-CONTAINMENT` | yes | Dropping generated failures must be statically visible. |
| `SCN02-NESTED-LOCUS-NON-AUTHORITY` | yes | Prevents ambient-authority interpretation of `S { ... }`. |

## Plan/72 gap intake for OBL-001 inventory

| Gap ID | Plan/72 pressure | Inventory consequence | Safe wording |
|---|---|---|---|
| `G72-READ-MAT` | SCN-01 same-field RHS and SCN-02 target/self RHS reads are required, but OPEN-014 keeps materialization open. | Add abstract `AllRhsReadsRecorded` / `CrossReadDependencyRecorded` vocabulary. | Dependency preservation is required; cache, freshness, reply, observe-vs-read-request, and projection policy remain open. |
| `G72-SIMPLE-COMPOUND` | `plan/71` scoped the first target to simple assignment. | Add `SimpleAssign`; defer compound assignment. | Compound assignment is read-plus-write and needs a separate lemma or extension. |
| `G72-FAIL-CONTAIN` | Static consequence map separated generated failures from diagnostics. | Keep success-side containment separate from negative diagnostic lemmas. | Underdeclared failure rows are static diagnostics, not generic runtime `Reject`. |
| `G72-NESTED-NONAUTH` | Nested `S { ... }` must not become ambient authority. | Add `NoAmbientAuthorityFromNestedLocus`. | Owner invariant checking under `S` does not change authorization source from the caller locus. |
| `G72-PUBLISH-OBSERVE` | Visible-field writes require explicit publish / observe consequence vocabulary. | Add `VisibleWriteConsequencesExplicit`. | Publish / observe edge inventory only; no runtime dispatch or telemetry ABI claim. |
| `G72-DIAG-ID` | Canon E-ROW-001/E-ROW-002 differ from LAB helper diagnostics. | Do not import helper diagnostic names into OBL-001. | Use canon diagnostic families only in separate negative statement work. |
| `G72-CSTATIC-BOUNDARY` | SCN rows are conformance pressure, not pass evidence. | Keep SCN mapping as fixture pressure. | No C-static, C-runtime, or C-distributed conformance claim. |

## Separation from adjacent obligations

| Adjacent item | Separation rule |
|---|---|
| OBL-020 | Well-formedness preservation can be a theorem dependency later, but OBL-001 inventory should not claim it. |
| OBL-021 | Determinism is a separate statement/proof family. OBL-001 may assume a successful result; it should not prove uniqueness. |
| OBL-002 | The proof of THM-001 comes after the statement. This inventory is not proof work. |
| OBL-004 | No-undeclared-communication is a whole-program corollary target, not this assignment-local statement. |
| OBL-015/016 | Authority soundness validates grant lineage later. OBL-001 only requires authority obligations to be represented. |
| OBL-017/018 | Observation noninterference validates observer-safe exports later. OBL-001 only keeps publish/observe consequences explicit. |

## Risks and overfit guards

- Do not mention LAB `MessageEnvelope` in the statement.
- Do not mention helper JSON keys or exact helper diagnostic
  `generated_failure_not_declared`.
- Do not mention exact SCN field names (`position`, `hp`, `atk`) except in
  commentary / scenario mapping.
- Do not encode exact edge counts if the theorem only needs coverage of all
  generated write/read/publish/observe consequences.
- Do not collapse `dependency` into runtime occurrence.
- Do not collapse `publish` / `observe` into untyped debug output.
- Do not treat `role`, key, transport, provider name, package artifact, or
  signature as authority.
- Do not freeze OPEN-014. State dependency/read-consequence preservation, not a
  cache / transport / projection policy.

## Open questions before an actual Lean statement

- What namespace and file path should hold the first repo-local OBL-001
  statement draft without implying canon `lean-stated` status?
- Should the first Lean file use a single theorem with conjunctions or a small
  family of named lemmas under one THM-001 wrapper?
- Should the statement model `C` and `O` separately, or use one obligation
  carrier with a later refinement into discharged constraints vs residual
  obligations?
- Should a helper/sample package add exact LAB rows for SCN-01 same-field
  dependency and SCN-02 two-read dependency before the Lean file, or should the
  Lean statement proceed with abstract `RhsReads` first?
- How much of BND-001 determinism should appear as an assumption in the OBL-001
  statement versus remain wholly in OBL-021?

## Close condition

This package closes only when the inventory, source-traceability mirror,
snapshot docs, validators, report, and local validations are synchronized.

Close condition is inventory-only: no Lean theorem file, no theorem statement,
no proof, no OBL-001 completion, no OBL-002/020/021 discharge, no G1 exit, no
C-static/C-runtime/C-distributed conformance claim, no runtime completion, no
runtime `MessageEnvelope` dispatch, no public grammar/API freeze, and no canon
semantic change.

## Non-claims

- No G0 exit.
- No G1 exit.
- No T1 transition.
- No OBL status movement in canon.
- No Lean statement file added by this inventory.
- No Lean proof completion.
- No theorem discharge.
- No OBL-020 / OBL-021 completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No final grammar, final Core IR JSON, public API, runtime, transport,
  projection, devtools, telemetry, provider, or product completion.

## Next safe packages

1. Actual OBL-001 repo-local Lean statement draft, statement only, with
   `theory/11` unchanged unless a later human-approved canon process says
   otherwise.
2. SCN exact LAB dependency-gap package for SCN-01 same-field RHS and SCN-02
   two-read RHS if the Lean statement would otherwise become too abstract.
3. OBL-020/021 dependency inventory, kept separate from OBL-001 proof work.
