# 0194 — try-rollback structural malformed source placement

## Objective

`TryFallback` / `AtomicCut` の structural malformed source を parser / loader / static gate のどこへ置くのが current L2 parser-free PoC に最も自然かを source-backed に比較し、docs-only current judgment を固定する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- text parser はまだ無い。
- dedicated AST structural helper は actualize せず、source placement comparison だけを扱う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. concrete syntax malformed、carrier/schema malformed、semantic structural malformed の 3 種を分けて current helper boundary を読み直した。
2. structural malformed source を parser、loader、static gate / dedicated AST helper の 3 案で比較した。
3. `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md` を追加し、current parser-free PoC では carrier/schema malformed は loader、semantic structural malformed は static gate / dedicated AST helper に置く judgment を固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
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
  - current mainline は parser-free PoC であり、text parser source を前提にできない
  - `specs/examples/30` は `TryFallback` / `AtomicCut` structural floor を checker 候補 cluster に置いている
  - `specs/examples/53` は dedicated AST structural helper が AST-only floor を読むべきだとしている

## What changed in understanding

- `TryFallback` / `AtomicCut` の structural malformed source は、current parser-free phase では parser でも loader でもなく、static gate / dedicated AST helper 側へ置くのが自然である。
- loader は carrier/schema malformed の入口に留める方が、future parser handoff と checker boundary の両方に整合する。

## Open questions

- malformed static family を実際に追加する価値があるか。
- loader error wording と static malformed wording を detached artifact でどう分けるか。

## Suggested next prompt

current runtime representative `E2` / `E21` / `E22` と parser/checker boundary inventory を前提に、`TryFallback` / `AtomicCut` の malformed static family を actual corpus に追加する価値があるかを narrow に比較してください。

