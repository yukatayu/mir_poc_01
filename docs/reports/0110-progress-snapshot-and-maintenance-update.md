# 0110 — progress snapshot and maintenance update

## Objective

`progress.md` を新設し、current repo の概算進捗、validation loop までの残課題、rough step estimate を簡潔に参照できるようにする。
あわせて、今後の task で `progress.md` を必要に応じて更新する運用を `AGENTS.md` と mirror 文書へ反映する。

## Scope and assumptions

- current L2 の core semantics / notation / parser-free PoC の規範判断は変更しない。
- 今回の `%` と残ステップは **rough estimate** として扱い、問題発見時の巻き戻りを許容する。
- 規範判断の正本は `specs/`、長期参照整理は `plan/`、`progress.md` は簡潔な status snapshot とする。

## Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `plan/00-index.md`
10. `plan/01-status-at-a-glance.md`
11. `plan/02-system-overview-and-positioning.md`
12. `plan/03-decision-strengths-and-boundaries.md`
13. `plan/04-core-semantics-current-l2.md`
14. `plan/05-fallback-lease-and-chain-semantics.md`
15. `plan/06-surface-notation-status.md`
16. `plan/07-parser-free-poc-stack.md`
17. `plan/08-representative-programs-and-fixtures.md`
18. `plan/09-helper-stack-and-responsibility-map.md`
19. `plan/10-roadmap-overall.md`
20. `plan/11-roadmap-near-term.md`
21. `plan/12-open-problems-and-risks.md`
22. `plan/13-heavy-future-workstreams.md`
23. `plan/14-glossary-and-boundary-rules.md`
24. `plan/15-current-l2-fixture-authoring-template.md`
25. `plan/91-maintenance-rules.md`
26. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
27. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`

## Actions taken

1. current repo の現況と near-term roadmap を `plan/` と `specs/` から再確認した。
2. repo root に `progress.md` を追加し、次を簡潔に整理した。
   - 現況サマリ
   - validation loop 前の残課題
   - 章 / 層ごとの rough progress
   - validation loop 入口までの rough step estimate
   - 「ある程度自律的に追加し、回し、比較し、次へ進む」状態までの rough step estimate
3. `AGENTS.md` に `progress.md` 維持ルールを追加し、status / roadmap task では `progress.md` を quick snapshot として読むこと、更新不要時は report に `progress.md 更新不要` と書くことを明記した。
4. `Documentation.md` に `progress.md` の導線を追加した。
5. `plan/91-maintenance-rules.md` に `progress.md` を mirror 対象として追加した。

## Files changed

- `progress.md`
- `AGENTS.md`
- `Documentation.md`
- `plan/91-maintenance-rules.md`

## Commands run

```bash
git status --short --branch
python3 scripts/validate_docs.py
git diff --check
git commit --no-gpg-sign -m '進捗スナップショットと維持ルールを追加する'
```

## Evidence / outputs / test results

- task 開始時 dirty state: `## main...origin/main [ahead 5]`、worktree は clean
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 109 numbered report(s).`
- `git diff --check`
  - 無出力
- commit:
  - `7463c29` `進捗スナップショットと維持ルールを追加する`

## What changed in understanding

- current repo は、Mir current L2 の意味論そのものより、**detached validation loop の運用面**と**fixture authoring / elaboration**が直近の実務 bottleneck である、という理解を `progress.md` に固定した。
- detached validation loop の入口自体はかなり近く、rough には **2〜4 task** 程度で見えている。
- 一方で、「ある程度自律的に追加し、回し、比較し、次へ進む」Mir mainline のループに入るには、fixture authoring 実務、parser 前 inventory、host coverage の narrow cut を含めて **6〜10 task** 程度を見込むのが妥当である。
- `progress.md` は `plan/` を置き換えず、あくまで quick status snapshot として扱うべきだと明確化した。

## Open questions

- `progress.md` の章立てを今後 `plan/` の章とどこまで厳密に同期させるか
- rough progress の % をどの task 変化で更新対象にするか
- detached validation loop の actual narrow API cut をどの時点で `progress.md` から具体的タスクへ降ろすか

## plan/ progress updates

- `plan/` 更新あり:
  - `plan/91-maintenance-rules.md`
- `progress.md` 更新あり

## Specification-document commit hashes

- 本 task で status snapshot / maintenance rule mirror を反映した commit:
  - `7463c29` `進捗スナップショットと維持ルールを追加する`
- report 自身の commit hash は self-reference の都合で本文に固定していない。

## Suggested next prompt

`progress.md` の「validation loop 前の残課題」のうち 1 つを選び、current detached validation loop の actual narrow API cut を 1 task でどこまで固定するかを docs / helper / smoke evidence つきで詰めてください。特に fixture authoring を 1〜2 本実地で回し、artifact 保存・比較・profile 影響確認までの実務手順を narrow に固めてください。
