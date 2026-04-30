# 1011 fairy05 reopen criteria inventory

## Objective

Make the current `FAIRY-05` reopen criteria explicit in repository memory, so the planned family stays honestly planned until the exact same-package evidence set is present.

This package is docs-first research / maintenance only. It does not promote `FAIRY-05` into the active helper surface and does not change any sample runner behavior.

## Scope and assumptions

- The target is repository memory and snapshot logs only: `plan/31`, `progress.md`, `tasks.md`, and this report.
- Reader-facing avatar docs already state the residual planned-family gate well enough, so this package does not widen or rewrite them.
- This package must not claim that `FAIRY-05` is runnable, that `carrier_choice` is settled, or that any final public avatar / visualization API is fixed.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/31-fairy05-visibility-return-carrier-bundling.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`

## Actions taken

- Re-read the current `FAIRY-05` planned-family docs to separate already fixed planned-only status from still-open widening choices.
- Added a dedicated reopen-criteria inventory to `plan/31` so the same-package prerequisites are explicit rather than inferred from surrounding prose.
- Updated `progress.md` and `tasks.md` to record the package close without reopening implementation.

## Files changed

- `plan/31-fairy05-visibility-return-carrier-bundling.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1011-fairy05-reopen-criteria-inventory.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the package |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `1ef7afb Inventory VerificationLayer naming gate` |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 16:08 JST` |
| `rg -n "FAIRY-05|reopen|reacquire|visibility-return|carrier bundling|planned family|representative slice" specs/10-open-questions.md specs/11-roadmap-and-workstreams.md plan progress.md tasks.md docs/research_abstract docs/hands_on samples/README.md \| sed -n '1,260p'` | pass | confirmed `FAIRY-05` is still a residual planned family and that reopen wording is already concentrated in `plan/31`, `specs/10`, `specs/11`, and the avatar reader-facing docs |
| `sed -n '1,240p' plan/31-fairy05-visibility-return-carrier-bundling.md` | pass | confirmed reopen conditions were present but still embedded in prose rather than inventory form |
| `sed -n '1,220p' docs/research_abstract/avatar_fairy_follow_plan_01.md` | pass | confirmed reader-facing summary already states the residual planned-family gate accurately |
| `sed -n '1,220p' docs/hands_on/avatar_fairy_follow_representative_slice_01.md` | pass | confirmed the hands-on landing already states the same stop line and did not need more detail |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |

## What changed in understanding

- The current `FAIRY-05` gate has two distinct layers:
  - fixed same-package reopen prerequisites
  - unresolved widening choices that must remain open even if the sample is later reopened
- The fixed prerequisites are already narrow and implementation-oriented:
  - one positive reacquire-after-return sample
  - at least one negative companion
  - explicit `state_timeline` and `anchor_switch` evidence
  - docs / report / snapshot sync
- The unresolved questions are still substantial:
  - `carrier_choice`
  - exact helper-local naming
  - how broad the negative companion set must become
  - whether widening stays in the same helper or moves behind another helper boundary

## Open questions

- If `FAIRY-05` is reopened later, should the negative companion requirement be satisfied by exactly one companion case, or should missing-return-witness and stale-membership both be mandatory?

## Suggested next prompt

Continue with the next safe docs-first research package: inventory projection-equivalence evidence boundaries or another open mixed-gate reopen criterion without promoting implementation.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/31`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No sample/Cargo floor was rerun because this package only clarified the reopen criteria in repository memory and snapshot docs.
- Reader-facing docs were intentionally left unchanged because their current planned-family gate wording already matched the refined repository-memory reading.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the docs-focused floor is rerun.

## Sub-agent session close status

- `Raman` (`docs_researcher`) was started for a parallel read but was closed before completion once local evidence proved sufficient. No sub-agent findings were incorporated into this package.
