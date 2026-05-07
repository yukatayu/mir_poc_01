# samples/product-alpha1

This root is reserved for the Product/Public-ready Mirrorea Spaces alpha-1 line.

Current status:

- `demo/` is the product alpha-1 schema / CLI fixture root introduced in `P-A1-26`.
- `demo/` also has the `P-A1-27` local same-session runtime first cut through `mirrorea-alpha run-local`, `session`, and `attach`.
- `demo/` has the `P-A1-28` bounded message recovery and local save first cut through `mirrorea-alpha save`, `load`, and `quiescent-save`.
- `demo/` and `docker/` have the `P-A1-29` local/Docker transport and non-final viewer first cut through `mirrorea-alpha transport`, `export-devtools`, and `view`.
- `demo/` has the `P-A1-30` native host launch bundle first cut through `mirrorea-alpha build-native-bundle`.
- `demo/` has the `P-A1-31` release-candidate workflow through `mirrorea-alpha demo` and `scripts/product_alpha1_release_check.py check-all`.
- `P-OPS-19` narrows the current shipped surface for that product line to built-binary `check` / `build-native-bundle` / `demo`, bundle replay `run.sh check` / `run.sh view`, and the bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md` plus observer-safe supporting artifacts. Other bundled reports and admin/debug session-store artifacts remain evidence-only.
- `P-OPS-20` narrows broader public distribution further: current delivery remains only a developer-built `mirrorea-alpha` binary plus a locally generated native host launch bundle, while archive / installer / system package / auto-update / hosted-service shapes remain undefined.
- `operational/` has the `P-OPS-01` canonical operational product sample suite through `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`, `scripts/operational_product_samples.py`, retained portal/shard blueprint inventory, the `P-OPS-03` direct host boundary plus `P-OPS-13` bounded room-oriented `MembershipChat` `ChatText("hello room") -> "room#lobby message accepted: hello room"` lane, the `P-OPS-04` bounded Sugoroku roll/publish/witness/handoff/stale-reject runtime evidence, the `P-OPS-05` schema-backed projection target / packet / FFI inventory surfaced through `check`, runtime plan, and devtools, the `P-OPS-06` bounded `portal-worldlink/` same-session discrete handoff root, the `P-OPS-07` bounded `two-shard-hard-boundary/` same-session hard-authority handoff root, and the `P-OPS-15` bounded `two-shard-gradient-observation/` observer-only gradient root while retaining `future/portal-worldlink/` and `future/two-shard-hard-boundary/` as blueprints.
- `P-OPS-21` keeps the current `membership-chat` lane narrow by adding helper-reported `room_chat_scope`; current room-chat remains bounded single-message room-oriented `ChatText`, while multi-message / transport-coupled / room-history / stdio shapes remain undefined.
- `P-OPS-22` keeps portal/shard authoring active-root-first by adding helper-reported `portal_shard_starter_scope`; the validated starter catalog still stops at `templates/sugoroku-world-starter`, and current portal/shard starter duplicates remain undefined.
- `P-OPS-23` keeps the current `sugoroku-world` carrier narrow by adding helper-reported `sugoroku_scope`; current gameplay remains bounded deterministic same-session roll / publish / witness / handoff / stale-membership reject, while interactive turn choice, broader negative rows, and networked multi-participant control remain undefined.
- `P-OPS-25` updates helper-reported `widening_queue_scope`; current room-chat reopening, portal/shard starter reopening, and broader Sugoroku reopening all remain non-promoted, and `later_user_final_distribution_decision` becomes the next promoted comparison.
- `P-OPS-26` adds helper-reported `user_final_decision_scope`; current delivery remains developer-built binary + generated host launch bundle, current catalog scope remains the bounded product alpha-1 narrow showcase, and broader final distribution / final shared-space catalog breadth stay on a user-spec-required gate.
- `operational/templates/` has the `P-OPS-09` / `P-OPS-10` validated authoring starter catalog. `world-core-starter/`, `membership-chat-starter/`, and `sugoroku-world-starter/` are `template_only`, pass `check` and `run-local`, and are documented by `docs/hands_on/operational_package_authoring_01.md`; they are not counted as active operational sample roots. `portal-worldlink/` and `two-shard-hard-boundary/` remain active roots instead of gaining starter duplicates in the current line, and that boundary is documented by `docs/hands_on/operational_portal_shard_starter_boundary_01.md`.
- `operational/future/gradient-observation.profile.json` has the `P-OPS-11` docs-first observer-only gradient widening profile; it remains non-executable even after `P-OPS-15` added the separate bounded `two-shard-gradient-observation/` runtime root.
- `operational/` backend feasibility remains docs-first. Current actualized path is `build-native-bundle` -> `native host launch bundle`; WASM/LLVM wording is inventory-only and is documented by `docs/hands_on/operational_backend_inventory_01.md`.
- The root is product alpha release-candidate workflow-ready, not final public product-ready.
- Full release-candidate evidence requires Docker Compose TCP. `--skip-docker` is a local probe and reports non-readiness.
- Product demo source explicitly declares the admin membership/capability authority needed by debug/auth/rate-limit attach; object/avatar-preview attach remains deferred boundary evidence.
- It must stay separate from `samples/practical-alpha1/`, which remains first-floor / bounded workflow evidence.
- It must stay separate from `samples/alpha/`, which remains alpha-0 evidence.

Current validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/auth-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/rate-limit-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/placeholder-object --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/custom-avatar-preview --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json
viewer_dir=$(mktemp -d /tmp/mirrorea-alpha1-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
bundle_dir=$(mktemp -d /tmp/mirrorea-alpha1-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
demo_dir=$(mktemp -d /tmp/mirrorea-alpha1-demo-XXXXXX)
cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out "$demo_dir" --format json
release_dir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$release_dir"
python3 scripts/operational_product_samples.py list --format json
python3 scripts/operational_product_samples.py check-all --format json
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
```

The Docker transport command requires local Docker and Docker Compose. If those
tools are unavailable, a closeout must record an environment-gated skip with the
same non-claims rather than treating the Docker path as passed.

Stop lines:

- This root does not define final textual `.mir` grammar.
- This root claims product alpha release-candidate workflow readiness through local/Docker controlled validation, not final public product readiness.
- `operational/` may expose broader package / deployment / future-boundary inventory than `demo/`, but it remains bounded alpha workflow evidence, not final public product.
- This root does not claim WAN/federation, distributed durable save/load R3/R4, final public viewer / telemetry ABI, direct Mir-to-machine-code, signature-is-safety, or arbitrary native package execution.
