# Report 2050 — P-OPS-07 two-shard hard-boundary first cut

- Date: 2026-05-07 00:00 JST
- Author / agent: Codex
- Scope: operational two-shard hard-boundary bounded runtime actualization, helper/devtools integration, snapshot/doc sync
- Decision levels touched: `L1`/`L2` wording sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-07` by moving the operational suite shard line from planned-only blueprint inventory to a bounded same-session two-shard hard-authority runtime root without claiming general model-check completion, gradient observation runtime, WAN federation, continuous infinite federation, final ABI, or distributed durable save/load.

## Scope and assumptions

- Scope includes:
  - `samples/product-alpha1/operational/two-shard-hard-boundary/` active root
  - `crates/mir-ast::product_alpha1` package-kind acceptance for `two_shard_hard_boundary`
  - `crates/mir-runtime::product_alpha1_session` bounded shard runtime evidence
  - `crates/mir-runtime::product_alpha1_devtools` shard panel/runtime chain updates
  - `scripts/operational_product_samples.py` shard helper surface and semantic checks
  - required specs / roadmap / hands-on / dashboard / report sync
- Scope excludes:
  - general model-check engine implementation
  - gradient observation runtime
  - WAN federation
  - continuous infinite shard federation
  - final server/client binary split
  - final ABI / SDK
  - direct LLVM backend
- Assumption: Docker and `docker compose` remain available for the unchanged operational transport leg in this environment.

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean
- Existing operational suite state at start:
  - `P-OPS-06` already actualized `portal-worldlink/` as a bounded same-session discrete handoff root
  - `future/two-shard-hard-boundary/` and `spatial-shard-future.profile.json` still held only blueprint/planned-only shard wording
  - RED-stage expectations for `OPS-07` already existed in tests and helper fixtures, but the active shard root and runtime/devtools implementation were not connected

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
- `docs/reports/2049-p-ops-06-portal-worldlink-first-cut.md`
- `sub-agent-pro/operational-product-sample-001/07-portal-spatial-future.md`
- `sub-agent-pro/operational-product-sample-001/15-next-packages.md`
- `sub-agent-pro/operational-product-sample-001/sample-blueprints/portal-and-spatial-blueprint.md`

## Actions taken

- Added an active executable shard root:
  - created `samples/product-alpha1/operational/two-shard-hard-boundary/README.md`
  - created representative source `two-shard-hard-boundary.mir`
  - created executable `package.mir.json` with dependency on `../portal-worldlink`
- Extended product alpha schema/runtime admission:
  - added `two_shard_hard_boundary` to accepted product package kinds
  - added world-like entry handling, default entry place, and representative place graph
  - extended CLI operational package-kind selection to cover portal/shard roots consistently
- Added bounded shard runtime evidence in the same session carrier:
  - accepted `two_shard_hard_boundary` in `run-local`
  - materialized offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject event DAG nodes
  - materialized `same_session_shard_handoff_offer`, `same_session_shard_handoff_commit`, `same_session_shard_old_owner_reject`, `same_session_shard_missing_witness_reject`, `same_session_shard_stale_config_reject` route lanes
  - materialized rejected message rows for `OldOwnerWriteRejected`, `MissingHandoffWitness`, `StaleShardConfig`
  - kept `typed_host_io_claimed = false`
- Extended observer-safe devtools and helper orchestration:
  - `shard_map_future` now becomes `bounded_two_shard_runtime` for the active shard root
  - canonical operational chain now includes `TwoShardHardBoundary`
  - helper script gained `run-two-shard-hard-boundary`
  - `release-check` / `check-all` now validate shard runtime evidence and shard devtools panel evidence
- Updated normative/repository-memory wording:
  - `specs/26` now lists `two-shard-hard-boundary/` as a runnable root and defines the bounded shard package role
  - `specs/27` now allows a bounded same-session two-shard hard-boundary root while preserving non-claims around gradient observation and general model-check completion
  - `plan/51` / `plan/52` now mark `P-OPS-07` actualized and move the practical reopen point to `P-OPS-09`
  - snapshot docs / expected JSON / source hierarchy / hands-on / research summary now distinguish active `two-shard-hard-boundary/` from retained blueprint inventory

## Files changed

- Schema / runtime / devtools / CLI:
  - `crates/mir-ast/src/product_alpha1.rs`
  - `crates/mir-runtime/src/product_alpha1_session.rs`
  - `crates/mir-runtime/src/product_alpha1_devtools.rs`
  - `crates/mirrorea-cli/src/main.rs`
- Tests:
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
  - `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
  - `scripts/tests/test_operational_product_samples.py`
- Helper / sample roots:
  - `scripts/operational_product_samples.py`
  - `samples/product-alpha1/operational/two-shard-hard-boundary/README.md`
  - `samples/product-alpha1/operational/two-shard-hard-boundary/two-shard-hard-boundary.mir`
  - `samples/product-alpha1/operational/two-shard-hard-boundary/package.mir.json`
- Docs / roadmap / dashboards / expected JSON:
  - `README.md`
  - `Documentation.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `samples/product-alpha1/operational/future/two-shard-hard-boundary/README.md`
  - `samples/product-alpha1/operational/future/spatial-shard-future.profile.json`
  - `samples/product-alpha1/operational/expected/workflow.expected.json`
  - `samples/product-alpha1/operational/expected/future-boundary.expected.json`
  - `docs/hands_on/README.md`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/README.md`
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
  - `docs/reports/2050-p-ops-07-two-shard-hard-boundary-first-cut.md`

## Commands run

```bash
python3 -m unittest scripts.tests.test_operational_product_samples
cargo test -p mir-ast --test product_alpha1_package_schema product_alpha1_package_schema_accepts_operational_sample_suite_roots -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_two_shard_hard_boundary_root -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools product_alpha1_operational_two_shard_hard_boundary_devtools_bundle_surfaces_bounded_shard_runtime -- --nocapture
cargo fmt
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json
python3 scripts/operational_product_samples.py run-two-shard-hard-boundary --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- First targeted failures reproduced as intended before implementation:
  - `python3 -m unittest scripts.tests.test_operational_product_samples` failed because `OPS-07` sample-row/evidence helpers were missing
  - `cargo test -p mir-ast --test product_alpha1_package_schema ...accepts_operational_sample_suite_roots...` failed because `samples/product-alpha1/operational/two-shard-hard-boundary` did not exist
  - `cargo test -p mir-runtime --test product_alpha1_session ...two_shard_hard_boundary_root...` failed because the shard root was not accepted by the current front door
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools ...bounded_shard_runtime...` failed for the same reason
- One implementation-side correction was required:
  - initial manifest used `message_recovery_policy.recovery = explicit_reject_only`, which the current schema rejects
  - corrected to `recovery = reject` while preserving reject-only runtime intent
- After implementation:
  - `python3 -m unittest scripts.tests.test_operational_product_samples`: pass
  - focused `mir-ast`, `mir-runtime session`, `mir-runtime devtools` shard tests: pass
  - direct `two-shard-hard-boundary` `check` payload returns `verdict = accepted`, `package_id = operational-two-shard-hard-boundary`, `package_kind = two_shard_hard_boundary`
  - `python3 scripts/operational_product_samples.py run-two-shard-hard-boundary --format json` returns `status = accepted` with `runtime_evidence_observed = true`
- Final full validation floor:
  - `python3 -m unittest scripts.tests.test_validate_docs`: pass
  - `python3 -m unittest scripts.tests.test_operational_product_samples`: pass
  - `python3 scripts/check_source_hierarchy.py`: pass (`required = 139`, `missing = 0`)
  - `python3 scripts/validate_docs.py`: pass
  - `cargo fmt --check`: pass
  - `git diff --check`: pass
  - `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`: pass
  - `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`: pass
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`: pass
  - `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`: pass
  - `python3 scripts/operational_product_samples.py check-all --format json`: `status = accepted`, `docker_included = true`, `failed_commands = []`
  - embedded operational semantic checks all passed:
    - `membership_chat_echo_text_ok = true`
    - `membership_chat_devtools_ok = true`
    - `portal_runtime_ok = true`
    - `portal_devtools_ok = true`
    - `shard_runtime_ok = true`
    - `shard_devtools_ok = true`
    - `projection_inventory_ok = true`
    - `sugoroku_runtime_ok = true`
    - `sugoroku_devtools_ok = true`
    - `attach_matrix_complete = true`

## What changed in understanding

- The first useful shard cut did not require a general model-check engine; the existing same-session product alpha carrier was sufficient for bounded authority-transfer evidence so long as non-claims stayed explicit.
- The key repository-taxonomy boundary is active shard runtime root versus retained shard blueprint inventory, not “no shard runtime at all” versus “full federation”.
- The current executable line can carry shard property-evidence rows while leaving gradient observation, broader replication profiles, and general model-check completion to later packages.

## Open questions

- Should `P-OPS-09` package-authoring guidance teach `two_shard_hard_boundary` first, or stay with `MembershipChat`/`SugorokuWorld` as the authoring starter path?
- When gradient observation is widened later, should it appear as a separate root or as an additional declared profile under the existing shard inventory?
- At what point should backend feasibility inventory (`P-OPS-08`) split host bundle, WASM, and LLVM requirement matrices into separate docs?

## Suggested next prompt

Open `P-OPS-09 developer package authoring guide` and document how an external developer creates a new operational package root, checks it, runs it, exports devtools, and closes it out with `scripts/operational_product_samples.py` without claiming final public grammar or ABI.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: operational suite current executable boundary now includes `TwoShardHardBoundary` bounded same-session hard-authority evidence in addition to the bounded text lane, bounded Sugoroku scenario, schema-backed projection inventory, and bounded portal scenario.

## progress.md update status

`progress.md` 更新済み: latest closeout, reopen point, line snapshot, blockers, validation floor, and recent log now reflect `P-OPS-07`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-07` is marked actualized, ordered packages now start at `P-OPS-09`, and the recommendation now points to the developer package authoring guide.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row now includes the active shard root, shard runtime evidence, and the new validation log row.

## Reviewer findings and follow-up

- Spawned reviewer session `Kant` was opened for read-only semantic review, but timed out twice and was then shut down without findings returned.
- Local focused review findings:
  - shard runtime had to stay explicitly bounded to same-session authority-transfer evidence rather than claiming general model-check completion
  - retained blueprint inventory (`future/two-shard-hard-boundary/`, `spatial-shard-future.profile.json`) needed explicit wording to avoid accidental promotion into the executable path
  - helper closeout needed explicit shard runtime/devtools semantic checks rather than exit-status-only validation
- Follow-up: keep later shard widenings explicit about whether they are runtime evidence, model-check evidence, or blueprint-only inventory.

## Skipped validations and reasons

- No intended validation was skipped.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- Reviewer `Kant` (`019dfdd3-1c44-7c81-bda8-1434b2c07e19`) timed out twice, returned no findings, and was shut down.
