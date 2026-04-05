# 0196 — try-rollback malformed static family actualization

## Objective

`TryFallback` / `AtomicCut` の malformed static family を current phase で actual corpus に追加する価値が本当にあるかを source-backed に比較し、docs-only current judgment を固定する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- runtime representative `E2` / `E21` / `E22` は維持する。
- 今回は comparison と mirror 更新だけを扱い、new fixture や helper 実装は追加しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. runtime representative と docs-only checker boundary の current evidence を読み直した。
2. malformed static family について、
   - まだ増やさない案
   - 2 例以上 actual corpus に追加する案
   - docs inventory に留める案
   を比較した。
3. `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md` を追加し、current phase では runtime representative だけで当面十分であり、malformed static family はまだ actualize しない judgment を固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
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
  - `E2` / `E21` / `E22` により runtime representative は既に source-backed である
  - malformed static family を actualize するには family 候補、helper contract、wording / artifact cut を同時に詰める必要がある
  - current docs chain はそこまでの actualization をまだ要求していない

## What changed in understanding

- `TryFallback` / `AtomicCut` の malformed static family は、current phase ではまだ actual corpus に入れない方が自然である。
- dedicated AST structural helper に進むなら、先に compare contract を docs-only で切る方が、fixture invent より手戻りが少ない。

## Open questions

- future dedicated AST structural helper の compare contract をどこに置くか。
- malformed family 候補を docs inventory としてどこまで明示するか。

## Suggested next prompt

current parser/checker boundary inventory と dedicated AST structural helper entry criteria を前提に、future dedicated AST structural helper の compare contract を narrow に比較してください。

