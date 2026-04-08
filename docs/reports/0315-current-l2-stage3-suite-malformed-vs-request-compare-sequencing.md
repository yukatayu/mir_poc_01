# 0315 — current L2 stage 3 suite malformed vs request compare sequencing

## Objective

Phase 3 mainline として、fixed two-slot suite bridge first tranche の後に
malformed/source family extension と fixture-side full request contract compare のどちらを先に扱うべきかを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- fixed two-slot suite bridge first tranche は actualize 済みである。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. suite bridge first tranche 後の next-step 候補を `malformed/source family extension` / `fixture-side full request contract compare` / `同時着手` の 3 案で比較した。
2. helper-local fail-closed path の可視化と request head parse pressure のどちらが current gap に近いかを見た。
3. current next step としては full request compare より先に malformed/source family extension を docs-only で比較する judgment を `specs/examples/102...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 314 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md](/home/yukatayu/dev/mir_poc_01/specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- fixed two-slot suite bridge first tranche の次に full request compare を先に開くと request head parse pressure が強い。
- malformed/source family extension を先に比較する方が、helper-local fail-closed path を hidden behavior にせず narrow に前進できる。
- current repo の staged line では diagnostics family と structural compare family を同時に太らせない方が自然である。

## What changed in understanding

- suite bridge first tranche の後は、まず malformed/source family の explicit compare を入れてから fixture-side full request compare へ進む方が staged line に合うと確認できた。
- request head / accepted cluster 側の widening はまだ 1 段後ろに置くべきだと再確認できた。

## Open questions

- helper-local malformed/source family の first pair を `duplicate ensure + unsupported direct child line` に置くか、`missing ensure block + duplicate ensure` に置くか。
- unsupported direct child line を dedicated family として扱うか、generic unexpected child wording に留めるか。

## Suggested next prompt

`Phase 3 の次段として、request-local suite bridge family の helper-local malformed/source pair をどこから actualize するかを整理してください。`
