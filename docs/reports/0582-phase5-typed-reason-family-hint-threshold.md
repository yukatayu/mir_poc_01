# Report 0582 — Phase5 Typed Reason Family Hint Threshold

## 1. Objective

Phase 5 checker-side line の current next pressure として、

- `typed_reason_family_hint` を checker-cluster row に足してよいか
- 足すとしても payload をどこまでに留めるか
- multi-family row の over-read を抑えるために lightweight `coverage_state` を足すべきか

を narrow に整理し、`specs/examples/259...261` と mirror / snapshot を current package close まで揃える。

## 2. Scope and assumptions

- current L2 / small decidable core / checker-cluster matrix line だけを扱う。
- actual checker helper、public checker API、actual payload schema には進まない。
- `typed_reason_family_hint` は row core ではなく attachment 候補としてのみ比較する。
- `supported_kind_refs[]` のような actual kind summary は current package では still later に残す。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/40-current-l2-typed-static-reason-family-selection.md`
- `specs/examples/41-current-l2-lineage-edge-pair-static-reason-actualization.md`
- `specs/examples/42-current-l2-missing-option-static-reason-actualization.md`
- `specs/examples/43-current-l2-capability-strengthening-static-reason-actualization.md`
- `specs/examples/49-current-l2-static-gate-minimal-artifact-schema.md`
- `specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
- `specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md`
- `specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md`
- `docs/reports/0143-current-l2-lineage-edge-pair-static-reason-actualization.md`
- `docs/reports/0144-current-l2-missing-option-static-reason-actualization.md`
- `docs/reports/0168-first-typed-static-reason-family-selection.md`
- `docs/reports/0578-phase5-handoff-low-level-memory-order-threshold.md`
- `docs/reports/0579-review-phase5-handoff-low-level-memory-order-threshold.md`
- `docs/reports/0580-phase5-checker-cluster-row-and-evidence-attachment-package.md`
- `docs/reports/0581-review-phase5-checker-cluster-row-and-evidence-attachment-package.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`

## 4. Actions taken

1. `specs/examples/259...261` を追加し、checker-cluster line の current next pressure を 3 段に分けて比較した。
2. `typed_reason_family_hint` は row core ではなく optional attachment に留める current first choice を固定した。
3. hint payload は single ref ではなく `family_refs[]` minimal bundle に留める current first choice を固定した。
4. multi-family row の over-read を抑えるため、`supported_kind_refs[]` ではなく lightweight `coverage_state` を足す current first choice を固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に更新した。

## 5. Files changed

- `Documentation.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0582-phase5-typed-reason-family-hint-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/259-current-l2-checker-cluster-fixture-evidence-attachment-ready-typed-reason-family-hint-threshold.md`
- `specs/examples/260-current-l2-typed-reason-family-hint-ready-checker-cluster-hint-bundle-shape-comparison.md`
- `specs/examples/261-current-l2-checker-cluster-hint-bundle-shape-ready-typed-family-coverage-state-threshold.md`
- `tasks.md`

## 6. Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 14:48 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 582 numbered report(s).

$ git diff --check
[no output]

$ git status --short --branch
## main...origin/main
 M Documentation.md
 M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M tasks.md
?? docs/reports/0578-phase5-handoff-low-level-memory-order-threshold.md
?? docs/reports/0579-review-phase5-handoff-low-level-memory-order-threshold.md
?? docs/reports/0580-phase5-checker-cluster-row-and-evidence-attachment-package.md
?? docs/reports/0581-review-phase5-checker-cluster-row-and-evidence-attachment-package.md
?? docs/reports/0582-phase5-typed-reason-family-hint-threshold.md
?? docs/reports/0583-review-phase5-current-docs-only-package.md
?? specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md
?? specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md
?? specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md
?? specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md
?? specs/examples/259-current-l2-checker-cluster-fixture-evidence-attachment-ready-typed-reason-family-hint-threshold.md
?? specs/examples/260-current-l2-typed-reason-family-hint-ready-checker-cluster-hint-bundle-shape-comparison.md
?? specs/examples/261-current-l2-checker-cluster-hint-bundle-shape-ready-typed-family-coverage-state-threshold.md
```

### specs/examples/259

- `typed_reason_family_hint` は current checker-side line に入れてよい。
- ただし row core ではなく optional attachment に留める。
- actual kind row / payload は still later に残す。

### specs/examples/260

- `typed_reason_family_hint` payload は single ref ではなく `family_refs[]` minimal bundle を current first choice にする。
- 理由は、same-lineage の 1-family row と malformed / underdeclared の multi-family row を同じ shape で扱えるからである。

### specs/examples/261

- `family_refs[]` だけでは partial coverage row を over-read しやすい。
- そのため current first choice は `coverage_state = full_cluster | partial_cluster` を 1 段だけ足すことである。
- `supported_kind_refs[]` は still later に残す。

## 7. What changed in understanding

- Phase 5 checker-side line は、minimal row core と `fixture_evidence_refs` attachment だけでなく、typed static reason actualization との proximity を **optional symbolic attachment** として 1 段だけ見せてよい段階に入っている。
- ただし、その proximity は actual checker payload や public checker API へ ratchet する前に、`family_refs[]` と lightweight `coverage_state` で止める方が current repo の docs-first discipline に合う。
- low-level memory-order family を Phase 5 theorem-line retained bridge に further actualize するより、small decidable core / checker-cluster line を narrow に整理する方が current next pressure として自然である。

## 8. Open questions

- `supported_kind_refs[]` のような actual kind summary を current checker-side line に足すべきか、それとも `coverage_state` で十分か。
- `coverage_state` の exact naming を later public line に持ち上げるべきか。
- request / predicate / `try` cluster に typed family actualization が later で広がったときも、この hint bundle shape をそのまま使えるか。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 9. Suggested next prompt

`typed-family-coverage-state-ready supported-kind-summary threshold` を narrow に比較し、Phase 5 checker-side line で `supported_kind_refs[]` のような actual kind summary を current line に足すべきか、それとも `coverage_state` で止めるべきかを整理してください。
