# Report 2048 — P-OPS-05 projection manifest / packet / FFI schema

- Date: 2026-05-06 23:01 JST
- Author / agent: Codex
- Scope: operational projection inventory schema formalization, runtime/devtools/helper integration, snapshot/doc sync
- Decision levels touched: `L1`/`L2` wording and bounded executable-surface sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-05 projection manifest / packet / FFI schema` by moving the operational suite projection inventory from manifest-only wording to schema-backed, validation-backed inventory without claiming final server/client binary split, direct LLVM backend, or richer placement/runtime completion.

## Scope and assumptions

- Scope includes:
  - `samples/product-alpha1/operational/deployments/projection/projection.profile.json`
  - `crates/mir-ast::product_alpha1` projection inventory schema + `check` summary
  - `crates/mir-runtime::product_alpha1_session` runtime-plan projection summary plumbing
  - `crates/mir-runtime::product_alpha1_devtools` observer-safe projection panel wiring
  - `scripts/operational_product_samples.py` semantic checks for projection inventory
  - required specs / roadmap / hands-on / dashboard / report sync
- Scope excludes:
  - final server/client binary split
  - placement optimizer / projection planner
  - direct Mir-to-machine-code or LLVM backend
  - portal / world-link runtime actualization
  - shard / federation runtime or model-check actualization
- Assumption: Docker and `docker compose` remain available for the unchanged operational transport leg in this environment.

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: dirty with intentional RED-stage `P-OPS-05` test edits after `P-OPS-04` commit / push `83f6ba1`
- Existing operational suite state at start:
  - `P-OPS-01` already provided the canonical `WorldCore -> MembershipChat -> SugorokuWorld` suite
  - `P-OPS-03` already actualized the bounded `MembershipChat` `EchoText("Taro") -> "Hello, Taro!"` direct host boundary
  - `P-OPS-04` already actualized the bounded `SugorokuWorld` same-session runtime scenario
  - projection inventory was still static wording / static devtools inventory rather than schema-backed checker/runtime/devtools/helper evidence

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/reports/2047-p-ops-04-sugoroku-behavior-widening.md`
- `sub-agent-pro/operational-product-sample-001/sample-blueprints/deployment-and-projection-blueprint.md`

## Actions taken

- Added bounded projection inventory schema support inside `crates/mir-ast::product_alpha1`:
  - formalized `ops-product-projection-v0` target / packet / FFI / backend fields
  - validated unique target and boundary names, required field families, `non_final = true`, and explicit backend non-claims
  - rejected projection targets that claim `native_binary_emitted = true`
  - surfaced accepted projection inventory as `check`-time `projection_inventory` summary plus accepted obligation
- Threaded projection inventory through the product alpha runtime/devtools path:
  - `run-local` now copies `check_report.projection_inventory` into the runtime plan
  - observer-safe devtools now render a schema-backed projection panel when inventory is available instead of the old static intent-only panel
- Extended the operational helper with projection semantic validation:
  - `release-check` / `check-all` now fail if the `sugoroku-world` `check` payload does not expose the expected projection summary
  - packet boundary and FFI boundary names are asserted in addition to counts and backend non-claims
- Updated operational suite docs and dashboards so `P-OPS-05` is reflected as the latest closeout and `P-OPS-06` as the next reopen point.
- Synced the prior `P-OPS-04` report with actual commit / push status.

## Files changed

- Schema / runtime / devtools:
  - `crates/mir-ast/src/product_alpha1.rs`
  - `crates/mir-runtime/src/product_alpha1_session.rs`
  - `crates/mir-runtime/src/product_alpha1_devtools.rs`
- Tests:
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
  - `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
  - `scripts/tests/test_operational_product_samples.py`
- Helper / docs / roadmap / dashboards:
  - `scripts/operational_product_samples.py`
  - `specs/26-operational-product-sample-suite.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `README.md`
  - `Documentation.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `samples/product-alpha1/operational/sugoroku-world/README.md`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/operational_product_sample_01.md`
  - `scripts/README.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report sync:
  - `docs/reports/2047-p-ops-04-sugoroku-behavior-widening.md`
  - `docs/reports/2048-p-ops-05-projection-manifest-packet-ffi-schema.md`

## Commands run

```bash
sed -n '1,260p' sub-agent-pro/operational-product-sample-001/sample-blueprints/deployment-and-projection-blueprint.md
sed -n '1,260p' samples/product-alpha1/operational/deployments/projection/projection.profile.json
sed -n '1,260p' specs/23-typed-external-host-boundary.md
sed -n '1,260p' plan/50-product-alpha1-public-boundary-roadmap.md
sed -n '1,340p' crates/mir-ast/src/product_alpha1.rs
sed -n '1,360p' crates/mir-runtime/src/product_alpha1_session.rs
sed -n '1,340p' crates/mir-runtime/src/product_alpha1_devtools.rs
python3 -m unittest scripts.tests.test_operational_product_samples
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json
cargo fmt
python3 scripts/operational_product_samples.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M:%S %z'
```

## Evidence / outputs / test results

- First targeted failures reproduced as intended before implementation:
  - `python3 -m unittest scripts.tests.test_operational_product_samples` failed because `sugoroku_projection_inventory_observed` was missing
  - `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture` failed on missing `projection_inventory` summary in the `check` report
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture` failed because the devtools projection panel had no schema-backed boundary-name fields
- After implementation:
  - `python3 -m unittest scripts.tests.test_operational_product_samples`: pass
  - `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`: pass
  - `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`: pass
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`: pass
  - direct `sugoroku-world` `check` payload now includes `projection_inventory` with `source_package = operational-sugoroku`, `target_count = 2`, `packet_boundary_count = 2`, and `ffi_boundary_count = 1`
  - `python3 scripts/operational_product_samples.py check-all --format json` reaches `status = accepted` with `projection_inventory_ok = true`
- `cargo fmt --check` initially reported formatting drift in the new projection inventory Rust code; `cargo fmt` fixed it before final closeout
- After adding this report:
  - `python3 -m unittest scripts.tests.test_validate_docs`: pass, `13` tests
  - `python3 scripts/check_source_hierarchy.py`: pass
  - `python3 scripts/validate_docs.py`: pass
  - `cargo fmt --check`: pass
  - `git diff --check`: pass

## What changed in understanding

- Projection inventory did not require a separate projection runtime; a bounded schema-backed summary carried from `check` into runtime/devtools was sufficient to strengthen the operational suite without overclaiming split execution.
- The correct acceptance surface is not just counts: packet boundary names, FFI boundary names, and backend non-claims must stay explicit or the inventory becomes too weak to be useful evidence.
- The existing observer-safe devtools panel set can absorb projection inventory widening without introducing a final public viewer ABI.

## Open questions

- When portal/world-link actualization starts, should it consume the current projection inventory summary directly or require a richer projection IR first?
- At what point should the current schema-backed summary widen into a placement/planner boundary rather than remaining a bounded report surface?
- Should the next `MembershipChat` widening remain on the narrow `EchoText` lane until portal/world-link evidence lands?

## Suggested next prompt

Open `P-OPS-06 portal / world-link first cut` and move the operational suite portal inventory from planned-only files to bounded runtime or model-check evidence without claiming continuous spatial sync or WAN federation.

## Plan update status

`plan/` 更新済み:

- `plan/51-operational-product-sample-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: operational suite current executable boundary now includes schema-backed projection inventory in addition to the bounded `MembershipChat` text lane and bounded `SugorokuWorld` runtime scenario.

## progress.md update status

`progress.md` 更新済み: latest closeout, reopen point, line snapshot, blockers, validation floor, and recent log now reflect `P-OPS-05`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-05` is marked actualized, ordered packages now start at `P-OPS-06`, and the recommendation now points to portal/world-link first cut.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row now includes schema-backed projection inventory and the new validation log row.

## Reviewer findings and follow-up

- No spawned reviewer sessions were opened in this package. Current tool policy required explicit user delegation permission before new sub-agent use, so this package used local focused review instead.
- Local review findings:
  - the projection summary needed to be threaded into both `check_report` and `runtime_plan`; checking only one would leave devtools/helper evidence inconsistent
  - devtools projection inventory needed explicit boundary-name fields; counts alone were not enough for observer-safe evidence
  - helper closeout needed a dedicated projection semantic check rather than relying on exit status
- Follow-up: when portal/shard packages widen, keep their runtime/model-check evidence bounded and avoid silently turning the projection summary into a split-runtime claim.

## Skipped validations and reasons

- No intended validation was skipped.
- No portal / shard runtime validations were added because `P-OPS-05` widened only the projection inventory boundary.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- No new sub-agent sessions were opened in this package.
