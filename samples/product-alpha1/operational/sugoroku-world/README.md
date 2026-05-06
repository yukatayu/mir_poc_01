# SugorokuWorld

`SugorokuWorld` imports `MembershipChat` and is the runnable root for the first operational workflow.

- current executable input: `package.mir.json`
- representative source: `sugoroku-world.mir`
- current direct host-I/O lane: typed `AddOne` adapter evidence only
- current bounded runtime evidence: same-session roll / publish / witness / handoff / stale membership reject scenario over the product alpha session carrier
- save/load, transport, devtools export, and native host launch bundle are exercised from this root

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json
```
