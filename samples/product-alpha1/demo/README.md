# samples/product-alpha1/demo

This directory is the initial Product Alpha-1 demo package root.

`P-A1-26` uses it for:

- versioned `package.mir.json` schema acceptance
- canonical `mirrorea-alpha check` / `mirrorea-cli` validation
- explicit residual obligations for runtime, quiescent-save, viewer, native bundle, and release validation

It is not yet the full product demo workflow. The same-session runtime, local/Docker transport, hot-plug package execution, quiescent save, devtools viewer, native launch bundle, and release-candidate guide are scheduled for `P-A1-27..31`.

Command:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
```
