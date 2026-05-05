# samples/product-alpha1

This root is reserved for the Product/Public-ready Mirrorea Spaces alpha-1 line.

Current status:

- `demo/` is the product alpha-1 schema / CLI fixture root introduced in `P-A1-26`.
- `demo/` also has the `P-A1-27` local same-session runtime first cut through `mirrorea-alpha run-local`, `session`, and `attach`.
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
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
```

Stop lines:

- This root does not define final textual `.mir` grammar.
- This root currently claims only local same-session runtime first-cut behavior, not product alpha release readiness.
- This root does not claim local/Docker transport command completion, quiescent-save, product viewer, native bundle, WAN/federation, distributed durable save/load, or arbitrary native package execution.
