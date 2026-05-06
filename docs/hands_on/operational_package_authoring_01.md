# Operational Package Authoring 01

この guide は、external developer が current product alpha operational package を **from scratch に近い最小手順**で author / check / run-local / export-devtools / release-check boundary まで理解するための入口です。

これは final public grammar / ABI guide ではありません。current executable input は `package.mir.json` であり、representative `.mir` は explanatory source に留まります。

## What To Start From

Use the validated template root:

```text
samples/product-alpha1/operational/templates/world-core-starter/
```

This root is:

- `template_only`
- valid for `check` and `run-local`
- not counted as an active operational sample root

## Validate The Starter As-Is

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/world-core-starter --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/world-core-starter --format json
```

Expected bounded evidence:

- `verdict = accepted`
- `package_kind = world_core`
- `product_alpha1_ready = false`
- `run-local` emits a same-session carrier rather than a final public runtime ABI

## Create Your Own Working Copy

```bash
work_root=$(mktemp -d /tmp/mirrorea-ops-authoring-root-XXXXXX)
cp -R samples/product-alpha1/operational/templates/world-core-starter "$work_root/my-world-core"
```

Before treating the copy as your own package, rename at least:

- `package_id`
- `auth_policy.policy_id`
- each `contracts[].contract_id`
- observation labels
- retention scope

For the concrete commands below, assume you changed:

```json
"package_id": "my-world-core"
```

Keep these invariants:

- `schema_version = mirrorea-product-alpha1-v0`
- `package_kind` must stay within the current accepted set
- `native_policy.execution_policy` must stay `disabled`
- `message_recovery_policy.recovery` must use the current accepted values
- direct textual `.mir` is not the executable front door

## Check Your Edited Package

```bash
cargo run -q -p mirrorea-cli -- check "$work_root/my-world-core" --format json
```

Read the payload in two parts:

- `accepted_obligations`
  what the static checker accepted now
- `residual_obligations`
  what still needs runtime, devtools, bundle, or release evidence

## Run A World-Like Package

For executable world-like package kinds:

- `world_core`
- `membership_chat`
- `sugoroku_world`
- `portal_worldlink`
- `two_shard_hard_boundary`

you can use the same local-session flow:

```bash
session_dir=$(mktemp -d /tmp/mirrorea-ops-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local "$work_root/my-world-core" --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- session 'session#my-world-core' --format json
viewer_dir=$(mktemp -d /tmp/mirrorea-ops-authoring-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#my-world-core' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
```

For layer/object/avatar-preview package kinds, use attach flows rather than `run-local`.

## Dependency Rules

If your package imports another operational package:

- point `dependencies` to sibling package roots, not to `.mir` files
- keep dependency roots executable through `package.mir.json`
- do not silently promote blueprint-only roots into runnable dependencies

Current canonical chain is:

```text
WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary
```

## Common Diagnostics

Typical mistakes and their current diagnostics:

- Passing a `.mir` file to the product front door:
  current diagnostic explains that product alpha-1 only accepts `package.mir.json`
- Using an unsupported `package_kind`:
  current diagnostic reports `unsupported package_kind`
- Using an unsupported recovery policy such as `explicit_reject_only`:
  current diagnostic reports `unsupported recovery policy`
- Referencing a missing dependency root:
  current diagnostic reports missing/invalid dependency package files

Treat these as alpha diagnostics, not final public API guarantees.

## Release-Check Boundary

`scripts/operational_product_samples.py` remains the suite helper for the promoted operational roots. It is not a generic authoring scaffold command.

For a new package, the practical order is:

1. `check`
2. `run-local` if the package kind is world-like
3. `session`
4. `export-devtools` / `view --check`
5. add the package to a bounded suite helper or package-specific validation script only after the standalone root is stable

If you need release-style closeout, do it in two stages:

1. stabilize the standalone root with direct `mirrorea-cli` commands
2. only then extend a bounded helper such as `scripts/operational_product_samples.py` or a package-specific closeout script with explicit semantic checks

## Non-Claims

- final textual `.mir` grammar
- final public ABI / SDK
- final server/client binary split
- direct LLVM backend
- WAN / federation
- distributed durable save/load
- arbitrary native package execution
- final public viewer / telemetry ABI
