---
id: meta/proposal-027
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0023]
summary: M8 deferred residualをsource-bound外部M9解決へ渡す有限 auth/verification seam の提案。
open_items: []
---

# PROPOSAL-027 — M9 auth and verification seam

## Owner disposition

Select one finite M9 outer resolution over a retained M7 checked artifact and
an embedded, structurally derived M8 instance. Its input preserves the M7/M8
identities, ordered source-to-Core map, and exact `AuthDeferred` /
`VerifyDeferred` rows. M8 `admit` alone remains `DeferredToM9`; it cannot
grant authority, return a proof verdict, or create runtime success.

An `ExtensionFrame` carries that provenance plus the current contract reference,
membership epoch/incarnation, capability/witness lineage where used, activation
cut, policy/verifier provenance, observation delta, and invalidation references.
It has non-coercible runtime-policy and verifier lanes. `MembershipAuth` and
`CapabilityAuth` are policy transforms; their bounded changes are
non-transparent and require the existing explicit `ContractUpdate`. The
`finite_refinement` verifier returns only `Evidence`, `Diagnostic`, or
`ResidualObligation`.

The accepted implementation evidence for this proposal is a fixed
one-subject/one-capability reachable-state graph to bound 4 over exactly
`admit`, `grant`, `revoke`, `use`, and `reacquire`. It uses input-sensitive
revocation/use/rejected-use/reacquire constraints to check monotone revocation
and rejected-use-no-M8-mutation, with concrete fault counterexamples. This is
bounded evidence only: it is neither a general proof nor an action-sequence
enumeration or authorization-composition theorem.

## Falsifier and non-effects

The proposal fails if a wrong source row/embedded identity resolves a deferred
row, verifier evidence becomes a grant or `admitted_by` value, or either lane
silently weakens a contract field. It does not claim a general correspondence
theorem, general M9/Rust behavior, M10/SCN conformance, transport, public
API/ABI/wire, or a general authority proof.
