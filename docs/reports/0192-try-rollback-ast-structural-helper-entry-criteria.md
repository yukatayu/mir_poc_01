# 0192 — try-rollback AST structural helper entry criteria

## Objective

`TryFallback` / `AtomicCut` の structural floor を current phase で dedicated AST structural helper に actualize してよい条件を source-backed に比較し、docs-only の entry criteria を固定する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- existing three checker spikes と `specs/examples/51` / `52` の judgment は維持する。
- 今回は docs-only comparison と mirror 更新だけを扱い、new helper 実装や malformed static family の actualization は行わない。

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
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. first parser cut inventory と first checker cut inventory を読み直し、`try { ... } fallback { ... }` / `atomic_cut` が parser / checker 候補 cluster である一方、dynamic gate と restore scope は runtime / proof boundary に残ることを再確認した。
2. dedicated AST structural helper の actualization 案を、
   - runtime valid fixture だけで今すぐ切る案
   - parser/loader source、AST-only floor、malformed family が揃ってから切る案
   - docs/runtime representative に留めて comparison 自体も先送りする案
   の 3 案で比較した。
3. `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md` を追加し、helper が読んでよい AST structural floor、読んではいけない runtime gate、reason-row family と分けるべき carrier、malformed family の最小 entry criteria を固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。

## Files changed

- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
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
  - numbered reports count が current chain に追従している
- `git diff --check`
  - 無出力
- local inspection で確認したこと
  - `specs/examples/29` は `try { ... } fallback { ... }` と `atomic_cut` を first parser cut 候補 cluster に入れている
  - `specs/examples/30` は `TryFallback` / `AtomicCut` structural floor を checker 候補 cluster に入れる一方、dynamic gate / restore scope を outside に残す
  - `specs/examples/52` は existing reason-row family helper の fourth spike actualization を保留している

## What changed in understanding

- `TryFallback` / `AtomicCut` の dedicated AST structural helper は、parser inventory があるだけではまだ actualize しない方が自然である。
- actualization に進む最小条件は、parser/loader malformed source の explicit 化、AST-only floor への限定、reason-row family と別 carrier であること、runtime gate を non-goal にすること、そして singleton ではない structural family を持つことである。

## Open questions

- structural malformed source を parser / loader / static gate のどこに置くか。
- malformed static fixture family を本当に追加するか。
- future dedicated AST structural helper の artifact / compare contract をどこに置くか。

## Suggested next prompt

current first parser cut inventory と first checker cut inventory を前提に、`TryFallback` / `AtomicCut` structural malformed source を parser / loader / static gate のどこへ置くのが最小かを narrow に比較してください。

