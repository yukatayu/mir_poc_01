# TwoShardHardBoundary

`TwoShardHardBoundary` imports `PortalWorldLink` and promotes the first bounded shard runtime cut in the operational suite.

- current executable input: `package.mir.json`
- representative source: `two-shard-hard-boundary.mir`
- current runtime evidence: bounded same-session two-shard hard authority handoff only
- current shard cut: offer -> prepare -> commit -> old-owner reject -> missing-witness reject -> stale-config reject
- current non-claims: no gradient observation runtime, no WAN federation, no continuous infinite shard federation, no distributed durable save/load
- the `future/two-shard-hard-boundary/` directory remains the planned blueprint and is not promoted away

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-shard-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-hard-boundary --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-hard-boundary' --out "$(mktemp -d /tmp/mirrorea-ops-shard-viewer-XXXXXX)" --format json
```
