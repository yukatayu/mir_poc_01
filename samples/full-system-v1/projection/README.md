# samples/full-system-v1/projection

This root is the source-first Full System V1 projection IR plus boundary-schema line for `P-PROJ-03`.

## Current Status

- `proj-03-effectful-sugoroku-positive` actualizes `source + typed IR + projection.request.json -> projection IR + target manifests + packet schemas + FFI schemas + preservation report`.
- `proj-03-client-write-authority-negative` proves that a client-owned world-write boundary is rejected before any target manifest or boundary schema is emitted.
- `proj-03-effect-contract-mismatch-negative` proves that one boundary cannot silently collapse same-shape effects with different capability/failure contracts into a single packet schema.
- `proj-03-payload-shape-mismatch-negative` proves that one boundary cannot silently collapse incompatible effect payload shapes into a single packet schema.
- Current rows now preserve payload shape plus effect/failure/capability/authority/provider-policy/rollback boundaries and reject same-shape heterogeneous effect contracts. Transport/runtime split remains later.

## Current Validation Anchor

```bash
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 -m unittest scripts.tests.test_projection_v1_samples
python3 scripts/projection_v1_samples.py matrix --format json
python3 scripts/projection_v1_samples.py check-all --format json
```

## Non-claims

- no executable server/client split runtime yet
- no packet or FFI payload schema semantics completion yet
- no LLVM/backend code generation
- no provider admission completion
