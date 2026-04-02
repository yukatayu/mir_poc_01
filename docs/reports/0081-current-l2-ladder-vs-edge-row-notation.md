# Report 0081 — current l2 ladder vs edge row notation

- Date: 2026-04-02T13:01:16.279091Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤と 0079 / 0080 を前提に、fallback / preference chain の compact syntax candidate として Candidate A と Candidate B を比較し、companion notation の暫定判断を再確認する
- Decision levels touched: current L2 examples companion notation の比較整理のみ。fallback semantics、parser grammar、runtime semantics は未変更

## 1. Objective

current L2 semantics を guarded option chain / left-to-right monotone degradation / no re-promotion のまま維持しつつ、fallback / preference chain の compact syntax candidate のうち line-leading `>` ladder を本当に companion notation 候補へ上げる価値があるか、それとも current explicit edge-row form を維持するべきかを比較する。

## 2. Scope and assumptions

- 今回は notation comparison だけを扱い、runtime semantics、fixture schema、interpreter、host harness は変更しない。
- machine-readable catalog asset / manifest や final parser grammar は導入しない。
- 開始時点の worktree は clean で、`main...origin/main` だった。
- current L2 の machine-check 根拠として、E3 variant / E6 / E7 / E8 の fixture と `mir-semantics` integration tests を既存のまま参照した。

## 3. Documents consulted

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
15. `specs/examples/04-current-l2-step-semantics.md`
16. `specs/examples/06-current-l2-interpreter-skeleton.md`
17. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
18. `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
19. `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
20. `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
21. `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
22. `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
23. `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
24. `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
25. `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
26. `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
27. `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
28. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
29. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
30. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
31. `crates/mir-ast/tests/fixtures/current-l2/`
32. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. `code_mapper` を先に使い、fallback / `lease` の正本箇所、A/B 比較の自然な mirror 箇所、syntax 変更で drift しそうな場所、既存 dirty state を read-only で棚卸しした。
2. `specs/04-mir-core.md`、`specs/12-decision-register.md`、`specs/examples/04-current-l2-step-semantics.md` を基準に、current L2 では guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` / rollback / cut non-interference が固定されていることを再確認した。
3. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` に A/B の直接比較表を追加し、視覚的 compactness と semantic honesty を同じ表で比べられるようにした。
4. `specs/examples/01-current-l2-surface-syntax-candidates.md` の chain declaration 節に、line-leading `>` ladder を比較対象として残しつつも、current L2 では explicit edge-row form を companion notation として維持する理由を短く追記した。
5. 今回は focused regression の追加は行わなかった。parser-free PoC の machine-check surface は syntax 非依存であり、E3 variant / E6 / E7 / E8 の既存 fixture が現行 semantics をすでに固定しているためである。
6. reviewer を最後に 1 回だけ起動し、返ってきた review report 0082 の low findings 2 件を 0081 の整形と証跡追記で解消した。
7. spec 側の比較 wording だけを先に 1 本の commit に切り出し、仕様本文 commit hash を固定した。

## 5. Files changed

- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`
- `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`

## 6. Evidence / outputs / test results

```bash
python3 scripts/new_report.py --slug current-l2-ladder-vs-edge-row-notation
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

主要な出力:

- `python3 scripts/new_report.py --slug current-l2-ladder-vs-edge-row-notation`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`
- `cargo test -p mir-semantics`
  - unit test 2 件 pass
  - integration test 33 件 pass
  - doc test 0 件 pass
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 81 numbered report(s).`
- `git diff --check`
  - 出力なし
- `git commit --no-gpg-sign -m "fallback の compact syntax 比較を更新する"`
  - `f317163690a0c220ea1b42cf22977f6d07ed7813`
- reviewer
  - `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md` を生成
  - semantics 破壊と comparison judgment については no findings
  - 0081 の report structure と evidence 書式について low findings 2 件

主要な確認点:

- A は edge-local lineage と successor declaration を prose なしでも読み取りやすい。
- B は vertical reading と compactness で優位だが、`>` が operator / expression に見えやすく、canonical chain と request-evaluation path の境界を prose 依存にしやすい。
- parser-free 実装と fixture schema は surface syntax 非依存であるため、今回は docs mirror の比較だけで十分だった。
- reviewer の最終所見では、spec 側の semantics 破壊や comparison judgment の言い過ぎは no findings だった。指摘は report 0081 の構造整形 2 点だけで、今回それを修正した。

仕様本文コミット hash:

- `f317163690a0c220ea1b42cf22977f6d07ed7813` `fallback の compact syntax 比較を更新する`

## 7. What changed in understanding

- B は単に「短い別案」ではなく、outer/inner 誤読を減らす面では A より強い一方で、edge-local lineage と `Reject` まで含む evaluation path を薄く見せる副作用があると整理できた。
- current L2 で今必要なのは compactness の最大化ではなく、canonical chain を declaration として読む honesty の維持だった。
- そのため A/B の比較は「どちらが prettier か」ではなく、「どちらが current semantics を prose 追加なしで誤読させにくいか」で決めるべきだと明確になった。

## 8. Open questions

- final parser syntax
- line-leading `>` ladder を parser grammar または正式 companion notation へ昇格させるかどうか
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

report 自身の commit hash は self-reference の都合でこの本文では未固定である。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、fallback / preference chain の compact syntax candidate として line-leading \`>\` ladder をもし examples の一部に試験導入するなら、どの representative examples に限定して A/B 比較を行うのが最も誤読差分を観察しやすいかを整理してください。`
