# Report 2452 — Oracle composition review and re-anchored research plan

- Date: 2026-07-28 06:23 JST
- Author / agent: Codex
- Scope: Critically incorporate an independent advisory review into LAB
  sequencing for C0--C7; no Canon semantics or candidate representation is
  selected.
- Decision levels touched: LAB plan, current snapshots, and reader navigation
  only.

## Objective

Turn the recorded P004/P008/P012/P013/P015 directions and bounded C1/C6
evidence into a safe autonomous research order without claiming a shared model.

## Scope and assumptions

`mirrorea_canon/` remains normative. The temporary Oracle answer is advisory;
only its source-compatible recommendations are mirrored. Frozen WRK-0025/0026
remain procedural falsifiers, not inputs to be repaired.

## Start state / dirty state

Started clean and equal to `origin/main` at
`eabaab0b5685022a12020d09a29feb0101d13139`, after WRK-0027 evidence linkage.
Root storage had 61 GiB free and approximately 6.0 GiB memory available.

## Documents consulted

- Canon README/MAP, ADR-0014, working annex, theory/01/03/04/05/06/10/11,
  spec/02/03/05/07, P004/P008/P012/P013/P015, and WRK-0004/0005/0024--0027.
- LAB Plans 180, 186, 192, 193, 199, and the current status/task snapshots.
- Temporary Oracle session `c0-c7-compositio-review-20260728` with the listed
  Canon and LAB inputs.

## Actions taken

1. Re-read current source facts, including the distinction between P004 detail
   and current grammar, P008 direction and totality domain, M1 claims and
   request identity, and C6's explicitness boundary.
2. Asked Oracle for a critical staged route and tested its high-level claims
   against local source hierarchy.
3. Added Plan 200, which re-anchors facts before splitting C0/C2 and orders
   C3/C5 before C4 integration and C7 last.
4. Synchronized reader navigation, project status, progress, and task map.
5. Registered the new numbered plan in both documentation-validation scaffolds
   after the existing registration test exposed the omission.
6. The first full `make docs` run then exposed a separate working-annex
   validator incompatibility; its correction and verification are isolated in
   Report 2453 rather than being folded into the research-plan claim.

## Files changed

- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `tasks.md`
- `docs/reports/2452-oracle-composition-review-and-reanchored-plan.md`

## Commands run

- Ordered Canon/LAB source reads and literal source searches.
- Resource check: `df -h .` and `free -h`.
- One temporary Oracle consultation, status monitoring, and local critical
  reconciliation of its conclusions.
- Focused numbered-plan registration test, `python3
  scripts/check_source_hierarchy.py`, Canon index check, and the first full
  `make docs` run.

## Evidence / outputs / test results

Oracle completed with GPT-5.6 Sol Pro after about ten minutes. The local source
check supports its key procedural recommendations: evidence cuts differ,
proposal directions are not current grammar/Core rules, P008 leaves the exact
domain unselected, P013 leaves request identity open, P012 requires explicit
future carriers, and P015 forbids hidden scalar/default shortcuts. The proposed
nominal issuance identity and all candidate models remain advisory, not Canon.

The first documentation validation failed only because a new numbered plan must
be present in both `REQUIRED` and `REQUIRED_PATHS["plan"]`. The existing
`test_all_repo_numbered_plan_files_are_registered` failed for that exact reason;
after the two-list registration it passed, and the source-hierarchy check found
750/750 required paths.

The first full `make docs` run reached the working-annex validator and stopped
because the four new WRK records declare `plan, docs/reports`, while the
validator did not recognize the direct-report operational lane. This was not a
failure of Plan 200's source hierarchy or a theory result. Report 2453 records
the separate root-cause analysis, regression test, constrained repair, and
final validation. After that repair, a stale `progress.md` last-updated header
was synchronized to its 06:23 JST log entry; the next full `make docs` passed
with Canon index 114/114 and source hierarchy 750/750.

## What changed in understanding

The next useful unit is not another broad source inventory. It is R0: a common
source-cut fact manifest, followed by separate C0 and C2 questions. This avoids
mixing grammar authority, front-end stage domain, diagnostics, equality,
identity, replay policy, and persistence into one untestable package.
Numbered plans are also a two-scaffold document contract, so their validator and
source-hierarchy registrations must be updated in the same package.

## Open questions

- Whether R0 satisfies ADR-0014 as a non-duplicative L3 literal record.
- The exact C0 totality domain and diagnostic abstraction.
- The semantic request-issuance identity and replay/persistence policy.
- C1 snapshot/evaluation candidates, C6 scalar/terminal candidates, and the
  C3--C7 candidate-local models.

## Suggested next prompt

Pre-register and execute R0 common-cut fact manifest, then open only the
standing-eligible C0-A/C2-A successors or report their reserved-boundary stop.

## plan/ update status

更新済み: Plan 200 is the detailed autonomous sequencing; Plan 199 now mirrors
the refined order and points to it.

## Documentation.md update status

更新済み: reader navigation now links the re-anchored composition plan.

## docs/project-status.md update status

更新済み: the stop line and evidence index distinguish the staged research plan
from a selected shared model.

## progress.md update status

更新済み: logical next boundary, blocker wording, research row, and dated log
now point to the R0/C0/C2 staging.

## tasks.md update status

更新済み: the autonomous composition package and C0/C2 rows now describe the
split research boundaries and their stop conditions.

## samples_progress.md update status

更新不要: no runnable sample, validation command, or workflow changed.

## Reviewer findings and follow-up

The temporary Oracle review recommended common-cut provenance control, C0/C2
splitting, C3/C5 before C4 integration, C6 scalar/terminal split, and C7 last.
Local source supports these as planning constraints. Its candidate choices,
especially nominal issuance identity, are retained only as advisory comparison
input and require a future proposal if selected.

## Skipped validations and reasons

No Lean/runtime/parser/sample run was appropriate: this package writes only a
research plan and must not create a candidate model before R0 pre-registration.

## Commit / push status

During the full-suite wait, the synchronized current snapshots were committed
and pushed as `28f3c23c9d66401f0c8f0f0855e63ba0321d92bb`; its `HEAD` matched
`origin/main`. The Plan 200 body, navigation, validator registrations, and
Reports 2452/2453 remain for the immediate follow-up commit, which will also
be pushed and parity-checked.

## Sub-agent session close status

No callable sub-agent session was available. The temporary Oracle session
completed; its external transcript is not committed, and its distilled advisory
status is recorded above.
