# 0206 — try-rollback AST helper shared carrier threshold

## Objective

future dedicated AST structural helper を detached artifact shared carrier へ上げる閾値を、helper-local dedicated contract、fixture-side expected field actualization、static corpus actualization、saved artifact compare need の観点から narrow に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、actual fixture schema field 追加、shared detached carrier actualization は行わない。
- `plan/07` / `plan/09` は helper stack の actual execution path 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
- `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
- `specs/examples/59-current-l2-try-rollback-ast-helper-structural-verdict-carrier.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. shared detached carrier への昇格を、helper-local dedicated contract 維持、minimal evidence cluster 充足後の昇格、helper actualization と同時昇格の 3 案で比較した。
2. 昇格 threshold を helper actualization、fixture-side contract actualization、static corpus actualization、loop stabilization、detached compare need の 5 条件へ分解した。
3. `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md` を追加し、current state では threshold 未充足で helper-local dedicated contract に留める judgment を固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0206-try-rollback-ast-helper-shared-carrier-threshold.md`
- `docs/reports/0207-review-try-rollback-ast-helper-shared-carrier-threshold.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 207 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - dedicated helper actualizationだけで shared carrier へ上げると、fixture-side contract と static corpus より先に artifact shape を凍らせやすい
  - shared carrier 昇格は、helper actualization、fixture-side field actualization、static corpus、loop stabilization、saved artifact compare need の 5 条件が揃ったときにだけ narrow に行う方が current helper boundary を壊しにくい

## What changed in understanding

- dedicated AST structural helper を static gate artifact loop に載せる判断と、shared detached carrier へ上げる判断は別である。
- current state は static gate helper-local smoke family への future insertion judgment までで止まっており、shared carrier promotion threshold はまだ未充足である。

## Open questions

- actual subcommand 名をいつ決めるか。
- malformed static family を actual corpus に増やす timing。
- future generic structural checker family とどこで合流させるか。

## Suggested next prompt

future dedicated AST structural helper の shared carrier promotion threshold を前提に、その helper の actual subcommand 名と wrapper family をいつ narrow に決めてよいかを docs-only で比較してください。current helper-local dedicated cut と static gate artifact loop 挿入 judgment は維持してください。
