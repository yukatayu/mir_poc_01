# 0218 — try-rollback malformed static tranche size

## Objective

future dedicated AST structural helper の first tranche に含める minimal malformed static family tranche の exact size を、dedicated helper entry criteria、subject-kind contrast、first tranche の重さの観点から narrow に比較する。

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
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/64-current-l2-try-rollback-malformed-static-family-timing.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. single fixture、`TryFallback` + `AtomicCut` の two-fixture pair、three-or-more tranche を比較した。
2. `specs/examples/66-current-l2-try-rollback-malformed-static-tranche-size.md` を追加し、first tranche の exact size は `TryFallback` 1 件 + `AtomicCut` 1 件の two-fixture pair が最小だという judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/66-current-l2-try-rollback-malformed-static-tranche-size.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0218-try-rollback-malformed-static-tranche-size.md`
- `docs/reports/0219-review-try-rollback-malformed-static-tranche-size.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 219 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - single fixture では dedicated helper entry criteria の複数 structural family 条件と緊張する
  - two-fixture pair なら `TryFallback` / `AtomicCut` の subject-kind contrast を最小数で満たせる
  - three-or-more tranche は first tranche としては重く、helper code / fields / smoke path と同時に切るには premature である

## What changed in understanding

- first tranche の malformed static corpus は、family contrast を持つ最小 pair に留めるのが自然である。

## Open questions

- `TryFallback` slot に入れる最初の malformed pattern。
- `AtomicCut` slot に入れる最初の malformed pattern。
- malformed wording family をどこまで fixture-side expected に載せるか。

## Suggested next prompt

future dedicated AST structural helper の malformed static tranche size judgment を前提に、その two-fixture first tranche の `TryFallback` slot と `AtomicCut` slot に最初に入れる malformed pattern を docs-only で比較してください。
