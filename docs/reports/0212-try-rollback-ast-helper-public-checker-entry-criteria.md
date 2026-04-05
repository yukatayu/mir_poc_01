# 0212 — try-rollback AST helper public checker entry criteria

## Objective

future dedicated AST structural helper family を later public checker API comparison に載せる entry criteria を、generic family 合流 threshold、actual helper/corpus/loop stabilization、public comparison pressure の観点から narrow に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、public checker API actualization、generic checker-side shared entry actualization は行わない。
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
- `specs/examples/62-current-l2-try-rollback-ast-helper-generic-family-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. generic family 合流 threshold 充足と同時に public checker comparison を始める案、public checker 専用 criteria を別に要求する案、parser / proof boundary 確定まで待つ案を比較した。
2. `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md` を追加し、public checker comparison 専用の additional entry criteria を別に要求する judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0212-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `docs/reports/0213-review-try-rollback-ast-helper-public-checker-entry-criteria.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 213 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - generic family 合流 threshold だけで public checker API comparison に入ると、public-looking command surface と shared output contract を早く背負いすぎる
  - parser / theorem prover boundary の完全確定まで待つ必要はないが、actual helper / fixture-side field / corpus / loop stabilization と concrete public comparison pressure は別 threshold として要る

## What changed in understanding

- future dedicated AST structural helper family の public checker API comparison 参加は、generic family 合流の延長ではなく、別の entry criteria を持つ後段判断として切る方が自然である。

## Open questions

- malformed static family を actual corpus に増やす timing。
- public checker API comparison の shared output contract をどの family まで共通化するか。

## Suggested next prompt

future dedicated AST structural helper family の public checker entry criteria を前提に、その malformed static family を actual corpus に増やす timing を docs-only で比較してください。current runtime representative 優先 judgment と helper-local progression は維持してください。
