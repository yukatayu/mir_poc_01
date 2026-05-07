# Report 2060 — P-OPS-17 installed binary adoption probe first cut

- Date: 2026-05-07 09:40 JST
- Author / agent: Codex
- Scope: product alpha installed-binary/public-ish adoption probe, product guide/summary sync, roadmap/snapshot queue advancement
- Decision levels touched: `L2` helper packaging/adoption evidence and roadmap/snapshot sequencing only; no new `L0`/`L1` runtime semantics

## Objective

Close `P-OPS-17` by actualizing the current first public-ish adoption candidate as **built `mirrorea-alpha` binary + native host launch bundle**, adding a dedicated installed-binary probe, shifting the product alpha hands-on path to that built-binary reading, and advancing the next reopen point from packaging-target ambiguity to `final grammar / ABI scoping`.

## Scope and assumptions

- Scope includes:
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
  - product alpha hands-on / research docs
  - root snapshot / roadmap docs that still pointed at packaging-target scoping
- Scope excludes:
  - any new Rust runtime/schema behavior
  - any final public CLI/API/ABI freeze
  - any hosted-service / WAN / distributed durable save/load implementation
  - any new operational suite runtime widening
- Assumptions:
  - `P-A1-31` already provides the release-candidate workflow floor
  - `P-OPS-16` already narrowed the next public-ish question to packaging/adoption target scoping
  - the concrete next step is to prove the already-built CLI/bundle line directly rather than reopen another runtime surface first

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: dirty
- Start dirty files:
  - uncommitted `scripts/product_alpha1_installed_binary_check.py`
  - uncommitted `scripts/tests/test_product_alpha1_installed_binary_check.py`
- Existing current status at start:
  - `P-OPS-16` had already moved the next promoted line to packaging/adoption target scoping
  - product docs and roadmap/snapshot docs still needed to reflect the new built-binary adoption reading

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

- Kept the in-progress installed-binary helper/test pair and closed the package around them:
  - preserved `scripts/product_alpha1_installed_binary_check.py check-all`
  - preserved `scripts/tests/test_product_alpha1_installed_binary_check.py`
- Updated product alpha hands-on guidance:
  - added a built-binary step for `target/debug/mirrorea-alpha`
  - switched the main manual walkthrough from `cargo run -q -p mirrorea-cli -- ...` to direct built-binary invocation
  - added an explicit installed-binary probe section
- Updated research/root snapshot wording:
  - `docs/research_abstract/product_alpha1_01.md`
  - `README.md`
  - `Documentation.md`
  now distinguish between installed-binary + host-bundle evidence and still-unfixed final public grammar / ABI / packaging
- Advanced roadmap/snapshot queue wording:
  - `plan/50` now records `P-OPS-17` and points the next promoted line to `final grammar / ABI scoping`
  - `plan/51` and `plan/52` now treat packaging-target ambiguity as closed for the current line
  - `progress.md`, `tasks.md`, and `samples_progress.md` now move latest closeout / next gap / recent log to `P-OPS-17`
- Re-ran:
  - unit/docs validation floor
  - the installed-binary probe itself
  - the full product alpha release-check floor

## Files changed

- New helper/test:
  - `scripts/product_alpha1_installed_binary_check.py`
  - `scripts/tests/test_product_alpha1_installed_binary_check.py`
- Product guide / summary:
  - `docs/hands_on/product_alpha1_01.md`
  - `docs/research_abstract/product_alpha1_01.md`
  - `scripts/README.md`
- Root / roadmap / snapshot sync:
  - `README.md`
  - `Documentation.md`
  - `plan/50-product-alpha1-public-boundary-roadmap.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report:
  - `docs/reports/2060-p-ops-17-installed-binary-adoption-probe-first-cut.md`

## Commands run

```bash
git status --short
git diff --stat
date '+%Y-%m-%d %H:%M:%S %Z'
sed -n '1,260p' docs/hands_on/product_alpha1_01.md
sed -n '1,260p' docs/research_abstract/product_alpha1_01.md
sed -n '1,260p' scripts/README.md
sed -n '1,260p' plan/50-product-alpha1-public-boundary-roadmap.md
sed -n '1,260p' plan/51-operational-product-sample-roadmap.md
sed -n '1,260p' plan/52-portal-spatial-world-roadmap.md
sed -n '1,260p' progress.md
sed -n '1,220p' tasks.md
sed -n '1,220p' samples_progress.md
rg -n "installed binary|public packaging|final grammar / ABI scoping|P-OPS-17" README.md Documentation.md docs/hands_on/product_alpha1_01.md docs/research_abstract/product_alpha1_01.md scripts/README.md plan/50-product-alpha1-public-boundary-roadmap.md plan/51-operational-product-sample-roadmap.md plan/52-portal-spatial-world-roadmap.md progress.md tasks.md samples_progress.md
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-XXXXXX
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-recheck-XXXXXX
```

## Evidence / outputs / test results

- Unit/docs floor passed:
  - `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check`
    - 18 tests passed
  - `python3 scripts/check_source_hierarchy.py`
    - `required = 155`
    - `present = 155`
    - `missing = 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1211 numbered report(s).`
  - `cargo fmt --check`
    - passed
  - `git diff --check`
    - passed
- Installed-binary probe passed:
  - `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-qindRf`
  - `status = accepted`
  - `binary_path = /home/yukatayu/dev/mir_poc_01/target/debug/mirrorea-alpha`
  - `include_docker = true`
  - `failed_commands = []`
  - `installed_binary_candidate_ready = true`
  - `public_packaging_candidate = "installed_binary_plus_native_host_launch_bundle"`
  - `final_public_api_frozen = false`
- Full product release-check floor passed:
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-recheck-k3o0FB`
  - `status = accepted`
  - `include_docker = true`
  - `failed_commands = []`
  - `product_alpha1_release_candidate_ready = true`
  - `product_alpha1_ready = true`
  - `final_product_claimed = false`
  - `final_public_api_frozen = false`

## What changed in understanding

- The packaging/adoption target ambiguity is now materially narrower than it was in `P-OPS-16`, because the repo has concrete evidence for the built `mirrorea-alpha` binary and the generated native host launch bundle running together.
- That concrete evidence is enough to advance the next promoted line to grammar / ABI scoping without claiming final public packaging.
- The operational suite itself did not need a new runtime widening for this package; the necessary work was product-side guide/probe hardening and queue synchronization.

## Open questions

- In the next grammar / ABI package, should the first hardening focus be `package.mir.json` evolution, CLI command compatibility, or both in one narrow prompt?
- How much of the current controlled local/Docker host model remains inside the documented adoption unit before any hosted-service reopening?
- At what point should “installed binary + host launch bundle” stop being only a public-ish candidate and become a user-facing shipped surface claim, if ever?

## Suggested next prompt

`P-OPS-18 final grammar / ABI scoping を開き、already-actualized installed binary + native host launch bundle probe を前提に、alpha `package.mir.json` evolution、direct textual `.mir` non-goal wording、CLI/API compatibility stop line を docs / roadmap / snapshot / report まで含めて丁寧に整理してください。`

## Plan update status

`plan/` 更新済み: `plan/50-product-alpha1-public-boundary-roadmap.md`, `plan/51-operational-product-sample-roadmap.md`, `plan/52-portal-spatial-world-roadmap.md` を `P-OPS-17` と `final grammar / ABI scoping` current queue に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: `P-OPS-17` の installed-binary / host-bundle probe と current first public-ish adoption candidate reading を current snapshot paragraph に追加した。

## progress.md update status

`progress.md` 更新済み: latest closeout package を `P-OPS-17` に進め、current promoted reopen point / blockers / validation floor / recent log を `final grammar / ABI scoping` へ同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-17` を current task-level status に追加し、ordered self-driven packages と current recommendation を `final grammar / ABI scoping` 先頭へ並べ替えた。

## samples_progress.md update status

`samples_progress.md` 更新済み: product alpha row と operational suite row の missing actualization を `final grammar / ABI scoping` へ進め、installed-binary probe command と recent validation log を追加した。

## Reviewer findings and follow-up

- Reviewer agent `Euclid` was started for read-only diff review of the helper/test and docs/roadmap/snapshot sync.
- Two waits timed out without a final reviewer response.
- Fallback:
  - local focused diff inspection was used
  - no concrete correctness or overclaim issue was found beyond the already-fixed hands-on duplicate command drift discovered during local spot-checking

## Skipped validations and reasons

- `python3 scripts/operational_product_samples.py check-all --format json` was not re-run in this package because no operational runtime/schema/helper behavior changed; the package only synchronized the product-side installed-binary probe and the related docs/roadmap/snapshot reading.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- Reviewer agent `Euclid`: closed after timeout without findings (`previous_status = running`, final notification `status = shutdown`)
