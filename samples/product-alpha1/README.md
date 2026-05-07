# samples/product-alpha1

This root is reserved for the Product/Public-ready Mirrorea Spaces alpha-1 line.

## Current Status

- `demo/` is the product alpha-1 release-candidate root. It exercises `mirrorea-alpha check`, `run-local`, `session`, `attach`, `save`, `load`, `quiescent-save`, local/Docker `transport`, `export-devtools`, `view`, `build-native-bundle`, and `demo`.
- `docker/` contains the controlled Docker Compose TCP transport fixture used by the product release check.
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
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
```

The Docker transport command requires local Docker and Docker Compose. If those tools are unavailable, closeout must record an environment-gated skip with the same non-claims rather than treating the Docker path as passed.

## Stop Lines

- This root does not define final textual `.mir` grammar.
- This root claims product alpha release-candidate workflow readiness through local/Docker controlled validation, not final public product readiness.
- `operational/` may expose broader package / deployment / future-boundary inventory than `demo/`, but it remains bounded alpha workflow evidence.
- This root does not claim WAN/federation, distributed durable save/load R3/R4, final public viewer / telemetry ABI, direct Mir-to-machine-code, signature-is-safety, arbitrary native package execution, or final shared-space catalog breadth.
