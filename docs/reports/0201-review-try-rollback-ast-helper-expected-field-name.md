# 0201 — review trail for try-rollback AST helper expected field name

## Objective

`0200` の docs-only comparison に対して review trail を残し、expected field 名と focused compare shape の judgment が current helper boundary を壊していないかを確認する。

## Scope and assumptions

- review 対象は `specs/examples/57` と mirror 更新だけである。
- reviewer は 1 回だけ起動し、長めの wait を 2 回行う。completion が返らなければ local evidence fallback を採る。

## Documents consulted

- `docs/reports/0200-try-rollback-ast-helper-expected-field-name.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`

## Actions taken

1. reviewer agent を 1 回だけ起動した。
2. 180s wait を 2 回行い、completion が返るか確認した。
3. reviewer completion を取得し、finding の有無を確認した。

## Evidence / outputs / test results

- reviewer completion:

```text
No findings.

specs/examples/57 は specs/examples/53..56 と整合しており、future-only / AST-structural-only / reason-row family 非共有 / helper-local compare という boundary を保っている。
Documentation.md、specs/00-document-map.md、plan/11、plan/12、plan/15、progress.md、plan/90 にも mirror drift は無い。
Residual risk は structural verdict carrier/name と detached-loop insertion timing が OPEN のまま残っている点だけで、矛盾ではない。
```

- local validation:
  - `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 201 numbered report(s).`
  - `git diff --check` → 無出力

## What changed in understanding

- expected field 名と focused compare shape の narrowing は、current helper boundary と整合したまま docs-only で固定できることが確認できた。
- 残る OPEN は structural verdict carrier/name と detached validation loop insertion timing であり、次段 comparison を切る余地が明確になった。

## Open questions

- structural verdict carrier/name をどこで比較するか。
- dedicated helper を detached validation loop のどこへ差し込むか。

## Suggested next prompt

future dedicated AST structural helper の expected field 名と focused compare shape judgment を前提に、dedicated helper を detached validation loop のどこへ差し込むのが最小かを docs-only で比較してください。
