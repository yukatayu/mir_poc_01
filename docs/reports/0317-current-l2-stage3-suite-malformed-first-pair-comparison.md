# 0317 — current L2 stage 3 suite malformed first pair comparison

## Objective

Phase 3 mainline として、request-local suite bridge family の helper-local malformed/source extension を
どの first pair から actualize するのが最小かを narrow に比較する。

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
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. request-local suite bridge family の malformed/source 候補を `duplicate ensure + unsupported child` / `duplicate ensure + missing ensure block` / `missing ensure block + unsupported child` の 3 案で比較した。
2. suite bridge family 固有の前進量と hidden behavior の抑制の観点から最小 pair を選んだ。
3. current next cut としては `duplicate ensure + unsupported direct child line` を first pair にする judgment を `specs/examples/103...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 318 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [103-current-l2-stage3-suite-malformed-first-pair-comparison.md](/home/yukatayu/dev/mir_poc_01/specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- `duplicate ensure + unsupported direct child line` が suite bridge family 固有の前進量を最も素直に増やす。
- `missing multiline ensure block` は still later に残しても bridge family の境界を保てる。
- current next step をこの pair actualization に置くのが最も narrow である。

## What changed in understanding

- suite bridge family の malformed/source extension では、multiline attachment bridge family の延長より先に at-most-one symmetry と non-generic continuation boundary を固める方が自然だと確認できた。

## Open questions

- unsupported direct child line の wording を dedicated family として安定化するか、generic unexpected child wording に留めるか。
- `missing multiline ensure block` を次段 immediately 扱うか、それとも full request compare comparison の後へ回すか。

## Suggested next prompt

`Phase 3 の次段として、duplicate ensure と unsupported direct child line の helper-local malformed/source pair を actualize してください。`
