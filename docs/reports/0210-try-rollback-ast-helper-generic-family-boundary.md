# 0210 — try-rollback AST helper generic family boundary

## Objective

future dedicated AST structural helper を future generic structural checker family とどこで合流させるかを、family facade 維持、actual helper actualization、later public checker API comparison の timing から narrow に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、generic checker-side shared entry、public checker API actualization は行わない。
- `plan/07` / `plan/09` は helper stack の actual execution path 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
- `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
- `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md`
- `specs/examples/61-current-l2-try-rollback-ast-helper-subcommand-and-wrapper-family.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. dedicated helper actualization と同時合流、later public checker API comparison と同時合流、恒久 dedicated family 維持の 3 案を比較した。
2. `specs/examples/62-current-l2-try-rollback-ast-helper-generic-family-boundary.md` を追加し、generic family 合流は later public checker API comparison と同時に扱う judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/62-current-l2-try-rollback-ast-helper-generic-family-boundary.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0210-try-rollback-ast-helper-generic-family-boundary.md`
- `docs/reports/0211-review-try-rollback-ast-helper-generic-family-boundary.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 211 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - current checker spike 群の family facade 維持 pattern と整合させるなら、future dedicated AST structural helper も generic family 合流は later public checker API comparison と同時に扱うのが自然である
  - actual helper 実装や corpus 実地反復より先に generic family 合流を始めると、helper-local dedicated contract と family-specific wrapper judgement を壊しやすい

## What changed in understanding

- future dedicated AST structural helper の generic family 合流は、actual helper actualization の問いではなく later public checker API comparison の問いとして扱う方が current checker-side sequencing に整合する。

## Open questions

- later public checker API comparison の entry criteria。
- malformed static family を actual corpus に増やす timing。

## Suggested next prompt

future dedicated AST structural helper の generic family boundary を前提に、later public checker API comparison にその family を載せる entry criteria をどこまで narrow に切るかを docs-only で比較してください。
