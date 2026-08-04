---
id: spec/10-m9-auth-verification
status: L1-fixed
maturity: draft
depends_on: [spec/08-m7-checked-elaboration, spec/09-m8-deterministic-runtime, theory/18-m9-auth-verification, adr/ADR-0024]
summary: M9 external residual resolution、Contract policy、verification outcome、invalidationのbounded contract。
open_items: []
---

# 10 — M9 authorization and verification

## External resolution

The only selected ordinary-source route is
`CheckedSurfaceV0 → retained I8 → external M9 resolution`. Its input keys every
resolution by checked identity, embedded M8 identity, residual kind, canonical
`SourceRef`, and named target. A wrong/missing/mismatched key is a Diagnostic;
an unavailable finite check remains a ResidualObligation. M8 admission for
`AuthDeferred` or `VerifyDeferred` remains `DeferredToM9` and does not rewrite
the M7 residual/source map.

## Lanes and invalidation

`MembershipAuth` and `CapabilityAuth` are Contract policy modules. In this
profile each is non-transparent and must carry ContractUpdate provenance,
existing admission/grant lineage, and an activation cut. `finite_refinement`
is a verifier: `Judgment | ResidualObligation → Evidence | Diagnostic |
ResidualObligation`; it emits no grant, effect, mutation, or activation.

The Lean fixture for invalidation remains only the fixed two-frame trace
`canonicalFrame → revokeOrRemove canonicalFrame`: membership verification is a
`Diagnostic`, finite refinement and M9 resolution are `Residual`, and attempted
activation leaves that revoked frame unchanged. Separate accepted Rust evidence
explores a fixed one-subject/one-capability reachable-state graph to bound 4
over exactly `admit`, `grant`, `revoke`, `use`, and `reacquire`; its
input-sensitive revocation/use/rejected-use/reacquire constraints cover
monotone revocation and rejected-use-no-M8-mutation, including concrete fault
counterexamples. This does not define general M9 behavior, enumerate action
sequences, or prove authorization composition. Any exported provenance in the
selected seam remains subject to label, redaction, retention, authority, and
source/proof references.

## Boundary

This is a finite contract only. It does not claim general M9 semantics,
THM-004/005 proof, M10/SCN conformance, transport, final diagnostics catalog,
or public API/ABI/wire behavior.
