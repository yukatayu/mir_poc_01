# Operational Portal/Shard Starter Boundary 01

この guide は、current operational suite で portal/shard package authoring を **どこから始めるべきか** を固定するための入口です。

これは final public scaffold policy ではありません。current executable input は引き続き `package.mir.json` であり、`future/` 配下の blueprint / profile inventory は executable root ではありません。

## Current Decision

- validated `template_only` starter catalog は `world-core-starter/`、`membership-chat-starter/`、`sugoroku-world-starter/` で一度止める
- `templates/portal-worldlink-starter/` と `templates/two-shard-hard-boundary-starter/` は current line では追加しない
- portal/shard authoring を今すぐ試す場合は、`future/` blueprint ではなく active executable roots を study/copy boundary として使う

Current active roots:

```text
samples/product-alpha1/operational/portal-worldlink/
samples/product-alpha1/operational/two-shard-hard-boundary/
samples/product-alpha1/operational/two-shard-gradient-observation/
```

Current non-executable future inventory:

```text
samples/product-alpha1/operational/future/portal-worldlink/
samples/product-alpha1/operational/future/two-shard-hard-boundary/
samples/product-alpha1/operational/future/gradient-observation.profile.json
```

## Why The Starter Catalog Stops Here

- `WorldCore -> MembershipChat -> SugorokuWorld` は mainstream world/chat/game chain として bounded authoring starter に向いている
- portal/shard line ではすでに
  - active executable roots
  - retained `future/` blueprints
  - non-executable gradient profile inventory
  の 3 層があるため、ここで duplicate starter roots を加えると category boundary が崩れやすい
- current authoring guide は mainstream chain を確実に再利用できることを先に優先し、portal/shard widening は active runtime root を直接読む段階に留める

## If You Need A Working Copy Today

Use the active roots directly:

```bash
work_root=$(mktemp -d /tmp/mirrorea-ops-portal-authoring-XXXXXX)
cp -R samples/product-alpha1/operational/portal-worldlink "$work_root/my-portal"
cp -R samples/product-alpha1/operational/two-shard-hard-boundary "$work_root/my-shard"
cp -R samples/product-alpha1/operational/two-shard-gradient-observation "$work_root/my-gradient-shard"
```

Before treating the copy as your own package, rename at least:

- `package_id`
- `auth_policy.policy_id`
- `contracts[].contract_id`
- observation labels
- retention scope

If the copied root has `dependencies`, retarget them to your sibling working copies before the first `check`.

Current chain:

```text
SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation
```

Do not leave copied portal/shard roots depending on repo-local originals unless that is explicitly your intent.

## Validate The Active Roots Before Copying

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-gradient-observation --format json
portal_session_dir=$(mktemp -d /tmp/mirrorea-ops-portal-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$portal_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/portal-worldlink --format json
shard_session_dir=$(mktemp -d /tmp/mirrorea-ops-shard-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$shard_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-hard-boundary --format json
gradient_session_dir=$(mktemp -d /tmp/mirrorea-ops-gradient-authoring-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$gradient_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-gradient-observation --format json
```

Expected bounded evidence:

- `portal-worldlink` remains the discrete same-session handoff root
- `two-shard-hard-boundary` remains the hard-authority same-session shard root
- `two-shard-gradient-observation` remains the observer-only same-session gradient root
- neither root is a starter duplicate of a `future/` blueprint

If you need observer-safe inspection after copying, reuse the same `session -> export-devtools -> view --check` order documented in `operational_package_authoring_01.md`.

## What Not To Copy

Do not use these as executable authoring roots:

- `samples/product-alpha1/operational/future/portal-worldlink/`
- `samples/product-alpha1/operational/future/two-shard-hard-boundary/`
- `samples/product-alpha1/operational/future/gradient-observation.profile.json`

Those files are inventory / blueprint evidence only.

## Reopen Condition For Future Starters

A later `portal-worldlink-starter/` or `two-shard-hard-boundary-starter/` is allowed only if all of the following are true:

- it is sourced from the active executable root, not from a `future/` blueprint
- it stays explicitly `template_only`
- active runtime roots remain present and are not silently replaced
- focused `check` / `run-local` evidence is added for the starter itself
- docs keep portal/shard runtime evidence, future inventory, and starter catalog as three separate categories

## Non-Claims

- no portal/shard starter catalog exists today
- no `future/` root becomes executable through this guide
- no portal/shard starter catalog exists for the gradient root either
- no WAN federation / continuous spatial sync / continuous infinite shard federation
- no final public scaffold CLI or final public authoring policy
