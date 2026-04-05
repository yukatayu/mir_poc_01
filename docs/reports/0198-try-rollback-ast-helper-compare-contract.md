# 0198 — try-rollback AST helper compare contract

## Objective

future dedicated AST structural helper を actualize する場合の compare contract を、existing reason-row family や detached artifact shared carrier と混ぜずにどこまで narrow に切るべきかを source-backed に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- malformed static family はまだ actualize しない current judgmentを維持する。
- 今回は docs-only comparison と mirror 更新だけを扱う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. dedicated AST structural helper の compare contract 候補を、row-family 流用、detached artifact shared carrier 先行、helper-local dedicated contract の 3 案で比較した。
2. `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md` を追加し、current phase の next narrow step は helper-local dedicated compare contract から始める judgmentを固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - 無出力
- local inspection で確認したこと
  - existing reason-row family helper は `TryFallback` / `AtomicCut` structural floor と責務が異なる
  - detached artifact shared carrier を先に増やすと boundary が重くなる
  - helper-local dedicated contract は current repo の progression と整合する

## What changed in understanding

- future dedicated AST structural helper の compare contract は、current phase では helper-local dedicated contract から始めるのが自然である。
- detached artifact shared carrier や public API は、その helper-local compare が安定してから後段に送る方がよい。

## Open questions

- future expected field 名をどうするか。
- detached artifact へ上げる閾値をどこに置くか。

## Suggested next prompt

current helper-local progression を前提に、future dedicated AST structural helper の expected field 名と focused compare shape を narrow に比較してください。

