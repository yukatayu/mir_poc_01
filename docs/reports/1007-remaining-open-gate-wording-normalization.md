# 1007 remaining-open-gate wording normalization

## Objective

Normalize the remaining targeted active-current-doc wording that still used `next open work` / `current next open work`, so current snapshot docs stop sounding like ambiguous moving-target status while preserving the actual `U1` gate and the dated historical anchors and recent-log evidence that are still intentional.

This package is maintenance only. It does not change any semantic boundary, queue priority, or implementation scope.

## Scope and assumptions

- The target is limited to active current docs on the main path: `specs/11`, the direct `plan/` mirrors, `progress.md`, and the corresponding hands-on landing page.
- Historical reports and older recent-log entries remain evidence and are not rewritten.
- `specs/00-document-map.md` and the dated addendum in `specs/10-open-questions.md` are treated as intentional historical anchors unless the wording itself misstates current authority.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`
- `docs/reports/1003-plan11-and-roadmap-mirror-point-in-time-audit.md`
- `docs/reports/1005-reader-facing-detailed-summary-audit.md`
- sub-agent result from `Hooke` (`docs_researcher`)

## Actions taken

- Audited the remaining targeted active-current-doc occurrences of `next open work` / `current next open work`.
- Confirmed via a narrow docs-research review that `specs/11` was the clearest stale-current ambiguity and that `specs/00` / `specs/10` should remain unchanged as dated historical anchors.
- Replaced the targeted active-current-doc lexical marker with `remaining open gate` or equivalent snapshot-aware phrasing in:
  - `specs/11-roadmap-and-workstreams.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/35-post-p20-hotplug-next-package-inventory.md`
  - `plan/38-post-p21-final-public-hotplug-abi-family.md`
  - `progress.md`
  - `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`
- Updated `progress.md` and `tasks.md` snapshots to record the maintenance closeout.
- Closed the `Hooke` sub-agent session after its review result was incorporated.

## Files changed

- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- `progress.md`
- `tasks.md`
- `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`
- `docs/reports/1007-remaining-open-gate-wording-normalization.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before edits |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `526335e Record full validation rerun` |
| `rg -n -F 'current next open work' .` | pass | active hits isolated to `specs/11`, `plan/17`, `plan/35`, and one hands-on page before edits |
| `rg -n -F 'next open work' specs/11-roadmap-and-workstreams.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md plan/35-post-p20-hotplug-next-package-inventory.md plan/38-post-p21-final-public-hotplug-abi-family.md progress.md docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md` | pass | broader active-current-doc hit set identified before edits; after edits, only historical recent-log rows in `progress.md` still matched |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 15:47 JST` |
| `python3 scripts/check_source_hierarchy.py` | pass | expected source-hierarchy set present |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold valid after wording normalization |
| `git diff --check` | pass | no whitespace or merge-marker issues |

## What changed in understanding

- The problematic drift was no longer limited to reader-facing summaries; the same wording had survived in active `specs/11`, `plan/` mirrors, and one hands-on landing page.
- `specs/00` and the dated `specs/10` addendum lines are better treated as deliberate historical anchors than as stale active references, so changing them in the same package would have reduced precision rather than improved it.
- `remaining open gate` is a better fit than `next open work` for the current repo state because the remaining item is a user-facing commitment gate, not another self-driven package on the same implementation queue.
- Historical recent-log rows are evidence, not active authority, so leaving their original wording intact is more precise than rewriting them during a maintenance-only editorial package.

## Open questions

- Should a future docs-maintenance package codify a machine-checkable lint for phrases like `next open work`, `next queue`, or other wording that tends to overstate current authority in active docs?

## Suggested next prompt

Continue with the next safe maintenance package: inspect whether the remaining current docs need a small machine-checkable stale-wording lint, or else move to the next validation / hygiene sweep until `U1` actual commitment is provided.

## plan/progress/tasks/samples updates

- `plan/`: updated
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- Full sample / cargo floor was not rerun in this package because `1006` had already taken a fresh full checkpoint at `2026-04-30 15:39 JST`, and this package changed wording only. The docs-focused floor was rerun instead.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the report is added.

## Sub-agent session close status

- `Hooke` (`docs_researcher`) completed a narrow wording audit and was closed in this package.
