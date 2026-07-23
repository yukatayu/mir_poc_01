---
id: meta/proposal-010
status: L3-open
maturity: draft
depends_on: [root/glossary, theory/00-overview, theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, adr/ADR-0012, adr/ADR-0014, plan/02-operating-model]
summary: theory/00-overview の child locus / admission path という未定義の主語を owner に問う。Locus 階層、membership provenance、Core、OBL、Gate は変更しない。
open_items: []
---

# PROPOSAL-010 - Locus/admission subject clarification

> Decision-request artifact only. This proposal records no owner answer and has
> no automatic repository effect.
>
> It does not amend `theory/00-overview`, add a Core constructor or relation, change a
> runtime rule, an OBL, `theory/11-metatheory-ledger` status, a Gate, a Phase, a contract, an
> implementation, or a public claim.

## Target and Authority Boundary

The target is only the final sentence of item 3 in the four-graph sketch in
`theory/00-overview`:

> A child locus cannot join without its admission path.

The phrase uses `child locus` and `admission path` without defining either
subject or relation in the current Core/account of admission. This proposal
asks how that overview sentence should be made subject-correct. It does not
ask whether every membership update has admission provenance, whether loci form
a hierarchy, or whether an admission path is unique.

Only the human owner may select a replacement, deletion, or extension
boundary. The proposal is not a proof request and cannot make a LAB reading
canonical.

## Current Source Reading

`CON-001` in `root/glossary` defines a Locus as an execution place and
explicitly says it is not a Participant. `CON-002` and `CON-003` distinguish a
Principal from its participant incarnation. `CON-005` defines Admission as a
locus's judgment of a participation request.

The Core `join ℓ as R via ℓa` elaborates to
`admitreq(π_self, R, ℓ)` targeted at `ℓa`; the subject is the principal, not
the target locus. `theory/05-authority` states the same carrier as
`AdmissionRequest(π, R, ℓ)` judged at `ℓa`. Its lifecycle text describes
membership epochs and participant incarnations, while the selected `[E-ADMIT]`
rule updates `M`, grants, and witnesses.

`theory/04-ordering-and-cuts` already records the causal family
`admit_request -> verdict -> activation_cut` and makes membership-dependent
dispatch depend on its membership frontier. It does not define a parent/child
locus relation, an admission-path object, path uniqueness, or a universal
equivalence between every possible update of `M` and an admission verdict.
The selected operational rules in `theory/01-mircore-v0` are not an exhaustive
global membership-provenance theorem.

## Question Presented

> How should the undefined `child locus` / `admission path` sentence in the
> `theory/00-overview` be handled?
>
> **A**, replace it with an existing subject-correct admission summary;
> **B**, delete only that sentence; or **C**, remove it from the Core overview
> now and retain a future locus-hierarchy intention only as a separately
> specified extension question.

No option accepts a hierarchy, a new membership invariant, or a Core feature by
implication. A replacement or deletion still requires the ordinary Canon
process before modifying `theory/00-overview`.

## Alternatives

| Option | Owner-level effect if selected | Required later verification boundary | Immediate non-effect |
| --- | --- | --- | --- |
| A - existing admission summary | Replace the sentence with a subject-correct recap of existing theory: a `join` is a principal's admission request to a target locus, judged at an admission locus; a role claim is not authority. | Check the final wording against `CON-001` through `CON-005`, `[JOIN]`, and the AdmissionRequest / AdmissionVerdict carriers. It must not imply accepted-only behavior, unique paths, locus hierarchy, or a global `M`-update theorem. | Does not alter `join`, M, epochs, grants, verdicts, causal order, activation, or any proof obligation. |
| B - delete the sentence | Remove only the undefined sentence and let the graph heading point readers to `theory/05-authority` for the established admission model. | Check that the remaining graph caption does not introduce a new subject or relation. | Does not assert that a hierarchy is impossible, and does not remove any existing admission behavior. |
| C - separate future extension | Remove the sentence from the Core overview and retain any intended locus-hierarchy concern only as a future extension-level question. | A later proposal must define the relation, its subject, lifecycle, authority consequences, and interaction with admission before it can amend the Core overview. | Does not reserve a syntax, path object, parent/child semantics, or implementation route. |

## Evidence and Verification Boundary

The decision evidence is the read-only Canon source at `9878757c`:
`root/glossary`, `theory/00-overview`, `theory/01-mircore-v0`,
`theory/04-ordering-and-cuts`, `theory/05-authority`, and `adr/ADR-0014`.
Independent local cross-reference review and a temporary external advisory
review both classify the issue as a wording/subject ambiguity rather than a
missing theorem or Core primitive.

Before a later package invokes an admission-provenance property beyond the
existing local statements, it must state the exact transition domain, relation,
and proof target. Discovery that this requires a parent/child relation,
Locus-as-Principal treatment, universal membership provenance, a new
well-formedness clause, an OBL, a scenario expectation, a Gate/Phase change, or
a public contract stops that package for the ordinary Canon process.

## Requested Owner Output

Record `A accepted`, `B accepted`, `C selected as future extension`,
`defer; no current Canon change`, or `return for clarification`. The LAB
recommendation is **A**: it removes the undefined subject while stating only
the already-fixed admission model.
Acceptance authorizes only the corresponding wording package. It requires the
ordinary Canon process before editing `theory/00-overview`, an ADR, a ledger,
a Gate, a Phase, or any proof-facing artifact.

## Non-effects

This proposal does not:

- add or select a child/parent-locus relation, path object, path uniqueness,
  PlacePath semantics, or a Locus hierarchy;
- identify a Locus with a Principal or Participant, or make loci join;
- state that every membership update is caused by, or uniquely traces to, an
  AdmissionRequest or AdmissionVerdict;
- alter `M`, epoch or incarnation semantics, grants, witnesses, the existing
  `admit_request -> verdict -> activation_cut` order, or the selected
  `[E-ADMIT]` rule;
- add, rename, discharge, or change the status, target, wording, or Lean
  target of any OBL or THM;
- change scenarios, conformance, contracts, implementation, helper/schema/CI
  surfaces, or public readiness; or
- supersede PROPOSAL-003, PROPOSAL-008, or PROPOSAL-009.
