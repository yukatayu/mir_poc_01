# Report 1119 — FAQ 013 Decision Inventory

- Date: 2026-05-02
- Author / agent: Codex
- Scope: compile an exhaustive current/future decision inventory for autonomous progress and save it to `tmp_faq/faq_013.md`
- Decision levels touched: no new normative decision; non-normative FAQ synthesis only

## Objective

Create a detailed repository-local FAQ that enumerates the current state, settled guardrails, explicit blockers, research-discovery items, alpha-lane reopen points, and user-decision gates needed for accurate autonomous progress.

## Scope and assumptions

- Preserve repository hierarchy: `specs/` remain normative, `plan/` remains repository memory, snapshots remain `progress.md` / `tasks.md` / `samples_progress.md`.
- The new file is a non-normative helper document under `tmp_faq/`.
- Do not silently create new queue decisions in the FAQ.
- Do not alter snapshot docs unless the task itself changes repo status; this task is documentation synthesis only.

## Start state / dirty state

- Started from `main` after pushed commits `e4988bd` and `1c422a7`.
- The worktree was clean at task start.
- No unrelated dirty changes were present.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `samples/alpha/README.md`
- `docs/reports/1117-p-a0-17-acceptance-artifact-closeout.md`
- `docs/reports/1118-post-p-a0-17-widening-review-blocker.md`

## Actions taken

1. Re-read the current snapshot, roadmap memory, and normative Alpha-0 specs together with base repo guardrails.
2. Extracted explicit user blockers, current reopen points, stage-map status, and strict non-claims from `progress.md`, `tasks.md`, and `samples_progress.md`.
3. Used parallel reviewers to summarize repo-wide guardrails, Alpha-0 type/layer/cut boundaries, and runtime/package/network/devtools decision points.
4. Wrote `tmp_faq/faq_013.md` as a non-normative dossier organized by authority, current state, explicit blockers, subsystem-by-subsystem judgment map, and autonomous stop conditions.
5. Prepared this report and kept snapshot docs unchanged because repo state itself did not change.

## Files changed

- `tmp_faq/faq_013.md`
- `docs/reports/1119-faq-013-decision-inventory.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/writing-plans/SKILL.md
rg -n "^(#|##|###|####|[-*] Current|[-*] Next|\\| Order |\\| lane |\\| Package|\\| A |\\| B |\\| C |\\| D |\\| E |\\| F )" README.md Documentation.md progress.md tasks.md samples_progress.md .docs/progress-task-axes.md plan/00-index.md plan/01-status-at-a-glance.md plan/39-type-system-freeze-roadmap.md plan/40-layer-compatibility-freeze-roadmap.md plan/41-save-load-checkpoint-roadmap.md plan/42-runtime-package-avatar-roadmap.md plan/43-alpha-e2e-roadmap.md docs/reports/1117-p-a0-17-acceptance-artifact-closeout.md docs/reports/1118-post-p-a0-17-widening-review-blocker.md samples/alpha/README.md
for f in specs/{00,01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,16,17}-*.md; do rg -n "^(#|##|###|####|-[[:space:]]*deferred|-[[:space:]]*current|-[[:space:]]*public|-[[:space:]]*final|-[[:space:]]*production|-[[:space:]]*open|-[[:space:]]*unresolved)" "$f"; done
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,360p' tasks.md
sed -n '1,260p' samples_progress.md
sed -n '1,260p' plan/39-type-system-freeze-roadmap.md
sed -n '1,260p' plan/40-layer-compatibility-freeze-roadmap.md
sed -n '1,260p' plan/41-save-load-checkpoint-roadmap.md
sed -n '1,260p' plan/42-runtime-package-avatar-roadmap.md
sed -n '1,260p' plan/43-alpha-e2e-roadmap.md
sed -n '1,260p' specs/13-type-system-lifetime-fallback.md
sed -n '1,260p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,260p' specs/15-cut-save-load-checkpoint.md
sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,260p' specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,220p' samples/alpha/README.md
rg -n "未決定|未解決|deferred|still later|open question|open questions|stop line|non-goal|remaining|blocked|kept-later|later" README.md Documentation.md progress.md tasks.md samples_progress.md specs/{05,06,07,08,09,10,11,13,14,15,16,17}-*.md plan/{01,39,40,41,42,43}-*.md
sed -n '181,310p' tasks.md
date '+%Y-%m-%d %H:%M:%S %Z'
```

Validation commands are recorded after execution in the evidence section.

## Evidence / outputs / test results

- `tmp_faq/faq_013.md` was created and includes:
  - authority hierarchy and source priority
  - current repo/alpha status
  - explicit user blockers
  - repo-side research/discovery items
  - subsystem-specific judgment maps for LIF/VAR/CUT/runtime-package/network/visualization/E2E
  - autonomous stop conditions and validation anchors
- Parallel reviewers returned:
  - repo-wide guardrails / user blockers / wording risks
  - runtime/package/network/devtools/E2E judgment map
  - Alpha-0 LIF/VAR/CUT decision inventory
- Validation results:
  - `python3 -m unittest scripts.tests.test_validate_docs` passed (`11` tests).
  - `python3 scripts/check_source_hierarchy.py` passed (`60/60` required paths present).
  - `python3 scripts/validate_docs.py` passed (`Documentation scaffold looks complete.`).
  - `git diff --check` passed.

## What changed in understanding

- The repo is not blocked by general ambiguity. Most ambiguity is already partitioned into explicit buckets: settled guardrails, mixed-gate research items, and user-facing `U1` decisions.
- The most practically important open issue today is not “what should the whole product be,” but “what honest carrier can widen the next remaining alpha rows without overclaim.”
- Snapshot drift is not the main risk for current autonomous work; the larger risk is confusing helper-local/runtime-private evidence with final public claims.

## Open questions

- Should future FAQ files under `tmp_faq/` be indexed anywhere, or remain intentionally ad hoc/non-normative?
- Is the next most valuable autonomous artifact another decision dossier for a narrower family, or a design note for a candidate post-`P-A0-17` carrier?

## Suggested next prompt

Read `tmp_faq/faq_013.md` and choose whether the next autonomous task should be a narrower carrier design note for one remaining positive-row family, or a pure validation/maintenance checkpoint while waiting for a user-facing `U1` decision.

## Plan update status

`plan/` 更新不要: this task synthesized existing repository memory and did not change long-lived roadmap state.

## Documentation.md update status

`Documentation.md` 更新不要: no current snapshot meaning changed.

## progress.md update status

`progress.md` 更新不要: no status/progress/blocker changed.

## tasks.md update status

`tasks.md` 更新不要: no task queue changed.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable/non-runnable sample status changed.

## Reviewer findings and follow-up

- `Faraday` reviewed repo-wide guardrails, research/discovery items, user blockers, and wording risks. Follow-up: mirror the strongest repo-wide separation and queue-authority warnings into the FAQ.
- `Carson` reviewed Alpha-0 `specs/13/14/15` and corresponding roadmap memory. Follow-up: mirror the exact helper-local acceptance boundaries, deferred lists, and blocker question shape into the FAQ.
- `Locke` reviewed runtime/package/avatar, network/transport, and Stage E/F decision points. Follow-up: mirror the concrete runtime-private subset and the remaining Stage E/F gates into the FAQ.

## Skipped validations and reasons

- No Cargo/runtime behavior tests were rerun because the task only creates a non-normative FAQ and a report; no code, snapshot status, or normative spec changed.
- No broader sample/helper validation rerun was needed because the task does not claim any new status or implementation evidence.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Faraday`: completed; closed after result capture.
- `Carson`: completed; closed after result capture.
- `Locke`: completed; closed after result capture.
