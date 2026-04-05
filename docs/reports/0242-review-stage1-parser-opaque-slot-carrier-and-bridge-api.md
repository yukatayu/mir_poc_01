# Report 0242 — review stage1 parser opaque slot carrier and bridge API

- Date: 2026-04-05T22:12:40+00:00
- Author / agent: Codex
- Scope: review-only pass over the uncommitted docs-only task for stage 1 parser-side opaque slot carrier naming and thin lowering bridge API
- Decision levels touched: L2

## 1. Objective

Review the uncommitted docs-only task for
`specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
with maintainer focus on semantic correctness, stage-boundary safety, mirror completeness,
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
13. `specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
14. `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
15. `specs/examples/73-current-l2-first-parser-spike-staging.md`
16. `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
17. `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
18. `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
19. `plan/00-index.md`
20. `plan/11-roadmap-near-term.md`
21. `plan/12-open-problems-and-risks.md`
22. `plan/90-source-traceability.md`
23. `docs/reports/0241-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
24. fixtures `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`,
    `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`,
    `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 3. Actions taken

1. Loaded the required repository context in AGENTS order through `README.md`,
   `Documentation.md`, `progress.md`, `specs/00` to `03`, `specs/09`,
   `plan/00-index.md`, and the relevant mirrors.
2. Cross-checked spec 76 against specs 02, 29, 30, 31, 37, 73, 74, and 75 to see whether
   the chosen naming and bridge cut silently widens stage 1 or weakens the stage split.
3. Checked the listed mirrors for coverage of the new judgment and verified whether
   `plan/90-source-traceability.md` cited only paths that actually exist.
4. Reviewed report 0241 against the repository report template and AGENTS report policy.
5. Added this review report as the required non-trivial task record.

## 4. Files changed

- Added: `docs/reports/0242-review-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
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
    ` M specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
    `?? docs/reports/0241-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
    `?? specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `ls docs/reports/0242-review-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
  - Output before this report was added: `ls: cannot access 'docs/reports/0242-review-stage1-parser-opaque-slot-carrier-and-bridge-api.md': No such file or directory`
- `python3 scripts/new_report.py --slug review-stage1-parser-opaque-slot-carrier-and-bridge-api`
  - Output:
    `/home/yukatayu/dev/mir_poc_01/docs/reports/0242-review-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `python3 scripts/validate_docs.py`
  - Output: `Documentation scaffold looks complete.`
  - Output: `Found 242 numbered report(s).`
- `git diff --check`
  - Output: no output

## 6. Evidence / findings

- Finding 1: Low. [specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md](/home/yukatayu/dev/mir_poc_01/specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md#L136) chooses `stage1 option declaration -> fixture OptionDecl` as the preferred bridge shape, but it does not state how that transfer avoids silently materializing stage-3-only fields such as option-local `admit`, which the fixture schema still treats as part of `OptionDecl` [specs/examples/02-current-l2-ast-fixture-schema.md](/home/yukatayu/dev/mir_poc_01/specs/examples/02-current-l2-ast-fixture-schema.md#L69) [specs/examples/02-current-l2-ast-fixture-schema.md](/home/yukatayu/dev/mir_poc_01/specs/examples/02-current-l2-ast-fixture-schema.md#L88). Because stage 1 explicitly leaves `admit` out of scope [specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md](/home/yukatayu/dev/mir_poc_01/specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md#L214), the current wording leaves room for an implementation to mirror too much of the fixture shape or to inject compatibility defaults without stating that this is compatibility-only. One sentence making that defaulting explicit would close the main early-freeze ambiguity.
- Finding 2: Low. [docs/reports/0241-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md](/home/yukatayu/dev/mir_poc_01/docs/reports/0241-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md#L61) still has report-hygiene drift relative to the repository template: it omits the template header fields (`Date`, `Author / agent`, `Scope`, `Decision levels touched`) and lists commands without exact outputs. This does not undermine the judgment itself, but it weakens reproducibility compared with neighboring review reports.
- No semantic contradiction found between spec 76 and specs 75, 73, 74, 29, or 30 on the main point: `decl_guard_slot` remains declaration-side and opaque, and the preferred bridge is still stated as structural-only rather than predicate parsing.
- No remaining mirror drift found in `Documentation.md`, `specs/00-document-map.md`, `plan/11-roadmap-near-term.md`, `plan/12-open-problems-and-risks.md`, or `progress.md`. `plan/90-source-traceability.md` already anticipated this review report as `0242`; adding the file closes that dangling path.

## 7. Changes in understanding

- The `decl_guard_slot` naming choice itself is sound and better aligned with the stage split than `lease` or generic `guard_slot`.
- The real ambiguity is not hidden predicate parsing in the current prose, but bridge shape overreach: once the bridge is framed as `option declaration -> fixture OptionDecl`, the docs should explicitly say that stage-3-only fields are passed through as compatibility placeholders rather than parsed facts.
- The mirror chain is otherwise complete for this judgment; the only repo-memory defect was the expected review-report path not existing yet.

## 8. Open questions

- Should spec 76 explicitly say that the option-level bridge may only populate the stage-1-parsed subset plus compatibility placeholders, and must not infer option-local `admit` or richer guard structure?
- Should report 0241 be normalized to the current template now, or is the team tolerating lighter report formatting on docs-only authoring tasks as long as review reports stay strict?

## 9. Suggested next prompt

Patch the remaining low-severity doc gaps by clarifying in
`specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
that the option-level bridge transfers only the stage-1-parsed declaration subset plus explicit
compatibility placeholders, then normalize report 0241 to the repository template.
