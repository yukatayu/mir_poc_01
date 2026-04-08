# 0309 — current L2 stage 3 request clause suite structural floor

## Objective

Phase 3 mainline として、request-local `require:` / `ensure:` の sibling clause suite structural floor を
どこまで current stage に入れてよいかを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- declaration-side `admit:` は current stage で at-most-one attachment に留める。
- task-start dirty state は `bb1270b` push 後の clean worktree を前提とする。

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
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `specs/examples/98-current-l2-stage3-suite-vs-malformed-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. request-local clause suite completion を `fixed two-slot suite` / `ordered clause list` / `generic keyed clause map` の 3 案で比較した。
2. current companion notation、shared single attachment frame actualization、first checker cut lineとの整合を見た。
3. current next floor としては `perform` owner の fixed two-slot suite を採る judgment を `specs/examples/99...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 14:39 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 310 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [99-current-l2-stage3-request-clause-suite-structural-floor.md](/home/yukatayu/dev/mir_poc_01/specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- current stage の request-local clause suite structural floor は `perform` owner の fixed two-slot suite に留めるのが最小である。
- `ordered clause list` や `generic keyed clause map` を先に入れると、current companion line より抽象 shape が先行しやすい。
- duplicate clause や clause-between blank line の malformed family は、suite structural floor を fixed two-slot で切った後段に残す方が自然である。

## What changed in understanding

- shared single attachment frame の次段として必要なのは generic clause list ではなく、request-local sibling suite の ownership / ordering / multiplicity / termination floor だと明確になった。
- declaration-side `admit:` は still at-most-one attachment のままでよく、request-local suite 側だけを 1 段進めればよいと確認できた。

## Open questions

- current preferred ordering `require` then `ensure` を companion rule から structural floor へどこまで上げるか。
- fixed two-slot suite floor を actualize するとき、duplicate clause を malformed として同時に切るか、still later stage に残すか。

## Suggested next prompt

`Phase 3 の次段として、request-local require/ensure の fixed two-slot suite floor を helper-local / test-only structural compare にどこまで actualize するかを整理してください。`
