# 0221 — review trail for try-rollback malformed pattern slot selection

## Objective

`0220` の docs-only comparison に対して review trail を残し、first tranche working pair judgement が malformed source placement、runtime representative、first tranche cut を壊していないかを確認する。

## Scope and assumptions

- review 対象は `specs/examples/67` と mirror 更新だけである。
- reviewer は 1 回だけ起動し、長めの wait を 2 回行う。completion が返らなければ local evidence fallback を採る。

## Documents consulted

- `docs/reports/0220-try-rollback-malformed-pattern-slot-selection.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `specs/examples/66-current-l2-try-rollback-malformed-static-tranche-size.md`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`

## Actions taken

1. reviewer evidence を取れれば 1 回だけ review する。
2. completion が返らない場合は local diff inspection と fresh validation を fallback evidence に採用する。

## Evidence / outputs / test results

- reviewer

```text
PENDING
```

- local fallback

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 221 numbered report(s).

git diff --check
<no output>
```

## What changed in understanding

- PENDING

## Open questions

- reviewer completion が返れば、その指摘を反映する。

## Suggested next prompt

future dedicated AST structural helper の malformed pattern slot selection judgment を前提に、fixture-side expected wording / `finding_kind` をどこまで narrow に固定して actual helper first tranche へ進めてよいかを比較し、可能なら helper code / fixture fields / two malformed fixtures / static gate smoke path まで actualize してください。
