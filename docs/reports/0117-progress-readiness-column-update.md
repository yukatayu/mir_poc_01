# 0117 — progress readiness column update

## Objective

`progress.md` に、各章 / 層について

- 着手可能
- 要仕様確認
- 後段依存

のいずれかを示す列を追加し、今後 agent がどこまで自走してよいかを rough status snapshot から読めるようにする。
あわせて `AGENTS.md` と `plan/91-maintenance-rules.md` に同ルールを反映する。

## Scope and assumptions

- current L2 semantics、parser-free PoC、helper stack の既決事項は変更しない。
- 今回は maintenance update であり、規範判断の正本は引き続き `specs/` にある。
- `着手可否` は rough operational judgment であり、問題発見時の巻き戻りを許容する。

## Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `plan/11-roadmap-near-term.md`
10. `plan/12-open-problems-and-risks.md`
11. `plan/13-heavy-future-workstreams.md`
12. `plan/91-maintenance-rules.md`
13. `progress.md`

## Actions taken

1. `AGENTS.md` に、`progress.md` が 3 軸 progress に加えて `着手可否` 欄を持つべきことを追記した。
2. `plan/91-maintenance-rules.md` に同ルールを mirror した。
3. `progress.md` の chapter table を更新し、各章 / 層に
   - `着手可能`
   - `要仕様確認`
   - `後段依存`
   を付けた。
4. `progress.md` に `着手可否の読み方` 節を追加し、3 値の意味を日本語で明示した。

## Files changed

- `AGENTS.md`
- `plan/91-maintenance-rules.md`
- `progress.md`
- `docs/reports/0117-progress-readiness-column-update.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
git commit --no-gpg-sign -m 'progress の着手可否列を追加する'
git push
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`
  - 実行結果は commit 前 verification で補記する
- `git diff --check`
  - 実行結果は commit 前 verification で補記する
- `cargo test` は未実行
  - docs-only / maintenance-only 変更のため

## What changed in understanding

- current repo では、rough progress の % だけでは「今その層を自走してよいか」が読み取りにくい。
- `着手可能 / 要仕様確認 / 後段依存` を足すことで、
  - mainline で narrow に進めてよい領域
  - user 側の仕様伝達を待つべき領域
  - 先行 decision の確定を待つべき領域
  を区別しやすくなった。

## Open questions

- `着手可否` を章単位だけでなく、将来は task 候補単位にも落とすか
- `要仕様確認` の領域で、どの粒度から user への質問を mandatory にするか

## plan/ progress updates

- `plan/` 更新あり:
  - `plan/91-maintenance-rules.md`
- `progress.md` 更新あり

## Specification-document commit hashes

- 本 task の maintenance / progress 更新 commit hash は commit 後に補記する。
- report 自身の commit hash は self-reference の都合で本文に固定しない。

## Suggested next prompt

`progress.md` の `着手可否` 列を前提に、`着手可能` な章だけを対象に next 3 task を narrow に選んでください。特に detached validation loop、fixture authoring、parser boundary inventory の 3 つから、手戻りを増やさずに前進量が大きい順で提案してください。
