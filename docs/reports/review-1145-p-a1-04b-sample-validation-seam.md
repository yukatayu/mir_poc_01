# Report 1145 — Review P-A1-04b Sample / Validation Seam

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review `P-A1-04b` sample rows, expected hot-plug reports, and validation/test coverage requirements for stale-membership reject, missing-witness reject, and a narrow object-attach seam
- Decision levels touched: none

## Objective

Return precise practical alpha-1 sample/test coverage requirements and drift risks for the next `P-A1-04b` package without collapsing the current layer-only hot-plug first floor into Stage-D/package/runtime completion.

## Scope and assumptions

- Review-only task except for this required report.
- Focused on the user-requested files plus the minimum normative/runtime anchors needed to judge sample matrix coverage honestly.
- `RUN-02` stale-membership evidence is treated as runtime evidence only unless a distinct attach-time hot-plug verdict row exists.

## Start state / dirty state

- start state: `main` at `5f62503a326016d0718c100608860bccb3d3cbec`
- worktree status: clean before this report

## Documents consulted

- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
- `docs/reports/review-1140-p-a1-04-boundary.md`
- `docs/reports/1144-p-a1-04a-layer-only-practical-hotplug-first-floor.md`

## Actions taken

1. Read the named `sub-agent-pro` handoff first, then the required repository front-door documents in AGENTS order.
2. Compared `specs/18` and `plan/44` hot-plug requirements against the current `scripts/practical_alpha1_attach.py` row inventory and tests.
3. Checked current practical package/expected taxonomy and lower hot-plug implementation gates to see what exact new rows are still missing and where drift can happen.

## Files changed

- `docs/reports/review-1145-p-a1-04b-sample-validation-seam.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,260p' sub-agent-pro/alpha-1/08-sample-matrix.md
rg --files README.md Documentation.md specs plan samples/practical-alpha1 scripts
date '+%Y-%m-%d %H:%M:%S %z %Z'
sed -n '1,220p' README.md
sed -n '1,240p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,240p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,280p' specs/18-practical-alpha1-scope.md
sed -n '1,220p' plan/00-index.md
sed -n '1,320p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' scripts/practical_alpha1_attach.py
sed -n '1,260p' scripts/tests/test_practical_alpha1_attach.py
sed -n '1,260p' samples/practical-alpha1/packages/README.md
sed -n '1,260p' samples/practical-alpha1/expected/README.md
find samples/practical-alpha1/packages -maxdepth 1 -mindepth 1 -type d | sort
find samples/practical-alpha1/expected -maxdepth 1 -type f | sort
rg -n "stale|witness|object|attach_profile|hotplug_plan_scope|reason_family|rejection_reason_refs|membership" -S scripts/practical_alpha1_attach.py scripts/tests/test_practical_alpha1_attach.py samples/practical-alpha1/packages samples/practical-alpha1/expected plan/44-practical-alpha1-roadmap.md specs/18-practical-alpha1-scope.md
rg -n "P-A1-04b|HP-A1-06|HP-A1-07|object package attach|missing-witness|stale-membership attach|freshness/missing-witness|safe object-attach seam" -S .
rg -n "package_kind|layer_kind_allowed|attach_profile_supported|missing_witness|membership|object_attach_claimed|freshness_negative_complete|hotplug_plan_scope|AttachPoint|object" crates/mir-ast/src/practical_alpha1_hotplug_plan.rs crates/mir-runtime/src/practical_alpha1_hotplug.rs crates/mir-runtime/tests/practical_alpha1_hotplug.rs crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git status --short
find docs/reports -maxdepth 1 -type f -name '*.md' | sed 's#.*/##' | sort | tail -n 10
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- `scripts/practical_alpha1_attach.py` currently enumerates only `HP-A1-01..05`, and its own stop-lines/limitations still state that object attach and stale-membership/missing-witness completion are not implemented.
- `specs/18` still requires the package/hot-plug stage to cover object attach plus typed reject for missing witness and stale membership.
- `plan/44` still defines `P-A1-04b` exactly as “freshness/missing-witness negatives plus object-attach seam”.
- The current hot-plug plan builder still rejects any non-`layer` package kind at plan-construction time, so object attach cannot be claimed without a deliberate new seam.
- No validation commands were executed for pass/fail status in this review-only task.

## What changed in understanding

- The missing work for `P-A1-04b` is not just “more `HP-*` rows”; it is a three-part seam:
  - attach-time freshness rejection must become its own practical hot-plug evidence, not reuse `RUN-02`
  - object attach must become a new practical package-kind path, not an Alpha-0 avatar/package promotion
  - `check-all` / `closeout` completion booleans must stop hard-coding `False` once those rows actually exist
- The current practical hot-plug tests already prove exact report matching and one malformed auth-profile mutation, but they do not yet prove attach-time freshness or object-package semantics are package-content-driven.

## Open questions

- Which single object attach path is the narrowest honest `HP-A1-06` seam: pure inert inspectable object, placeholder object, or object with explicit fallback representation?
- Should attach-time freshness split into two dedicated rows outside the current `HP-A1-01..07` numbering, or should it be represented as subcases under `HP-A1-06`? The current matrix does not assign IDs for those negatives.

## Suggested next prompt

Implement `P-A1-04b` with one narrow `HP-A1-06` object-package attach row plus two dedicated practical attach-time negatives for stale membership and missing witness, then update `scripts/practical_alpha1_attach.py`, Rust/Python tests, expected reports, and closeout booleans so `freshness_negative_complete` and `object_attach_claimed` are evidence-backed rather than hard-coded.

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory の事実関係は変更していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり current snapshot の叙述は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current progress snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり current task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  `specs/18` requires object attach plus typed reject for incompatible patch, missing capability, missing witness, and stale membership at the package/hot-plug stage, but the current practical attach script only inventories `HP-A1-01..05` and hard-codes the remaining seam as incomplete. `scripts/practical_alpha1_attach.py` lines 16-47, 49-60, and 145-156 show that no stale-membership row, no missing-witness row, and no object-attach row are present yet.

- Finding 2:
  `P-A1-04b` needs three exact new practical artifacts, not one umbrella update:
  - one new package row for `HP-A1-06` object attach because the matrix already reserves that ID
  - one exact expected hot-plug report for the accepted object attach path
  - two exact expected hot-plug reports for attach-time `stale_membership` and `missing_witness` rejects
  The matrix reserves `HP-A1-06` for object attach in `sub-agent-pro/alpha-1/08-sample-matrix.md` lines 34-40, while `plan/44-practical-alpha1-roadmap.md` lines 108-116 defines `P-A1-04b` as freshness/missing-witness negatives plus object-attach seam.

- Finding 3:
  Reusing `RUN-02` would be a category error. `RUN-02` is local-runtime dispatch rejection, but `specs/18-practical-alpha1-scope.md` lines 180-185 require typed reject inside the package/hot-plug stage itself. The current script also encodes this distinction explicitly by keeping `freshness_negative_complete = False` and by warning that stale-membership/missing-witness completion is not yet part of attach.

- Finding 4:
  The current implementation still hard-gates hot-plug plan construction to `package_kind == "layer"`. `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs` lines 102-115 therefore make `HP-A1-06` impossible unless the plan surface is widened intentionally. The safe seam is to add exactly one explicit non-layer kind with its own attachpoint/profile rules and exact report, not to silently generalize all package kinds.

- Finding 5:
  The current Python helper tests are too weak for `P-A1-04b`. `scripts/tests/test_practical_alpha1_attach.py` lines 26-56 only assert the stage stays incomplete, lines 58-66 only assert the hotplug-plan boundary key exists, and lines 84-100 only assert bare package-path normalization. Missing tests:
  - a mutated stale-membership fixture proving attach verdict changes from package contents
  - a mutated missing-witness fixture proving attach verdict changes from package contents
  - a closeout/check-all test that flips `object_attach_claimed` and `freshness_negative_complete` only when the new rows are present
  - a direct-package-path `check` test for the new object package row

- Finding 6:
  The current Rust tests are also incomplete for `P-A1-04b`. `crates/mir-runtime/tests/practical_alpha1_hotplug.rs` covers exact `HP-A1-01..05` equality plus one malformed auth-profile mutation, but nothing for membership drift, missing witness, or object package kind. `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs` likewise proves only layer-path lowering and missing contract-update rejection. `P-A1-04b` needs:
  - one plan test that lowers the accepted object package row through the distinct hotplug-plan carrier
  - one plan/runtime test that rejects stale membership with `reason_family = membership_freshness`
  - one plan/runtime test that rejects missing witness with a witness-specific reason family or reason refs
  - one mutated temporary package test per new negative, so the verdict is proven to depend on manifest/hotplug input rather than curated sample ID

- Finding 7:
  `samples/practical-alpha1/packages/README.md` lines 10-16 and `samples/practical-alpha1/expected/README.md` lines 5-13 need concrete taxonomy widening together with the new artifacts. Without that, the filesystem could gain new freshness/object rows while the human-facing front door still claims the hot-plug family is just the existing layer-only floor.

## Skipped validations and reasons

- Cargo/Python validation commands were not run because the user asked for a review of required coverage and drift risk, not implementation verification.
- No focused sample execution was run because current gaps were provable from the script/test/spec inventory alone.

## Commit / push status

Pending at report write. No commit created for this review-only task.

## Sub-agent session close status

- No sub-agents used.
