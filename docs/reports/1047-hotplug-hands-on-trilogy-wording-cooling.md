# 1047 - Hot-plug hands-on trilogy wording cooling

## Objective

`docs/hands_on/` の post-`P20` / post-`P21` hot-plug landing pages から、live queue / recommendation-order wording を取り除き、closed historical trilogy と current snapshot authority を明確に分ける。

## Scope and assumptions

- Scope は hot-plug hands-on reader-facing wording の maintenance に限定する。
- Normative specs、plan semantics、sample/script taxonomy、Rust implementation は変更しない。
- `P21` runtime-private engine-state floor を final public ABI、completed rollback / durable migration、distributed activation ordering protocol、production transport、actual `U1` commitment と混同しない。
- Current live status / next reopen authority は `progress.md` と `tasks.md` に残す。

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
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/37-post-p21-distributed-activation-ordering-family.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- `docs/hands_on/post_p20_hotplug_next_package_inventory_01.md`
- `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/post_p21_rollback_durable_migration_family_01.md`
- `docs/hands_on/post_p21_distributed_activation_ordering_family_01.md`
- `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`

## Actions taken

- `post_p20_hotplug_next_package_inventory_01.md` を historical `R7` closeout memory、`P20` thin assembly、`P21` runtime-private engine-state floor、post-`P21` family pointers の boundary page として書き直した。
- `hotplug_real_migration_rollback_boundary_01.md` の `current reading` / ranked next-relation wording を repository-memory reading と snapshot pointer に冷やした。
- `post_p21_rollback_durable_migration_family_01.md` の `first recommendation closeout memory` wording を closed post-`P21` trilogy の historical first boundary family wording に置き換えた。
- `post_p21_distributed_activation_ordering_family_01.md` の `second recommendation` / preceding-family / third-family wording を historical second boundary family と separately documented third boundary family wording に置き換えた。
- `post_p21_final_public_hotplug_abi_family_01.md` の `third recommendation` wording を closed trilogy の last historical boundary family wording に置き換え、live status / next reopen point は snapshot docs を参照するようにした。
- `progress.md` の last updated と recent log を、この docs-only maintenance package に合わせて更新した。

## Files changed

- `docs/hands_on/post_p20_hotplug_next_package_inventory_01.md`
- `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/post_p21_rollback_durable_migration_family_01.md`
- `docs/hands_on/post_p21_distributed_activation_ordering_family_01.md`
- `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`
- `progress.md`
- `docs/reports/1047-hotplug-hands-on-trilogy-wording-cooling.md`

## Evidence / outputs / test results

- `git status --short` at package start was clean after commit `be62f27`.
- `git branch --show-current` -> `main`
- `git log -1 --oneline` -> `be62f27 Cool hands-on landing hierarchy`
- `python3 scripts/check_source_hierarchy.py` -> pass; required 23 / present 23 / missing 0.
- `python3 scripts/validate_docs.py` -> pass after report creation; documentation scaffold complete, 1045 numbered reports found.
- `git diff --check` -> pass.
- Targeted wording check:
  `rg -n "current closeout line|current reading|completed-engine|canonical runtime-side|remaining open gate|current repository memory|current repo state|exact next label|exact package label|first recommendation|second recommendation|third recommendation|current close 済み|current line" docs/hands_on/*hotplug* docs/hands_on/post_p20_hotplug_next_package_inventory_01.md docs/hands_on/post_p21_*.md`
  returned no matches after the final patch.

## What changed in understanding

- The post-`P21` trilogy can remain useful as hands-on memory only if the ordered family wording is explicitly historical.
- `progress.md` / `tasks.md` should remain the authority for live status and reopen point; hands-on pages should not restate queue ownership.
- `P21` wording should emphasize runtime-private engine-state report rather than `completed-engine` phrases that can be misread as broad hot-plug completion.

## Open questions

- Actual `U1` commitment remains open and user-facing.
- A later audit can inspect research-abstract hot-plug pages for the same “historical trilogy vs live queue” temperature, but this package did not change them.

## Suggested next prompt

Continue autonomous maintenance with a research-abstract hot-plug trilogy wording audit, then run docs focused validation and commit/push.

## Plan update status

`plan/` 更新不要。No long-lived repository-memory interpretation changed; this package only cooled reader-facing hands-on wording.

## progress.md update status

`progress.md` 更新済み。Last updated and recent log now record the hot-plug hands-on trilogy wording cooling.

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

Docs researcher sub-agent `019de0c9-d312-7552-80de-60472f3f878f` audited the hot-plug hands-on pages, returned residual wording candidates, and was closed after incorporation.
