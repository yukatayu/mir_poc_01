# 1046 - Hands-on landing hierarchy cooling

## Objective

`docs/hands_on/` の landing hierarchy を、live queue / package-status narration から command入口・boundary summary・repository-memory pointer に冷やす。

## Scope and assumptions

- Scope は reader-facing hands-on docs の wording maintenance に限定する。
- Normative semantics、sample taxonomy、validation script、Rust implementation は変更しない。
- `U1` actual commitment、final public parser/API/ABI、rollback protocol、durable migration engine、distributed activation ordering、production transport、final viewer/API completion は claim しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`

## Actions taken

- `docs/hands_on/README.md` の `まず読む` を table 化し、各 child page の role を command/use-case pointer と stop-line qualifier に寄せた。
- `docs/hands_on/current_phase_closeout_01.md` の long package ledger を、current runnable floor / evidence carriers / stop lines / storage guardrail / report-plan pointer の compact summary に圧縮した。
- `docs/hands_on/current_phase_closeout_01.md` の deferred gate 見出しと next reopen point wording を、live queue ownership ではなく `progress.md` / `tasks.md` snapshot pointer として書き直した。
- `docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md` の `current queue` / `current hot-plug engine owner` wording を request / verdict carrier boundary wording に置き換えた。
- `progress.md` の last updated と recent log を、この docs-focused package の closeout に合わせて更新した。
- `tasks.md` と `samples_progress.md` は current status と照合し、今回の docs-only maintenance では更新不要と判断した。

## Files changed

- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `progress.md`
- `docs/reports/1046-hands-on-landing-hierarchy-cooling.md`

## Evidence / outputs / test results

- `git status --short` before package edits showed only the in-scope hands-on docs changes from this package.
- `git branch --show-current` -> `main`
- `git log -1 --oneline` -> `25e943b Cool research abstract index`
- `python3 scripts/check_source_hierarchy.py` -> pass; required 23 / present 23 / missing 0.
- `python3 scripts/validate_docs.py` -> pass after report creation; documentation scaffold complete, 1044 numbered reports found.
- `git diff --check` -> pass.
- `rg -n "current queue|current hot-plug engine owner|remaining mixed gate|actual next reopen point|package sequence .*recut" docs/hands_on/README.md docs/hands_on/current_phase_closeout_01.md docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md` -> no matches after the final cooling patch.

## What changed in understanding

- `docs/hands_on/` should stay closer to an execution guide than a current task ledger.
- Detailed package history belongs in `docs/reports/` and long-lived interpretation belongs in `plan/`; `docs/hands_on/current_phase_closeout_01.md` should point to those sources instead of replaying the ledger.
- Hot-plug landing pages remain useful when they emphasize boundary confirmation and stop lines rather than historical queue sequencing.

## Open questions

- `docs/hands_on/post_p20_hotplug_next_package_inventory_01.md` still has some process-temperature wording and should be audited in a later maintenance package.
- Actual `U1` commitment remains open and user-facing.

## Suggested next prompt

Continue autonomous maintenance with a focused audit of `docs/hands_on/post_p20_hotplug_next_package_inventory_01.md` and adjacent hot-plug hands-on pages, preserving boundary wording and avoiding public ABI freeze claims.

## Plan update status

`plan/` 更新不要。No roadmap, semantic boundary, or long-lived repository-memory interpretation changed.

## progress.md update status

`progress.md` 更新済み。Last updated and recent log now record the hands-on landing hierarchy cooling.

## tasks.md update status

`tasks.md` 更新不要。Current task map remains maintenance lane plus `U1` actual commitment gate.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only reader-facing docs wording and `progress.md`; focused docs floor is the appropriate validation.
- Cargo tests and sample closeouts were not rerun because no code, sample, or script behavior changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after staged diff validation.

## Sub-agent session close status

Docs researcher sub-agent `019de0c3-8216-7803-aab6-8023cd6ae8eb` was used to identify hot hands-on wording candidates and then closed after its findings were incorporated.
