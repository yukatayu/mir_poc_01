# WorldCore Starter

`WorldCore Starter` is a validated authoring starter for external developers who need the smallest operational `world_core` package that still passes the current product alpha front door.

- current status: `template_only`
- executable input: `package.mir.json`
- representative source: `world-core-starter.mir`
- validated surfaces: `check`, `run-local`
- not an active operational sample root; copy and rename its identifiers before treating it as your own package
- bounded authoring guide: `docs/hands_on/operational_package_authoring_01.md`

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/world-core-starter --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/world-core-starter --format json
```
