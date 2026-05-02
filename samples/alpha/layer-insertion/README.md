# alpha sample family — Layer Insertion

- Status: first Rust layer-insertion runtime floor
- Phase: Phase 4
- Stage: Stage D
- Current runner is a non-public Rust attach-time layer-insertion floor in `mir-runtime`.
- This family is still not an active parser/runtime sample root.

## Current reading

- `LI-01..05` actualize the smallest honest attach-time cut for `P-A0-08`:
  one `MessageDispatch` attach point over the `P-A0-07` local-runtime floor, one accepted debug attach, one rejected non-admin debug attach, one explicit auth contract-update path, one declared-failure rate-limit path, and one incompatible patch reject.
- The source-ish `.mir` files here are anchors for sample identity and intended scenario only.
  The current runner does not parse them yet.
- `HP-02..06` and `VAR-12` remain planned/sample-mirror rows.
- `VAR-08/11/13` are now actualized in `../contract-variance/` only as runtime-mirror rows.
  Runtime-sensitive closeout authority for those rows still lives in this directory through
  `LI-04/01/03`.
- This package does not claim:
  completed hot-plug lifecycle, detach runtime, rollback, durable migration, distributed activation ordering, parser integration, runtime package/avatar admission, network/Docker runtime, save/load completion, or final public layer-attachment ABI.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `LI-01` | `li-01-debug_layer_attach_authorized.mir` | positive | accepted + redacted trace after attach |
| `LI-02` | `li-02-debug_layer_non_admin_rejected.mir` | negative | rejected before activation |
| `LI-03` | `li-03-auth_layer_contract_update_path.mir` | positive | accepted only via explicit contract update |
| `LI-04` | `li-04-ratelimit_declared_failure.mir` | positive/runtime-preview | accepted attach + preview `Reject(RateLimited)` |
| `LI-05` | `li-05-incompatible_patch_rejected.mir` | negative | rejected before activation |

## Policy

- `.mir` files here are source-ish anchors, not currently parsed executable sources.
- `.expected.json` sidecars record the runtime-floor contract checked by Rust tests.
- `claims.runnable = true` means a dedicated Rust test/example now exists for the row, not that `samples/alpha/` became an active front-door sample root.
- `mirrors` lists identify planned/sample-mirror targets that may reuse this runtime-floor evidence through helper-local mirror checks. They do not make those target families runnable roots.
- Promotion to active/runnable root status still requires dedicated runner docs, broader closeout evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/layer-insertion -maxdepth 1 -type f | sort
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout
```
