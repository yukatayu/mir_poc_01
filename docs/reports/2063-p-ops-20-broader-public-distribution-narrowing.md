# Report 2063 — P-OPS-20 broader public distribution narrowing

- Date: 2026-05-07 10:33 JST
- Author / agent: Codex
- Scope: product alpha broader-distribution boundary narrowing, installed-binary helper surface, spec/docs/roadmap/dashboard synchronization
- Decision levels touched: `L1`/`L2` wording in `specs/25`; no new runtime semantics

## Objective

Close `P-OPS-20` by making the broader public distribution stance explicit: keep the already narrowed front door and shipped surface intact, add machine-readable `distribution_scope` to the installed-binary helper, and state that current product alpha-1 still does not define archive / installer / system-package / auto-update / hosted-service shapes beyond the developer-built binary plus generated host launch bundle.

## Scope and assumptions

- Scope includes:
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
  - `specs/25-product-alpha1-public-boundary.md`
  - product alpha guides / summaries / roadmap / snapshot docs
- Scope excludes:
  - new CLI/runtime behavior
  - new native bundle artifact schema beyond existing `shipped_surface`
  - hosted-service, WAN, distributed durable save/load, final packaging
  - final textual `.mir` grammar or final ABI freeze
- Assumptions:
  - `P-OPS-18` already narrowed the hardening target to versioned `package.mir.json`, documented `mirrorea-alpha`, and native host launch bundle replay
  - `P-OPS-19` already narrowed the current shipped surface to the alpha replay bundle unit
  - the safest current move is queue narrowing, not widening into archive/install/service claims

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean immediately after `P-OPS-19` commit `d8f4cc06`
- Existing current status at start:
  - installed-binary adoption probe, grammar / ABI scoping, and shipped-surface hardening were already validated
  - remaining ambiguity was whether any broader installed/public distribution shape should be treated as current product alpha scope
- Dirty state during this package before final validation:
  - `Documentation.md`
  - `README.md`
  - `docs/hands_on/product_alpha1_01.md`
  - `docs/research_abstract/product_alpha1_01.md`
  - `plan/50-product-alpha1-public-boundary-roadmap.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `progress.md`
  - `samples/product-alpha1/README.md`
  - `samples_progress.md`
  - `scripts/README.md`
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
  - `specs/25-product-alpha1-public-boundary.md`
  - `tasks.md`

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
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/00-index.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/hands_on/product_alpha1_01.md`
- `docs/research_abstract/product_alpha1_01.md`
- `scripts/README.md`
- `samples/product-alpha1/README.md`

## Actions taken

- Added a machine-readable `distribution_scope` block to `scripts/product_alpha1_installed_binary_check.py`.
- Made the current distribution stance explicit:
  - current delivery unit is only developer-built `mirrorea-alpha` plus locally generated native host launch bundle
  - archive / installer / system-package / auto-update / hosted-service shapes remain undefined
- Extended `scripts/tests/test_product_alpha1_installed_binary_check.py` with red/green coverage for `distribution_scope`, including the preflight error path.
- Updated `specs/25` with a normative broader-distribution stance section and a post-`P-OPS-20` reading.
- Updated product hands-on / research summary, root summaries, roadmap memory, and dashboards so they distinguish:
  - current hardening target
  - current shipped surface
  - current broader distribution stance
  - later user/final distribution decisions
- Moved the self-driven reopen point from broader public distribution narrowing to broader room-chat revisit.

## Files changed

- Helper / tests:
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
- Normative / reader-facing docs:
  - `specs/25-product-alpha1-public-boundary.md`
  - `README.md`
  - `Documentation.md`
  - `docs/hands_on/product_alpha1_01.md`
  - `docs/research_abstract/product_alpha1_01.md`
  - `scripts/README.md`
  - `samples/product-alpha1/README.md`
- Roadmap / snapshot:
  - `plan/50-product-alpha1-public-boundary-roadmap.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report:
  - `docs/reports/2063-p-ops-20-broader-public-distribution-narrowing.md`

## Commands run

```bash
git status --short
rg -n "distribution_scope|broader public distribution|installed binary|hosted-service|archive|installer" README.md Documentation.md docs/hands_on/product_alpha1_01.md docs/research_abstract/product_alpha1_01.md scripts/README.md samples/product-alpha1/README.md specs/25-product-alpha1-public-boundary.md plan/50-product-alpha1-public-boundary-roadmap.md plan/51-operational-product-sample-roadmap.md plan/52-portal-spatial-world-roadmap.md progress.md tasks.md samples_progress.md
python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-installed-binary-check-XXXXXX) && echo "$tmpdir" && python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out "$tmpdir"
git status --short
```

## Evidence / outputs / test results

- TDD red phase:
  - `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
    - failed with missing `distribution_scope`
- Focused green phase:
  - `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
    - 5 tests passed after helper update
- Fresh validation and helper outputs:
  - `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check`
    - 18 tests passed
  - `python3 scripts/check_source_hierarchy.py`
    - `required = 155`
    - `present = 155`
    - `missing = 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1215 numbered report(s).`
  - `cargo fmt --check`
    - passed
  - `git diff --check`
    - passed
  - `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-mY12PJ`
    - `status = accepted`
    - `failed_commands = []`
    - `installed_binary_candidate_ready = true`
    - `public_packaging_candidate = "installed_binary_plus_native_host_launch_bundle"`
    - `compatibility_scope.package_format = "versioned_package_mir_json"`
    - `compatibility_scope.cli_surface = "mirrorea_alpha_documented_command_family"`
    - `compatibility_scope.bundle_surface = "native_host_launch_bundle_run_sh"`
    - `shipped_surface.delivery_model = "installed_binary_plus_native_host_launch_bundle"`
    - `distribution_scope.current_delivery_unit = "developer_built_binary_plus_generated_host_launch_bundle"`
    - `distribution_scope.archive_distribution_defined = false`
    - `distribution_scope.installer_distribution_defined = false`
    - `distribution_scope.system_package_distribution_defined = false`
    - `distribution_scope.auto_update_channel_defined = false`
    - `distribution_scope.hosted_service_distribution_defined = false`

## What changed in understanding

- The shipped surface and the broader distribution stance are separate questions. `P-OPS-19` answered the former; `P-OPS-20` answers the latter.
- The safest current product alpha reading is not “some future archive/install shape is implied,” but “no broader distribution shape is defined yet.”
- Once that narrowing is written down, the next self-driven queue can move back to operational behavior shaping rather than staying stuck on packaging ambiguity.

## Open questions

- Should broader room-chat stay on the current bounded `ChatText` lane or reopen as multi-message / transport-coupled widening?
- If broader public distribution is revisited later, should it be archive-first, installer-first, or remain hosted-service deferred?
- Which later user/final decision should own the first widening beyond developer-built binary + generated host bundle?

## Suggested next prompt

`P-OPS-21 broader room-chat revisit を開き、current bounded ChatText lane を維持するのか、multi-message / transport-coupled widening を separate package として reopen するのかを specs / roadmap / dashboard / validation まで含めて整理してください。`

## Plan update status

`plan/` 更新済み: `plan/50-product-alpha1-public-boundary-roadmap.md`, `plan/51-operational-product-sample-roadmap.md`, `plan/52-portal-spatial-world-roadmap.md` を `P-OPS-20` closeout と `broader room-chat revisit` next queue に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: `P-OPS-20` の `distribution_scope` / broader-distribution narrowing reading を current snapshot paragraph に追加した。

## progress.md update status

`progress.md` 更新済み: latest closeout package を `P-OPS-20` に進め、current promoted reopen point / blockers / recent log を `broader room-chat revisit` へ同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-20` を current task-level status に追加し、ordered self-driven packages と current recommendation を `broader room-chat revisit` / `portal-shard starter revisit` へ進めた。

## samples_progress.md update status

`samples_progress.md` 更新済み: product row に `distribution_scope` evidence を追加し、recent validation log と next gap を later user/final broader distribution decision / `broader room-chat revisit` へ同期した。

## Reviewer findings and follow-up

- Local focused review only:
  - the narrowest coherent machine-readable addition point remained `scripts/product_alpha1_installed_binary_check.py`
  - the safer reading was to avoid copying `distribution_scope` into broader runtime/bundle surfaces unless a later package explicitly needs that contract
- Follow-up applied:
  - helper-only `distribution_scope`
  - spec/docs/roadmap/dashboard synchronization

## Skipped validations and reasons

- `python3 scripts/product_alpha1_release_check.py --format json check-all --out ...` was not re-run because `P-OPS-20` changed helper/docs/spec/dashboard wording only and did not change product runtime/bundle execution semantics.
- `python3 scripts/operational_product_samples.py check-all --format json` was not re-run because no operational suite runtime/schema/helper behavior changed in this package.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- No new sub-agent sessions were started for `P-OPS-20`; local focused review was used.
