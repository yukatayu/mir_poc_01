# Report 0240 — review stage1 parser guard slot handoff

- Date: 2026-04-05T21:57:25.698238Z
- Author / agent: Codex
- Scope: review-only pass over the uncommitted docs task for stage 1 parser guard-slot handoff
- Decision levels touched: L2

## 1. Objective

Review the uncommitted docs-only task for
`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
with maintainer focus on semantic correctness, helper-boundary safety, mirror completeness,
traceability, and report hygiene before commit.

## 2. Inputs consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `progress.md`
5. `specs/00-document-map.md`
6. `specs/01-charter-and-decision-levels.md`
7. `specs/02-system-overview.md`
8. `specs/03-layer-model.md`
9. `specs/09-invariants-and-constraints.md`
10. `specs/examples/02-current-l2-ast-fixture-schema.md`
11. `specs/examples/29-current-l2-first-parser-subset-inventory.md`
12. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
13. `specs/examples/73-current-l2-first-parser-spike-staging.md`
14. `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
15. `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
16. `plan/00-index.md`
17. `plan/11-roadmap-near-term.md`
18. `plan/12-open-problems-and-risks.md`
19. `plan/90-source-traceability.md`
20. `docs/reports/0239-current-l2-stage1-parser-guard-slot-handoff.md`
21. fixtures `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`,
    `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`,
    `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 3. Actions taken

1. Loaded the required repository context in AGENTS order through `README.md`,
   `Documentation.md`, `progress.md`, `specs/00` to `03`, `specs/09`,
   `plan/00-index.md`, and the relevant mirrors.
2. Inspected the current contents and uncommitted diff for spec 75, spec 74, the parser-free
   AST schema, the listed mirrors, and report 0239.
3. Cross-checked the handoff judgment against representative fixture-side `lease` / `admit`
   shapes to see whether the chosen cut silently widens stage 1 or forces early predicate parsing.
4. Verified mirror and traceability coverage, including whether cited report paths actually exist.
5. Ran lightweight doc hygiene validation for the current working tree.
6. Added this review report as the required non-trivial task record.

## 4. Files changed

- Added: `docs/reports/0240-review-stage1-parser-guard-slot-handoff.md`
- No normative docs changed as part of this review.
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

- `git status --short --branch`
  - Output:
    `## main...origin/main`
    ` M Documentation.md`
    ` M plan/11-roadmap-near-term.md`
    ` M plan/12-open-problems-and-risks.md`
    ` M plan/90-source-traceability.md`
    ` M progress.md`
    ` M specs/00-document-map.md`
    ` M specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
    `?? docs/reports/0239-current-l2-stage1-parser-guard-slot-handoff.md`
    `?? specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `python3 scripts/validate_docs.py`
  - Output: `Documentation scaffold looks complete.`
  - Output: `Found 240 numbered report(s).`
- `git diff --check`
  - Output: no output

## 6. Evidence / findings

- Finding 1: `docs/reports/0239-current-l2-stage1-parser-guard-slot-handoff.md` has report-hygiene drift. It does not include an explicit `Commands run` section, and its consulted-doc list omits `plan/00-index.md` and `progress.md` even though this is a current L2 parser-boundary task and `progress.md` was updated in the same task. This is low severity, but it weakens traceability and makes the task look less reproducible than the surrounding spec / plan / mirror work.
- No semantic inconsistency found between `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`, `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`, and `specs/examples/02-current-l2-ast-fixture-schema.md`. Spec 75 preserves the stage split from spec 74, keeps predicate parsing out of stage 1, and uses the existing fixture-side `OptionDecl.lease` only as a compatibility anchor rather than as a frozen parser-side carrier.
- No remaining mirror drift found in `Documentation.md`, `specs/00-document-map.md`, `plan/11-roadmap-near-term.md`, `plan/12-open-problems-and-risks.md`, `progress.md`, or `plan/90-source-traceability.md` after adding this review report. The source-traceability addendum was already prepared to cite report 0240, so this review closes that loop.

## 7. Changes in understanding

- The parser-side opaque carrier plus thin lowering bridge cut is semantically acceptable as written because the bridge is still explicitly subordinate to the parser-free fixture schema and does not force stage 1 to absorb predicate grammar.
- The main residual risk is not a current semantic contradiction but future actualization drift: when the bridge API is specified, it should remain structural-only and avoid hidden elaboration.
- The only concrete issue in the current patch set is report hygiene in 0239, not mismatch among spec 75, spec 74, the fixture schema, or the mirror chain.

## 8. Open questions

- When the next task narrows the bridge API, should spec 75 state explicitly that lowering must not perform predicate parsing or semantic normalization beyond fixture-compatibility transfer?
- Should report 0239 be normalized to the current template now, or is the team willing to tolerate light report-format drift for docs-only tasks when the spec / plan / mirror chain itself is consistent?

## 9. Suggested next prompt

Patch the remaining report-hygiene gap for the stage 1 parser guard-slot handoff task by aligning
`docs/reports/0239-current-l2-stage1-parser-guard-slot-handoff.md` with the repository template,
then narrow the next docs-only decision for the parser-side opaque slot carrier naming and
thin-lowering bridge API while keeping the bridge structural-only.
