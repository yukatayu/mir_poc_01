# Plan 185 - Cost-bound substitutability primary-literature audit

## Role and authority

This is LAB decision support for
`PROPOSAL-011-transparent-cost-bound-substitutability`. `mirrorea_canon/`
remains normative. This comparison neither selects an A/B/C/D alternative nor
changes the Contract, transparent overlay, OBL-026, a patch carrier, a cost
algebra, runtime accounting, a sample workflow, or a Gate/Phase.

## Question

What do primary sources establish about making a cost or resource bound part
of substitutability and compositional contract reasoning, and which premises
would be necessary before that evidence could support one of PROPOSAL-011's
owner choices?

## Canon baseline

`theory/02-types-effects-failures.md` places `cost_bound` in `Contract`, but
does not name it among the transparent-overlay conditions. `OPEN-013` leaves
the final bound representation and comparison open. Consequently, no current
Canon relation tells an overlay how to compare an old and a new bound, how to
handle incomparable bounds, or how that comparison composes. The proposal
correctly leaves this as an owner/canon decision rather than treating absence
from the list as either preservation or permission to weaken.

## Primary-literature comparison

| Source | Relevant result | What it supports here | What it does not support here |
| --- | --- | --- | --- |
| [Liskov and Wing, *A Behavioral Notion of Subtyping*](https://www.cs.columbia.edu/~wing/publications/LiskovWing94.pdf) | A subtype must preserve the observable behavioral obligations of the supertype; the paper states this through preconditions, postconditions, exceptions, and history constraints. | If a cost bound is declared to be an observable Contract guarantee, a silent weakening is incompatible with ordinary substitutability reasoning. | It does not classify Mir `cost_bound` as observable, choose a resource order, or provide a resource-composition theorem. |
| [Atkey, *Amortised Resource Analysis with Separation Logic*](https://bentnib.org/amortised-sep-logic-journal.pdf) | Resource specifications use an explicit resource interpretation and composition discipline. | A meaningful bound relation needs a stated carrier and composition law; a counter total is not automatically a Contract refinement relation. | It does not select scalar, vector, or opaque bounds for Mir, nor does it define transparent patch compatibility. |
| [Das, Hoffmann, and Pfenning, *Work Analysis with Resource-Aware Session Types*](https://arxiv.org/abs/1712.08310) | Communication cost is explicit, interaction-dependent, and specified compositionally under a chosen operational cost semantics. | Distributed cost claims need a chosen metric, bound function, and accounting semantics before a non-weakening rule can be checked or composed. | It does not license a universal scalar projection, validate the three LAB CostBudget counters, or choose a Mir runtime model. |
| [Sharf, Besselink, and Johansson, *Verifying Compositional Refinement of Assume/Guarantee Contracts using Linear Programming*](https://arxiv.org/abs/2103.13743) | Refinement and composition are proved against an explicit contract relation and model assumptions. | Any Contract-level cost rule needs its comparison direction and composition premises in the relation, not as an unstated overlay convention. | It does not choose whether Mir cost is transparent, advisory, or deferred, and it does not supply Mir's relation or proof premises. |

## Findings

The sources agree on an explicitness requirement, not on a Mir-specific
algebra: a cost bound can participate in substitutability only after the
project states (1) the resource carrier or bounded fragment, (2) comparison
direction, (3) treatment of unknown or incomparable values, and (4) the
composition premise appropriate to the Contract and patch operation.

No source determines whether the present `cost_bound` field is an observable
guarantee, whether it is scalar or pointwise, whether the active LAB counters
are the relevant carrier, or whether a transparent overlay is the applicable
composition operation. Those remain project decisions.

## Consequences for the proposal alternatives

| Alternative | Literature-backed precondition | Current decision support |
| --- | --- | --- |
| A. Scoped non-weakening | Owner defines a bounded carrier fragment, an old/new relation with direction, an outside-fragment response, and a composition premise. | A is a defensible target only after those terms are recorded. It is not already entailed by Canon or by the LAB CostBudget example. |
| B. Explicit update for every change | No implicit comparison is used before an algebra/fragment is selected. | This is the conservative operational policy while the comparison remains open; it preserves an explicit change boundary without asserting a resource law. |
| C. Advisory treatment | Owner explicitly classifies cost as outside observable substitutability and records the resulting Contract meaning. | The sources do not justify that inference. It may be coherent, but it needs an affirmative project policy rather than omission from the overlay list. |
| D. Interpretation defer | No transparent cost comparison is available. | This accurately preserves the present Canon openness, but it prevents OBL-026, compatibility, or runtime reasoning from relying on an implicit rule. |

This table is decision support, not a recommendation that silently selects B
or a claim that A is currently specified. The owner may choose an interim A,
B, C, or D through the Canon process according to the intended Contract
meaning and deployment needs.

## Safe follow-up boundary

- A future canon proposal for A must state the carrier/fragment, relation
  direction, incomparable/unknown behavior, composition law, and whether the
  rule is a compatibility check or a required explicit `ContractUpdate`.
- A future implementation or OBL work item must cite that decision; it may not
  derive it from a helper-local counter or this literature comparison.
- A distinct L3 experiment may still investigate a fresh admitted LAB fact
  under its own pre-registration, but cannot choose a Contract rule, final
  algebra, runtime semantics, or proposal outcome.

## Non-claims

No theorem, model check, Lean result, countermodel, runtime experiment, API,
grammar, cost metric, order, patch rule, OBL, Canon text, Gate, Phase, or
public-readiness status is created or advanced by this audit.
