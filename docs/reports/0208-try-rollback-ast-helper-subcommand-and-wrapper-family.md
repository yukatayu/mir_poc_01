# 0208 — try-rollback AST helper subcommand and wrapper family

## Objective

future dedicated AST structural helper の actual subcommand 名と wrapper family を、static gate artifact loop judgement と shared carrier threshold judgement を保ったままどこまで narrow に切ってよいかを比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、actual subcommand 追加、shared detached carrier actualization は行わない。
- `plan/07` / `plan/09` は helper stack の actual execution path 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
- `specs/examples/59-current-l2-try-rollback-ast-helper-structural-verdict-carrier.md`
- `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. exact subcommand 名を今 fix する案、family-specific wrapper family だけ先に固定する案、`smoke-static-gate --helper ...` の generic option 案を比較した。
2. `specs/examples/61-current-l2-try-rollback-ast-helper-subcommand-and-wrapper-family.md` を追加し、family-specific wrapper family だけを先に固定し、exact subcommand 名は actual helper actualization task まで deferred にする judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/61-current-l2-try-rollback-ast-helper-subcommand-and-wrapper-family.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0208-try-rollback-ast-helper-subcommand-and-wrapper-family.md`
- `docs/reports/0209-review-try-rollback-ast-helper-subcommand-and-wrapper-family.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 209 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - `smoke-static-gate --helper ...` の generic option は current phase では generic checker-side shared entry の先取りになりやすい
  - exact subcommand 名を今 fix するより、family-specific wrapper family だけを先に固定し、actual helper actualization 開始時に naming cut を narrow に決める方が current helper boundary を壊しにくい

## What changed in understanding

- static gate artifact loop judgement と shared carrier threshold judgement の間に、family-specific wrapper family を維持しつつ exact naming を defer する段が必要である。

## Open questions

- exact subcommand 名を actual helper actualization task でどう切るか。
- generic structural checker family と later public checker API comparison の接続。

## Suggested next prompt

future dedicated AST structural helper の wrapper family judgement を前提に、その helper を future generic structural checker family とどこで合流させるかを docs-only で比較してください。current family-specific wrapper cut と generic option 非採用 judgment は維持してください。
