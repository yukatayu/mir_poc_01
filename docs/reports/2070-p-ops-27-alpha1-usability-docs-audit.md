# Report 2070 — P-OPS-27 alpha-1 usability and snapshot-doc audit

- Date: 2026-05-07 13:08 JST
- Author / agent: Codex
- Scope: alpha-1 usability audit, operational command rerun, overview-doc concision / drift repair
- Decision levels touched: none; snapshot / roadmap memory only

## Objective

Check whether any alpha-1 work remains before practical use, rerun the product and operational workflows, review snapshot docs for drift / overlong package history / unclear wording, and compact overview documents without changing runtime behavior or normative specs.

## Scope and assumptions

- Scope includes:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/product-alpha1/README.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - this new report
- Scope excludes:
  - existing reports under `docs/reports/`
  - runtime behavior widening
  - new normative claims in `specs/`
  - final public grammar / ABI / SDK decisions
- Assumptions:
  - `P-OPS-26` correctly made broader distribution / final catalog breadth a user-spec-required gate
  - overview docs should summarize current capability and next gate, while package chronology remains in `docs/reports/` and `plan/`

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start commit: `02540921e72658718b4572dbe68b3095007c5502`
- Start worktree: clean
- Resource preflight:
  - `df -h .`: root filesystem 99G total, 81G used, 14G available, 86% used
  - `free -h`: 960Mi total memory, 312Mi available, 19Gi swap with 1.5Gi used
- Dirty state after edits:
  - `README.md`
  - `Documentation.md`
  - `mir_hilight.html`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/product-alpha1/README.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `docs/reports/2070-p-ops-27-alpha1-usability-docs-audit.md`

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/hands_on/product_alpha1_01.md`
- `docs/hands_on/operational_product_sample_01.md`
- `samples/product-alpha1/README.md`

## Actions taken

- Reran product alpha release candidate workflow with Docker included.
- Reran installed-binary + generated host launch bundle adoption probe.
- Reran canonical operational product sample suite check-all with Docker included.
- Ran full Python test discovery after snapshot-doc edits.
- Root-caused a Python test failure in `mir_hilight.html`: the static embedded active sample list missed the `samples/clean-near-end/avatar-follow/*.mir` family.
- Updated `mir_hilight.html` to include the five active `avatar-follow` samples. This changed only the single-file viewer inventory, not Mir semantics or runtime behavior.
- Reviewed overview docs for:
  - stale current status
  - overlong package chronology
  - hidden final-public claims
  - unclear next gate wording
  - missing validation anchors
- Rewrote `progress.md` as a compact current snapshot:
  - current position
  - practical usability reading
  - workflow axes
  - line snapshot
  - subsystem status
  - macro phase map
  - blockers
  - validation floor
  - compressed recent log
- Rewrote `tasks.md` as a compact task map centered on the current user-spec gate.
- Rewrote `samples_progress.md` as a dashboard with shorter product / operational rows and current validation anchors.
- Compressed `Documentation.md`, root `README.md`, and `samples/product-alpha1/README.md` so they point to current capability and non-claims instead of repeating full package chronology.
- Added `P-OPS-27 current scope` to `plan/51`.

## Files changed

- `README.md`
- `Documentation.md`
- `mir_hilight.html`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/product-alpha1/README.md`
- `plan/51-operational-product-sample-roadmap.md`
- `docs/reports/2070-p-ops-27-alpha1-usability-docs-audit.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
df -h .
free -h
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-audit
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-audit
python3 scripts/operational_product_samples.py check-all --format json
python3 -m unittest discover scripts.tests
python3 -m unittest scripts.tests.test_mir_hilight_html
python3 -m unittest discover -s scripts/tests
cargo test --workspace -- --nocapture
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-audit-final
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-audit-final
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M JST'
wc -l README.md Documentation.md progress.md tasks.md samples_progress.md samples/product-alpha1/README.md
```

Final validation commands after this report is added are listed in the evidence section.

## Evidence / outputs / test results

- Initial focused Python tests:
  - `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check scripts.tests.test_operational_product_samples`
  - 52 tests passed
- Initial docs / formatting checks:
  - `python3 scripts/check_source_hierarchy.py`: `required = 155`, `present = 155`, `missing = 0`
  - `python3 scripts/validate_docs.py`: `Documentation scaffold looks complete.`, `Found 1221 numbered report(s).`
  - `cargo fmt --check`: passed
  - `git diff --check`: passed
- Product release check:
  - `status = accepted`
  - `product_alpha1_release_candidate_ready = true`
  - `include_docker = true`
  - `failed_commands = []`
  - passed commands included validation floor, focused Cargo tests, check, run-local, session, five attach rows, save, load, quiescent-save, local transport, Docker transport, export-devtools, view, build-native-bundle, bundle `run.sh check`, bundle `run.sh view`, and demo
- Installed-binary probe:
  - `status = accepted`
  - `installed_binary_candidate_ready = true`
  - `public_packaging_candidate = installed_binary_plus_native_host_launch_bundle`
  - `include_docker = true`
  - `failed_commands = []`
  - `distribution_scope.current_delivery_unit = developer_built_binary_plus_generated_host_launch_bundle`
  - archive / installer / system package / auto-update / hosted service flags remained `false`
- Operational product sample suite:
  - `status = accepted`
  - `docker_included = true`
  - `failed_commands = []`
  - `release_check.status = accepted`
  - `attach_matrix_complete = true`
  - `membership_chat_chat_text_ok = true`
  - `projection_inventory_ok = true`
  - `portal_runtime_ok = true`
  - `shard_runtime_ok = true`
  - `gradient_runtime_ok = true`
  - `widening_queue_scope.next_promoted_reopen_requires_user_decision = true`
  - `user_final_decision_scope.self_driven_operational_reopenings_exhausted = true`
- Documentation concision:
  - `progress.md`: 263 lines -> 127 lines
  - `tasks.md`: 147 lines -> 75 lines
  - `samples_progress.md`: 203 lines -> 96 lines
  - `samples/product-alpha1/README.md`: 75 lines -> 44 lines

Final-tree validation after adding this report:

- `python3 -m unittest discover scripts.tests` failed due invocation shape, not repository behavior:
  - failure: `TypeError: expected str, bytes or os.PathLike object, not NoneType`
  - corrected command: `python3 -m unittest discover -s scripts/tests`
- `python3 -m unittest scripts.tests.test_mir_hilight_html`: 5 tests passed after the viewer inventory fix
- active clean-near-end viewer inventory probe:
  - missing embedded `.mir` paths: 0
  - embedded sample count: 32
  - embedded families include `avatar-follow`
- `python3 -m unittest discover -s scripts/tests`: 465 tests passed
- `python3 scripts/check_source_hierarchy.py`: `required = 155`, `present = 155`, `missing = 0`
- `python3 scripts/validate_docs.py`: `Documentation scaffold looks complete.`, `Found 1222 numbered report(s).`
- `cargo fmt --check`: passed
- `cargo test --workspace -- --nocapture`: passed
  - note: `mir-ast` emitted two pre-existing dead-code warnings in `practical_alpha1_checker_support.rs`
- `git diff --check`: passed
- final product release check:
  - `status = accepted`
  - `product_alpha1_release_candidate_ready = true`
  - `product_alpha1_ready = true`
  - `include_docker = true`
  - `failed_commands = []`
- final installed-binary probe:
  - `status = accepted`
  - `installed_binary_candidate_ready = true`
  - `public_packaging_candidate = installed_binary_plus_native_host_launch_bundle`
  - `include_docker = true`
  - `failed_commands = []`
  - `distribution_scope.current_delivery_unit = developer_built_binary_plus_generated_host_launch_bundle`
- final operational suite check:
  - `status = accepted`
  - `docker_included = true`
  - `failed_commands = []`
  - nested `release_check.status = accepted`
  - `attach_matrix_complete = true`
  - `membership_chat_chat_text_ok = true`
  - `projection_inventory_ok = true`
  - `portal_runtime_ok = true`
  - `shard_runtime_ok = true`
  - `gradient_runtime_ok = true`
  - top-level `product_alpha1_ready = false` and `final_public_api_frozen = false`, preserving the intentional non-claim that the operational suite is not final public product completion

## What changed in understanding

- Alpha-1 is practically usable for controlled external-developer reproduction through the documented product and operational workflows.
- It is not final public product completion. The remaining meaningful gate is not another self-driven implementation package; it is the user decision about broader distribution / shipped surface and final shared-space catalog breadth.
- The biggest documentation issue was not incorrect semantics, but overview-document weight: several snapshot files repeated package chronology that belongs in reports / plan, making the current gate harder to see.
- Full Python discovery also found one real dashboard drift: the single-file syntax viewer did not embed active `avatar-follow` samples. The fix confirms the active clean-near-end viewer now covers all `.mir` samples again.

## Open questions

- Should alpha-1 remain developer-built binary + generated host launch bundle only?
- Should the final shared-space operational catalog remain the bounded narrow showcase, or broaden toward a wider product catalog?
- If the user chooses broader distribution, which delivery shape should be considered first: archive, installer, system package, auto-update channel, or hosted service?

## Suggested next prompt

`U1_beyond_alpha_packaging_host_target_shipped_surface と final_shared_space_operational_catalog_breadth について、alpha-1 の current narrow showcase を維持するのか、archive / installer / hosted-service / broader final catalog へ広げるのかを decision-level つきで指定してください。`

## Plan update status

`plan/` 更新済み: `plan/51-operational-product-sample-roadmap.md` に `P-OPS-27 current scope` と snapshot-doc audit closeoutを追加した。`plan/52` は portal/shard roadmap の規範・順序が変わらないため更新不要。

## Documentation.md update status

`Documentation.md` 更新済み: package chronology paragraph を current runnable workflow / usability / non-claim / next gate summary へ圧縮した。

## progress.md update status

`progress.md` 更新済み: latest closeout package、current practical usability、self-driven queue exhausted reading、validation floor、recent logを `P-OPS-27` に同期した。

## tasks.md update status

`tasks.md` 更新済み: current task-level status を compact task map に rewrite し、ordered self-driven package を `no active self-driven package` に維持した。

## samples_progress.md update status

`samples_progress.md` 更新済み: product / operational dashboard rows、validation anchors、recent validation logを compressed current-state reading に同期した。

## Reviewer findings and follow-up

- No sub-agent reviewer was spawned because the active tool policy only permits sub-agent spawning when the user explicitly asks for delegation.
- Local focused review findings:
  - `mir_hilight.html` missed the active `avatar-follow` sample family. Fixed by adding the five active source entries to the embedded JSON.
  - `progress.md` current reading repeated package history and obscured the next gate. Fixed by replacing it with a compact current position and line snapshot.
  - `tasks.md` listed many completed packages as task-level status, making the active task map harder to read. Fixed by replacing it with current capability rows and user-decision items.
  - `samples_progress.md` had long evidence cells that made the dashboard hard to scan. Fixed by splitting product root status, validation anchors, and compact workflow rows.
  - `Documentation.md`, `README.md`, and `samples/product-alpha1/README.md` repeated long chronology in overview positions. Fixed with current-scope summaries.

## Skipped validations and reasons

- Existing historical reports under `docs/reports/` were not edited, per user instruction.
- No requested alpha-1 validation was intentionally skipped. The malformed `python3 -m unittest discover scripts.tests` command was corrected to `python3 -m unittest discover -s scripts/tests`.

## Commit / push status

- Commit: pending
- Push: pending

## Sub-agent session close status

- No sub-agent session was opened for this package.
