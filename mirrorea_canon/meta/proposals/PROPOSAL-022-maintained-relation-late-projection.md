---
id: meta/proposal-022
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0018]
summary: owner-approved M4 scopeで、owner-held maintained relation と consumer-local late projection を採用する提案。
open_items: []
---

# PROPOSAL-022 — Maintained relation / late projection

## Owner disposition

Apply the owner-approved M4 scope in PROPOSAL-018 and ADR-0015. Adopt a
relation-first carrier: an authoritative owner holds `RelationDef` and its
mutable `BindingState`; a consumer receives an admitted projected relation and
evaluates it only in one coherent `PresentationContext`. A concrete application
instance is recorded exclusively in SCN-12.

The publication crossing the boundary is M3 `publish-relation`. The consumer's
presentation-frame evaluation remains consumer-local and `local-only`; it
does not receive an absolute derived-value publication or an adapter stream.
The source relation, its owner, authority, activation frontier, lineage, and
redaction constraints therefore remain visible rather than being hidden in a
renderer or transport cache.

## Selected finite rule

`RelationDef` contains only domain-neutral relation references, owner,
relative transform, relation policy, and ordered guarded anchors. Its owner
holds `BindingState = ⟨selected-option, lineage, binding-epoch,
witness-epoch, activation-frontier, anchor-epochs⟩`. A semantic invalidation
may advance primary to fallback for that lineage; it cannot re-promote. A
fresh witness plus fresh binding epoch is an explicit reacquire that starts a
new lineage. Consumer-local sample gaps return a presentation fallback only;
they do not change `BindingState` or create a semantic occurrence.

Each `PresentationContext` names one frontier and carries the relation's
admitted anchor samples at their recorded anchor epochs. Missing, stale, or
split-frontier input rejects evaluation. The derived release label is the
greatest restriction in the finite M4 order of the relation label and every
input label. A consumer mutation, relation cycle, or requested release weaker
than that label is rejected.

## Alternative and falsifier

The sole smaller alternative is to send the consumer an absolute evaluated
derived pose.
It is rejected: it erases the maintained relation and its activation/lineage
from C's input, pressures an unannounced value/stream materialization, and
cannot establish the M4 `project then evaluate ≃ evaluate then project`
finite law. A split-frame or stale-anchor counterexample is the required
falsifier for any purported projection implementation.

## Scope and non-effects

This authorizes ADR-0019, the M4 theory/spec/scenario amendments, finite Lean
evidence, and their exact ledger classification. It does not select Surface
grammar, final Core/JSON/wire/API fields, an arbitrary label lattice,
transport/retry, renderer algorithm, general relation-DAG proof, or a
save/load implementation. General OBL-005--008 and OBL-022--023 remain
deferred.
