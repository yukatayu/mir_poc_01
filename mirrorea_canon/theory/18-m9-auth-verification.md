---
id: theory/18-m9-auth-verification
status: L1-fixed
maturity: draft
depends_on: [theory/02-types-effects-failures, theory/05-authority, theory/07-observation, theory/11-metatheory-ledger, theory/16-m7-checked-elaboration, theory/17-m8-deterministic-runtime, adr/ADR-0024]
summary: M9外部source-bound resolution、Contract transformerとverifierの分離、有限 provenance/invalidation model。
open_items: []
---

# 18 — M9 authorization and verification

## 1. Outer source-bound judgment

Let `A7` retain its checked identity, source-to-Core map, and residual rows,
and let `I8` be its losslessly derived embedded M8 instance. The selected
finite judgment is:

```text
⟨A7, I8, ExtensionFrame, Rauth@SourceRef, Rverify@SourceRef⟩
  ⇝9 M9AdmittedRuntime | Rejected(Diagnostic) | ResidualObligation
```

`Rauth = AuthDeferred(MembershipAuth)` and
`Rverify = VerifyDeferred(finite_refinement)` must each match kind, source,
named target, and retained identities exactly. M8 alone remains
`DeferredToM9`. The judgment records M9 resolution beside the retained rows;
it never changes M7 `execution_is_admissible`, redefines Core, or erases a
residual/source map.

## 2. Shared provenance, separate lanes

`ExtensionFrame` contains identities, exact residual rows, Contract reference,
current membership epoch/incarnation, applicable capability/witness lineage,
activation cut, policy/verifier provenance, observer policy reference, and
append-only invalidation records.

Runtime policy maps `Contract → Contract` and is the sole activation lane.
The bounded `MembershipAuth`/`CapabilityAuth` modules strengthen a precondition,
failure row, or capability requirement, so they require explicit non-transparent
ContractUpdate, admission, and cut. Credential evidence becomes a principal
claim; an ordinary admission/policy decision, not a verifier, may then issue a
capability grant.

Verifier modules map `Judgment | ResidualObligation` to `Evidence | Diagnostic
| ResidualObligation`. `finite_refinement` evidence can attest a candidate
stronger Contract but cannot replace/activate it, mint authority, erase a
failure, permit an undeclared effect, or weaken observation policy.

## 3. Invalidation and finite evidence

The selected seam requires invalidation to retain historical provenance only
through observer-safe typed projection. Its Lean fixture remains only the fixed
two-frame trace `canonicalFrame → revokeOrRemove canonicalFrame`: the
membership verifier returns `Diagnostic`, finite refinement and outer M9
resolution return `Residual`, and activation leaves the revoked frame
unchanged.

Separate accepted Rust evidence explores the fixed one-subject/one-capability
reachable-state graph to depth 4 over exactly `admit`, `grant`, `revoke`,
`use`, and `reacquire`. It is input-sensitive for revocation, attempted use,
rejected use, and reacquire inputs; it checks monotone revocation and that a
rejected use does not mutate the M8 payload, with concrete fault
counterexamples. The ledger records the status and exact evidence references.
This is neither a general proof nor an enumeration theorem for action
sequences, and it supplies no general authorization-composition theorem.

OBL-026 records only the exact finite Lean facts: SourceRef/identity/deferred-row
resolution, M8-only deferral, verifier non-authority, and `ContractRef` equality
for two transparent overlays. `MembershipAuth` and `CapabilityAuth` remain
separate non-transparent `ContractUpdate` cases. The carrier has no cost field
or cost-bound theorem. OBL-028's separate bounded model evidence does not alter
the Lean carrier or make either result a general theorem.

## 4. Boundary

No general M9 calculus, authority/noninterference proof, M10/SCN conformance,
transport, public ABI/wire, or final grammar follows from this chapter.
