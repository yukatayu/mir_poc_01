# Report 2171 — Product Alpha release path portability

- Date: 2026-07-04 12:03 JST
- Author / agent: Codex
- Scope: Product Alpha release-check and generated evidence path-portability hardening
- Decision levels touched: LAB maintenance only; no L0/L1/L2/L3 normative decision changed

## Objective

Prevent Product Alpha release-check output and generated Product Alpha evidence from
recording repo-root absolute paths for repo-owned source inputs, while preserving
actual execution paths and the existing alpha release-candidate boundary.

## Scope and assumptions

Scope was limited to `scripts/product_alpha1_release_check.py`, its focused unit
tests, `mirrorea-alpha` Product Alpha demo/native/Docker evidence fields, and
snapshot/report documentation.

Assumptions:

- Repo-owned source inputs should be displayed as repo-relative `samples/...`
  paths when used as CLI arguments or generated source provenance.
- Output directories, session stores, and runtime artifacts remain execution-local
  paths unless the release-check aggregate is explicitly presenting its own output
  tree, where output-root-relative display is preferred.
- This is path-portability hardening only. It does not change Product Alpha
  workflow status, final grammar/API/ABI claims, native execution policy, or
  distribution scope.

## Start state / dirty state

Package 33 started from a clean worktree at `34cf0e3d` after the Full System V1
provider/renderer path-portability package had been committed and pushed.

Initial Product Alpha path audit showed:

- `scripts/product_alpha1_release_check.py --skip-docker` planned package argv
  as `/home/codex/dev/mir_poc_01/samples/product-alpha1/demo...`.
- Generated demo report `package_path` used the repo-root absolute sample path.
- Native bundle `provenance.json` used repo-root absolute
  `source_package_root`.
- Docker compose evidence had a repo-root absolute compose file path when Docker
  execution was included.

## Documents consulted

- `AGENTS.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/TEMPLATE.md`
- `scripts/product_alpha1_release_check.py`
- `scripts/tests/test_product_alpha1_release_check.py`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`

## Actions taken

- Added release-check helpers to render repo-owned sample CLI input paths as
  repo-relative argv.
- Added release-check aggregate display normalization for paths owned by the
  chosen output root.
- Added focused Python unit tests for repo-relative sample argv, output-root
  relative aggregate display, and external-path fallback.
- Added CLI evidence display helper for repo-owned source paths.
- Applied that helper to Product Alpha demo `package_path`, native bundle
  `source_package_root`, and Docker compose source fixture path.
- Added Rust integration assertions for demo report path portability, native
  provenance path portability, and external package fallback preservation.
- Ran path-focused and full Product Alpha release checks and scanned generated
  output for repo-root absolute source paths.
- Updated Product Alpha roadmap memory and current snapshot dashboards.

## Files changed

- `scripts/product_alpha1_release_check.py`
- `scripts/tests/test_product_alpha1_release_check.py`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2171-product-alpha1-release-path-portability.md`

## Commands run

- `python3 -m unittest scripts.tests.test_product_alpha1_release_check`
- `cargo test -p mirrorea-cli --test alpha_cli demo_skip_docker_runs_local_probe_without_release_candidate_claim -- --nocapture`
- `cargo test -p mirrorea-cli --test alpha_cli build_native_bundle_emits_host_launch_bundle_without_native_package_execution -- --nocapture`
- `cargo test -p mirrorea-cli --test alpha_cli build_native_bundle_copies_only_declared_package_files -- --nocapture`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --skip-docker --out /tmp/mirrorea-alpha1-release-path-reviewfix-20260704120140`
- `rg -n '"[^"\n]*(/home/codex/dev/mir_poc_01|/Users/)' /tmp/mirrorea-alpha1-release-path-reviewfix-20260704120140`
- `rg -n 'source_package_root|package_path|docker_compose_file' /tmp/mirrorea-alpha1-release-path-reviewfix-20260704120140`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-path-full-202607041206`
- `rg -n '"[^"\n]*(/home/codex/dev/mir_poc_01|/Users/)' /tmp/mirrorea-alpha1-release-path-full-202607041206`
- `rg -n 'source_package_root|package_path|docker_compose_file' /tmp/mirrorea-alpha1-release-path-full-202607041206`
- `python3 -m py_compile scripts/product_alpha1_release_check.py scripts/tests/test_product_alpha1_release_check.py`
- `cargo fmt --check`
- `python3 -m unittest scripts.tests.test_product_alpha1_release_check.ProductAlpha1ReleaseCheckTests.test_check_all_serializes_release_owned_paths_without_host_prefixes`
- `cargo test -p mirrorea-cli docker_compose_evidence_reports_repo_relative_compose_file -- --nocapture`
- `python3 -m unittest discover -s scripts/tests`
- `cargo test -p mirrorea-cli -- --nocapture`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-path-final-reviewfix-202607041203`
- `rg -n '"[^"\n]*(/home/codex/dev/mir_poc_01|/Users/)' /tmp/mirrorea-alpha1-release-path-final-reviewfix-202607041203`
- `rg -n 'source_package_root|package_path|docker_compose_file' /tmp/mirrorea-alpha1-release-path-final-reviewfix-202607041203`

## Evidence / outputs / test results

- Python release-check tests first failed as expected on absolute sample argv and
  missing helper, then passed after implementation: 8 tests OK.
- Rust demo test first failed as expected because `package_path` was
  `/home/codex/dev/mir_poc_01/samples/product-alpha1/demo`; the targeted demo,
  native bundle, and external package fallback tests then passed.
- Skip-Docker Product Alpha release-check returned `partial` by design with all
  planned non-Docker commands passing.
- Full Product Alpha release-check with Docker returned `accepted`, 29/29
  planned commands passed, and `failed_commands` was empty.
- Generated output scans under both path-focused and full release-check output
  roots found no repo-root absolute source paths.
- Reviewer follow-up added embedded-path scrubbing for release-check `stderr`;
  regression coverage now checks `stderr` text containing both `REPO_ROOT` and
  the release output root.
- Reviewer follow-up added a Docker-independent Rust unit test that verifies
  `apply_docker_compose_evidence` reports the repo-owned compose fixture as
  `samples/product-alpha1/docker/docker-compose.product-alpha1.yml`.
- Representative generated fields after the fix:
  - `package_path`: `samples/product-alpha1/demo`
  - `source_package_root`: `samples/product-alpha1/demo`
  - `docker_compose_file`: `samples/product-alpha1/docker/docker-compose.product-alpha1.yml`
- Final focused validation after reviewer follow-up:
  - docs validation found 1323 numbered reports and no scaffold errors.
  - source hierarchy required/present count remained 602/602.
  - `python3 -m unittest discover -s scripts/tests` passed 673 tests.
  - `cargo test -p mirrorea-cli -- --nocapture` passed the binary unit test,
    20 `alpha_cli` tests, 10 `full_system_v1_cli` tests, and 6
    `surface_mir_cli` tests.
  - Product Alpha full release-check with Docker passed 29/29 commands.
  - Final generated output scan found no
    `/home/codex/dev/mir_poc_01` or `/Users/` source path strings.

## What changed in understanding

The remaining Product Alpha host-specific path leak was not in the compatibility
semantics. It was a display/provenance issue split between the Python
release-check wrapper and a few Rust CLI evidence fields. The correct fix is to
normalize repo-owned source inputs for display/provenance while leaving runtime
artifact paths operational.

## Open questions

- Whether future public bundles should also hide all host-local output/session
  paths inside every generated runtime artifact remains a separate product
  policy question. This package only removed repo-root source path leakage.

## Suggested next prompt

Continue autonomous maintenance from the current task map, with priority on the
next narrow G1/canon-aligned package unless the user promotes a broader Product
Alpha distribution or public packaging decision.

## Plan update status

`plan/` 更新済み:

- `plan/50-product-alpha1-public-boundary-roadmap.md` records the Product Alpha
  release-check/generated evidence path-portability hardening as post-P-A1-31
  maintenance, without changing product status or final claims.

## Documentation.md update status

`Documentation.md` 更新不要:

- The root reader-facing structure and high-level command index did not change.

## progress.md update status

`progress.md` 更新済み:

- Added the Product Alpha path-portability maintenance note and recent log entry.

## tasks.md update status

`tasks.md` 更新済み:

- Added the same current holding-state maintenance note.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Updated the Product Alpha row and recent validation log to include repo-relative
  sample argv and generated source evidence.

## Reviewer findings and follow-up

Reviewer sub-agent found two issues:

- `scripts/product_alpha1_release_check.py` normalized strings that were paths
  but did not scrub embedded repo/output-root paths inside `stderr`.
  Follow-up: added `release_display_text`, used it from `release_display_value`,
  and extended the release-check regression to include embedded `REPO_ROOT` and
  output-root paths in `stderr`.
- `crates/mirrorea-cli/src/main.rs` changed Docker compose source path display
  but had no deterministic test for that field.
  Follow-up: added `docker_compose_evidence_reports_repo_relative_compose_file`
  as a Docker-independent unit test for `apply_docker_compose_evidence`.

## Skipped validations and reasons

None. Docker-backed Product Alpha release-check was run and accepted.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Reviewer sub-agent `019f2b12-e54b-7bf0-90ab-1103363cca4b` completed and reported
the two findings above. Follow-up was implemented locally.
