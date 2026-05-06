# Report 2049 — P-OPS-06 portal / world-link first cut

- Date: 2026-05-06 23:32 JST
- Author / agent: Codex
- Scope: operational portal/world-link bounded runtime actualization, helper/devtools integration, snapshot/doc sync
- Decision levels touched: `L1`/`L2` wording sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-06 portal / world-link first cut` by moving the operational suite portal/world-link line from planned-only inventory to a bounded same-session discrete handoff runtime root without claiming WAN federation, continuous spatial sync, final portal ABI, or shard completion.

## Scope and assumptions

- Scope includes:
  - `samples/product-alpha1/operational/portal-worldlink/` active root
  - `crates/mir-ast::product_alpha1` package-kind acceptance for `portal_worldlink`
  - `crates/mir-runtime::product_alpha1_session` bounded portal runtime evidence
  - `crates/mir-runtime::product_alpha1_devtools` portal panel/runtime chain updates
  - `scripts/operational_product_samples.py` portal helper surface and semantic checks
  - required specs / roadmap / hands-on / dashboard / report sync
- Scope excludes:
  - continuous spatial sync
  - WAN federation
  - two-shard hard-boundary model-check
  - final portal ABI
  - final server/client binary split
  - direct LLVM backend
- Assumption: Docker and `docker compose` remain available for the unchanged operational transport leg in this environment.

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: dirty with intentional RED-stage `P-OPS-06` test edits after `P-OPS-05` commits / push
- Existing operational suite state at start:
  - `P-OPS-01` already provided the canonical `WorldCore -> MembershipChat -> SugorokuWorld` suite
  - `P-OPS-03` already actualized the bounded `MembershipChat` `EchoText("Taro") -> "Hello, Taro!"` direct host boundary
  - `P-OPS-04` already actualized the bounded `SugorokuWorld` same-session runtime scenario
  - `P-OPS-05` already actualized the schema-backed projection inventory summary
  - `future/portal-worldlink/` still held only blueprint/planned-only portal wording

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
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/hands_on/operational_product_sample_01.md`
- `docs/research_abstract/operational_product_sample_01.md`
- `docs/reports/2048-p-ops-05-projection-manifest-packet-ffi-schema.md`
- `sub-agent-pro/operational-product-sample-001/07-portal-spatial-future.md`
- `sub-agent-pro/operational-product-sample-001/15-next-packages.md`
- `sub-agent-pro/operational-product-sample-001/sample-blueprints/portal-and-spatial-blueprint.md`

## Actions taken

- Added an active executable portal root:
  - created `samples/product-alpha1/operational/portal-worldlink/README.md`
  - created representative source `portal-worldlink.mir`
  - created executable `package.mir.json` with dependency on `../sugoroku-world`
- Extended product alpha schema acceptance:
  - added `portal_worldlink` to accepted product package kinds
  - kept `runtime_input` optional and avoided introducing any new host-I/O requirement
- Added bounded portal runtime evidence in the same session carrier:
  - accepted `portal_worldlink` in `run-local`
  - added portal entry place / place graph
  - materialized resolve / handoff offer / witness emit / destination admit event DAG nodes
  - materialized `same_session_portal_resolve`, `same_session_portal_handoff`, `same_session_portal_admit` route lanes
  - kept `typed_host_io_claimed = false`
- Extended observer-safe devtools and helper orchestration:
  - `portal_graph_future` now becomes `bounded_discrete_handoff_runtime` for the active portal root
  - canonical operational chain now includes `PortalWorldLink`
  - helper script gained `run-portal-worldlink`
  - `release-check` / `check-all` now validate portal runtime evidence and portal devtools panel evidence
- Updated normative/repository-memory wording:
  - `specs/26` now lists `portal-worldlink/` as a runnable root and defines the bounded portal package role
  - `specs/27` now allows a bounded same-session portal root while keeping the future blueprint
  - `plan/51` / `plan/52` now mark `P-OPS-06` actualized and move the reopen point to `P-OPS-07`
  - snapshot docs / expected JSON / source hierarchy / hands-on / research summary now distinguish active portal root from retained blueprint root

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
- Helper / sample roots:
  - `scripts/operational_product_samples.py`
  - `samples/product-alpha1/operational/portal-worldlink/README.md`
  - `samples/product-alpha1/operational/portal-worldlink/portal-worldlink.mir`
  - `samples/product-alpha1/operational/portal-worldlink/package.mir.json`
- Docs / roadmap / dashboards / expected JSON:
  - `README.md`
  - `Documentation.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `samples/product-alpha1/operational/future/portal-worldlink/README.md`
  - `samples/product-alpha1/operational/expected/workflow.expected.json`
  - `samples/product-alpha1/operational/expected/future-boundary.expected.json`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/operational_product_sample_01.md`
  - `scripts/README.md`
  - `specs/00-document-map.md`
  - `specs/26-operational-product-sample-suite.md`
  - `specs/27-spatial-portal-and-shard-extension-boundary.md`
  - `plan/00-index.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `scripts/check_source_hierarchy.py`
- Report:
  - `docs/reports/2049-p-ops-06-portal-worldlink-first-cut.md`

## Commands run

```bash
sed -n '1,260p' specs/27-spatial-portal-and-shard-extension-boundary.md
sed -n '1,240p' plan/52-portal-spatial-world-roadmap.md
sed -n '1,220p' samples/product-alpha1/operational/future/portal-worldlink/README.md
sed -n '1,260p' samples/product-alpha1/operational/future/portal-worldlink/portal-worldlink.package.mir.json
python3 -m unittest scripts.tests.test_operational_product_samples
cargo test -p mir-ast --test product_alpha1_package_schema product_alpha1_package_schema_accepts_operational_sample_suite_roots -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_portal_worldlink_root -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools product_alpha1_operational_portal_worldlink_devtools_bundle_surfaces_discrete_handoff -- --nocapture
cargo fmt
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json
python3 scripts/operational_product_samples.py run-portal-worldlink --format json
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- First targeted failures reproduced as intended before implementation:
  - `python3 -m unittest scripts.tests.test_operational_product_samples` failed because portal helper expectations were missing
  - `cargo test -p mir-ast --test product_alpha1_package_schema ...accepts_operational_sample_suite_roots...` failed because `samples/product-alpha1/operational/portal-worldlink` did not exist
  - `cargo test -p mir-runtime --test product_alpha1_session ...portal_worldlink_root...` failed because the portal root was not accepted by the current front door
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools ...portal_worldlink_devtools_bundle...` failed for the same reason
- After implementation:
  - `python3 -m unittest scripts.tests.test_operational_product_samples`: pass
  - `cargo test -p mir-ast --test product_alpha1_package_schema ...accepts_operational_sample_suite_roots...`: pass
  - `cargo test -p mir-runtime --test product_alpha1_session ...portal_worldlink_root...`: pass
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools ...portal_worldlink_devtools_bundle...`: pass
  - direct `portal-worldlink` `check` payload now returns `verdict = accepted`, `package_id = operational-portal-worldlink`, `package_kind = portal_worldlink`
  - `python3 scripts/operational_product_samples.py run-portal-worldlink --format json` now returns `status = accepted` with `runtime_evidence_observed = true`
- `cargo fmt` was run after the code/docs patch set and completed successfully
- Final full validation floor after report addition:
  - `python3 scripts/operational_product_samples.py check-all --format json`: `status = accepted`, `docker_included = true`, `failed_commands = []`
  - embedded validations all passed:
    - `validation:test-validate-docs`
    - `validation:test-operational-helper`
    - `validation:source-hierarchy`
    - `validation:validate-docs`
    - `validation:cargo-fmt`
    - `validation:git-diff-check`
    - `test:mir-ast-product-schema`
    - `test:mir-runtime-session`
    - `test:mir-runtime-devtools`
    - `test:mirrorea-cli-alpha`
  - embedded operational semantic checks all passed:
    - `membership_chat_echo_text_ok = true`
    - `membership_chat_devtools_ok = true`
    - `portal_runtime_ok = true`
    - `portal_devtools_ok = true`
    - `projection_inventory_ok = true`
    - `sugoroku_runtime_ok = true`
    - `sugoroku_devtools_ok = true`
    - `attach_matrix_complete = true`

## What changed in understanding

- Portal actualization did not require a separate transport/runtime family; the existing same-session product alpha carrier was sufficient for a bounded discrete handoff first cut.
- The critical boundary was not “portal as a planned manifest” versus “portal as WAN/world federation”, but “portal as a bounded discrete handoff root” versus “continuous sync / shard federation / final portal ABI”.
- Keeping `future/portal-worldlink/` as a retained blueprint root while adding `portal-worldlink/` as the active executable root preserves the repository taxonomy rule that planned skeleton families must not be silently promoted.

## Open questions

- Should `P-OPS-07` consume the portal root directly as the handoff source, or keep shard/model-check evidence decoupled at first?
- When shard model-check evidence lands, should the current portal route semantics gain negative stale-epoch rows immediately or remain a positive bounded cut?
- At what point should `portal-worldlink` gain richer admission authority splits beyond the current membership/capability/witness bounded scenario?

## Suggested next prompt

Open `P-OPS-07 two-shard hard-boundary model-check sample` and add finite two-shard authority transfer evidence with no-double-owner, stale-owner reject, and missing-handoff-witness reject without claiming continuous federation or distributed durable save/load.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: operational suite current executable boundary now includes `PortalWorldLink` bounded same-session discrete handoff in addition to the bounded text lane, bounded Sugoroku scenario, and schema-backed projection inventory.

## progress.md update status

`progress.md` 更新済み: latest closeout, reopen point, line snapshot, blockers, validation floor, and recent log now reflect `P-OPS-06`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-06` is marked actualized, ordered packages now start at `P-OPS-07`, and the recommendation now points to the two-shard hard-boundary model-check sample.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row now includes the active portal root, portal runtime evidence, and the new validation log row.

## Reviewer findings and follow-up

- No spawned reviewer sessions were opened in this package. Current tool policy required explicit user delegation permission before new sub-agent use, so this package used local focused review instead.
- Local review findings:
  - portal participant places must keep the standard `ParticipantPlace` kind or the runtime shell rejects duplicate place registrations
  - helper closeout needed explicit portal runtime/devtools semantic checks rather than only exit-status checks
  - snapshot docs needed a clear split between active `portal-worldlink/` and retained `future/portal-worldlink/` blueprint root to avoid taxonomy drift
- Follow-up: when `P-OPS-07` starts, keep the shard boundary equally explicit and avoid silently reusing portal wording as if it already implied continuous sync or federation.

## Skipped validations and reasons

- No intended validation was skipped.

## Commit / push status

- Commit: `a267019`
- Push: `origin/feature/operational-product-sample-001` へ push 済み

## Sub-agent session close status

- No new sub-agent sessions were opened in this package.
