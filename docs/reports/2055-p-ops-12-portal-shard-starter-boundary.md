# Report 2055 — P-OPS-12 portal/shard starter boundary

- Date: 2026-05-07 01:32 JST
- Author / agent: Codex
- Scope: portal/shard starter-boundary decision, authoring guide sync, roadmap/dashboard refresh
- Decision levels touched: `L1`/`L2` wording sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-12` by fixing the current authoring boundary for the operational portal/shard line: keep the validated starter catalog intentionally capped at `world-core` / `membership-chat` / `sugoroku-world`, direct portal/shard authoring to the active executable roots, and keep `future/` portal/shard inventory explicitly non-executable.

## Scope and assumptions

- Scope includes:
  - a new reader-facing portal/shard starter-boundary guide
  - a short research summary for the same decision
  - normative / roadmap / snapshot wording that distinguishes active portal/shard roots from `future/` inventory and from `template_only` starters
  - focused runtime evidence that the active `portal-worldlink/` and `two-shard-hard-boundary/` roots remain valid authoring study references
- Scope excludes:
  - any new `portal-worldlink-starter/` or `two-shard-hard-boundary-starter/`
  - any CLI/helper/runtime implementation change
  - any gradient observation runtime widening
  - any final public scaffold policy
- Assumptions:
  - current validated starter catalog remains intentionally limited to `world-core-starter/`, `membership-chat-starter/`, and `sugoroku-world-starter/`
  - portal/shard authoring today should start from active executable roots rather than from `future/` inventory
  - `future/portal-worldlink/`, `future/two-shard-hard-boundary/`, and `future/gradient-observation.profile.json` remain non-executable inventory

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean after `P-OPS-11` commit `8283c683` and push
- Existing authoring state at start:
  - template-only authoring starters already existed for `world-core`, `membership-chat`, and `sugoroku-world`
  - active runnable portal/shard roots already existed as `portal-worldlink/` and `two-shard-hard-boundary/`
  - `future/` portal/shard blueprint inventory and `gradient-observation.profile.json` already existed
  - roadmap/task memory promoted portal/shard starter decision as the next reopen point

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
- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/hands_on/operational_product_sample_01.md`
- `docs/hands_on/operational_package_authoring_01.md`
- `docs/research_abstract/operational_product_sample_01.md`
- `docs/research_abstract/operational_package_authoring_01.md`
- `samples/product-alpha1/operational/README.md`
- `samples/product-alpha1/operational/templates/README.md`
- `sub-agent-pro/operational-product-sample-001/07-portal-spatial-future.md`
- `sub-agent-pro/operational-product-sample-001/sample-blueprints/portal-and-spatial-blueprint.md`

## Actions taken

- Added a dedicated authoring-boundary guide:
  - created `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
  - created `docs/research_abstract/operational_portal_shard_starter_boundary_01.md`
- Fixed the current repository reading:
  - validated starter catalog intentionally stops at `SugorokuWorld`
  - portal/shard authoring today uses active executable roots as study/copy references
  - `future/` portal/shard files stay inventory-only and non-executable
  - any later portal/shard starter must be sourced from active roots rather than future blueprints
- Synced the decision across source hierarchy:
  - updated `specs/26` and `specs/27`
  - updated `plan/51` and `plan/52`
  - updated snapshot / dashboard / index docs so the reopen point moves from starter decision to broader room-chat lane widening
- Revalidated the active portal/shard study roots directly with `check` and `run-local`

## Files changed

- New docs:
  - `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
  - `docs/research_abstract/operational_portal_shard_starter_boundary_01.md`
- Updated authoring/product/operational docs:
  - `README.md`
  - `Documentation.md`
  - `docs/hands_on/README.md`
  - `docs/hands_on/operational_package_authoring_01.md`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/operational_package_authoring_01.md`
  - `docs/research_abstract/operational_product_sample_01.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `samples/product-alpha1/operational/templates/README.md`
  - `scripts/README.md`
- Updated normative/index/roadmap/snapshot files:
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
  - `docs/reports/2055-p-ops-12-portal-shard-starter-boundary.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,260p' plan/52-portal-spatial-world-roadmap.md
sed -n '1,260p' tasks.md
sed -n '1,260p' sub-agent-pro/operational-product-sample-001/07-portal-spatial-future.md
sed -n '1,260p' sub-agent-pro/operational-product-sample-001/sample-blueprints/portal-and-spatial-blueprint.md
find samples/product-alpha1/operational -maxdepth 2 -type f | sort | rg 'portal-worldlink|two-shard-hard-boundary|templates/|README.md$|package.mir.json$'
sed -n '1,260p' samples/product-alpha1/operational/templates/README.md
sed -n '1,260p' samples/product-alpha1/operational/portal-worldlink/README.md
sed -n '1,260p' samples/product-alpha1/operational/two-shard-hard-boundary/README.md
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json
tmpdir=$(mktemp -d /tmp/mirrorea-ops-portal-boundary-XXXXXX) && MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/portal-worldlink --format json
tmpdir=$(mktemp -d /tmp/mirrorea-ops-shard-boundary-XXXXXX) && MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-hard-boundary --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- `portal-worldlink` static check passed:
  - `verdict = accepted`
  - `package_kind = portal_worldlink`
  - `product_alpha1_ready = false`
- `two-shard-hard-boundary` static check passed:
  - `verdict = accepted`
  - `package_kind = two_shard_hard_boundary`
  - `product_alpha1_ready = false`
- `portal-worldlink` runtime evidence passed:
  - `surface_kind = product_alpha1_run_local_report`
  - `typed_host_io_claimed = false`
  - runtime plan still points to `declared_dependencies = ["../sugoroku-world"]`
  - bounded portal lanes still include `same_session_portal_resolve`, `same_session_portal_handoff`, `same_session_portal_admit`
- `two-shard-hard-boundary` runtime evidence passed:
  - `surface_kind = product_alpha1_run_local_report`
  - `typed_host_io_claimed = false`
  - runtime plan still points to `declared_dependencies = ["../portal-worldlink"]`
  - bounded shard rejects still include `OldOwnerWriteRejected`, `MissingHandoffWitness`, `StaleShardConfig`
- docs/source-hierarchy floor passed:
  - `python3 -m unittest scripts.tests.test_validate_docs`
  - `python3 scripts/check_source_hierarchy.py` with `required = 155`, `present = 155`, `missing = 0`
  - `python3 scripts/validate_docs.py` with `Found 1207 numbered report(s).`
  - `cargo fmt --check`
  - `git diff --check`

## What changed in understanding

- The right current authoring boundary is not “add more starter templates by default,” but “make the category split explicit”:
  - mainstream starter catalog
  - active executable portal/shard roots
  - future portal/shard inventory
- Portal/shard widening is already concrete enough to study through the active roots, but not yet worth duplicating into a separate starter family because that would blur active-vs-template-vs-blueprint roles.

## Open questions

- Should a later broader room-chat package come before any portal/shard starter revisit, or should portal/shard starter revisit stay permanently unnecessary?
- If gradient observation gains bounded runtime evidence later, should that be the first moment to reconsider shard-specific starter templates?

## Suggested next prompt

`P-OPS-13 broader room-chat lane widening を進め、MembershipChat の current EchoText lane を room-oriented ChatText へ広げるか、または narrow lane 維持を実装・検証・docs 同期まで含めて閉じてください。`

## Plan update status

`plan/` 更新済み: `plan/00-index.md`, `plan/51-operational-product-sample-roadmap.md`, `plan/52-portal-spatial-world-roadmap.md` に `P-OPS-12` boundary と next reopen order を同期した。

## Documentation.md update status

`Documentation.md` 更新済み: `P-OPS-12` portal/shard starter boundary と corresponding hands-on guide を current operational reading に追加した。

## progress.md update status

`progress.md` 更新済み: latest closeout package を `P-OPS-12` に進め、next reopen point を broader room-chat lane widening へ移した。

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-12` を actualized package として追加し、ordered self-driven packages を broader room-chat lane widening 起点へ並べ替えた。

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row と recent validation log に portal/shard starter boundary docs と next gap を同期した。

## Reviewer findings and follow-up

- Reviewer `Noether` (`019dfe22-eab4-7ad3-81f8-3cecf2fa0922`) was started for semantic review of the boundary wording but did not return within two wait windows and was shut down.
- Local focused review findings:
  - starter catalog remains explicitly capped at `world-core-starter/`, `membership-chat-starter/`, and `sugoroku-world-starter`
  - active `portal-worldlink/` and `two-shard-hard-boundary/` roots remain the current study/copy boundary
  - `future/portal-worldlink/`, `future/two-shard-hard-boundary/`, and `future/gradient-observation.profile.json` remain non-executable inventory
  - no helper/runtime command or package kind was added by this package
- Follow-up:
  - if a later portal/shard starter is ever introduced, keep it sourced from the active executable root and validate it separately from the `future/` inventory line

## Skipped validations and reasons

- no Rust/runtime/helper test target beyond direct `check` / `run-local` on the active portal/shard roots was rerun because `P-OPS-12` changed docs / roadmap / snapshot wording only and did not modify any runtime/helper implementation path
- `python3 scripts/operational_product_samples.py check-all --format json` was not rerun because helper semantics were unchanged; this package only fixed the authoring boundary around already-validated roots

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- Reviewer `Noether` (`019dfe22-eab4-7ad3-81f8-3cecf2fa0922`) timed out twice, returned no findings, and was shut down.
