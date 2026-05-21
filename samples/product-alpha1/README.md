# samples/product-alpha1

This root is reserved for the Product/Public-ready Mirrorea Spaces alpha-1 line.

## Current Status

- `demo/` is the product alpha-1 release-candidate root. It exercises `mirrorea-alpha check`, `run-local`, `session`, `attach`, `save`, `load`, `quiescent-save`, local/Docker `transport`, `export-devtools`, `view`, `build-native-bundle`, and `demo`.
- `docker/` contains the controlled Docker Compose TCP transport fixture used by the product release check.
- `computational/` now contains one direct executable Mir-owned computation root, `add-one-pure-mir/`, ten helper-executable `P-COMP-03` rows under positive/negative subdirectories, and one planned-only `P-COMP-04` root. `package.mir.json` is the current executable input; adjacent `.mir` files remain explanatory only.
- `posegraph/` is the `P-POSE-01` planned-only Transform / PoseGraph scaffold. It contains representative `.mir` sketches, `matrix.json`, and helper-validated planned rows, but no executable runtime row yet.
- `projection/` is the `P-PROJ-01` planned-only projection boundary inventory scaffold. It contains target-manifest / packet / FFI / compatibility JSON artifacts plus helper-validated planned rows, but no code generation or server/client binary split.
- `engine-adapter/` is the `P-ENG-01` planned-only engine / WASM / FFI adapter inventory scaffold. It contains provider contract JSON artifacts plus helper-validated planned rows, but no admitted provider execution.
- `operational/` is the canonical operational product sample suite. It contains the six runnable roots `world-core`, `membership-chat`, `sugoroku-world`, `portal-worldlink`, `two-shard-hard-boundary`, and `two-shard-gradient-observation`.
- `operational/templates/` contains `template_only` authoring starters for `world-core`, `membership-chat`, and `sugoroku-world`. Portal/shard starter duplicates remain undefined in the current line.
- `operational/future/` contains retained blueprint/profile inventory. The future files are not executable roots unless a separate active root says so.

## Current Scope Blocks

- `distribution_scope` keeps the current delivery unit at developer-built `mirrorea-alpha` plus locally generated native host launch bundle. Archive, installer, system package, auto-update, and hosted-service shapes remain undefined.
- `shipped_surface` narrows the alpha replay bundle surface to bundled CLI, bundled package root, `manifest.json`, `launch.json`, `run.sh`, `README.md`, and observer-safe supporting artifacts. Other reports and admin/debug session-store artifacts are evidence-only.
- `room_chat_scope` keeps `membership-chat` at bounded single-message room-oriented `ChatText`.
- `sugoroku_scope` keeps `sugoroku-world` at bounded deterministic same-session roll / publish / witness / handoff / stale-membership reject.
- `portal_shard_starter_scope` keeps portal/shard authoring active-root-first and keeps `future/` inventory non-executable.
- `widening_queue_scope` keeps room-chat, portal/shard starter, and broader Sugoroku reopenings non-promoted.
- `user_final_decision_scope` marks broader distribution / final shared-space catalog breadth as a user-spec-required gate.

## Validation Anchor

Use the all-up commands first:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
```

The Docker transport command requires local Docker and Docker Compose. If those tools are unavailable, closeout must record an environment-gated skip with the same non-claims rather than treating the Docker path as passed.

## Stop Lines

- This root does not define final textual `.mir` grammar.
- This root claims product alpha release-candidate workflow readiness through local/Docker controlled validation, not final public product readiness.
- `computational/` now proves one bounded direct Mir-owned runtime row plus helper-executable first-floor widening rows. It is not final grammar, not a completed effect boundary, and it does not reinterpret legacy adapter-owned `typed_host_io.add_one`.
- `posegraph/` is a machine-readable planned-only scaffold, not same-snapshot runtime proof.
- `projection/` is a machine-readable planned-only scaffold, not server/client code generation or backend realization.
- `engine-adapter/` is a machine-readable planned-only scaffold, not provider admission or final FFI ABI.
- `operational/` may expose broader package / deployment / future-boundary inventory than `demo/`, but it remains bounded alpha workflow evidence.
- This root does not claim WAN/federation, distributed durable save/load R3/R4, final public viewer / telemetry ABI, direct Mir-to-machine-code, signature-is-safety, arbitrary native package execution, or final shared-space catalog breadth.
