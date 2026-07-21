---
id: meta/proposal-006
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0012]
summary: 採択済みの standing bounded autonomy。agent は既存 LAB lane で可逆な理論・実験・実装検証を自走し、working annex に active L3 record と future L2 route を置く。L2 は owner-authenticated trust anchor まで fail-closed、reserved boundary は owner に留保する。
open_items: []
---

# PROPOSAL-006 - Standing bounded autonomy

> Adopted design memo. The effective rule is ADR-0014 together with
> `working/README.md`, `plan/02-operating-model.md`, and
> `meta/agent-instructions.md`. This memo records the owner's 2026-07-21
> disposition. It does not create a Gate exit, Phase entry, SCN result,
> implementation state, final proof, or public claim.

## Owner disposition

Research agents are expected to choose and progress appropriate theory targets,
formal experiments, and bounded implementation validations autonomously. Routine
target selection is not an owner approval gate. The purpose is to discover the
smallest coherent theory and the evidence that supports or falsifies it, while
retaining the project's North Star and avoiding both premature detail and
outcome-driven specifications.

The delegated current state lives only in the canonical `working/` annex. Each
`WRK-####` record anchors existing canon read-only, pre-registers an alternative
and falsifier before its outcome is known, names its non-effects and rollback,
and keeps experiments and generated evidence in existing LAB lanes. An agent may
open and update an L3 record under ADR-0014's standing eligibility predicate.
Promotion to an L2 working position requires the frozen-material independent
review defined there: an author-signed base, a distinct reviewer-signed direct
admission, and exact record/source digest bindings. A `working/` record is a
reversible research proposition, not a change to settled theory.

The owner continues to decide L0/L1 direction and interpretation, primitives,
external contracts, SCN/Gate/Phase and lifecycle actions, every `theory/11`
movement, final proof / OBL discharge, and public completion. An ambiguity or a
candidate that reaches those boundaries is escalated with its evidence rather
than silently settled.

## Supersession and non-effects

This replaces PROPOSAL-005's owner-maintained exact-target-table mechanism with
a standing negative-list boundary and the `working/` annex. PROPOSAL-005 remains
historical evidence of the earlier, more restrictive control. This decision does
not select an OBL-020 organization, close Surface grammar, define a calculus
carrier, change a ledger entry, create a parser/runtime, promote a sample, or
change a public contract.
