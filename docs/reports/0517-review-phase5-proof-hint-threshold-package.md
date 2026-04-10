# Report 0517 — review phase5 proof hint threshold package

- Date: 2026-04-10T08:34:55.944356Z
- Author / agent: Codex
- Scope: Phase 5 docs-only proof-hint threshold package review across the scoped spec, mirrors, traceability note, abstract, and package report
- Decision levels touched: no normative change; review only

## 1. Objective

Review the current Phase 5 docs-only package for proof-hint threshold and identify semantic inconsistency, stale mirror snapshot, missing traceability edges, or report hygiene problems.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`
- `docs/reports/0516-phase5-proof-hint-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 3. Actions taken

1. Read the required repo context in AGENTS order, including the Phase 5 mirrors and traceability note.
2. Cross-checked the scoped spec (`200`) against `Documentation.md`, `specs/00`, `plan/11`, `plan/12`, `plan/13`, `plan/17`, `progress.md`, `tasks.md`, and the Phase 5 research abstract.
3. Inspected the package report (`0516`) for section order, evidence quality, and command/result consistency.
4. Ran docs validation and diff hygiene checks in the current tree.

## 4. Files changed

- `docs/reports/0517-review-phase5-proof-hint-threshold-package.md`

`plan/` 更新不要

`progress.md` 更新不要

`tasks.md` 更新不要

## 5. Commands run and exact outputs

```bash
$ date '+%Y-%m-%d %H:%M:%S %Z %z'
2026-04-10 17:32:34 JST +0900

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 517 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

1. `plan/12-open-problems-and-risks.md` still summarizes the current theorem-line cut as stopping at `retained_payload_body_materialization_external_contract_payload` and leaving all payload enrichment to a later step, but `specs/examples/200...` and the other Phase 5 mirrors now place `retained_payload_body_materialization_external_contract_proof_hint` inside the current first choice and leave only `consumer_hint` / second-consumer pressure for the next reopen. This is a stale mirror, not just historical prose, because it appears in the current-state rollup paragraph. Evidence: `plan/12-open-problems-and-risks.md:294-295`, `specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md:102-110`, `plan/11-roadmap-near-term.md:16,28-31`, `progress.md:21`.
2. `docs/reports/0516-phase5-proof-hint-threshold.md` records `python3 scripts/validate_docs.py` and `git diff --check` as “pending rerun after file updates”, but both commands succeed cleanly in the current tree. That leaves the package report with stale evidence and makes its validation section weaker than the actual state. Evidence: `docs/reports/0516-phase5-proof-hint-threshold.md:69-84`, current command outputs in section 5 of this report.
3. No missing traceability edge was found in the scoped mirrors: `plan/90-source-traceability.md` already carries a dedicated addendum for spec `200` and report `0516`, and the document map, roadmap, phase gate, progress snapshot, tasks snapshot, and research abstract all point to the same next reopen (`proof-hint-ready consumer-hint / second-consumer-pressure comparison`). Evidence: `plan/90-source-traceability.md:1477-1482`, `specs/00-document-map.md:363-366`, `plan/17-research-phases-and-autonomy-gates.md:65,90,101,125`.

## 7. Changes in understanding

- The package itself is semantically coherent around spec `200`; the main defect is mirror drift in `plan/12`, not a disagreement among the spec, roadmap, progress, tasks, and abstract files.
- The traceability layer for this package is present.
- The package report needs evidence cleanup even though the underlying docs validation currently passes.

## 8. Open questions

- Should `plan/12-open-problems-and-risks.md` keep the long theorem-line rollup as a single cumulative paragraph, or should it be split so that current-state cut summaries are less likely to lag behind the latest threshold file?
- Should package reports be required to paste final command outputs instead of placeholder “pending rerun” text before close?

## 9. Suggested next prompt

Update `plan/12-open-problems-and-risks.md` and `docs/reports/0516-phase5-proof-hint-threshold.md` to remove the stale Phase 5 proof-hint package drift, then rerun `python3 scripts/validate_docs.py` and `git diff --check` and record the final outputs in the report.
