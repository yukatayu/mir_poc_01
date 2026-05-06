# MembershipChat Starter

`MembershipChat Starter` is a validated authoring starter for external developers who need a bounded operational `membership_chat` package with the current `ChatText` room-oriented host-boundary lane already declared.

- current status: `template_only`
- executable input: `package.mir.json`
- representative source: `membership-chat-starter.mir`
- validated surfaces: `check`, `run-local`
- validated dependency anchor: `../world-core-starter`
- not an active operational sample root; copy and rename its identifiers and retarget `dependencies` before treating it as your own package
- bounded authoring guide: `docs/hands_on/operational_package_authoring_01.md`

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/membership-chat-starter --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/membership-chat-starter --format json
```
