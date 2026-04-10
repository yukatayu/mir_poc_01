# Report 0515 — review phase5 consumer specific external contract payload threshold package

- Date: 2026-04-10 17:23 JST
- Author / agent: Codex
- Scope: docs-only review of the Phase 5 consumer-specific external contract payload threshold package and its mirrors
- Decision levels touched: none; review only

## 1. Objective

Review the current Phase 5 docs-only package for consumer-specific external contract payload threshold and check the scoped files for semantic inconsistency, stale mirror snapshot, missing traceability edges, and report hygiene problems.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
- `specs/examples/199-current-l2-theorem-line-actual-external-contract-ready-consumer-specific-external-contract-payload-threshold.md`
- `docs/reports/0514-phase5-consumer-specific-external-contract-payload-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `progress.md`
- `tasks.md`

## 3. Actions taken

1. Read the repo-level documents in the required order before judging the package.
2. Cross-checked the scoped Phase 5 spec, report, mirrors, roadmap, risks, abstract, and task snapshot against the current promoted-line wording.
3. Ran doc validation and whitespace checks to separate content drift from mechanical issues.
4. Wrote this review report.

## 4. Files changed

- `docs/reports/0515-review-phase5-consumer-specific-external-contract-payload-threshold-package.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 5. Commands run and exact outputs

```bash
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 17:23 JST
```

```bash
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 514 numbered report(s).
```

```bash
$ git diff --check
```

```bash
$ python3 scripts/new_report.py --slug review-phase5-consumer-specific-external-contract-payload-threshold-package
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0515-review-phase5-consumer-specific-external-contract-payload-threshold-package.md
```

## 6. Evidence / findings

1. `plan/12-open-problems-and-risks.md:294-295` is stale relative to the promoted Phase 5 state. It still says consumer-specific external contract payload remains a second-step candidate after `specs/examples/198...`, but `specs/examples/199...` already promotes `retained_payload_body_materialization_external_contract_payload` into the current first choice. The rest of the mirrors read `126...199` as current package close, so this risk entry is the main snapshot mismatch.
2. `docs/reports/0514-phase5-consumer-specific-external-contract-payload-threshold.md:84-87` has unresolved placeholder evidence. The report lists `python3 scripts/validate_docs.py` and `git diff --check` under commands run, but the evidence section still says both are "pending rerun after file updates". That is a report-hygiene defect because it leaves the task’s verification state ambiguous.
3. `docs/reports/0514-phase5-consumer-specific-external-contract-payload-threshold.md:29-33` omits `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` from "Documents consulted" even though the package rationale and `specs/examples/199...:27-29` rely on the settled premise that `goal_text` is the notebook-side lightweight attachment. This is a traceability/report-hygiene gap, not a semantic contradiction.
4. No further semantic inconsistency was found in the scoped mirrors. `Documentation.md:79`, `specs/00-document-map.md:361-364`, `plan/11-roadmap-near-term.md:16,28-31`, `plan/13-heavy-future-workstreams.md:196-197`, `plan/17-research-phases-and-autonomy-gates.md:64-69`, `progress.md:21,33`, `tasks.md:21,28`, `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md:278-290`, and `plan/90-source-traceability.md:1469-1474` all align on the current promoted state and next reopen line.

## 7. Changes in understanding

- The package semantics around `retained_payload_body_materialization_external_contract_payload` are internally consistent.
- The main defect is mirror hygiene in the risk register and the package report, not disagreement across the promoted-line mirrors.
- The traceability addendum in `plan/90-source-traceability.md` is present and sufficient for the updated mirrors.

## 8. Open questions

- Should `plan/12-open-problems-and-risks.md` be updated by narrowing the stale `198` summary into a `198...199` summary, or should it split the pre-`199` and post-`199` thresholds into separate bullets for auditability?
- Should report templates or review practice require direct antecedent docs such as `specs/examples/135...` whenever a package depends on a settled premise from them?

## 9. Suggested next prompt

Fix the review findings for the Phase 5 consumer-specific external contract payload package by updating `plan/12-open-problems-and-risks.md` to reflect `specs/examples/199...`, and clean up `docs/reports/0514-phase5-consumer-specific-external-contract-payload-threshold.md` so its consulted-doc list and verification evidence match what was actually used and run.
