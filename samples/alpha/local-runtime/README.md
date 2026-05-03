# alpha sample family — Local Runtime

- Status: current-scope Stage B closeout over a non-public Rust local-runtime floor
- Phase: Phase 3
- Stage: Stage B
- Current runner is a non-public in-memory Rust runtime floor in `mir-runtime`, surfaced through `scripts/alpha_local_runtime_samples.py`.
- This family is still not an active parser/runtime sample root.

## Current reading

- `LR-01` actualizes one narrow local room/game dispatch path in Rust:
  queue -> `MessageEnvelope` dispatch -> membership freshness check -> event DAG export hook.
- `LR-02` actualizes one negative local-runtime row:
  stale membership frontier is rejected before state mutation.
- `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json` additionally reuses
  `CUT-04/17` as the local-only save/load subset required for current-scope Stage B closeout.
  This does not make the whole `cut-save-load/` family complete.
- The source-ish `.mir` files here are anchors for sample identity and intended scenario only.
  The current runner does not parse them yet.
- This package does not claim:
  hot-plug lifecycle completion, layer insertion runtime, runtime package/avatar admission, network/Docker runtime, distributed save/load completion, or final public ABI.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `LR-01` | `lr-01-local_sugoroku_roll_publish_handoff.mir` | positive | accepted + event DAG export |
| `LR-02` | `lr-02-stale_membership_rejected.mir` | negative | rejected before state mutation |

## Policy

- `.mir` files here are source-ish anchors, not currently parsed executable sources.
- `.expected.json` sidecars record the runtime-floor contract checked by Rust tests.
- the dedicated Python runner validates `LR-01/02` directly and exposes a Stage B closeout command that composes them with `CUT-04/17`.
- `claims.runnable = true` means a dedicated Rust test/example now exists for the row, not that `samples/alpha/` became an active front-door sample root.
- Promotion to active/runnable root status still requires dedicated runner docs, broader closeout evidence, and snapshot updates.

## Validation anchor for this package

```bash
cargo test -p mir-runtime --test alpha_local_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership
python3 scripts/alpha_local_runtime_samples.py check-all --format json
python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json
```
