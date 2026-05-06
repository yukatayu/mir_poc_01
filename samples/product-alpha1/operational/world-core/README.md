# WorldCore

`WorldCore` is the minimal server-side world base for the operational suite.

- current executable input: `package.mir.json`
- representative source: `world-core.mir`
- current scope: world identity, membership frontier, event DAG policy, observer-safe observation policy, typed host boundary placeholder

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/world-core --format json
```
