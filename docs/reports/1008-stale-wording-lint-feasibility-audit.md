# 1008 stale-wording lint feasibility audit

## Objective

Decide whether the recent stale-wording cleanup passes should be promoted into a machine-checkable lint now, or whether the current repo topology still makes that too noisy to automate safely.

This package is docs-first research / maintenance only. It does not implement a new lint and does not change any runtime, sample, or semantic surface.

## Scope and assumptions

- The question is limited to repo-local docs validation and wording drift around active current docs.
- Existing historical evidence surfaces (`docs/reports/`, `sub-agent-pro/`, dated addenda, and `progress.md` recent-log history) are intentionally preserved and must not be rewritten by a naive lexical scan.
- A future lint would only be acceptable if it can stay narrow, explicit, and consistent with the repo's source-hierarchy split.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/reports/1003-plan11-and-roadmap-mirror-point-in-time-audit.md`
- `docs/reports/1005-reader-facing-detailed-summary-audit.md`
- `docs/reports/1007-remaining-open-gate-wording-normalization.md`
- sub-agent result from `Lorentz` (`code_mapper`)

## Actions taken

- Inspected the current docs-validation scripts to see whether wording checks already exist and where a future lint would have to attach.
- Audited the recent stale-wording maintenance reports to identify the actual drift class that kept recurring.
- Searched active docs and historical evidence surfaces separately to see whether the candidate lexical markers are mostly active-doc drift or mostly preserved history.
- Confirmed that the current repo would produce too many false positives for a naive repo-wide lexical lint.
- Recorded the safe future boundary in `tasks.md` and the maintenance log in `progress.md`.
- Closed the `Lorentz` sub-agent session after incorporating its code-map result.

## Files changed

- `progress.md`
- `tasks.md`
- `docs/reports/1008-stale-wording-lint-feasibility-audit.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the feasibility audit |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `364b1c4 Normalize remaining-open-gate wording` |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 15:57 JST` |
| `sed -n '1,260p' scripts/validate_docs.py` | pass | confirmed the current docs check is scaffold-only and does not inspect wording |
| `sed -n '1,240p' scripts/check_source_hierarchy.py` | pass | confirmed the current hierarchy check is presence-only and already has grouped path buckets |
| `rg -n "actual next open work|current next open work|next open work|next queue|next promoted line|20[0-9]{2}-[01][0-9]-[0-3][0-9] 時点" README.md Documentation.md progress.md tasks.md specs plan docs/hands_on docs/research_abstract samples/README.md scripts/README.md --glob '!docs/reports/**' --glob '!sub-agent-pro/**'` | pass | showed that even outside reports, many intentional historical/example surfaces still carry these markers, especially `specs/00`, `specs/10`, `specs/examples/`, and snapshot recent-log lines |
| `rg -n "actual next open work|current next open work|next open work|next queue|next promoted line|20[0-9]{2}-[01][0-9]-[0-3][0-9] 時点" docs/reports progress.md sub-agent-pro \| sed -n '1,260p'` | pass | confirmed the same lexical markers are common in historical reports, handoff text, and preserved recent-log evidence |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |

## What changed in understanding

- The repo already has a clean structural split for docs validation, but it does not yet have a safe semantic-lint boundary.
- A repo-wide or directory-wide banned-phrase lint would be misleading because the same phrases legitimately appear in historical reports, dated addenda, and long-lived `specs/examples/` comparison documents.
- The only safe implementation shape, if this is revisited later, is a standalone allowlisted pass over explicitly chosen active-current docs, likely with section-aware exclusions for `progress.md` recent-log content.
- That future lint is maintenance tooling, not a new product package, and should not be promoted merely because the phrase class is annoying.

## Open questions

- Is there enough recurring drift left on the active current-doc path to justify a standalone allowlisted lint, or should the repo stay on manual wording audits until another concrete recurrence appears?

## Suggested next prompt

Continue with the next safe maintenance package: either leave stale-wording lint unimplemented and move to another docs/validation hygiene sweep, or explicitly design a standalone allowlisted active-doc lint with section-aware exclusions before writing any script.

## plan/progress/tasks/samples updates

- `plan/`: 更新不要
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No sample/Cargo floor was rerun because this package only audited docs-validation feasibility and updated snapshot/report text.
- No lint implementation or test authoring was attempted because the feasibility audit concluded that a naive repo-wide lexical scan would currently be too noisy to promote safely.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the report is added and the docs-focused floor is rerun.

## Sub-agent session close status

- `Lorentz` (`code_mapper`) completed the docs-validation surface audit and was closed in this package.
