# 12 — Risk Register

## R1: Progress overclaim

Risk: Current-scope 100% is mistaken for practical 100%.
Mitigation: introduce separate metrics and rename tables.

## R2: Parser front-door skipped

Risk: sample-ID keyed runners continue indefinitely.
Mitigation: practical alpha-1 requires source-to-runtime path.

## R3: Synthetic evidence mistaken for implementation

Risk: sidecar / helper rows treated as checker/runtime implementation.
Mitigation: keep evidence rows separate and use explicit non-claim text.

## R4: Native signature mistaken for safety

Risk: signed native package is treated as safe.
Mitigation: require sandbox/capability/resource/audit policy; native execution later.

## R5: Local save/load mistaken for distributed durability

Risk: local savepoint is described as durable distributed save/load.
Mitigation: keep distributed consistent cut as explicit later boundary.

## R6: Transport lanes collapse

Risk: socket protocol swallows auth/membership/capability/witness.
Mitigation: lane-separated MessageEnvelope schema and tests.

## R7: Avatar compatibility hard-coded

Risk: VRM/VRChat/Unity becomes core semantics.
Mitigation: runtime package / adapter model.

## R8: Devtools leak

Risk: debug traces leak raw witness/auth/private data.
Mitigation: label/redaction/retention rules and negative tests.

## R9: Repo restructure breaks active floor

Risk: moving files breaks existing active sample suite.
Mitigation: add wrappers, do not remove active paths, update taxonomy docs.

## R10: User decision fixed autonomously

Risk: Codex picks packaging/host/public surface without user.
Mitigation: keep U1 separate.
