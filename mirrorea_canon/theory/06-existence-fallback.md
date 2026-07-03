---
id: theory/06-existence-fallback
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, adr/ADR-0004]
summary: 存在 DAG、lease、guarded option chain、正規化法則、静的証拠 floor、単調劣化と THM-002。
open_items: [OPEN-018, OPEN-019]
---

# 06 — Existence, lifetime, fallback

## Existence DAG

State cells and objects form a DAG of existence dependency: a child may hold
references to ancestors freely; referencing something with *possibly shorter*
lifetime is legal **only through a fallback chain** that ends in a value whose
lifetime dominates the reader's ("eventually an ancestor's value"). Changing a
parent is an occurrence (graph 1), not an edit of the existence DAG in place.
Schema-level dependency cycles (ranking ↔ score[*]) are rejected or converted
into explicit residual obligations; never an implicit back-edge.

## Chains, options, leases

A guarded reference is a finite chain over one logical access path:

```text
o₁ > o₂ > ... > oₙ      oᵢ = option(name, target, cap, lease [, admit])
```

`lease` is an option-local lifetime guard — not a failure class. Expiry means
the option can no longer be a success-side choice; it is one species of
monotone degradation.

## Canonical normalization law (settled)

`canon` flattens left-to-right: singleton for a lone option;
`canon(fallback(x, y)) = canon(x) ++ canon(y)`, applicable only when x and y
share the logical access path / semantic lineage and each later stage is a
monotone degradation of its predecessor. Canonical form retains (order, guard,
contract, capability) per option — not the nested inner/outer syntax.
Evaluation reads the leftmost admissible option. Nested forms with the same
order denote the same chain. (OBL-005 flattening laws; OBL-006 confluence.)

## Static evidence floor (settled)

Same-lineage is a static claim requiring **both**: a `declared access target`
on each option, and an **edge-local lineage annotation** (predecessor ref,
successor ref, affirmative same-lineage claim) on exactly the fallback edge it
decorates. Neither alone suffices. Successor compatibility requires: no
capability strengthening, no explicit contract contradiction. Classification:

- **malformed** (static reject, E-LIN-002): annotation points at the wrong
  edge; target mismatch; lineage denial; capability strengthening; explicit
  contract contradiction.
- **underdeclared** (static error, E-DECL-001): any floor element missing.
  Never silently admitted, never demoted to dynamic Reject.
- **dynamic Reject**: well-formed chain, admissible options exhausted at run
  time (lease expiry, require miss, explicit failure, write-after-expiry with
  no later write-capable option).

## Monotone degradation and THM-002

Later options carry ≤ guarantees; write capability may weaken to read-only
along the chain. Write-after-expiry: try later write-capable options
explicitly, else request-level `Reject`; never hidden buffering or hidden
resurrection. `try`/rollback restores state, `atomic_cut` moves the rollback
frontier — **neither rewinds the degradation order**.

```text
THM-002: In any well-formed trace, for each chain instance the selected-option
index is non-decreasing along one lineage; re-selection of an earlier option
occurs only via an explicit reacquire occurrence that starts a new lineage
with fresh witness/epoch (ADR-0004).
```

(OBL-007 statement, OBL-008 proof.) Observation: admit-miss and lease-expired
are audit-side non-admissible subreasons, not dedicated occurrences; request-
level outcomes dominate the event surface.

## Why this matters for virtual space (informative)

Hosts leave, shards split, anchors vanish, links drop: "degrade to an ancestor
value, recover explicitly and freshly" is the graceful-degradation law for
avatars, LOD, and interest management. The known intuition gap ("outer wrapper
extends lifetime and we return to it") is documented in mental-model/03 and in
diagnostics wording (spec/07, E-LIN family).

OPEN-018: option-local `admit` predicate fragment breadth (currently: atoms,
application, `and`, grouping). OPEN-019: dedicated observation surface for
lease expiry (currently audit metadata only).
