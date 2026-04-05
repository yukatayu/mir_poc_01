# 0214 — try-rollback malformed static family timing

## Objective

future dedicated AST structural helper の malformed static family を actual corpus に増やす timing を、runtime representative 優先 judgment、dedicated helper actualization、public checker comparison requirement の観点から narrow に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper と malformed static family は future option であり、今回は docs-only comparison だけを扱う。
- actual malformed fixture 追加、actual helper 実装、fixture schema actualization は行わない。
- `plan/07` / `plan/09` は helper stack の actual execution path 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. malformed static family を今すぐ actualize する案、dedicated helper first tranche と同時に actualize する案、public checker comparison 直前まで待つ案を比較した。
2. `specs/examples/64-current-l2-try-rollback-malformed-static-family-timing.md` を追加し、dedicated helper actualization first tranche と同時に malformed static family tranche を切る judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/64-current-l2-try-rollback-malformed-static-family-timing.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0214-try-rollback-malformed-static-family-timing.md`
- `docs/reports/0215-review-try-rollback-malformed-static-family-timing.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 215 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - 今すぐ malformed static family を足すと、runtime representative 優先 judgment と helper未actualize状態が衝突しやすい
  - public checker comparison 直前まで待つと、dedicated helper actualization の実地反復に必要な AST-only corpus が不足する
  - dedicated helper actualization first tranche と同時に切るのが、helper code / expected field / malformed corpus / smoke path を narrow に揃えやすい

## What changed in understanding

- malformed static family の actualization は current phase の今ではなく、dedicated helper actualization first tranche の一部として切るのが自然である。

## Open questions

- malformed static family の最小 tranche size。
- malformed wording family をどこまで fixture-side expected に載せるか。

## Suggested next prompt

future dedicated AST structural helper の malformed static family timing judgment を前提に、その first tranche を helper code / fixture fields / malformed static family / smoke path のどこまで一体で切るべきかを docs-only で比較してください。
