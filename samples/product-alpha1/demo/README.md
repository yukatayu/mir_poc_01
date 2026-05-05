# samples/product-alpha1/demo

This directory is the initial Product Alpha-1 demo package root.

`P-A1-26` uses it for:

- versioned `package.mir.json` schema acceptance
- canonical `mirrorea-alpha check` / `mirrorea-cli` validation
- explicit residual obligations for runtime, quiescent-save, viewer, native bundle, and release validation

`P-A1-27` uses it for:

- local file-backed product session carrier
- `run-local` runtime plan and same-session state
- core fabric envelope validation
- typed host-I/O `AddOne` observation
- debug-layer `attach` lifecycle over the same session file

`P-A1-28` uses it for:

- bounded message recovery rows for timeout / retry-then-reject
- R0 local `save` / `load`
- R2 local `quiescent-save` with `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`
- observable rejected quiescent-save when an in-flight message remains

It is not yet the full product demo workflow. Local/Docker transport command behavior, devtools viewer, native launch bundle, and release-candidate guide are scheduled for later `P-A1-29..31` packages. R3/R4 durable distributed save/load remains a non-goal.

Commands:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json
```
