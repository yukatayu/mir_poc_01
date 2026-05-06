# 15 — next packages after P-OPS-01

## P-OPS-02 — package dependency and import resolver hardening

Goal:

- Make `WorldCore -> MembershipChat -> SugorokuWorld` dependency chain executable rather than manifest-only if P-OPS-01 could not fully implement it.

## P-OPS-03 — operational chat / EchoText direct host boundary

Goal:

- Add text host-I/O lane such as `EchoText` or `ChatText`.
- Demonstrate `Taro -> Hello, Taro!` or room chat message.

## P-OPS-04 — Sugoroku product runtime behavior widening

Goal:

- Move more Sugoroku behavior into product alpha package/session path.
- Add roll/publish/witness/handoff in product alpha operational sample if P-OPS-01 only declares it.

## P-OPS-05 — operational projection manifest and packet schema

Goal:

- Formalize server/client projection IR and packet/FFI boundary schema.
- Still no LLVM codegen.

## P-OPS-06 — portal/world-link first cut

Goal:

- Implement first discrete Portal / WorldLink sample.
- Treat as WWW hyperlink equivalent.

## P-OPS-07 — two-shard hard-boundary model-check sample

Goal:

- finite two-shard handoff model-check with no-double-owner property.

## P-OPS-08 — backend feasibility inventory

Goal:

- Audit options for LLVM/native backend vs WASM vs host bundle.
- Do not implement backend without new decision.

## P-OPS-09 — developer package authoring guide

Goal:

- Explain how external developer creates a new operational package from scratch.
- Include schema examples and diagnostics.

## Final-public gate

Separate from OPS packages:

- final textual grammar
- final ABI
- final SDK/API
- production federation
- distributed durable save/load
- final viewer service
