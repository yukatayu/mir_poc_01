# PortalWorldLink

`PortalWorldLink` imports `SugorokuWorld` and is the first active portal/world-link root in the operational suite.

- current executable input: `package.mir.json`
- representative source: `portal-worldlink.mir`
- current runtime evidence: bounded same-session discrete handoff only
- current portal cut: resolve -> handoff offer -> witness emit -> destination admit
- current non-claims: no WAN federation, no continuous spatial sync, no final portal ABI
- the `future/portal-worldlink/` directory remains the planned blueprint and is not promoted away

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-portal-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/portal-worldlink --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-portal-worldlink' --out "$(mktemp -d /tmp/mirrorea-ops-portal-viewer-XXXXXX)" --format json
```
