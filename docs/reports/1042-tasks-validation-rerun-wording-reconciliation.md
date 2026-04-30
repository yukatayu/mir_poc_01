# 1042 — tasks validation-rerun wording reconciliation

## Objective

直前 package `1041` の validation-scope wording 変更を `tasks.md` の current snapshot に mirror し、stale な `post-sweep full validation rerun` / `still green` phrasing と stale timestamp を解消する。

## Scope and assumptions

- scope は snapshot docs maintenance に限る。
- `tasks.md` の該当 bullet と timestamp、`progress.md` の recent-log mirror だけを更新する。
- validation inventory 自体、sample taxonomy、runtime behavior、規範判断、`plan/` は変更しない。

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
- `docs/reports/1041-post-docs-sweep-validation-rerun.md`

## Actions taken

1. Audited `tasks.md` after `1041` and found the stale `post-sweep full validation rerun` wording plus a stale `最終更新` stamp.
2. Reworded the affected `tasks.md` bullet so it matches `1041`'s narrower validation-scope language and avoids the unsupported `still green` shortcut.
3. Updated `tasks.md` / `progress.md` timestamps and added a `progress.md` recent-log entry for this snapshot reconciliation.
4. Wrote this report.

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- after `1041`, the only immediate stale mirror was in `tasks.md`; the repo-level validation rerun wording itself was already corrected in `progress.md` and the report.
- for this maintenance lane, `tasks.md` needs the same timestamp discipline as `progress.md`, otherwise the current task map looks older than the recent-log evidence it summarizes.

## Open questions

- none for this package.

## Suggested next prompt

`U1` 未決のまま自走を続けるなら、active docs の stale wording audit を queue-authority family の外側へ広げ、example / plan / reader-facing mirror で snapshot authority に逆らう語をさらに圧縮する。

## Files changed

- `tasks.md`
- `progress.md`
- `docs/reports/1042-tasks-validation-rerun-wording-reconciliation.md`

## Commands run

- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## plan/ 更新の有無

- 更新不要

## progress.md 更新の有無

- 更新した

## tasks.md 更新の有無

- 更新した

## samples_progress.md 更新の有無

- 更新不要

## skipped validations and reasons

- sample / cargo / storage guardrail validations は未実行。今回は `tasks.md` と `progress.md` の snapshot wording reconciliation だけの docs-only maintenance closeout であり、source hierarchy / docs scaffold / diff check を focused validation とした。

## commit / push status

- report authoring時点では未実行。same-package closeout で commit / push を行う。

## sub-agent session close status

- docs_researcher `Kepler` は broader stale-wording candidate search を継続中だが、この package の blocking dependency ではないため、結果受領後に必要なら次 package へ引き継ぐ。
