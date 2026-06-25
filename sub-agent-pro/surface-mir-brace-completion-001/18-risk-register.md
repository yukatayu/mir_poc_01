# 18 — Risk Register

## R1. Syntax ambiguity

Risk: `S { ... }` conflicts with record literal.
Mitigation: namespace separation + context distinction + diagnostics.

## R2. Communication becomes hidden magic

Risk: user cannot tell what messages are generated.
Mitigation: Core IR export + devtools source mapping.

## R3. Auto publish leaks private data

Risk: hidden data exposure.
Mitigation: visible fields only + redaction policy + failure row.

## R4. Indexed key becomes authority

Risk: participant writes server state because key is self.
Mitigation: key != authority; write requires owner locus or capability.

## R5. Role spoofing

Risk: client claims Server role.
Mitigation: claim is not capability; admission grant required.

## R6. Patch direct eval

Risk: arbitrary source mutates runtime without checks.
Mitigation: parse/typecheck/elaborate/admit/activation_cut.

## R7. Backend semantic drift

Risk: Unity/WASM/native owns world logic.
Mitigation: provider boundary must declare effects/failures/capabilities; Mir remains semantic owner.

## R8. Overclaiming production readiness

Risk: bounded alpha described as final product.
Mitigation: progress/tasks non-claims and release-check scope.
