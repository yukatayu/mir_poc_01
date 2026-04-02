# Report 0088 — current l2 representative prose drift check

- Date: 2026-04-03T08:30:22+09:00
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤と 0079 / 0080 / 0081 / 0082 / 0083 / 0084 / 0087 を前提に、representative examples と plan mirror に残る fallback / preference chain の prose wording を narrow scope で点検し、A2 rendering rollout 後も outer/inner wrapper 誤読を誘発しうる箇所だけを最小修正する。runtime semantics、fixture schema、interpreter、host harness、bundle / batch / selection / profile helper、tests の machine-check surface は変更しない。
- Decision levels touched: current L2 companion notation と fallback 読みの docs / plan mirror に関する L2 wording だけを更新し、意味論判断自体は変更しない。

## 1. Objective

representative examples prose、surface syntax candidate prose、fallback reconciliation prose、plan mirror prose を横断し、A2（hanging lineage continuation）を前提にしても outer/inner wrapper 誤読、write-after-expiry の過剰な success 読み、rollback / `atomic_cut` による degradation order 巻き戻し誤読が残っていないかを確認する。

## 2. Inputs consulted

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
13. `specs/examples/00-representative-mir-programs.md`
14. `specs/examples/01-current-l2-surface-syntax-candidates.md`
15. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
16. `plan/01-status-at-a-glance.md`
17. `plan/05-fallback-lease-and-chain-semantics.md`
18. `plan/06-surface-notation-status.md`
19. `plan/08-representative-programs-and-fixtures.md`
20. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
21. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
22. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
23. `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`
24. `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`
25. `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md`
26. `docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md`
27. `docs/reports/0087-current-l2-a2-rollout-boundary.md`

## 3. Actions taken

1. task-start 状態として、前タスク `0087` がすでに commit 済みであること、今回の tracked dirty state はないことを確認した。未追跡は `diff_investigation_01.md` と一時参照フォルダ 2 件であり、今回の変更対象から外した。
2. A2 rollout 後の prose drift source を、`specs/examples/00`、`specs/examples/01`、`specs/examples/15`、`plan/05`、`plan/06`、`plan/08` で棚卸しした。
3. wording 上の主要 drift source を次の 2 つに絞った。
   - A2 の hanging continuation の追加 indent が、edge-local metadata continuation ではなく outer/inner wrapper の追加入れ子に見える余地。
   - write-after-expiry について、「later write-capable option がある」という条件を success guarantee のように読みすぎる余地。
4. `specs/examples/00-representative-mir-programs.md` に、A2 indent は outer/inner wrapper の入れ子ではなく直前 edge row にだけ属する metadata continuation だと明記した。
5. 同じく `specs/examples/00-representative-mir-programs.md` の E6 prose に、後段 option の存在そのものは automatic success を意味しないことを明記した。
6. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` の一致点の prose を、later write-capable option は試行可能性を与えるだけで success guarantee ではない、と読めるよう補強した。
7. `plan/05-fallback-lease-and-chain-semantics.md`、`plan/06-surface-notation-status.md`、`plan/08-representative-programs-and-fixtures.md` を同 task で更新し、plan mirror でも同じ boundary を維持した。
8. `specs/examples/01-current-l2-surface-syntax-candidates.md` は、A2 indent を edge-local metadata continuation と読む rule と limited rollout boundary がすでに十分明示されていたため、今回の変更対象から外した。
9. machine-check surface は意図的に変更しなかった。fixture / tests / interpreter は current L2 reading の public behavior coverage を持つ層であり、今回の task は prose wording のみに限定するためである。
10. local validation を行った。今回のセッションでは reviewer を起動して completion を待つための tool surface が使えなかったため、retry は行わず、対象 diff の spot-check と `validate_docs` / `git diff --check` を local evidence として report に残す方針へ切り替えた。

## 4. Files changed

- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/06-surface-notation-status.md`
- `plan/08-representative-programs-and-fixtures.md`
- `docs/reports/0088-current-l2-representative-prose-drift-check.md`

## 5. Commands run and exact outputs

```bash
git status --short --branch
sed -n '1,220p' AGENTS.md
sed -n '1,240p' README.md
sed -n '1,240p' Documentation.md
sed -n '150,390p' specs/examples/01-current-l2-surface-syntax-candidates.md
sed -n '250,360p' specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
sed -n '1,220p' docs/reports/0087-current-l2-a2-rollout-boundary.md
sed -n '1,260p' specs/examples/00-representative-mir-programs.md
sed -n '1,220p' plan/05-fallback-lease-and-chain-semantics.md
sed -n '1,220p' plan/06-surface-notation-status.md
sed -n '1,240p' plan/08-representative-programs-and-fixtures.md
python3 scripts/new_report.py --slug current-l2-representative-prose-drift-check
python3 scripts/validate_docs.py
git diff --check
git diff --stat
```

### Exact outputs

- task 中の `python3 scripts/new_report.py --slug current-l2-representative-prose-drift-check`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0088-current-l2-representative-prose-drift-check.md
```

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 88 numbered report(s).
```

- `git diff --check`

```text
(no output)
```

- `git diff --stat`

```text
 plan/05-fallback-lease-and-chain-semantics.md                           | 2 +-
 plan/06-surface-notation-status.md                                      | 1 +
 plan/08-representative-programs-and-fixtures.md                         | 2 +-
 specs/examples/00-representative-mir-programs.md                        | 2 ++
 .../15-current-l2-fallback-reconciliation-and-compact-syntax.md         | 2 +-
5 files changed, 6 insertions(+), 3 deletions(-)
```

- reviewer

```text
今回のセッションでは reviewer 起動 / completion 待機用の tool surface が利用できなかった。
そのため local evidence と diff inspection を report に残した。
```

## 6. Evidence / findings

### 6.1 drift source が実際に残っていた箇所

- `specs/examples/00-representative-mir-programs.md`
  - A2 rollout 後の prose では、hanging continuation を first-choice rendering と書いていても、「その追加 indent が何を表すか」を 1 箇所で言い切っていなかった。representative code block を先に読む読者には、outer/inner nesting の追加に見える余地があった。
- `specs/examples/00-representative-mir-programs.md`
  - E6 の prose は final `Reject` を正しく説明していたが、「後段 candidate が存在することは success guarantee ではない」という否定文がなかったため、E7 と並べて読むと general law を強めすぎる余地があった。
- `plan/05-fallback-lease-and-chain-semantics.md`
  - write-after-expiry table の「後段に write-capable option がある | later option を試行してよい」は正しいが、試行可能性と success guarantee の区別を prose で補った方が安全だった。
- `plan/08-representative-programs-and-fixtures.md`
  - `e7` の補足は fixture 固有の success case を説明しているが、条件を短く書きすぎると「later write-capable option があれば success しうる」が一般法のように見える余地があった。

### 6.2 現状で十分だった箇所

- `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - A2 indent は edge-local metadata continuation であり、request-evaluation boundary を保つための rendering だと、declaration rule と block reading ruleの両方で十分に書かれていた。
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
  - outer/inner wrapper 読み、no re-promotion、rollback / cut non-interference、A2 rollout boundary の大枠整理はすでに十分だった。今回は write-after-expiry の読みを強めすぎない補強だけで足りた。
- machine-check surface
  - fixture schema、interpreter、tests、host harness、bundle / batch / selection / profile helper は今回触らない方が正しい。current task は wording 整理であり、public behavior coverage を持つ層に notation 由来の変更を持ち込む理由がない。
- reviewer evidence
  - reviewer 不返却ではなく、そもそも reviewer 起動 / wait の tool surface がこのセッションでは利用できなかったため、`validate_docs`、`git diff --check`、対象 5 ファイルの wording spot-check を evidence として採用した。

## 7. Changes in understanding

 - A2 rollout 後の main drift source は code block 自体より、**その hanging indent が何を意味しないか**を prose で言い切っていない箇所に残る。
 - write-after-expiry は、current L2 では「later write-capable option を試行できる」ことと「成功する」ことを常に分けて書いた方が安全である。E7 は success case の fixture だが、general law はあくまで try-later-or-`Reject` である。
 - actual PoC をもっと広く実装して回す前に、notation より重い論点として次が残る。
   - parser grammar を final fix しないままでも回せる fixture authoring / elaboration の維持
   - detached trace / audit serialization
   - richer host interface と host plan coverage analysis
   - multi-request scheduler
   - static analysis / theorem prover 側へ送る境界の具体化
   - `Approximate` / `Compensate` を current minimal runtime loop にどの時点で入れるか
 - つまり、現状の PoC は「狭い current L2 を確実に回す」段階としてはかなり強いが、「回しまくる」段階へ進むには syntax そのものより **serialization / host integration / scheduler / analysis boundary** の整備が次の重いボトルネックになる。

## 8. Open questions

- final parser syntax
- line-leading `>` ladder を将来再比較する閾値
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`
- reviewer tool surface がない session での evidence trail をどこまで report 標準化するか

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached trace / audit serialization と richer host interface の最小 boundary を narrow scope で整理し、いまの parser-free fixture loop を “より回せる PoC” に進めるために何を先に固定すべきかを比較してください。`
