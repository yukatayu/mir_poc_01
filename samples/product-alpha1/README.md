# samples/product-alpha1

This root is reserved for the Product/Public-ready Mirrorea Spaces alpha-1 line.

Current status:

- `demo/` is the product alpha-1 schema / CLI fixture root introduced in `P-A1-26`.
- `demo/` also has the `P-A1-27` local same-session runtime first cut through `mirrorea-alpha run-local`, `session`, and `attach`.
- `demo/` has the `P-A1-28` bounded message recovery and local save first cut through `mirrorea-alpha save`, `load`, and `quiescent-save`.
- `demo/` and `docker/` have the `P-A1-29` local/Docker transport and non-final viewer first cut through `mirrorea-alpha transport`, `export-devtools`, and `view`.
- `demo/` has the `P-A1-30` native host launch bundle first cut through `mirrorea-alpha build-native-bundle`.
- The root is not yet product/release-ready.
- It must stay separate from `samples/practical-alpha1/`, which remains first-floor / bounded workflow evidence.
- It must stay separate from `samples/alpha/`, which remains alpha-0 evidence.

Current validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
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
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
```

The Docker transport command requires local Docker and Docker Compose. If those
tools are unavailable, a closeout must record an environment-gated skip with the
same non-claims rather than treating the Docker path as passed.

Stop lines:

- This root does not define final textual `.mir` grammar.
- This root currently claims local same-session runtime/save/transport/viewer/native-bundle first-cut behavior, not product alpha release readiness.
- This root does not claim CLI `demo` completion, WAN/federation, distributed durable save/load R3/R4, final public viewer / telemetry ABI, direct Mir-to-machine-code, signature-is-safety, or arbitrary native package execution.
