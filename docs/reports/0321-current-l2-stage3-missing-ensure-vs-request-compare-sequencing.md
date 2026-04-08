# 0321 — current L2 stage 3 missing ensure vs request compare sequencing

## Objective

Phase 3 mainline として、request-local suite bridge family の first malformed/source pair actualization の後に
`missing multiline ensure block` family と fixture-side full request contract compare のどちらを先に扱うべきかを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- first malformed/source pair actualization は already 済みである。

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
- `specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md`
- `specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md`
- `specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. first pair actualization 後に残る主要 line を `missing multiline ensure block` family と fixture-side full request contract compare に絞って比較した。
2. helper-local hidden behavior の source-backed surfacing と request head parse pressure のどちらが current gap に近いかを見た。
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

- [105-current-l2-stage3-missing-ensure-vs-request-compare-sequencing.md](/home/yukatayu/dev/mir_poc_01/specs/examples/105-current-l2-stage3-missing-ensure-vs-request-compare-sequencing.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- `missing multiline ensure block` family を先に actualize する方が、full request compare より narrow で staged line に合う。
- current helper が already 持つ hidden fail-closed path を先に surfaced することで、suite bridge family の remaining malformed/source extension を閉じられる。
- fixture-side full request contract compare は request head parse pressure が強いため still later に残すのが自然である。

## What changed in understanding

- first pair actualization の次は structural compare widening ではなく、remaining hidden malformed path をもう 1 件だけ narrow に surfaced する方が自然だと確認できた。

## Open questions

- `missing multiline ensure block` wording を current helper-local string のまま維持するか、later generic multiline family と寄せるか。
- full request compare を reopen する前に bare clause line family を改めて比較する必要があるか。

## Suggested next prompt

`Phase 3 の次段として、missing multiline ensure block family の helper-local / test-only first tranche を actualize してください。`
