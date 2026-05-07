# Operational Product Sample 01

この guide は、clean clone から `P-OPS-01 operational product sample suite scaffold and first workflow` とその `P-OPS-03` / `P-OPS-04` / `P-OPS-05` / `P-OPS-06` / `P-OPS-07` / `P-OPS-13` / `P-OPS-15` / `P-OPS-21` / `P-OPS-22` / `P-OPS-23` / `P-OPS-24` / `P-OPS-25` queue-state hardening を再現するための入口です。

これは final public product ではありません。portal は bounded same-session first cut、shard は bounded same-session two-shard hard-authority first cut、gradient observation は separate `two-shard-gradient-observation/` root による bounded observer-only runtime first cut まで actualize 済みです。`future/gradient-observation.profile.json` は引き続き non-executable profile inventory であり、general model-check completion、continuous spatial sync、WAN federation / continuous infinite shard federation / final portal ABI ではありません。validated starter catalog は intentionally `SugorokuWorld` で止まり、portal/shard authoring は active roots を study/copy boundary に留めます。

## Validate The Repository

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Check The Six Roots

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-gradient-observation --format json
```

Expected bounded evidence:

- `sugoroku-world` の `check` payload は `projection_inventory` summary を返し、`source_package = operational-sugoroku`
- same summary には `target_count = 2`、`packet_boundary_count = 2`、`ffi_boundary_count = 1` が含まれる
- packet / FFI inventory は schema-backed だが、final server/client binary split や direct LLVM backend claim ではない
- current backend comparison inventory is documented separately in `operational_backend_inventory_01.md`; current actualized native path remains the host launch bundle

## Run The MembershipChat Room-Chat Boundary

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
- observer-safe host-I/O lane includes `ChatText:Text("hello room")->Text("room#lobby message accepted: hello room")`
- helper-facing `room_chat_scope.lane_kind = bounded_single_message_room_oriented_chat_text`
- `room_chat_scope.multi_message_room_surface_defined = false`
- `room_chat_scope.transport_coupled_chat_lane_defined = false`
- event DAG / devtools export show the same bounded room-oriented request/response without introducing stdio as a Mir core primitive

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

## Run The Gradient Observation Runtime

```bash
gradient_session_dir=$(mktemp -d /tmp/mirrorea-ops-gradient-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$gradient_session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-gradient-observation --format json
gradient_viewer_dir=$(mktemp -d /tmp/mirrorea-ops-gradient-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$gradient_session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-gradient-observation' --out "$gradient_viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$gradient_viewer_dir" --check --format json
```

Expected bounded evidence:

- `typed_host_io_claimed = false`
- `run-local` / `export-devtools` payload には `gradient_view_observed`, `gradient_handoff_hint_published`, `gradient_write_rejected`, `gradient_stale_view_dropped`, `gradient_missing_freshness_rejected` が入る
- route lanes は `same_session_gradient_view`, `same_session_gradient_handoff_hint`, `same_session_gradient_write_reject`, `same_session_gradient_stale_view_drop`, `same_session_gradient_missing_freshness_reject` を observer-safe に保持する
- `gradient_write_rejected` は observer copy に write authority を与えず、freshness tuple が欠けた row は reject lane に落ちる
- devtools `shard_map_future.current_status = bounded_gradient_observation_runtime`
- これは bounded same-session observer-only cut であり、continuous sync、write authority、WAN federation は主張しない

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
- helper-facing `sugoroku_scope.scenario_kind = bounded_deterministic_same_session_sugoroku`
- `sugoroku_scope.interactive_turn_choice_surface_defined = false`
- `sugoroku_scope.broader_negative_row_catalog_defined = false`
- `sugoroku_scope.networked_multi_participant_control_defined = false`
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
python3 scripts/operational_product_samples.py run-two-shard-gradient-observation --format json
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/operational_product_samples.py release-check --format json
```

## Read The Future Boundary Inventory

- `samples/product-alpha1/operational/deployments/projection/projection.profile.json`
- `samples/product-alpha1/operational/portal-worldlink/`
- `samples/product-alpha1/operational/two-shard-hard-boundary/`
- `samples/product-alpha1/operational/two-shard-gradient-observation/`
- `samples/product-alpha1/operational/future/portal-worldlink/`
- `samples/product-alpha1/operational/future/two-shard-hard-boundary/`
- `samples/product-alpha1/operational/future/gradient-observation.profile.json`

`projection.profile.json` は current schema-backed inventory です。`portal-worldlink/`、`two-shard-hard-boundary/`、`two-shard-gradient-observation/` は current bounded runtime roots、`future/portal-worldlink/` と `future/two-shard-hard-boundary/` は retained blueprint roots、`gradient-observation.profile.json` は separate runtime root と paired の non-executable future profile です。

portal/shard authoring boundary を確認したい場合は `operational_portal_shard_starter_boundary_01.md` を参照してください。helper `check-all` の `portal_shard_starter_scope` も同じ current decision を machine-readable に返します。
Sugoroku boundedness を helper surface で確認したい場合は `python3 scripts/operational_product_samples.py run-sugoroku --format json` または `check-all --format json` を使ってください。`sugoroku_scope` が current bounded deterministic carrier を machine-readable に返します。
queue-state を helper surface で確認したい場合は `check-all --format json` の `widening_queue_scope` を見てください。current line では `room_chat_reopen_recommended = false`, `portal_shard_starter_reopen_recommended = false`, `sugoroku_reopen_recommended = false`, `next_promoted_reopen_point = later_user_final_distribution_decision` です。
