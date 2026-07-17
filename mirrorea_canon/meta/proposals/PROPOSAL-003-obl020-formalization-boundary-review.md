---
id: meta/proposal-003
status: L3-open
maturity: draft
depends_on: [plan/00-gates, plan/01-phases, plan/02-operating-model, theory/01-mircore-v0, theory/11-metatheory-ledger, meta/proposal-001]
summary: OBL-020 proof-facing formalization organization の owner decision request。shared five-heading review checklist / no required shared checklist / defer の三択のみを問う。PROPOSAL-001 scope、rule equation、proof status、Gate は動かさない。
open_items: []
---

# PROPOSAL-003 - OBL-020 Formalization Boundary Review

> Decision-request artifact only. It records no owner answer and has no
> automatic repository effect.
>
> It does not change a canonical transition rule, `theory/11` status, Gate or
> Phase state, proof interface, runtime, conformance, or public API.
>
> It does not narrow, condition, reopen, or supersede the owner disposition
> recorded in PROPOSAL-001.

## Target and Authority Boundary

The target is the preparation boundary for the existing open obligation
`OBL-020` (`MirCore.Step.WF`): well-formedness preservation of step rules.
`theory/11-metatheory-ledger` remains the only proof-status authority;
`plan/00-gates` remains the only Gate authority; and only the human owner may
make an L0/L1 decision or an ADR effective.

This proposal requests a decision about how later formalization should be
organized. It does not request a status, identify a Lean artifact as the canon
target, or introduce a concrete semantic equation.

## Current State

`theory/01` names a configuration with `H / Q / S / M / G / W / L / P`, five
well-formedness clauses, and selected step-rule sketches. It also states
OBL-020, which remains open.

LAB:T-RESEARCH-006 audited 13 selected operational cases against the five
named well-formedness clauses. Under its frozen source-adequacy classification,
0 of 65 cells were derivation-complete (`direct` or `delegated`); all 65 were
classified `missing`.

In that audit, `missing` means only that the frozen canon source cut did not by
itself supply a complete preservation argument without an unstated update,
frame, freshness, history, graph, or record assumption. It does not mean that
canon supplies no partial evidence, that the intended preservation claim is
false, or that the five normalized groups are canon vocabulary.

PROPOSAL-001 accepted only the current abstract OBL-020 Lean statement shape as
a G1-supporting scope artifact for proposal preparation. That acceptance remains
unchanged under A, B, or C. This proposal neither narrows nor conditions that
accepted scope, and it does not select a concrete `Config`, `Step`,
`WellFormed`, artifact identity, wrapper, or per-step proof interface.

## Question Presented

> Which organizational posture should govern later owner-reviewed OBL-020
> proof-facing formalization packages:
>
> **A**, a shared five-heading review checklist;
> **B**, no required shared checklist, with each package choosing its own
> formalization organization; or
> **C**, deferral of the organizational choice?
>
> This question does not decide any carrier, transition relation or equation,
> theorem argument, proof decomposition, rule taxonomy, scheduler, Lean artifact
> identity, implementation, or language feature.

The five candidate headings below reproduce T-RESEARCH-006's LAB grouping for
its selected 13-case by five-clause matrix:

1. **LAB `H_EXTENSION` - history-change review:** if a rule changes `H`, what
   relation accounts for retained history, any occurrence identity or freshness
   condition, added events or edges, and causal orientation?
2. **LAB `COMPONENT_FRAME` - unaffected-state review:** what relation, if any,
   governs configuration aspects not changed by the rule?
3. **LAB `STATE_MEMBERSHIP_COHERENCE` - state/membership review:** what indexed
   key, epoch, tombstone, and cross-`S`/`M` coherence facts are relied upon?
4. **LAB `AUTHORITY_RECORD_BINDING` - authority/record review:** what request,
   use, validation, grant, witness, frontier, and lineage facts are relied upon?
5. **LAB `CHAIN_TRANSITION_DEFINITION` - chain-state review:** what
   lease-position, history-maximum, lineage, and cut-interaction facts are
   relied upon?

These are candidate review headings, not canon predicates, theorem premises,
record fields, transition constructors, necessary or sufficient conditions, or
an exhaustive taxonomy. They are not the five well-formedness clauses and are
not claimed to be one-to-one with those clauses.

A later package may handle a heading definitionally, through an abstract
interface, invariant, or shared lemma, through per-rule reasoning, or by
recording it as inapplicable or outside that package's scope. It may add
package-local headings and may split, merge, or rename headings if it records
the mapping explicitly. No omitted heading supplies an implicit transition
equation or frame default.

This organizational choice is not a prerequisite for citing the abstract scope
artifact under PROPOSAL-001, for bounded LAB counterexample research, for
statement transcription, or for disposable exploratory proof work permitted
elsewhere. It governs only the organization of later owner-reviewed
proof-facing packages and may be superseded by a later owner/canon decision.

## Alternatives

| Option | Organizational meaning | Immediate organizational effect | Semantic / proof-status delta |
| --- | --- | --- | --- |
| A - shared review checklist | Adopt the five LAB-derived headings as a required, non-exhaustive review index for later OBL-020 packages. A package records how each heading is handled at the granularity it uses and may add package-local headings. | Establishes a common comparison surface. | None. |
| B - independent per-rule or per-package formalization | Adopt no repository-wide required checklist. Each later package chooses its own granularity and decomposition and may voluntarily reuse any subset of the five headings, shared abstractions, or lemmas. | Establishes no standardized cross-package review index. | None. |
| C - defer organizational choice | Make no organizational decision now. Reopen only when a later owner-reviewed OBL-020 package demonstrates a concrete need to select an organization. | Establishes no owner-adopted organizational convention. | None. |

Under all three options, concrete representations, equations, proof interfaces,
artifact identity, proof status, and Gate or Phase state remain unresolved.
Under all three options, the PROPOSAL-001 owner disposition remains unchanged.

## Advisory Recommendation Recorded by the LAB Bundle

T-RESEARCH-007 records A as its advisory recommendation because a common review
index would make the selected source-adequacy findings easier to compare. This
paragraph records that advisory provenance only; it is not an owner disposition
and has no repository effect.

The advisory recommendation does not assert that the headings are necessary,
sufficient, exhaustive, independently semantic, or required as explicit Lean
premises. It does not claim that A is the narrowest possible disposition, that
A keeps the language core smaller than B or C, or that accepting A validates any
project theorem or semantic direction.

Under A, a later package records how each review heading is handled; it is not
restricted to classifying record fields as framed, extended, updated, or
irrelevant. The package may use equality, another relation, constructors,
abstraction interfaces, generic preservation lemmas, per-rule arguments, or
another separately selected proof organization. No omission supplies an
implicit semantic default.

## Effect of a Recorded Owner Disposition

A recorded owner disposition has only the organizational effect stated in the
alternatives table. It does not itself authorize implementation, Lean proof
claims, proof-status movement, automatic canon edits, or adoption of a
transition equation or proof interface.

Before a later package proposes adoption of a canonical transition rule or proof
interface, that package must separately identify:

- the rule, rule family, or generic constructor in scope;
- the representation it relies upon, including any intentionally abstract part
  and the assumptions on that abstraction;
- how the claimed preservation result is obtained at the package's chosen
  granularity;
- the well-formedness clauses and preservation coverage actually claimed; and
- every additional semantic assumption not derivable from existing canon,
  together with the separate owner/canon decision being requested for it.

No disposition in this proposal requires a record-shaped `Config`, fieldwise
equality, one theorem per audit cell, one lemma per heading, a fixed per-rule
case split, a fixed ordering of proof obligations, or exposure of the checklist
as the final Lean theorem interface.

## Non-effects

This request does not:

- narrow, revoke, condition, or reopen the PROPOSAL-001 owner disposition;
- change any canonical transition rule, well-formedness clause, ADR,
  `theory/11` proof status, OBL status, Gate, or Phase;
- complete, prove, discharge, restate, or move OBL-020 or any other obligation;
- make a LAB Lean statement draft the canon `MirCore.Step.WF` artifact;
- convert the five LAB headings into canon semantic vocabulary or assert that
  they are exhaustive, necessary, sufficient, minimal, or one-to-one with the
  five well-formedness clauses;
- choose a configuration or history carrier, occurrence identity, graph-update
  representation, frame equality or relation, record schema, rule taxonomy,
  scheduler semantics, theorem-argument shape, proof decomposition, lemma
  granularity, or proof order;
- prohibit or delay bounded LAB research, counterexample work, statement-shape
  work, or disposable exploratory proofs otherwise permitted by the operating
  rules;
- create a new evidence lane, helper family, proof API, source-language concept,
  builtin, effect, transport behavior, runtime implementation, conformance
  claim, product claim, or public API;
- establish G0/G1 exit, T1/T2 entry, proof readiness, conformance, or runtime
  correctness; or
- authorize the OBL-001 concrete-evidence bridge.

## Requested Owner Output

Record one of the following:

- `A accepted`, `B accepted`, or `C deferred`; or
- `return for clarification`, with the specific family or scope ambiguity.

For C, the recorded deferral trigger is:

> Reopen when a later owner-reviewed OBL-020 proof-facing package demonstrates
> a concrete need to choose a shared organizational boundary.

That trigger returns the question to owner review; it does not select A or B and
does not authorize semantic adoption or proof-status movement.

No recommendation, silence, generic continuation instruction, LAB
classification, or index regeneration counts as an owner disposition. Recording
an owner disposition in this proposal does not itself change transition
semantics, theorem wording, proof status, or Gate or Phase state. Any later
normative edit requires separate authorization and must follow the ordinary canon
route applicable to that specific edit.
