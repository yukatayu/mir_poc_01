# 0286 — research phase map and progress phase snapshot

## Objective

repo 全体の長期研究について、phase / autonomy gate を `plan/` に集約し、`progress.md` から

- いまどの phase にいるか
- どれくらい重いか
- どこまで self-driven に進めてよいか

が即座に読める状態へ整理する。

## Scope and assumptions

- 今回は docs / plan / progress の整理だけを扱う。
- 規範判断の正本は引き続き `specs/` と relevant `plan/` 本文に残す。
- phase はウォーターフォールではなく rough phase reading として扱う。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`

## Actions taken

1. repo 全体の research line を phase と autonomy gate で読み直した。
2. `plan/17-research-phases-and-autonomy-gates.md` を新設し、Phase 0〜7 の current reading を集約した。
3. `progress.md` に phase section を追加し、current mainline / side line / 自走可否を snapshot として mirror した。
4. `AGENTS.md` と `plan/91` に、phase snapshot を `progress.md` に維持する運用を追加した。
5. `plan/00`、`plan/10`、`Documentation.md`、`plan/90` に導線と traceability を追加した。

## Files changed

- `AGENTS.md`
- `Documentation.md`
- `docs/reports/0286-research-phase-map-and-progress-phase-snapshot.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:30 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 287 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Reviewer:

- reviewer subagent を 1 回起動
- current interface では completion を取得できなかったため、local evidence fallback で close out
- local diff review では substantive semantic inconsistency は見つからず、phase reading / traceability / progress mirror の整合だけを確認した

## What changed in understanding

- repo 全体の current mainline は、単に「current L2 / shared-space が同時進行」ではなく、**Phase 2 終盤 + Phase 3 前半〜中盤が主線で、Phase 4 前半は side line** と読む方が進行管理しやすい。
- `progress.md` の 3 軸進捗だけでは「いま何を凍らせてよく、何を勝手に決めてはいけないか」が見えにくかったため、phase / heaviness / autonomy gate を別 section に置く方が自然である。

## Open questions

- `progress.md` の phase table を chapter table とどこまで連動させるか。
- future phase change の閾値を、どの `plan/` 文書で authoritative に扱うか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space identity / auth layering を participant carrier / authority placement / fairness trust model と混ぜずにどこで切るべきか、current shared-space line の残課題として narrow に比較してください。`
