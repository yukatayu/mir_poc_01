---
id: meta/proposal-008
status: L3-open
maturity: draft
depends_on: [theory/03-elaboration, theory/11-metatheory-ledger, adr/ADR-0014, plan/02-operating-model]
summary: BND-001 の outcome totality と OBL-021 determinism の関係について、totality を別 obligation とする owner disposition を記録する。意味論、OBL、Gate、proof status は変更しない。
open_items: []
---

# PROPOSAL-008 - Elaboration outcome-totality boundary review

> Decision-request artifact. The owner disposition is recorded below and has
> only the stated design-package effect; it has no automatic repository effect.
>
> It does not add or change an obligation, theorem, ledger entry, Core
> primitive, Surface grammar, contract, Gate, Phase, implementation, or public
> claim.

## Target and Authority Boundary

The target is the relation between the first sentence of BND-001 in
`theory/03-elaboration` and the existing open ledger entry `OBL-021`
(`MirCore.Elab.Det`). Only the human owner may interpret or revise the BND-001
contract, select an obligation boundary, or change `theory/11`.

This proposal asks neither for a Lean target nor for a proof. It does not make
the LAB OBL-021 statement draft a canonical artifact.

## Current Source Reading

`theory/03` says that, for a well-scoped Surface item, elaboration "either
produces" the stated tuple or a Diagnostic. Its numbered contractual clause 5
separately says that elaboration is a function of its inputs and names
`OBL-021`. The ledger currently names OBL-021 only "Elaboration determinism."

The current LAB draft records pairwise coherence of successful results,
pairwise coherence of Diagnostics, and success/rejection exclusion. Its
manifested no-outcome countermodel shows that those three clauses alone permit
a well-scoped input with neither outcome. That countermodel is evidence about
the LAB draft only. It neither falsifies BND-001 nor selects a Canon
interpretation of the word "either."

The source reading therefore exposes one owner-reserved question: is BND-001
intended to require outcome existence for every well-scoped input, and, if so,
where should that requirement be tracked before a proof package relies on it?

## Question Presented

> Which reading and obligation placement should govern a later proof-facing
> elaboration package?
>
> **A**, BND-001 requires total outcome production and that direction is a
> separate obligation from OBL-021 determinism; **B**, BND-001 requires total
> outcome production and it is part of OBL-021's eventual statement; **C**,
> revise BND-001 so it only classifies an outcome when elaboration produces
> one; or **D**,
> defer the interpretation and do not let a proof package rely on totality.

## Owner disposition

Recorded on 2026-07-28: **A accepted — separate totality obligation.**

A later Canon design package must state outcome existence separately from
OBL-021's determinism boundary and explain their relation. Before it can amend
theory text or the ledger, that package must select the statement domain,
well-scopedness predicate, result/Diagnostic abstraction, and a new or
otherwise explicitly placed obligation identity through the ordinary Canon
process. This disposition does not select any of those details, change BND-001
wording today, alter OBL-021, or claim totality/proof status.

## Alternatives

| Option | Owner-level effect if selected | Immediate non-effect |
| --- | --- | --- |
| A - separate totality obligation | Later canon work may define a distinct outcome-existence obligation and its relation to determinism. | Does not choose a result/Diagnostic carrier, equality, algorithm, proof, or status. |
| B - totality within OBL-021 | Later canon work may state that the OBL-021 boundary includes both outcome existence and determinism. | Does not identify a Lean artifact, equality, Diagnostic ABI, proof, or status. |
| C - revise to outcome classification only | Later canon work revises BND-001 so it no longer promises an outcome for every well-scoped input. | This is a semantic change requiring the ordinary canon revision route, not a clarification. |
| D - defer | No proof package may use totality as a canon premise until this decision is reopened. | Does not decide that totality is false, optional, or unimportant. |

## Evidence Boundary

The relevant evidence is the read-only BND-001 wording, the open ledger, and
LAB WRK-0004/0005 statement-shape evidence. The latter distinguishes outcome
existence from actual-outcome coherence, but cannot answer this proposal by
itself. No experiment-local premise or model is promoted through this document.

## Requested Owner Output

Record `A accepted`, `B accepted`, `C accepted`, `D deferred`, or `return for
clarification` with the disputed reading. Any acceptance requires the ordinary
canon process before it changes `theory/03`, `theory/11`, an ADR, a Gate, or a
proof-facing artifact.

Recorded output on 2026-07-28: `A accepted`.

## Non-effects

This proposal does not:

- add, rename, discharge, or change the status or target of any OBL or THM;
- decide a final `Result`, `Diagnostic`, outcome, equality, comparison,
  projection, carrier, transition rule, scheduler, or proof interface;
- change the existing OBL-021 LAB statement draft, WRK evidence, or L3 status;
- claim total elaboration, determinism, no-hidden-communication, parser
  correctness, runtime correctness, conformance, or a Gate/Phase transition;
- introduce a new language primitive, failure/effect rule, transport behavior,
  public contract, helper, evidence lane, or implementation; or
- supersede PROPOSAL-001, PROPOSAL-003, or PROPOSAL-004.
