# TwoShardGradientObservation

`TwoShardGradientObservation` imports `TwoShardHardBoundary` and actualizes the first bounded observer-only gradient widening in the operational suite.

- current executable input: `package.mir.json`
- representative source: `two-shard-gradient-observation.mir`
- current runtime evidence: bounded same-session observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject
- current shard cut: overlap observe -> observer projection -> write reject -> stale-view drop -> missing-freshness reject
- current non-claims: no write authority in gradient zones, no continuous spatial synchronization runtime, no WAN federation, no distributed durable save/load
- `future/gradient-observation.profile.json` remains non-executable inventory even though this bounded runtime root now exists

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-gradient-observation --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-gradient-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-gradient-observation --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-gradient-observation' --out "$(mktemp -d /tmp/mirrorea-ops-gradient-viewer-XXXXXX)" --format json
```
