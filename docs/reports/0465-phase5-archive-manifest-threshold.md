# 0465 — phase5 archive manifest threshold

## Objective

Phase 5 theorem-line の current next later reopen candidate として、
archive-bundle-ready retained bridge に actual archive bundle manifest family をどこまで足し、
actual archive bundle member family をどこまで後段に残すかを docs-first で整理する。

## Scope and assumptions

- `proof_notebook` first bridge の retained bridge line だけを扱う。
- actual archive bundle family までは already current first choice に入っている前提を使う。
- actual archive bundle member family、archive member body compare、bless/update policy、retained archive payload は本 task で fixed しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/175-current-l2-theorem-line-archive-body-ready-archive-bundle-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `tasks.md`

## Actions taken

1. current theorem-line package の next pressure を再確認し、archive bundle family の次段では manifest family と member family を同時 actualize せず、1 段 narrow に切る comparison が自然かを確認した。
2. `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md` を追加し、`archive_bundle_manifest_ref` だけを持つ archive-manifest-ready retained bridge を current first choice として整理した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を mirror 更新した。
4. reviewer を最後に 1 回だけ回し、completion まで待って結果を反映した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0465-phase5-archive-manifest-threshold.md`
- `docs/reports/0466-review-phase5-archive-manifest-threshold.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M:%S %Z'
df -h .
free -h
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `date '+%Y-%m-%d %H:%M:%S %Z'` → `2026-04-10 07:54:14 JST`
- `df -h .` → `/dev/vda2` 使用率 `98%`、空き `2.6G`
- `free -h` → `Mem 960Mi / available 152Mi`、`Swap 17Gi free`
- `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 176 numbered spec example(s).` / `Found 466 numbered report(s).`
- `git diff --check` → 無出力
- reviewer → completion を取得。finding は 3 件:
  - `specs/examples/176...` の `decided` / `not decided` wording が self-contradictory
  - `plan/17...` の immediate execution order が theorem-line を `173` までで止めていた
  - `progress.md` の作業ログに `175` package の 1 行が欠けていた
  いずれも反映済み。

## What changed in understanding

- archive bundle family の次段では、manifest family と member family を同じ reopen で扱うより、manifest family だけを symbolic ref 化して narrow に 1 段進める方が current theorem-line の ratchet に合う。
- current next later reopen は actual archive bundle comparison ではなく、**actual archive bundle member family comparison** と読むのが自然である。
- review により、archive-manifest package の主要リスクは semantic claim ではなく mirror hygiene と threshold wording の整合だと確認できた。

## Open questions

- actual archive bundle member family の最小 shape
- archive bundle member family と archive member body compare / bless/update policy をどこまで分離できるか
- concrete consumer pressure が archive bundle member family を docs-only retained bridge に上げる threshold をいつ満たすか

## Suggested next prompt

`actual archive bundle member family comparison を、archive-manifest-ready retained bridge の次段として docs-first で比較し、member-family ref と retained archive payload pressure の cut を narrow に整理してください。`
