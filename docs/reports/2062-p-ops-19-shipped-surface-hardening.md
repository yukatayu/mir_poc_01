# Report 2062 — P-OPS-19 shipped-surface hardening

- Date: 2026-05-07 10:09 JST
- Author / agent: Codex
- Scope: product alpha shipped-surface boundary hardening, native bundle/user-facing artifact classification, helper/release-check/spec/docs/dashboard synchronization
- Decision levels touched: `L1`/`L2` wording in `specs/25`; no new runtime semantics

## Objective

Close `P-OPS-19` by defining the current user-facing shipped surface inside the already-narrowed product alpha front door, making that surface machine-readable in helper and bundle artifacts, and keeping evidence-only reports/admin-debug artifacts outside the compatibility promise.

## Scope and assumptions

- Scope includes:
  - `crates/mirrorea-cli` native bundle report / manifest / verification-report surface
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/product_alpha1_release_check.py`
  - `specs/25-product-alpha1-public-boundary.md`
  - product alpha guides / summaries / roadmap / snapshot docs
- Scope excludes:
  - new command-family implementation
  - hosted-service, WAN, distributed durable save/load, final packaging
  - final textual `.mir` grammar or final ABI freeze
- Assumptions:
  - `P-OPS-18` already narrowed the hardening target to versioned `package.mir.json`, documented `mirrorea-alpha`, and native host launch bundle replay
  - current built-binary adoption evidence should remain narrow rather than silently widening to final-public packaging

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean immediately after `P-OPS-18` commit `bd696b81`
- Existing current status at start:
  - installed-binary adoption probe and grammar / ABI scoping were already validated
  - current ambiguity was no longer front-door scope, but how much of the built-binary + host-bundle unit counted as the current shipped surface

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

- Added a machine-readable `shipped_surface` block to the product alpha native bundle surfaces:
  - `build-native-bundle` stdout report
  - native bundle `manifest.json`
  - native bundle `reports/verification-report.json`
- Narrowed that shipped surface to:
  - built-binary `check` / `build-native-bundle` / `demo`
  - bundle replay `run.sh check` / `run.sh view`
  - bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md`
  - observer-safe supporting artifacts `devtools/bundle.json`, `devtools/index.html`, `reports/verification-report.json`
- Kept other bundled reports and admin/debug local artifacts outside the compatibility promise.
- Extended `scripts/product_alpha1_installed_binary_check.py` to emit the same `shipped_surface` block and to validate it semantically against the bundle report.
- Extended `scripts/product_alpha1_release_check.py` unit-level semantic guard so `build-native-bundle` also carries the shipped-surface boundary.
- Updated `specs/25`, product hands-on / summary, root summaries, roadmap memory, and dashboards to distinguish:
  - current hardening target
  - current shipped surface
  - evidence-only artifacts

## Files changed

- Runtime / CLI bundle surface:
  - `crates/mirrorea-cli/src/main.rs`
  - `crates/mirrorea-cli/tests/alpha_cli.rs`
- Helper / tests:
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/product_alpha1_release_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_release_check.py`
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
  - `docs/reports/2062-p-ops-19-shipped-surface-hardening.md`

## Commands run

```bash
git status --short
rg -n "P-OPS-18|installed-binary|shipped-surface|compatibility_scope|run.sh|manifest.json" README.md Documentation.md docs/hands_on/product_alpha1_01.md docs/research_abstract/product_alpha1_01.md scripts/README.md samples/product-alpha1/README.md specs/25-product-alpha1-public-boundary.md plan/50-product-alpha1-public-boundary-roadmap.md plan/51-operational-product-sample-roadmap.md plan/52-portal-spatial-world-roadmap.md progress.md tasks.md samples_progress.md
sed -n '381,520p' crates/mirrorea-cli/tests/alpha_cli.rs
sed -n '120,320p' scripts/product_alpha1_installed_binary_check.py
sed -n '180,280p' scripts/product_alpha1_release_check.py
cargo test -p mirrorea-cli --test alpha_cli build_native_bundle_emits_host_launch_bundle_without_native_package_execution -- --nocapture
python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check
python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check
cargo fmt
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-installed-binary-check-XXXXXX)
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out "$tmpdir"
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$tmpdir"
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
git status --short
```

## Evidence / outputs / test results

- TDD red phase:
  - `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
    - failed with missing `shipped_surface`
  - `cargo test -p mirrorea-cli --test alpha_cli build_native_bundle_emits_host_launch_bundle_without_native_package_execution -- --nocapture`
    - failed because bundle report/manifest lacked `shipped_surface`
- Focused green phase:
  - `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
    - 5 tests passed
  - `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check`
    - 10 tests passed
  - `cargo test -p mirrorea-cli --test alpha_cli build_native_bundle_emits_host_launch_bundle_without_native_package_execution -- --nocapture`
    - 1 test passed
- Fresh unit/docs floor after report creation:
  - `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check`
    - 23 tests passed
  - `python3 scripts/check_source_hierarchy.py`
    - `required = 155`
    - `present = 155`
    - `missing = 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1214 numbered report(s).`
  - `cargo fmt --check`
    - passed after one local `cargo fmt` repair
  - `git diff --check`
    - passed
- Actual installed-binary adoption probe:
  - `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-7COHFY`
  - `status = accepted`
  - `failed_commands = []`
  - `installed_binary_candidate_ready = true`
  - `public_packaging_candidate = "installed_binary_plus_native_host_launch_bundle"`
  - `compatibility_scope.package_format = "versioned_package_mir_json"`
  - `compatibility_scope.cli_surface = "mirrorea_alpha_documented_command_family"`
  - `compatibility_scope.bundle_surface = "native_host_launch_bundle_run_sh"`
  - `shipped_surface.delivery_model = "installed_binary_plus_native_host_launch_bundle"`
  - `shipped_surface.supported_replay_commands = ["check", "view"]`
  - `shipped_surface.observer_safe_supporting_artifacts = ["devtools/bundle.json", "devtools/index.html", "reports/verification-report.json"]`
- Actual full release-check:
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-lwITpl`
  - `status = accepted`
  - `failed_commands = []`
  - `product_alpha1_release_candidate_ready = true`
  - `product_alpha1_ready = true`
  - `final_product_claimed = false`
  - `final_public_api_frozen = false`

## What changed in understanding

- The product alpha front door and the product alpha shipped surface must stay separate. The front door includes the documented alpha CLI family, but the current built-binary shipped surface is narrower.
- The safest current shipped unit is the alpha replay bundle, not “installed distribution” in a broader sense.
- `run.sh` replay is intentionally narrow. Treating bundle-local reports or admin/debug artifacts as compatibility promises would overclaim current alpha evidence.

## Open questions

- Should broader public distribution narrowing stay at “built binary + host bundle” only, or add an explicit archive/install shape later?
- If broader distribution is reopened, which extra artifact shape belongs in the shipped promise without freezing final public ABI too early?
- Should the eventual broader distribution remain repo-local alpha evidence, or become the first public-facing delivery unit after additional user decision?

## Suggested next prompt

`P-OPS-20 broader public distribution narrowing を開き、current hardening target と current shipped surface を versioned package.mir.json / documented mirrorea-alpha / native host launch bundle replay / alpha replay bundle unit に保ったまま、それより広い installed distribution shape を本当に開く必要があるかを docs / roadmap / dashboard / validation まで含めて整理してください。`

## Plan update status

`plan/` 更新済み: `plan/50-product-alpha1-public-boundary-roadmap.md`, `plan/51-operational-product-sample-roadmap.md`, `plan/52-portal-spatial-world-roadmap.md` を `P-OPS-19` closeout と `broader public distribution narrowing` next queue に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: `P-OPS-19` の shipped-surface hardening reading を current snapshot paragraph に追加した。

## progress.md update status

`progress.md` 更新済み: latest closeout package を `P-OPS-19` に進め、current promoted reopen point / blockers / recent log を `broader public distribution narrowing` へ同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-19` を current task-level status に追加し、ordered self-driven packages と current recommendation を `broader public distribution narrowing` 先頭へ進めた。

## samples_progress.md update status

`samples_progress.md` 更新済み: product row に `shipped_surface` evidence を追加し、recent validation log と next gap を `broader public distribution narrowing` / `broader room-chat revisit` へ同期した。

## Reviewer findings and follow-up

- Sidecar code-structure review confirmed that the smallest coherent insertion point was:
  - helper-side `compatibility_scope` vicinity in `scripts/product_alpha1_installed_binary_check.py`
  - bundle-local `native_bundle_manifest_payload` in `crates/mirrorea-cli/src/main.rs`
- Sidecar semantic review recommended:
  - keeping shipped surface narrower than the full CLI family
  - keeping replay limited to built-binary `check` / `build-native-bundle` / `demo` and bundle `run.sh check` / `run.sh view`
  - keeping bundled reports/session-store artifacts outside the shipped promise
- Follow-up applied:
  - machine-readable `shipped_surface` block added to helper and bundle surfaces
  - reader-facing docs synchronized to the narrowed alpha replay bundle reading

## Skipped validations and reasons

- `python3 scripts/operational_product_samples.py check-all --format json` was not re-run because `P-OPS-19` changed product-side bundle/helper/docs surfaces only and did not change operational suite runtime/schema/helper behavior.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- `Raman` completed read-only bundle-implementation inspection.
- `Banach` completed read-only semantic review and surfaced wording/validation risks.
