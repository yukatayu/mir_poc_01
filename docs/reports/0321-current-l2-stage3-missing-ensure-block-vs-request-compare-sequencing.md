# 0321 — current L2 stage 3 missing ensure block vs request compare sequencing

## Objective

Phase 3 mainline として、request-local suite bridge family の first pair actualization の後に
`missing multiline ensure block` family と fixture-side full request contract compare のどちらを先に扱うべきかを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- helper は private / test-only のまま維持する。
- current task では full request node compare や public parser API へ進まない。

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
- `specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md`
- `specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. first pair actualization 後に残る next-step 候補を `missing multiline ensure block` family / fixture-side full request contract compare / 同時着手 の 3 案で比較した。
2. helper-local hidden fail-closed path の surfacing と request head parse pressure のどちらが current gap に近いかを見た。
3. current next step としては full request compare より先に `missing multiline ensure block` family を actualize する judgment を `specs/examples/105...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 322 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [105-current-l2-stage3-missing-ensure-block-vs-request-compare-sequencing.md](/home/yukatayu/dev/mir_poc_01/specs/examples/105-current-l2-stage3-missing-ensure-block-vs-request-compare-sequencing.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- `missing multiline ensure block` family を先に surfaced する方が current helper family の連続性に合う。
- full request compare は request head parse pressure を増やしやすく、current Phase 3 では still later に置く方が自然である。
- current repo の staged line では malformed family と structure compare family を同時に太らせない方がよい。

## What changed in understanding

- first pair actualization の後も、next step は helper-local hidden fail-closed path の surfacing を続ける方が、full request compare を早く開くより staged line に合うと確認できた。

## Open questions

- `missing multiline ensure block` wording を current helper-local string のまま維持するか。
- bare `require` / `ensure` line family をこの後すぐ扱うか、それとも full request compare comparison の後へ回すか。

## Suggested next prompt

`Phase 3 の次段として、missing multiline ensure block family を helper-local / test-only actual evidence として actualize してください。`
