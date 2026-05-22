# samples/full-system-v1/server-client

This root is the Full System V1 `FS-07` local server/client role-run lane.

## Current Status

- `proj-04-local-role-split-positive` proves that one accepted `projection.request.json` can launch server and client target entries from the same runtime binary while leaving the adapter target passive.
- `proj-04-client-entry-override-negative` proves that a valid client target cannot execute an undeclared server-owned entry transition via runtime override.
- This root depends on the `FS-06` projection IR + boundary-schema floor but does not claim final server/client code generation, real packet/FFI transport semantics, or provider execution.

## Current Validation Anchor

```bash
python3 scripts/projection_v1_samples.py matrix --format json
python3 scripts/projection_v1_samples.py check-all --format json
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
```

## Non-claims

- no final packet or FFI transport semantics completion here
- no generated server/client binaries or distributed planner here
- no provider admission completion here
