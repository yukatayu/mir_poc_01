# 0467 — phase5 archive member-family threshold

## Objective

Phase 5 theorem-line の current next later reopen candidate として、
archive-manifest-ready retained bridge に actual archive bundle member family をどこまで足し、
actual archive member body compare をどこまで後段に残すかを docs-first で整理する。

## Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- actual archive bundle manifest family までは already current first choice に入っている前提を使う。
- actual archive member body compare、bless / update policy、retained archive payload は本 task で fixed しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0469-review-phase5-archive-member-family-package.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. current theorem-line package の next pressure を再確認し、archive-manifest-ready package の次段では member family と member body compare を同時 actualize せず、1 段 narrow に切る comparison が自然かを確認した。
2. `specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md` を追加し、`archive_bundle_member_family_ref` だけを持つ archive-member-family-ready retained bridge を current first choice として整理した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を mirror 更新した。
4. reviewer を最後に 1 回だけ回し、completion まで待って結果を反映した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0467-phase5-archive-member-family-threshold.md`
- `docs/reports/0468-review-phase5-archive-member-family-threshold.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `date '+%Y-%m-%d %H:%M:%S %Z'` → `2026-04-10 08:36:15 JST`
- `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 469 numbered report(s).`
- `git diff --check` → 無出力
- `git status --short --branch` → `## main...origin/main`
- reviewer → completion を取得。finding は 2 件:
  - `plan/11-roadmap-near-term.md` に `176` 止まり / actual archive bundle member family comparison の stale reopen wording が 2 箇所残っていた
  - `docs/reports/0467-phase5-archive-member-family-threshold.md` の evidence section が `PENDING` のままだった
  いずれも反映済み。詳細 review record は `docs/reports/0469-review-phase5-archive-member-family-package.md`。

## What changed in understanding

- archive manifest family の次段では、member family と member body compare を同じ reopen で扱うより、member family だけを symbolic ref 化して narrow に 1 段進める方が current theorem-line の ratchet に合う。
- current next later reopen は actual archive bundle member family comparison ではなく、**actual archive member body compare comparison** と読むのが自然である。
- review により、remaining risk は semantic claim ではなく roadmap mirror drift と report evidence hygiene だと確認できた。

## Open questions

- actual archive member body compare の最小 shape
- member body compare を member-body ref 1 本に留めるか、bless / update policy まで further split するか
- retained archive payload pressure をどこで theorem-line bridge と接続するか

## Suggested next prompt

`actual archive member body compare comparison を、archive-member-family-ready retained bridge の次段として docs-first で比較し、member-body ref と bless/update policy pressure の cut を narrow に整理してください。`
