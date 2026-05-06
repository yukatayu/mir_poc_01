# 14 — risk register

## R1. Treating representative `.mir` as executable final source

Risk:

- Developer sees `.mir` files and assumes final textual grammar exists.

Mitigation:

- Mark every `.mir` file representative-only.
- Keep `package.mir.json` as executable alpha input.
- Product CLI should continue returning `direct_mir_non_goal` for direct `.mir` input.

## R2. Overclaiming native output

Risk:

- Native host bundle is described as LLVM codegen.

Mitigation:

- Use phrase `native host launch bundle`.
- Keep manifest non-claims.
- Document future backend boundary.

## R3. Import chain overclaim

Risk:

- package dependency metadata is treated as true source import execution.

Mitigation:

- If dependency resolver is not implemented, mark manifest-only.
- Add diagnostics / expected reports.

## R4. Product sample root confused with final product SDK

Risk:

- `samples/product-alpha1/operational/` is read as final SDK.

Mitigation:

- Say product alpha operational sample, not final SDK.
- Keep final grammar/API/ABI non-claim.

## R5. Portal/spatial future overclaim

Risk:

- Future skeletons are described as implemented federation.

Mitigation:

- Place in `future/` with planned-only status.
- Add explicit no WAN/federation claim.

## R6. Docker skip overclaim

Risk:

- Docker skipped but release readiness claimed.

Mitigation:

- Docker skip is partial local probe.
- Record environment skip.
- Do not claim Docker pass.

## R7. Hidden auth authority

Risk:

- Layer attach succeeds due to implicit bootstrap, not source/package authority.

Mitigation:

- Source packages must declare admin/membership/capability authority.
- Devtools must show auth/capability decision.

## R8. Debug leak

Risk:

- Observer-safe view exposes raw witness/auth/capability secrets.

Mitigation:

- Viewer/export must state role, redaction level, retention scope.
- Tests should search for forbidden raw fields if possible.

## R9. Save/load overclaim

Risk:

- R0/R2 local save described as distributed durable save.

Mitigation:

- Use savepoint classes.
- R3/R4 remain non-goals.

## R10. Projection/backend premature freeze

Risk:

- Server/client split profile freezes final deployment model too early.

Mitigation:

- Projection profile is intent / future boundary.
- Do not claim placement optimizer or emitted binaries.
