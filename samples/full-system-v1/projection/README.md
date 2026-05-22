# samples/full-system-v1/projection

This root is the source-first Full System V1 projection IR line for `P-PROJ-02`.

## Current Status

- `proj-02-effectful-sugoroku-positive` actualizes `source + typed IR + projection.request.json -> projection IR + target manifests + preservation report`.
- `proj-02-client-write-authority-negative` proves that a client-owned world-write boundary is rejected before any target manifest is emitted.
- Packet and FFI payload semantics are still deferred to `P-PROJ-03`. Current rows preserve references and authority/capability/failure boundaries only.

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
