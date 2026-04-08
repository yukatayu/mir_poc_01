# 0311 — current L2 stage 3 request clause suite first tranche comparison

## Objective

Phase 3 mainline として、request-local `require:` / `ensure:` の fixed two-slot suite floor を
helper-local / test-only actual evidence にどこまで actualize するのが最小かを narrow に比較する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- shared single attachment frame と fixed two-slot suite floor は already comparison 済みである。

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
- `specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md`
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. fixed two-slot suite floor の actualization scope を `summary only` / `suite bridge` / `full request compare` の 3 案で比較した。
2. current multiline attachment bridge と isolated predicate fragment helper の接続を保てるかを見た。
3. current next cut としては、`require_fragment_text` / `ensure_fragment_text` を持つ helper-local suite bridge を first tranche にする judgment を `specs/examples/100...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

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

- [100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md](/home/yukatayu/dev/mir_poc_01/specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- clause presence / order summary だけでは、current multiline attachment bridge と fragment compare が分断される。
- `require_fragment_text` / `ensure_fragment_text` を持つ fixed two-slot suite bridge を切れば、request-local suite owner / ordering / multiplicity / termination を hidden にせず narrow compare できる。
- full request node compare は still too early であり、request head parse を先に太らせる pressure が強い。

## What changed in understanding

- request-local suite completion の first tranche は、summary-only helper ではなく clause slot ごとの fragment bridge を持つ helper として切る方が staged line に合う。
- duplicate clause や blank-line family の public wideningを deferred にしたままでも、helper-local structural fail-closed を first tranche に含めてよい line が見えた。

## Open questions

- fixed two-slot suite bridge helper が multiline attachment bridge の logic をどこまで internal reuse するか。
- first tranche で fail-closed に含める structural check をどこまで success-side compare と同居させるか。

## Suggested next prompt

`Phase 3 の次段として、request-local require/ensure の fixed two-slot suite bridge first tranche を helper-local / test-only actual evidence として実装し、success-side compare と最小 structural fail-closed smoke を通してください。`
