# 0301 — current L2 stage 3 fragment vs attachment sequencing

## Objective

Phase 3 mainline として、predicate fragment helper first tranche の次段を

- fragment malformed-source pair
- request head + clause attachment multiline shape

のどちらから開くべきかを narrow に比較する。

## Scope and assumptions

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- current isolated fragment helper は private / test-only に留める。
- task-start dirty state は、`be138d2` push 後の clean worktree を前提とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md`
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. predicate fragment helper first tranche の current役割を、first checker cut cluster と照合した。
2. helper-local malformed diagnostics を先に増やす案と、surface attachment bridge を先に比較する案を並べた。
3. request-local / option-local clause attachment cluster を先に見る方が staged line に合うという judgment を `specs/examples/95...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 14:01 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 302 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [95-current-l2-stage3-fragment-vs-attachment-next-step-sequencing.md](/home/yukatayu/dev/mir_poc_01/specs/examples/95-current-l2-stage3-fragment-vs-attachment-next-step-sequencing.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- `specs/examples/30...` の cluster 順を踏まえると、shared isolated predicate fragment helper の success-side compare 後に必要なのは helper-local malformed wording の追加よりも、request-local / option-local clause attachment の bridge である。
- declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment shape を shared structural floor として先に比較する方が、fragment helper を diagnostics contract 側へ unnecessary に太らせずに済む。
- したがって current Phase 3 next step は `fragment malformed pair` ではなく `attachment multiline shape comparison` へ切り替えるのが自然である。

## What changed in understanding

- predicate fragment helper の first tranche を actualize したことで、今の主不足は helper 内部の accepted fragment family ではなく、surface syntax から isolated fragment helper へどう橋を架けるかに移った。
- このため、次段は malformed diagnostics ではなく structural attachment line を切る方が staged line と整合する。

## Open questions

- declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment shape を、本当に 1 つの shared structural floor として切れるか。
- request head attachment と clause block extraction のどちらを still later stage に残すか。

## Suggested next prompt

`Phase 3 の次段として、declaration-side admit と request-local require/ensure の multiline attachment shape を shared structural floor としてどこまで比較するかを整理してください。`
