# SugorokuWorld Starter

`SugorokuWorld Starter` is a validated authoring starter for external developers who need a bounded operational `sugoroku_world` package with the current `AddOne` host-I/O lane, witness requirement, and turn handoff rows already declared.

- current status: `template_only`
- executable input: `package.mir.json`
- representative source: `sugoroku-world-starter.mir`
- validated surfaces: `check`, `run-local`
- validated dependency anchor: `../membership-chat-starter`
- not an active operational sample root; copy and rename its identifiers and retarget `dependencies` before treating it as your own package
- bounded authoring guide: `docs/hands_on/operational_package_authoring_01.md`

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/sugoroku-world-starter --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/sugoroku-world-starter --format json
```
