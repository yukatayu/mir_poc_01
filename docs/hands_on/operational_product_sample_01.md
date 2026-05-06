# Operational Product Sample 01

この guide は、clean clone から `P-OPS-01 operational product sample suite scaffold and first workflow` とその `P-OPS-03` / `P-OPS-04` / `P-OPS-05` / `P-OPS-06` / `P-OPS-07` widening を再現するための入口です。

これは final public product ではありません。portal は bounded same-session first cut、shard は bounded same-session two-shard hard-authority first cut まで actualize 済みです。gradient observation は `planned_only` profile inventory まで actualize 済みですが、general model-check completion、gradient observation runtime、WAN federation / continuous spatial sync / continuous infinite shard federation / final portal ABI ではありません。

## Validate The Repository

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Check The Five Roots

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json
```

Expected bounded evidence:

- `sugoroku-world` の `check` payload は `projection_inventory` summary を返し、`source_package = operational-sugoroku`
- same summary には `target_count = 2`、`packet_boundary_count = 2`、`ffi_boundary_count = 1` が含まれる
- packet / FFI inventory は schema-backed だが、final server/client binary split や direct LLVM backend claim ではない
- current backend comparison inventory is documented separately in `operational_backend_inventory_01.md`; current actualized native path remains the host launch bundle

## Run The MembershipChat Text Boundary

```bash
chat_session_dir=$(mktemp -d /tmp/mirrorea-ops-chat-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$chat_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/membership-chat --format json
MIRROREA_ALPHA_SESSION_DIR="$chat_session_dir" cargo run -q -p mirrorea-cli -- session 'session#operational-membership-chat' --format json
chat_viewer_dir=$(mktemp -d /tmp/mirrorea-ops-chat-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$chat_session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-membership-chat' --out "$chat_viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$chat_viewer_dir" --check --format json
```

Expected bounded evidence:

- `typed_host_io_claimed = true`
- observer-safe host-I/O lane includes `EchoText:Text("Taro")->Text("Hello, Taro!")`
- event DAG / devtools export show the same request/response without introducing stdio as a Mir core primitive

## Run The Portal First Cut

```bash
portal_session_dir=$(mktemp -d /tmp/mirrorea-ops-portal-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$portal_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/portal-worldlink --format json
portal_viewer_dir=$(mktemp -d /tmp/mirrorea-ops-portal-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$portal_session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-portal-worldlink' --out "$portal_viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$portal_viewer_dir" --check --format json
```

Expected bounded evidence:

- `typed_host_io_claimed = false`
- `run-local` / `export-devtools` payload には `portal_resolve_requested`, `portal_handoff_offered`, `portal_handoff_witness_emitted`, `portal_admission_requested`, `portal_admission_accepted` が入る
- route lanes は `same_session_portal_resolve`, `same_session_portal_handoff`, `same_session_portal_admit` を observer-safe に保持する
- devtools `portal_graph_future.current_status = bounded_discrete_handoff_runtime`
- これは discrete handoff first cut であり、continuous spatial sync や WAN federation completion は主張しない

## Run The Two-Shard Hard Boundary Cut

```bash
shard_session_dir=$(mktemp -d /tmp/mirrorea-ops-shard-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$shard_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-hard-boundary --format json
shard_viewer_dir=$(mktemp -d /tmp/mirrorea-ops-shard-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$shard_session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-hard-boundary' --out "$shard_viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$shard_viewer_dir" --check --format json
```

Expected bounded evidence:

- `typed_host_io_claimed = false`
- `run-local` / `export-devtools` payload には `shard_handoff_offer_published`, `shard_handoff_prepare_accepted`, `shard_handoff_commit_applied`, `shard_old_owner_write_rejected`, `shard_missing_handoff_witness_rejected`, `shard_stale_config_rejected` が入る
- route lanes は `same_session_shard_handoff_offer`, `same_session_shard_handoff_commit`, `same_session_shard_old_owner_reject`, `same_session_shard_missing_witness_reject`, `same_session_shard_stale_config_reject` を observer-safe に保持する
- rejected message rows は `OldOwnerWriteRejected`, `MissingHandoffWitness`, `StaleShardConfig` を明示する
- devtools `shard_map_future.current_status = bounded_two_shard_runtime`
- これは bounded same-session hard-authority cut であり、general model-check completion、gradient observation、WAN federation は主張しない

## Run The First Operational Workflow

```bash
session_dir=$(mktemp -d /tmp/mirrorea-ops-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- session 'session#operational-sugoroku' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/auth-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/rate-limit-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/placeholder-object --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/custom-avatar-preview --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r0' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r2' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode docker --format json
viewer_dir=$(mktemp -d /tmp/mirrorea-ops-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-sugoroku' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
bundle_dir=$(mktemp -d /tmp/mirrorea-ops-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/operational/sugoroku-world --out "$bundle_dir" --format json
```

If Docker / Docker Compose are unavailable, record the Docker leg as an environment-gated skip. Do not rewrite that skip as passed release evidence.

Current boundedness:

- `run-local` Sugoroku root は bounded same-session roll / publish / witness / handoff / stale membership reject scenario を 1 本だけ materialize する。final interactive game runtime ではない
- `run-local` / `session` / `export-devtools` payload には同じ `projection_inventory` summary が入り、observer-safe projection panel から `roll_request_packet` / `chat_message_packet` / `host_io_adapter` を確認できる
- `export-devtools` / `view --check` では `sugoroku_roll_requested` / `sugoroku_roll_published` / `sugoroku_witness_emitted` / `sugoroku_turn_handoff` / `sugoroku_stale_membership_rejected` と corresponding route lanes を observer-safe に確認できる
- attach acceptance uses the current same-session product alpha carrier and explicit package declarations; it is not a final external issuer / membership attestation pipeline
- `quiescent-save` is current bounded `R2` evidence on the same session carrier; it is not durable/distributed proof completion

## Use The Orchestration Helper

```bash
python3 scripts/operational_product_samples.py list --format json
python3 scripts/operational_product_samples.py run-membership-chat --format json
python3 scripts/operational_product_samples.py run-sugoroku --format json
python3 scripts/operational_product_samples.py run-portal-worldlink --format json
python3 scripts/operational_product_samples.py run-two-shard-hard-boundary --format json
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/operational_product_samples.py release-check --format json
```

## Read The Future Boundary Inventory

- `samples/product-alpha1/operational/deployments/projection/projection.profile.json`
- `samples/product-alpha1/operational/portal-worldlink/`
- `samples/product-alpha1/operational/two-shard-hard-boundary/`
- `samples/product-alpha1/operational/future/portal-worldlink/`
- `samples/product-alpha1/operational/future/two-shard-hard-boundary/`
- `samples/product-alpha1/operational/future/gradient-observation.profile.json`

`projection.profile.json` は current schema-backed inventory です。`portal-worldlink/` と `two-shard-hard-boundary/` は current bounded runtime roots、`future/portal-worldlink/` と `future/two-shard-hard-boundary/` は retained blueprint roots、`gradient-observation.profile.json` は observer-only widening を `planned_only` で固定した future profile です。
