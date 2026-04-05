# 0115 — fourth remaining problem host interface and progress axes

## Objective

次の 2 点を同じ task で処理する。

1. `progress.md` を、論理仕様 / ユーザ向け仕様 / 実装・運用の 3 軸表示へ更新する。
2. current repo の near-term mainline に残っている大きな問題群のうち、第 4 の問題として
   - richer host interface / typed coverage carrier
   を選び、何と何で迷っているか、各案でどの性質が保証されるかを簡潔に整理する。

## Scope and assumptions

- current L2 semantics、parser-free PoC、detached validation loop の既決事項は変更しない。
- 今回は conversational な整理と maintenance update であり、規範的な semantics は変えない。
- `progress.md` の % は rough estimate とし、問題発見時の巻き戻りを許容する。

## Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/04-mir-core.md`
9. `specs/09-invariants-and-constraints.md`
10. `specs/10-open-questions.md`
11. `specs/11-roadmap-and-workstreams.md`
12. `specs/12-decision-register.md`
13. `plan/06-surface-notation-status.md`
14. `plan/09-helper-stack-and-responsibility-map.md`
15. `plan/11-roadmap-near-term.md`
16. `plan/12-open-problems-and-risks.md`
17. `plan/13-heavy-future-workstreams.md`
18. `plan/91-maintenance-rules.md`
19. `progress.md`
20. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
21. `scripts/current_l2_detached_loop.py`

## Actions taken

1. `AGENTS.md` の `progress.md 維持ルール` に、進捗率を
   - 論理仕様
   - ユーザ向け仕様
   - 実装 / 運用
   の 3 軸で書く方針を追加した。
2. mirror として `plan/91-maintenance-rules.md` に同内容を追加した。
3. `progress.md` の章別 progress table を 3 軸表示へ更新した。
4. 第 4 の残問題として richer host interface / typed coverage carrier を選び、比較軸を整理した。

## Files changed

- `AGENTS.md`
- `plan/91-maintenance-rules.md`
- `progress.md`
- `docs/reports/0115-fourth-remaining-problem-host-interface-and-progress-axes.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
git commit --no-gpg-sign -m '進捗表示の軸を整理する'
git commit --no-gpg-sign -m '進捗表示の軸と第四の残課題を整理する'
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 115 numbered report(s).`
- `git diff --check`
  - 無出力
- `cargo test` は未実行
  - 今回は `AGENTS.md`、`plan/91`、`progress.md`、report の docs-only 変更のため
- commit:
  - `acba193` `進捗表示の軸を整理する`

## What changed in understanding

- `progress.md` は 1 本の概算 % よりも、論理仕様 / ユーザ向け仕様 / 実装・運用の 3 軸へ分けた方が current repo の phase を正確に表せる。
- current repo の第 4 の残問題は、type system / theorem prover そのものより先に、richer host interface / typed coverage carrier をどこまで current helper stack に持ち込むかである。
- ここでは
  - current host harness を verification basis のまま維持する
  - typed coverage carrier を narrow に切る
  - preflight / uncovered call detection / richer production host API は後段
  が自然である。

## Open questions

- `progress.md` の 3 軸 % をどの変化で update 対象にするか
- typed coverage carrier を bundle / batch / host plan のどこに actual implementation として入れるか
- richer host interface へ進む前に、coverage explanation をどこまで detached validation loop に残すか

## plan/ progress updates

- `plan/` 更新あり:
  - `plan/91-maintenance-rules.md`
- `progress.md` 更新あり

## Specification-document commit hashes

- 本 task の maintenance / progress 更新 commit:
  - `acba193` `進捗表示の軸を整理する`
- report 自身の commit hash は self-reference の都合で本文に固定しない。

## Suggested next prompt

第 4 の残問題として、current host harness と richer host interface の間に置く最小 typed coverage carrier を narrow に比較してください。特に uncovered call detection、preflight coverage analysis、coverage explanation の 3 つのうち、どれを current helper stack に先に入れてよいかを source-backed に整理してください。
