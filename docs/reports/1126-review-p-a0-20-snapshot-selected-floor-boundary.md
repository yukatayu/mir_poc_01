# Report 1126 — P-A0-20 snapshot-selected floor wording boundary review

- Date: 2026-05-03 02:29 JST
- Author / agent: Codex
- Scope: docs/snapshot review for planned `P-A0-20` wording around a separate `alpha-snapshot-selected-floor` for `LIF-13` only
- Decision levels touched: none; review only

## Objective

Review the current snapshot / roadmap / sample-taxonomy docs for wording drift risk if `P-A0-20` introduces a separate `alpha-snapshot-selected-floor` for `LIF-13` only.

## Scope and assumptions

- Reviewed the repository snapshot and roadmap docs named in the request.
- Read the required repo entry docs/specs first, then the requested files.
- Inspected the dirty working-tree helper/sidecar files for `LIF-13` only to confirm the proposed carrier name and validation shape.
- Did not edit normative specs, snapshot docs, or code outside this report.

## Start state / dirty state

- Working tree was not clean at start.
- Pre-existing dirty entries relevant to this review:
  - modified `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json`
  - untracked `scripts/alpha_lifetime_fallback_snapshot.py`
  - untracked `scripts/current_l2_family_snapshot_support.py`
  - untracked `scripts/tests/test_alpha_lifetime_fallback_snapshot.py`
  - untracked `scripts/tests/test_current_l2_family_snapshot_support.py`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `samples/alpha/lifetime-fallback/README.md`
- `scripts/README.md`
- `plan/01-status-at-a-glance.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- dirty working-tree context:
  - `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json`
  - `scripts/alpha_lifetime_fallback_snapshot.py`
  - `scripts/current_l2_family_snapshot_support.py`
  - `scripts/tests/test_alpha_lifetime_fallback_snapshot.py`

## Actions taken

- Read required repo entry docs/specs in repository order.
- Read the requested snapshot / roadmap / taxonomy docs.
- Searched the inspected docs for `LIF-13`, carrier-scope vocabulary, runnable-root wording, and parser/runtime bridge wording.
- Cross-checked the untracked `LIF-13` helper/sidecar files to confirm the proposed scope name `alpha-snapshot-selected-floor` and its validation mode.
- Produced reviewer findings focused on acceptance-floor widening risk, runnable-root promotion risk, parser/runtime bridge risk, and broader-row actualization risk.

## Files changed

- Added this report only:
  - `docs/reports/1126-review-p-a0-20-snapshot-selected-floor-boundary.md`

## Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg --files -g 'README.md' -g 'Documentation.md' -g 'progress.md' -g 'tasks.md' -g 'samples_progress.md' -g 'specs/*.md' -g 'plan/*.md' -g 'samples/**/README.md' -g 'scripts/README.md'
git status --short
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,240p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
rg -n "alpha-snapshot-selected-floor|LIF-13|acceptance_scope|acceptance floor|runnable root|runtime bridge|parser/runtime bridge|selected floor|selected positive|actualize|promotion|alpha-acceptance-floor|alpha-runtime-mirror-floor|alpha-static-floor" Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/alpha/README.md samples/alpha/lifetime-fallback/README.md scripts/README.md plan/01-status-at-a-glance.md plan/39-type-system-freeze-roadmap.md plan/43-alpha-e2e-roadmap.md specs/13-type-system-lifetime-fallback.md specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,260p' tasks.md
sed -n '1,260p' samples_progress.md
sed -n '1,220p' samples/README.md
sed -n '1,220p' samples/alpha/README.md
sed -n '1,220p' samples/alpha/lifetime-fallback/README.md
sed -n '1,220p' scripts/README.md
sed -n '1,260p' plan/01-status-at-a-glance.md
sed -n '1,260p' plan/39-type-system-freeze-roadmap.md
sed -n '1,260p' plan/43-alpha-e2e-roadmap.md
sed -n '1,260p' specs/13-type-system-lifetime-fallback.md
sed -n '1,260p' specs/17-mirrorea-spaces-alpha-scope.md
nl -ba Documentation.md | sed -n '45,70p'
nl -ba samples/alpha/lifetime-fallback/README.md | sed -n '1,80p'
nl -ba plan/39-type-system-freeze-roadmap.md | sed -n '20,130p'
nl -ba tasks.md | sed -n '70,150p'
nl -ba progress.md | sed -n '24,40p;80,90p;140,147p;212,216p'
nl -ba samples/README.md | sed -n '10,50p'
nl -ba samples/alpha/README.md | sed -n '1,80p'
nl -ba scripts/README.md | sed -n '24,50p'
nl -ba samples_progress.md | sed -n '1,40p;90,110p'
nl -ba plan/43-alpha-e2e-roadmap.md | sed -n '170,190p'
nl -ba plan/01-status-at-a-glance.md | sed -n '165,176p'
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,260p' docs/reports/TEMPLATE.md
sed -n '1,220p' scripts/alpha_lifetime_fallback_snapshot.py
sed -n '1,220p' scripts/current_l2_family_snapshot_support.py
sed -n '1,220p' scripts/tests/test_alpha_lifetime_fallback_snapshot.py
sed -n '1,220p' samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json
nl -ba plan/43-alpha-e2e-roadmap.md | sed -n '88,150p'
nl -ba samples/alpha/README.md | sed -n '39,76p'
nl -ba plan/01-status-at-a-glance.md | sed -n '158,174p'
nl -ba samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json | sed -n '1,80p'
nl -ba scripts/alpha_lifetime_fallback_snapshot.py | sed -n '1,80p'
nl -ba scripts/tests/test_alpha_lifetime_fallback_snapshot.py | sed -n '1,120p'
python3 scripts/validate_docs.py
git diff --check
git status --short docs/reports/1126-review-p-a0-20-snapshot-selected-floor-boundary.md
```

## Evidence / outputs / test results

- Evidence inspected from the dirty working tree shows the planned carrier shape is already named and scoped as:
  - `checked_snapshot_scope = alpha-snapshot-selected-floor`
  - `current_validation.mode = synthetic-snapshot-floor`
  - helper cluster name `alpha_lifetime_fallback_snapshot_floor`
  - dedicated unit runner `python3 -m unittest scripts.tests.test_alpha_lifetime_fallback_snapshot`
- Current snapshot docs still describe only three carrier families: `reason_codes_scope`, `acceptance_scope`, and `runtime_mirror.scope`.
- Post-report doc guardrail passed:
  - `python3 scripts/validate_docs.py` -> `Documentation scaffold looks complete. Found 1128 numbered report(s).`
  - `git diff --check` -> clean
- Report file is currently untracked:
  - `git status --short docs/reports/1126-review-p-a0-20-snapshot-selected-floor-boundary.md` -> `?? docs/reports/1126-review-p-a0-20-snapshot-selected-floor-boundary.md`

## What changed in understanding

- The planned `LIF-13` change is not just abstract wording; the dirty working tree already carries a concrete fourth helper-local carrier shape.
- The main documentation risk is not runnable-root promotion. It is carrier-taxonomy drift: front-door docs currently teach a three-carrier model, while the pending `LIF-13` helper introduces a fourth scoped carrier.
- Validation-anchor drift is also material: if snapshot docs say the new floor exists, they must expose the evidence path, because this repository consistently ties “actualized floor” language to named validation commands/tests.

## Open questions

- Is `P-A0-20` intended to claim actualization of the `LIF-13` snapshot-selected floor now, or only to reserve/update wording ahead of commit of the helper/test files?
- Should `alpha-snapshot-selected-floor` be documented as a lifetime-family-only carrier, or as a reusable carrier class that could later host other snapshot-selected rows?

## Suggested next prompt

Review the pending `P-A0-20` doc patch against report `1126`, and update all front-door carrier taxonomy / validation-anchor references so `LIF-13` is the only row moved into `alpha-snapshot-selected-floor`, without implying acceptance widening, runnable-root promotion, or parser/runtime implementation.

## Plan update status

`plan/` 更新不要:
review only; no repository-memory decision was changed

## Documentation.md update status

`Documentation.md` 更新不要:
review only; findings recorded in this report

## progress.md update status

`progress.md` 更新不要:
review only; no snapshot state was changed

## tasks.md update status

`tasks.md` 更新不要:
review only; no task map change was applied

## samples_progress.md update status

`samples_progress.md` 更新不要:
review only; no dashboard state was changed

## Reviewer findings and follow-up

- High: front-door docs currently hardcode a three-carrier taxonomy, so a `P-A0-20` patch that adds `alpha-snapshot-selected-floor` in only one place will read as acceptance-floor widening by omission. The coordination points are `Documentation.md`, `samples/README.md`, `samples/alpha/README.md`, `scripts/README.md`, `plan/39`, `plan/43`, `progress.md`, and `tasks.md`.
- High: the pending `LIF-13` sidecar/helper already claims a concrete validation mode and scope, but the inspected validation-anchor docs do not expose that evidence path yet. If snapshot docs say the floor was “added” or “actualized” without also adding the new unit-test/helper anchor, the wording will overclaim.
- Medium: grouped `LIF-11/13/15` blocker wording must be split everywhere once `LIF-13` moves. Leaving grouped inventory prose in place would blur “`LIF-13` actualized” and “`LIF-11/15` still inventory-only”, which risks broader-row actualization drift.

## Skipped validations and reasons

- Did not run unit tests or Cargo validations:
  review-only task; user requested docs/snapshot review rather than implementation verification
- Did not modify the pre-existing dirty helper/sidecar files:
  they appear to be user work or in-flight changes and were inspected only as review context

## Commit / push status

Not performed. Review-only task; this report was added locally and no commit was created.

## Sub-agent session close status

No sub-agent used.
