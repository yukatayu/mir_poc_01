# 1048 - Research abstract hot-plug trilogy wording cooling

## Objective

`docs/research_abstract/` の hot-plug post-`P20` / post-`P21` summary pages から live queue / recommendation-order wording を冷やし、closed historical trilogy と current snapshot authority を分ける。

## Scope and assumptions

- Scope は research abstract reader-facing wording の maintenance に限定する。
- Normative specs、plan semantics、sample/script taxonomy、Rust implementation は変更しない。
- `P21` runtime-private engine-state floor を completed engine、rollback / migration completion、distributed activation ordering completion、final public hot-plug ABI freeze と混同しない。
- Live status / next reopen authority は `progress.md` と `tasks.md` に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/05-mirrorea-fabric.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/37-post-p21-distributed-activation-ordering-family.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- `docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/research_abstract/post_p21_rollback_durable_migration_family_01.md`
- `docs/research_abstract/post_p21_distributed_activation_ordering_family_01.md`
- `docs/research_abstract/post_p21_final_public_hotplug_abi_family_01.md`

## Actions taken

- `post_p20_hotplug_next_package_inventory_01.md` を `R7` historical closeout bridge として冷やし、`completed-engine` / `canonical runtime-side` / package-level reopen wording を runtime-private engine-state floor と snapshot pointer に置き換えた。
- `post_p21_rollback_durable_migration_family_01.md`、`post_p21_distributed_activation_ordering_family_01.md`、`post_p21_final_public_hotplug_abi_family_01.md` の `current summary` / `current reading` / first-second-third recommendation wording を closed post-`P21` trilogy の historical boundary family wording に置き換えた。
- `hotplug_real_migration_rollback_boundary_01.md` の current recommendation paragraph を repository-memory reading に冷やし、`R5..P21` を historical closeout chain、post-`P21` families を closed trilogy pointers として読めるようにした。
- `runtime_crate_hotplug_carrier_admission_cut_01.md` と `runtime_crate_hotplug_engine_ownership_cut_01.md` の residual current recommendation wording を close-time historical recommendation / repository-memory use に置き換えた。
- `progress.md` の last updated と recent log を、この docs-only maintenance package に合わせて更新した。

## Files changed

- `docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md`
- `docs/research_abstract/post_p21_rollback_durable_migration_family_01.md`
- `docs/research_abstract/post_p21_distributed_activation_ordering_family_01.md`
- `docs/research_abstract/post_p21_final_public_hotplug_abi_family_01.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `progress.md`
- `docs/reports/1048-research-abstract-hotplug-trilogy-wording-cooling.md`

## Evidence / outputs / test results

- `git status --short` at package start was clean after commit `265f6f0`.
- `git branch --show-current` -> `main`
- `git log -1 --oneline` -> `265f6f0 Cool hotplug hands-on trilogy wording`
- `python3 scripts/check_source_hierarchy.py` -> pass; required 23 / present 23 / missing 0.
- `python3 scripts/validate_docs.py` -> pass after report creation; documentation scaffold complete, 1046 numbered reports found.
- `git diff --check` -> pass.
- Targeted wording check:
  `rg -n "current queue ownership|current summary|current reading|completed-engine|canonical runtime-side|first recommendation|second recommendation|third recommendation|current repository memory|current repo state|current close 済み|exact next label|package-level reopen next|remaining lane|next narrow implementation line|current self-driven|current recommendation" docs/research_abstract/*hotplug* docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md docs/research_abstract/post_p21_*.md`
  returned no matches after the final patch.

## What changed in understanding

- Research abstracts need the same temperature discipline as hands-on pages: they may summarize historical closeout chains, but should not become live queue authorities.
- `first / second / third recommendation` wording is useful in `plan/` history, but reader-facing summaries should frame it as a closed historical trilogy unless the package is actively being promoted.
- `P21` should be described as a runtime-private engine-state floor, not as completed engine language.

## Open questions

- Actual `U1` commitment remains open and user-facing.
- A later maintenance package can audit non-hot-plug research abstract pages for the same current-summary temperature, but this package only addressed hot-plug pages.

## Suggested next prompt

Continue autonomous maintenance with a focused audit of non-hot-plug research abstract pages for `current summary` / live queue wording, preserving `progress.md` / `tasks.md` as the live status authority.

## Plan update status

`plan/` 更新不要。No long-lived repository-memory interpretation changed; this package only cooled reader-facing research abstract wording.

## progress.md update status

`progress.md` 更新済み。Last updated and recent log now record the research abstract hot-plug trilogy wording cooling.

## tasks.md update status

`tasks.md` 更新不要。Current task map remains maintenance lane plus actual `U1` commitment gate.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only reader-facing docs wording and `progress.md`.
- Cargo tests and sample closeouts were not rerun because no Rust code, sample source, or script behavior changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after staged diff validation.

## Sub-agent session close status

Docs researcher sub-agent `019de0ce-7f55-73e1-bb92-274303a37f73` audited the hot-plug research abstract pages, returned residual wording candidates, and was closed after incorporation.
