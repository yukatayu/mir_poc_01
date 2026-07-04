# ELAB-17 - SCN-01 visibility failure-row negative

This sample is LAB evidence for the SCN-01 static negative gap identified in
`plan/122`: removing `VisibilityDenied` from the SCN-01-shaped visible write
failure row rejects as an `E-ROW-002` / `VisibilityDenied` underdeclaration.

It is not a C-static conformance pass, runtime request-serving trace, proof
discharge, final diagnostic / repair ABI, or final Core IR JSON contract.
