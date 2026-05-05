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

`P-A1-29` uses it for:

- same-session `transport --mode local` loopback TCP
- controlled Docker Compose TCP `transport --mode docker` using `samples/product-alpha1/docker/`
- non-final `export-devtools` JSON/HTML bundle with observer-safe redaction
- `view --check` static viewer openability and panel presence validation

`P-A1-30` uses it for:

- `build-native-bundle` native host launch bundle generation
- bundled compiled CLI, versioned package files, observer-safe devtools assets, manifest, launch metadata, run script, verification report, and provenance report
- explicit `NativeExecutionPolicy = Disabled`, package-native execution non-claim, signature-is-safety non-claim, and direct Mir-to-machine-code non-goal diagnostics

`P-A1-31` uses it for:

- `demo` release-candidate workflow output
- source-backed debug/auth/rate-limit layer attach plus deferred object/avatar-preview attach boundary evidence
- local and Docker Compose TCP transport in the same product command family
- clean-clone release check through `scripts/product_alpha1_release_check.py`
- observer-safe `sessions/` output plus admin/debug `session-store/` carrier replay output

It is now the product alpha release-candidate demo root. It is not final public product readiness. R3/R4 durable distributed save/load remains a non-goal.

Commands:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/auth-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/rate-limit-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/placeholder-object --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/custom-avatar-preview --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json
viewer_dir=$(mktemp -d /tmp/mirrorea-alpha1-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
bundle_dir=$(mktemp -d /tmp/mirrorea-alpha1-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
demo_dir=$(mktemp -d /tmp/mirrorea-alpha1-demo-XXXXXX)
cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out "$demo_dir" --format json
release_dir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$release_dir"
```

The Docker transport command requires local Docker and Docker Compose. If those
tools are unavailable, record an environment-gated skip with explicit non-claims
instead of claiming the Docker probe passed. `--skip-docker` is a partial local
probe and does not set release-candidate readiness.

Non-claim:

- no R3/R4 durable distributed save/load
- no exactly-once transport
- no WAN partition recovery
- no final public viewer / telemetry ABI
- no arbitrary native package execution
- no signature-is-safety claim
- no direct Mir-to-machine-code
