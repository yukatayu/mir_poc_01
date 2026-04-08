# 0307 — current L2 stage 3 suite vs malformed sequencing

## Objective

Phase 3 mainline として、shared single attachment frame first tranche の次段を

- request-local clause suite completion
- multiline attachment malformed-source pair extension

のどちらから開くべきかを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- declaration-side `admit:` は current stage で at-most-one attachment として読む。
- task-start dirty state は `15a6e03` push 後の clean worktree を前提とする。

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
- `specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md`
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. shared single attachment frame first tranche actualization 後に remaining gap を structural side と malformed side に分け直した。
2. request-local clause suite completion、multiline malformed extension、両方同時着手の 3 案を比較した。
3. current next step としては malformed pair extension より先に request-local clause suite completion を docs-only で比較する judgment を `specs/examples/98...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 14:30 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 308 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [98-current-l2-stage3-suite-vs-malformed-sequencing.md](/home/yukatayu/dev/mir_poc_01/specs/examples/98-current-l2-stage3-suite-vs-malformed-sequencing.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- shared single attachment frame actualization 後の次 gap は malformed wording ではなく request-local clause suite completion 側にある。
- duplicate clause や clause-between blank line の malformed familyは、suite ownership / ordering / termination の判断を先に切らないと diagnostics contract が先行しやすい。
- したがって next narrow step は request-local `require:` / `ensure:` の sibling suite structural floor comparison に置くのが最も自然である。

## What changed in understanding

- multiline attachment bridge の first tranche は、request-local clause suite completion と malformed pair extension を分けて進める enough floor になっていると確認できた。
- declaration-side `admit:` は still at-most-one attachment のままでよく、next step を request-local suite 側に絞っても line はねじれない。

## Open questions

- request-local clause suite completion を比較するとき、`require` / `ensure` の current preferred ordering を structural floor に含めるか。
- duplicate clause や clause-between blank line の malformed family を、suite completion comparison のどの後で actualize するか。

## Suggested next prompt

`Phase 3 の次段として、request-local require/ensure を sibling clause suite としてどこまで structural floor に入れてよいかを整理してください。`
