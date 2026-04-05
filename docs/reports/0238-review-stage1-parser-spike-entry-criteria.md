# Report 0238 — review stage1 parser spike entry criteria

- Date: 2026-04-05T21:40:04.460913Z
- Author / agent: Codex
- Scope: review-only pass over the uncommitted docs task for stage 1 parser spike entry criteria
- Decision levels touched: L2

## 1. Objective

Review the uncommitted docs-only task for `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
with maintainer focus on semantic correctness, drift against existing parser/checker docs,
mirror completeness, and report hygiene before commit.

## 2. Scope and assumptions

- This review treats `specs/` as the normative source of truth and `plan/` / `progress.md` / reports as mirrors.
- The review checks consistency against `specs/examples/29-current-l2-first-parser-subset-inventory.md`,
  `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`, and
  `specs/examples/73-current-l2-first-parser-spike-staging.md`.
- No normative edits were made as part of this task review.

## 3. Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/02-current-l2-ast-fixture-schema.md`
10. `specs/examples/29-current-l2-first-parser-subset-inventory.md`
11. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
12. `specs/examples/73-current-l2-first-parser-spike-staging.md`
13. `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
14. `plan/00-index.md`
15. `plan/06-surface-notation-status.md`
16. `plan/11-roadmap-near-term.md`
17. `plan/12-open-problems-and-risks.md`
18. `plan/90-source-traceability.md`
19. `progress.md`
20. `docs/reports/0237-current-l2-stage1-parser-spike-entry-criteria.md`

## 4. Actions taken

1. Loaded the required repository context in the prescribed order through `README.md`,
   `Documentation.md`, `specs/00` to `03`, `specs/09`, `plan/00-index.md`, and the relevant mirrors.
2. Inspected the uncommitted diff for the listed files and read the full current contents of
   specs 29, 30, 73, and 74 to verify staged-order and checker-boundary consistency.
3. Cross-checked the new stage 1 judgment against the parser-free AST carrier and representative
   fixtures `e3-option-admit-chain` and `e4-malformed-lineage`.
4. Checked whether mirror and traceability files reflect the new spec/report pair.
5. Ran lightweight doc hygiene validation for current working tree state.
6. Added this review report as the required non-trivial task record.

## 5. Evidence / outputs / test results

- `python3 scripts/validate_docs.py`
  - Output: `Documentation scaffold looks complete.`
  - Output: `Found 237 numbered report(s).`
- `git diff --check`
  - Output: no output
- Semantic review result:
  - no inconsistency found between the new stage 1 judgment and the accepted candidate set in spec 29,
    the first-checker floor in spec 30, or the staged order in spec 73
  - the opaque attached-slot framing in spec 74 remains docs-only and explicitly leaves carrier/API shape OPEN
- Maintainer findings:
  - missing traceability update in `plan/90-source-traceability.md`
  - duplicate numbering in `Documentation.md`
  - report-template hygiene gaps in `docs/reports/0237-current-l2-stage1-parser-spike-entry-criteria.md`

## 6. What changed in understanding

- The new spec does not materially widen stage 1; it narrows stage 1 by separating
  declaration attachment from predicate parsing.
- The real regression risk in this patch set is not semantic drift but repository-memory drift:
  `plan/90-source-traceability.md` still stops at the preceding parser-staging addendum.
- `plan/06-surface-notation-status.md` remains directionally correct without update, but it is now
  less precise than the roadmap mirrors about the stage 1 guard-slot cut.

## 7. Open questions

- Should `plan/06-surface-notation-status.md` also mirror the stage 1 opaque-slot refinement, or is
  `plan/11-roadmap-near-term.md` intended to carry that level of parser-boundary detail by itself?
- When `plan/90-source-traceability.md` is updated, should the new addendum trace only
  `plan/11-roadmap-near-term.md` / `plan/12-open-problems-and-risks.md` / `progress.md`,
  or also `Documentation.md` and `specs/00-document-map.md` as mirrors touched in the same task?

## 8. Suggested next prompt

Review and patch the remaining pre-commit hygiene issues for the stage 1 parser spike entry criteria task:
update `plan/90-source-traceability.md`, fix duplicated numbering in `Documentation.md`,
and align `docs/reports/0237-current-l2-stage1-parser-spike-entry-criteria.md` with the repository report template.
