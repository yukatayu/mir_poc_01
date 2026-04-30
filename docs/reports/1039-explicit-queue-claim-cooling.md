# 1039 — explicit queue-claim cooling

## Objective

active `specs/examples/569`、`561`、`481`、`482`、`483` に残っていた concrete package-ladder / reopen-line claims を historical package-memory へ冷やし、current repo-level queue authority と混同しないようにする。

## Scope and assumptions

- scope は docs-only maintenance closeout に限る。
- touch するのは `569`、`561`、`481`、`482`、`483` の explicit queue-claim wording と snapshot docs / report のみ。
- current repo-level queue authority は `progress.md` / `tasks.md` を正とする。
- touched files の ladder / reopen line は historical package-memory / compare-anchor memory として保持してよい。
- 規範判断、runtime behavior、sample taxonomy、`plan/` は変更しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `specs/examples/483-current-l2-witness-provider-artifact-public-shape-threshold-default.md`
- `specs/examples/561-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold-helper-mirror.md`
- `specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md`

## Actions taken

1. Took the next narrow package recommended by docs-researcher `Nietzsche`.
2. Rewrote `569` so `Package 96/97/98` is clearly historical closeout queue memory, not active queue authority.
3. Rewrote `561` so the parser-side reopen line is historical queue memory, not a live parser-side reopen plan.
4. Rewrote `481/482/483` so the `M1 -> M2 -> M3` ladder is historical threshold-ladder memory, not current self-driven queue authority.
5. Mirrored the closeout in `progress.md` and `tasks.md`.
6. Added this report.

## Evidence / outputs / test results

- `git status --short`
  - before package start: clean after commit `66d481b`
- `git branch --show-current`
  - `main`
- `git log -1 --oneline`
  - `66d481b Normalize active next-line headings`
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- the remaining misleading queue wording was no longer generic heading temperature; it had collapsed to explicit concrete ladders (`M1/M2/M3`, parser-side reopen line, `Package 96/97/98`) that needed direct cooling.
- after removing those concrete queue claims, the remaining active-doc drift is likely to be heading-only or mixed-gate phrasing rather than a direct promoted implementation queue.

## Reviewer findings and follow-up

- docs-researcher `Nietzsche` identified `569`、`561`、`481`、`482`、`483` as the narrowest coherent package that still asserted concrete active queue ladders.
- reviewer `Linnaeus` first flagged that live queue nouns were still present in body text and that the report ordering was out of policy.
- follow-up: rewrote those body nouns to historical package-memory wording and reordered the report so `Evidence / outputs / test results` appears before later wrap-up sections.
- `Linnaeus` follow-up review then reported no remaining findings in the scoped `1039` diff.

## Open questions

- heading-only residual drift remains possible in the broader mixed-gate family (`478/479/480/484..497/530/568`), but this package removes the most explicit concrete queue claims.

## Suggested next prompt

`U1` 待ちのまま自走を続けるなら、mixed-gate / coupled-later family の heading-only `## next line` drift をまとめて冷やす。

## Commands run

- `git status --short`
- `git branch --show-current`
- `git log -1 --oneline`
- `date '+%Y-%m-%d %H:%M JST'`
- `sed -n '178,206p' specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- `sed -n '178,206p' specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `sed -n '164,192p' specs/examples/483-current-l2-witness-provider-artifact-public-shape-threshold-default.md`
- `sed -n '72,90p' specs/examples/561-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold-helper-mirror.md`
- `sed -n '152,178p' specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Files changed

- `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `specs/examples/483-current-l2-witness-provider-artifact-public-shape-threshold-default.md`
- `specs/examples/561-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold-helper-mirror.md`
- `specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1039-explicit-queue-claim-cooling.md`

## plan/ 更新の有無

- 更新不要

## progress.md 更新の有無

- 更新した

## tasks.md 更新の有無

- 更新した

## samples_progress.md 更新の有無

- 更新不要

## skipped validations and reasons

- sample / cargo / full validation floor は未実行。今回は active example wording と snapshot docs だけの maintenance closeout であり、source hierarchy / docs scaffold / diff check を focused validation とした。

## remaining user decision blockers

- `U1` actual commitment:
  packaging / installed binary target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth は引き続き user-facing decision を要する。

## commit / push status

- report authoring時点では未実行。same-package closeout で commit / push を行う。

## sub-agent session close status

- docs-researcher `Nietzsche` と reviewer `Linnaeus` を使用し、findings を取り込んだ後に両 session を close した。
