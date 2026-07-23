---
id: meta/proposal-011
status: L3-open
maturity: draft
depends_on: [root/glossary, theory/02-types-effects-failures, theory/08-patch-hotplug, theory/11-metatheory-ledger, plan/03-risks, adr/ADR-0012]
summary: Contract の cost_bound と transparent overlay の列挙条件の対応を owner に問う。最終 cost algebra、runtime、OBL、patch carrier は変更しない。
open_items: []
---

# PROPOSAL-011 - Transparent cost-bound substitutability

> Decision-request artifact only. This proposal records no owner answer and has
> no automatic repository effect.
>
> It does not amend `theory/02-types-effects-failures`, select the cost-bound
> algebra, add a Core constructor, change a runtime rule, an OBL, a Gate, a
> Phase, a scenario, a patch carrier, an implementation, or a public claim.

## Target and authority boundary

`theory/02-types-effects-failures` includes `cost_bound` in `Contract`, then
lists conditions that all must hold for a transparent overlay. Those conditions
name input/output, conditions, effects, failures, capabilities, surfaces,
observation, redaction, and retention, but do not name `cost_bound`.

The text does not say whether the listed conditions are the complete
substitutability predicate, or how a changed `cost_bound` is treated. This
proposal asks the owner to choose the treatment of a layer that changes only a
cost bound while all named conditions still hold. It does not assume that every
cost representation has a total order or that a cost comparison already exists.

Only the human owner may select the Contract/substitutability rule. An answer
requires the ordinary Canon process before editing theory text or any proof,
runtime, compatibility-carrier, or scenario artifact.

## Current source reading

`CON-018` defines Contract as the bundle transformed by layers, and `CON-019`
defines a transparent overlay as one satisfying substitutability. In
`theory/02-types-effects-failures`, `cost_bound` is explicitly a Contract
field. The immediately following all-must-hold condition omits it, while
`OPEN-013` says the current bound is opaque and checked by a simple numeric
comparison while final algebra and runtime semantics remain deferred. It does
not define an old/new bound relation for transparent layers.

`OBL-026` is the open obligation that overlay substitutability composes over
stacks. `theory/08-patch-hotplug` refers to the theory/02 layer laws when it
requires compatibility-preserving overlays, while its own compatibility carrier
does not currently claim to be a complete cost-bound model. `plan/03-risks`
lists the cost-bound obligation as the mitigation for the verification-cost
blind spot.

LAB is consistent evidence, not a decision: the clean-near-end index declares
three named `CostBudget` counters with a pointwise law, its cost-negative
sample rejects one remote call against a zero remote-call allowance, and the
runtime's bounded local checker performs the same pointwise comparison. The
legacy layer roadmap separately lists cost degradation as rejected while
deferring the final algebra. None of this LAB evidence settles the Canon rule.

## Question presented

> When a layer changes `cost_bound`, how should that change affect transparent
> overlay classification before OPEN-013 selects a final cost algebra?

| Option | Owner-level effect if selected | Later verification boundary | Immediate non-effect |
| --- | --- | --- | --- |
| A - scoped non-weakening (recommended) | Amend theory/02 to select an interim comparison fragment and require the new bound to be no weaker than the old one under that selected relation. The same amendment specifies that a case outside the fragment requires `ContractUpdate`. | Define the selected relation and prove/check only that fragment before relying on it for OBL-026 or an implementation. | Does not claim that an existing Canon comparison already supplies the relation, or select a final algebra, aggregate metric, runtime accounting model, Core primitive, or patch compatibility carrier. |
| B - explicit update for every bound change | Until OPEN-013 chooses an algebra, any change to `cost_bound` requires `ContractUpdate`, even when a local numeric check could show non-weakening. | Reopen after a cost algebra and its comparison law are selected. | Does not imply that cost is advisory or delete it from Contract. |
| C - advisory cost outside transparency | Clarify that `cost_bound` is not part of substitutability until a later model is selected. | Explain how this preserves the Contract/risk wording and whether `cost_bound` remains a contract field. | Does not silently retain a non-degradation guarantee. |
| D - defer the omitted-field interpretation | Make no current theory change and leave open whether the listed condition is complete or how a changed bound affects transparency. | Reopen before OBL-026, a runtime, or a scenario relies on any cost-preservation reading. | Does not create a hidden no-worsening rule or declare cost advisory. |

The LAB recommendation is **A**. It would preserve explicit-contract
discipline without pretending that named counter vectors, scalar totals, and
future runtime accounting are already one universal algebra. The owner-selected
interim fragment, rather than an already-defined Canon comparator, would decide
what happens outside that fragment.

## Evidence and verification boundary

The decision evidence is the read-only Canon source at `1eca4f61b4a085c111f4c15647cca09c7f89981f`:
`root/glossary`, `theory/02-types-effects-failures`,
`theory/08-patch-hotplug`, `theory/11-metatheory-ledger`, and `plan/03-risks`.
The pinned LAB comparison evidence is `plan/40-layer-compatibility-freeze-roadmap.md`,
`samples/clean-near-end/00_index_theories.mir`,
`samples/clean-near-end/typing/05_cost_bound_rejected.mir`, and
`crates/mir-runtime/src/clean_near_end.rs` at the same source cut.

Before a later package changes Canon text, it must verify that the selected
wording does not overclaim a total cost order, present the current numeric
comparison as an existing old/new preservation relation, collapse cost into
effects, reclassify static checking as runtime accounting, amend OBL-026, or
silently extend the patch compatibility carrier. A discovery that any of those
is required stops that package for its own owner/canon decision.

## Requested owner output

Record `A accepted`, `B accepted`, `C accepted`, `D retained`, `defer; no
current Canon change`, or `return for clarification`. The recommended A answer
authorizes only a subsequent wording package for the theory/02 transparent
overlay condition. It does not authorize a final cost algebra, an OBL change,
a proof claim, an implementation, a patch-carrier schema, or a public claim.

## Non-effects

This proposal does not:

- choose scalar total, named-vector, weighted, probabilistic, or other final
  cost algebra, nor a conversion between them;
- add a cost primitive, effect, failure, authority, contract field, comparison
  interface, runtime meter, checker, helper, schema, CLI, CI surface, or API;
- amend `OBL-026`, any theorem/obligation status, proof target, assumption, or
  proof artifact;
- interpret the LAB clean-near-end counter vector or runtime helper as a Canon
  implementation or as general Contract/layer behavior;
- change theory/08's patch compatibility carrier, activation semantics,
  `ContractUpdate` fields, or patch DAG discipline; or
- supersede PROPOSAL-003, PROPOSAL-008, PROPOSAL-009, or PROPOSAL-010.
