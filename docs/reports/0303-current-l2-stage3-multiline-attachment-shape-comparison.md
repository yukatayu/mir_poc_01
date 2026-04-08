# 0303 — current L2 stage 3 multiline attachment shape comparison

## Objective

Phase 3 mainline として、declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment shape を
どこまで shared structural floor として切るのが最小かを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- request head / option declaration の full parser actualization はまだ行わない。
- task-start dirty state は `a666b8a` push 後の clean worktree を前提とする。

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
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md`
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
- `specs/examples/95-current-l2-stage3-fragment-vs-attachment-next-step-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `admit:` / `require:` / `ensure:` の multiline companion candidate を、predicate fragment helper と first checker cut inventory の line に照らして再確認した。
2. shared floor を
   - single attachment frame
   - request-local clause suite 限定
   - generic statement continuation
   の 3 案で比較した。
3. current next step としては、generic suite や generic continuation ではなく、`<clause-head>:` + deeper predicate block だけを shared structural floor にする judgment を `specs/examples/96...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 14:19 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 304 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [96-current-l2-stage3-multiline-attachment-shape-comparison.md](/home/yukatayu/dev/mir_poc_01/specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- current stage で shared にするべきなのは clause suite 全体ではなく、`<clause-head>:` + 直下 1 段深い predicate block という **single attachment frame** だけである。
- declaration-side `admit:` と request-local `require:` / `ensure:` は、この frame に限れば shared source carrier を持てる。
- 一方で request-local clause suite owner / ordering / multiplicity と option-local metadata owner は shared にしない方が、clause attachment cluster と predicate fragment cluster の分離を保ちやすい。

## What changed in understanding

- shared isolated predicate fragment helper の次段として必要なのは generic continuation rule ではなく、isolated fragment string をどう multiline source から切り出すかという attachment frame の floor だと明確になった。
- request-local clause suite と option-local metadata continuation を同時に generic 化しない方が、later-stage actualization を narrow に保ちやすいと確認できた。

## Open questions

- shared single attachment frame を helper-local actual evidence に上げるとき、multiline block extraction の compare surface を raw source retention と extracted fragment compare のどちらに寄せるか。
- request-local clause suite の exact ordering / multiplicity / dedent completion を、attachment frame actualization と同時に扱うべきか、still later stage に残すべきか。

## Suggested next prompt

`Phase 3 の次段として、declaration-side admit と request-local require/ensure の shared single attachment frame を helper-local / test-only actual evidenceにどこまで actualize するかを整理してください。`
