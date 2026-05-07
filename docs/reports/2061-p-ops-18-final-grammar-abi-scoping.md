# Report 2061 — P-OPS-18 final grammar / ABI scoping

- Date: 2026-05-07 09:57 JST
- Author / agent: Codex
- Scope: product alpha front-door hardening scope, installed-binary helper compatibility surface, roadmap/snapshot queue advancement
- Decision levels touched: `L1`/`L2` product alpha public-boundary wording in `specs/25`; no new runtime semantics

## Objective

Close `P-OPS-18` by narrowing the current product alpha hardening target to a concrete front-door set, namely **versioned `package.mir.json` + documented `mirrorea-alpha` command family + native host launch bundle replay surface**, while keeping final textual grammar, Rust library ABI, viewer/devtools bundle ABI, hosted service, WAN, and distributed durable save/load explicitly outside the current hardening scope.

## Scope and assumptions

- Scope includes:
  - `specs/25-product-alpha1-public-boundary.md`
  - `scripts/product_alpha1_installed_binary_check.py` compatibility-scope output
  - product alpha guide / summary / root snapshot docs
  - `plan/50..52`, `progress.md`, `tasks.md`, `samples_progress.md`
- Scope excludes:
  - any Rust CLI/runtime/schema behavior change
  - any final grammar freeze
  - any final ABI freeze
  - any shipped installer / package-manager / hosted-service implementation
- Assumptions:
  - `P-OPS-17` already actualized the built-binary + host-bundle probe
  - the immediate need is to define what that probe is actually hardening before reopening further shipped-surface work

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean immediately after `P-OPS-17` commit `dff7363d`
- Existing current status at start:
  - installed-binary adoption probe was already validated and pushed
  - next promoted reopen point was `final grammar / ABI scoping`
  - the remaining ambiguity was which exact surfaces count as the current hardening target versus still-non-final ABI/grammar areas

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

## Actions taken

- Added a normative scoping section to `specs/25`:
  - current hardening target:
    versioned `package.mir.json`
    documented `mirrorea-alpha` command family
    native host launch bundle replay surface
  - explicit out-of-scope surfaces:
    final textual `.mir`
    final Rust library ABI
    final viewer/devtools bundle ABI
    hosted service / WAN ABI
- Extended `scripts/product_alpha1_installed_binary_check.py` so the probe now reports a machine-readable `compatibility_scope` object.
- Updated its unit test to assert the new compatibility-scope fields.
- Synchronized product docs / root snapshot docs with the new reading:
  - `README.md`
  - `Documentation.md`
  - `docs/hands_on/product_alpha1_01.md`
  - `docs/research_abstract/product_alpha1_01.md`
  - `scripts/README.md`
- Advanced roadmap/snapshot queue wording:
  - `plan/50` now records `P-OPS-18` and moves the next promoted line to `shipped-surface hardening`
  - `plan/51` and `plan/52` now treat grammar / ABI scoping as closed
  - `progress.md`, `tasks.md`, and `samples_progress.md` now move latest closeout / next gap / recent log to `P-OPS-18`

## Files changed

- Normative boundary:
  - `specs/25-product-alpha1-public-boundary.md`
- Helper / tests:
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
- Product docs / root snapshot:
  - `README.md`
  - `Documentation.md`
  - `docs/hands_on/product_alpha1_01.md`
  - `docs/research_abstract/product_alpha1_01.md`
  - `scripts/README.md`
- Roadmap / snapshot:
  - `plan/50-product-alpha1-public-boundary-roadmap.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report:
  - `docs/reports/2061-p-ops-18-final-grammar-abi-scoping.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
rg -n "grammar|ABI|package.mir.json|textual \\.mir|CLI/API|front-door|compatibility|native host launch bundle" specs/25-product-alpha1-public-boundary.md plan/50-product-alpha1-public-boundary-roadmap.md README.md Documentation.md progress.md tasks.md
sed -n '1,260p' specs/25-product-alpha1-public-boundary.md
sed -n '420,520p' plan/50-product-alpha1-public-boundary-roadmap.md
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-XXXXXX
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-AV9rYg
```

## Evidence / outputs / test results

- Unit/docs floor passed:
  - `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check`
    - 18 tests passed
  - `python3 -m unittest scripts.tests.test_validate_docs`
    - 13 tests passed
  - `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
    - 5 tests passed
  - `python3 scripts/check_source_hierarchy.py`
    - `required = 155`
    - `present = 155`
    - `missing = 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1213 numbered report(s).`
  - `cargo fmt --check`
    - passed
  - `git diff --check`
    - passed
- Installed-binary probe passed after helper widening:
  - `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-AV9rYg`
  - `status = accepted`
  - `failed_commands = []`
  - `installed_binary_candidate_ready = true`
  - `public_packaging_candidate = "installed_binary_plus_native_host_launch_bundle"`
  - `compatibility_scope.package_format = "versioned_package_mir_json"`
  - `compatibility_scope.cli_surface = "mirrorea_alpha_documented_command_family"`
  - `compatibility_scope.bundle_surface = "native_host_launch_bundle_run_sh"`
  - `compatibility_scope.final_textual_mir_grammar_frozen = false`
  - `compatibility_scope.final_rust_library_abi_frozen = false`
  - `compatibility_scope.final_viewer_bundle_api_frozen = false`

## What changed in understanding

- The current public-ish product front door is narrow enough to describe explicitly without claiming final public grammar or ABI.
- That front door is not “all product JSON / all CLI / all viewer data.” It is specifically the versioned package format, the documented command family, and the host-bundle replay unit needed by the installed-binary probe.
- Once that distinction is explicit, the next self-driven question is not grammar/ABI scoping again. It is how much of that already-narrowed unit should be treated as the user-facing shipped surface.

## Open questions

- How much of the current built-binary + host-bundle unit should be hardened as the shipped surface before any hosted-service reopening?
- Should future shipped-surface work treat the bundle manifest and `run.sh` as required public-ish adoption artifacts, or only as repo-local bridge evidence?
- If package-schema evolution and CLI-compatibility promises diverge, which one should be allowed to change faster during alpha?

## Suggested next prompt

`P-OPS-19 shipped-surface hardening を開き、current hardening target を versioned `package.mir.json`、documented `mirrorea-alpha` command family、native host launch bundle replay surface に保ったまま、built-binary + host-bundle unit のどこまでを user-facing shipped surface として扱うかを docs / roadmap / snapshot / report まで含めて整理してください。`

## Plan update status

`plan/` 更新済み: `plan/50-product-alpha1-public-boundary-roadmap.md`, `plan/51-operational-product-sample-roadmap.md`, `plan/52-portal-spatial-world-roadmap.md` を `P-OPS-18` closeout と `shipped-surface hardening` next queue に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: `P-OPS-18` の front-door hardening scope を current snapshot paragraph に追加した。

## progress.md update status

`progress.md` 更新済み: latest closeout package を `P-OPS-18` に進め、current promoted reopen point / blockers / recent log を `shipped-surface hardening` へ同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-18` を current task-level status に追加し、ordered self-driven packages と current recommendation を `shipped-surface hardening` 先頭へ進めた。

## samples_progress.md update status

`samples_progress.md` 更新済み: product alpha row と operational suite row の missing actualization を `shipped-surface hardening` に進め、compatibility-scope reading と recent validation log を追加した。

## Reviewer findings and follow-up

- No new sub-agent reviewer was started in this package.
- Local focused diff inspection was used because:
  - the package changed one Python helper/test pair plus spec/docs/roadmap wording
  - the immediately preceding reviewer attempt in `P-OPS-17` timed out twice
  - the key risk in this package was overclaim / wording drift, not runtime semantic regression

## Skipped validations and reasons

- `python3 scripts/product_alpha1_release_check.py --format json check-all --out ...` was not re-run in this package because no Rust CLI/runtime/schema behavior changed, and `P-OPS-17` had already rerun the full release-check floor immediately before this narrowing package. The changed helper (`product_alpha1_installed_binary_check.py`) was rerun directly after the compatibility-scope widening.
- `python3 scripts/operational_product_samples.py check-all --format json` was not re-run because this package did not change operational runtime/schema/helper behavior.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- No new sub-agent session was opened in this package.
