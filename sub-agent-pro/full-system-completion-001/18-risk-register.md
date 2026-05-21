# 18 — Risk Register

## R1: Adapter drift

Risk:

- computation hides in adapter again.

Mitigation:

- every computation sample must say whether transform is Mir-owned or adapter-owned.
- host boundaries only read/write/provide external primitive data.

## R2: Manifest replaces language

Risk:

- `package.mir.json` remains forever and Mir source never becomes real.

Mitigation:

- textual Mir alpha grammar milestones are mandatory.
- package JSON may be generated artifact or alpha compatibility layer.

## R3: Backend hides semantics

Risk:

- Unity/WASM/native plugin owns world logic.

Mitigation:

- provider manifests declare effects/failures/capabilities.
- world authority remains Mir-owned.
- runtime rejects over-capability providers.

## R4: Projection mismatch

Risk:

- server/client split changes semantics.

Mitigation:

- projection correctness report.
- packet/FFI boundary schema.
- model-check small split examples.

## R5: Save/load overclaim

Risk:

- local save marketed as distributed durable recovery.

Mitigation:

- savepoint class R0/R1/R2/R3/R4 visible in all reports.

## R6: Debug leak

Risk:

- observer-safe view leaks raw witness/auth/capability info.

Mitigation:

- redaction tests.
- separate admin/debug and observer-safe artifacts.

## R7: Continuation unsoundness

Risk:

- multi-shot continuation duplicates stateful resources.

Mitigation:

- do not implement first-class continuations until capture/replay-safety spec.

## R8: Vector-clock membership explosion

Risk:

- participant vector clock becomes unbounded.

Mitigation:

- epoch/incarnation membership.
- tombstone/retention frontier.
- optional causal profiles only for replicated object state.

## R9: Premature LLVM

Risk:

- backend freezes wrong semantics.

Mitigation:

- implement typed IR/interpreter/projection first.
- LLVM only after boundary preservation tests.
