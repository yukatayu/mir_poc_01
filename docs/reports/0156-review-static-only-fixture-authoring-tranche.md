# Report 0156 — review static-only fixture authoring tranche

- Date: 2026-04-05T21:18:00+09:00
- Author / agent: Codex
- Scope: Report 0155 の reviewer 指摘を受け、static-only fixture authoring tranche の docs / plan / progress drift を整流する
- Decision levels touched: L2

## 1. Objective

- reviewer が指摘した corpus summary drift、source example traceability drift、`progress.md` task-close log 欠落を解消する。

## 2. Scope and assumptions

- runtime semantics、helper boundary、machine-check policy は変えない。
- fix は docs / plan / progress / traceability に留める。

## 3. Documents consulted

- `docs/reports/0155-static-only-fixture-authoring-tranche.md`
- `plan/01-status-at-a-glance.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 4. Actions taken

1. `plan/01-status-at-a-glance.md` の fixture corpus summary を current 13-bundle / 4 static-only の状態へ更新した。
2. `specs/examples/00-representative-mir-programs.md` に `E12` と `E13` を追加し、fixture `source_example_id` の traceability を補った。
3. `progress.md` に current task の timestamped close log を追加した。
4. `plan/90-source-traceability.md` に `0156` を追記し、review follow-up を relevant plan memory に接続した。

## 5. Files changed

- `plan/01-status-at-a-glance.md`
- `specs/examples/00-representative-mir-programs.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0156-review-static-only-fixture-authoring-tranche.md`

## 6. Evidence / outputs / test results

- reviewer completion:
  - `plan/01-status-at-a-glance.md` の corpus drift
  - `E12` / `E13` source example traceability drift
  - `progress.md` の task-close log 欠落
- local inspection では Rust/test change 自体の semantic regression は追加で見つからなかった。

## 7. Commands run

```text
$ cargo test -p mir-semantics
... 2 unit + 2 aggregate support + 2 bundle support + 40 interpreter + 6 static gate support tests passed ...
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 156 numbered report(s).
```

```text
$ git diff --check
(no output)
```

## 8. What changed in understanding

- static-only fixture を actual corpus に広げる task では、fixture JSON / tests / plan/08 だけでなく `plan/01` と `specs/examples/00` まで drift source になりうる。

## 9. Open questions

- `E12` / `E13` に続く static-only source example を representative prose にどこまで増やすか。

## 10. Suggested next prompt

- duplicate declaration 系の static-only cluster を actual corpus に入れる前に、fixture 増加が `plan/01`、representative prose、named profile、aggregate summary の maintenance cost にどう効くかを narrow に比較してください。
