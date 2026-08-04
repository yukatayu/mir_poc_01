# MirTheoryV0M9AuthVerification.lean

This self-contained finite M9 fixture proves exact source-bound external
resolution, M8-only deferral, verifier non-authority, and transparent-overlay
composition. It also evaluates one fixed two-frame revocation/no-mutation
trace. It retains an M7/M8 identity and two exact deferred rows as finite
values; it is not a Rust type correspondence.

`ExtensionFrame` is shared provenance only. MembershipAuth and CapabilityAuth
propose non-transparent ContractUpdates; finite_refinement returns Evidence,
Diagnostic, or Residual only. In the fixed revocation trace, membership
verification is a Diagnostic, finite refinement/M9 resolution are Residual,
and activation leaves the revoked frame unchanged. Observer output is described
as typed/redacted provenance, not a raw credential export.

OBL-026 records only exact SourceRef/identity/deferred-row resolution,
M8-only deferral, verifier non-authority, and two transparent-overlay
`ContractRef` equalities. OBL-028 has separate accepted bounded Rust evidence
for a fixed one-subject/one-capability graph to bound 4; this Lean fixture
remains only its fixed two-frame trace and its theorems are unchanged. Neither
carrier is a general M9 theorem,
action-sequence enumeration theorem, authorization-composition theorem,
M10/SCN claim, transport, public API/ABI/wire, or final grammar claim.

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M9AuthVerification.lean
```

The file declares no user axiom and uses no `sorry` or `admit`.
