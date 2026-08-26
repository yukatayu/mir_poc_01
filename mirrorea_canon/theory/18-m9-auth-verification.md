---
id: theory/18-m9-auth-verification
status: L1-fixed
maturity: draft
depends_on: [theory/02-types-effects-failures, theory/05-authority, theory/07-observation, theory/11-metatheory-ledger, theory/16-m7-checked-elaboration, theory/17-m8-deterministic-runtime, adr/ADR-0024, adr/ADR-0028]
summary: M9外部source-bound resolution、Contract transformer/verifier分離、finite provenance/invalidation、SYS-2 immutable successor generation。
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

## 4. SYS-2 immutable successor generation

For the selected SYS-1 kernel, the initial admitted seam retains one M9-owned
successor publisher. A production caller may request revocation of the exact
checked owner operation, but may not construct a generation, capability, or
replacement authority inventory. The publisher runs the M9 revocation path,
then retranslates the complete admitted inventory into an immutable generation
which retains:

```text
checked program identity
strict generation successor
monotone revocation tombstones
remaining owner lineages and authority uses
remaining designated remote-input release lineages
translated M8 authority state
```

The kernel validates the successor against its current generation, installs
the translated state at the sole owner runtime, waits for acknowledgement in
OW1, and publishes the new generation only afterward. A failed install leaves
the prior generation and publisher live. The successor view is crate-private
and read-only to the kernel; it cannot mint authority or act as a wire
credential.

This finite mechanism closes the SYS-1 revoke-after-enqueue/serve residual for
ST and OW1 only. OBL-059 records the executable cases; OBL-028's older bounded
M9 model remains separate evidence and is not widened.

## 5. Boundary

No general M9 calculus, authority/noninterference proof, M10/SCN conformance,
transport, public ABI/wire, or final grammar follows from this chapter.
