# alpha sample family — Local Runtime

- Status: first Rust local-runtime floor
- Phase: Phase 3
- Stage: Stage B
- Current runner is a non-public in-memory Rust runtime floor in `mir-runtime`.
- This family is still not an active parser/runtime sample root.

## Current reading

- `LR-01` actualizes one narrow local room/game dispatch path in Rust:
  queue -> `MessageEnvelope` dispatch -> membership freshness check -> event DAG export hook.
- `LR-02` actualizes one negative local-runtime row:
  stale membership frontier is rejected before state mutation.
- The source-ish `.mir` files here are anchors for sample identity and intended scenario only.
  The current runner does not parse them yet.
- This package does not claim:
  hot-plug lifecycle completion, layer insertion runtime, runtime package/avatar admission, network/Docker runtime, save/load completion, or final public ABI.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `LR-01` | `lr-01-local_sugoroku_roll_publish_handoff.mir` | positive | accepted + event DAG export |
| `LR-02` | `lr-02-stale_membership_rejected.mir` | negative | rejected before state mutation |

## Policy

- `.mir` files here are source-ish anchors, not currently parsed executable sources.
- `.expected.json` sidecars record the runtime-floor contract checked by Rust tests.
- `claims.runnable = true` means a dedicated Rust test/example now exists for the row, not that `samples/alpha/` became an active front-door sample root.
- Promotion to active/runnable root status still requires dedicated runner docs, broader closeout evidence, and snapshot updates.

## Validation anchor for this package

```bash
cargo test -p mir-runtime --test alpha_local_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership
```
