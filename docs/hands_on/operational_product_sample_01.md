# Operational Product Sample 01

この guide は、clean clone から `P-OPS-01 operational product sample suite scaffold and first workflow` とその `P-OPS-03` / `P-OPS-04` / `P-OPS-05` widening を再現するための入口です。

これは final public product ではありません。portal / shard は planned-only inventory を含みますが、runtime 実装 claim ではありません。

## Validate The Repository

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Check The Three Roots

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json
```

Expected bounded evidence:

- `sugoroku-world` の `check` payload は `projection_inventory` summary を返し、`source_package = operational-sugoroku`
- same summary には `target_count = 2`、`packet_boundary_count = 2`、`ffi_boundary_count = 1` が含まれる
- packet / FFI inventory は schema-backed だが、final server/client binary split や direct LLVM backend claim ではない

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
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/operational_product_samples.py release-check --format json
```

## Read The Future Boundary Inventory

- `samples/product-alpha1/operational/deployments/projection/projection.profile.json`
- `samples/product-alpha1/operational/future/portal-worldlink/`
- `samples/product-alpha1/operational/future/two-shard-hard-boundary/`

`projection.profile.json` は current schema-backed inventory です。portal / shard files は current runtime completion ではなく、next package inventory です。
