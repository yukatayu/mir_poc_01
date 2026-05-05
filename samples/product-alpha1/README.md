# samples/product-alpha1

This root is reserved for the Product/Public-ready Mirrorea Spaces alpha-1 line.

Current status:

- `demo/` is the product alpha-1 schema / CLI fixture root introduced in `P-A1-26`.
- The root is not yet workflow-ready.
- It must stay separate from `samples/practical-alpha1/`, which remains first-floor / bounded workflow evidence.
- It must stay separate from `samples/alpha/`, which remains alpha-0 evidence.

Current validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
```

Stop lines:

- This root does not define final textual `.mir` grammar.
- This root does not claim same-session product runtime readiness.
- This root does not claim product viewer, native bundle, WAN/federation, distributed durable save/load, or arbitrary native package execution.
