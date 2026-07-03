# ELAB-12 — SCN-02 two-read dependency positive

This sample is LAB evidence for the G1 static consequence gap identified in
`plan/72`: the SCN-02-shaped attack assignment records both RHS reads,
`player[target].hp` and `player[self].atk`, as dependency rows.

It is not a C-static conformance pass, runtime request-serving trace, proof
discharge, final read-materialization policy, or final Core IR JSON contract.
