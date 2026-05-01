# Report 1099 — Alpha-0 save/load and Z-cycle consistency review

## Title and identifier

- Report ID: 1099
- Review target: Alpha-0 save/load / consistent-cut / Z-cycle draft
- Timestamp: 2026-05-02 06:21:01 JST

## Objective

Review the Alpha-0 save/load and Z-cycle consistency draft with focus on:

- `specs/15-cut-save-load-checkpoint.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `samples/alpha/cut-save-load/`
- touched snapshot docs that summarize save/load scope

Priority was semantic correctness around consistent-cut discipline, rollback/load resurrection, in-flight message treatment, witness/publish/observe/hot-plug closure, and distributed save/load non-claims.

## Scope and assumptions

- This task is review-only. No normative spec edits or implementation changes were made.
- Findings are based on current worktree contents, including dirty/uncommitted draft files.
- Snapshot docs were treated as non-normative summaries; `specs/` remained the normative source.

## Start state / dirty state

- Worktree was already dirty before this review.
- Notable existing dirty paths included `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `plan/00-index.md`, `plan/01-status-at-a-glance.md`, `plan/11-roadmap-near-term.md`, `specs/00-document-map.md`, `specs/11-roadmap-and-workstreams.md`, `specs/12-decision-register.md`, and the newly added Alpha-0 files under `specs/13..17`, `plan/39..43`, `samples/alpha/`.
- Existing untracked draft report: `docs/reports/1098-mirrorea-alpha0-theory-freeze-and-roadmap-sync.md`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `tasks.md`
- `progress.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/alpha/cut-save-load/README.md`
- `samples/alpha/cut-save-load/*.mir`
- `samples/alpha/cut-save-load/*.expected.json`

## Actions taken

1. Read required repo front-door and foundational specs in repository-prescribed order.
2. Read Alpha-0 save/load norm, roadmap memory, sample scaffold, and touched snapshot summaries.
3. Cross-checked save/load closure obligations against hot-plug admission and alpha completion requirements.
4. Collected review findings with file/line references.
5. Added this report and ran minimal docs validation.

## Files changed

- `docs/reports/1099-alpha0-save-load-zcycle-review.md` (new)

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
find samples/alpha/cut-save-load -maxdepth 3 -type f | sort
rg -n "save/load|checkpoint|consistent cut|rollback|resurrection|witness|publish|observe|hot-plug|distributed" Documentation.md progress.md tasks.md samples_progress.md docs plan specs samples -g '*.md'
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/15-cut-save-load-checkpoint.md
sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,260p' specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' plan/01-status-at-a-glance.md
sed -n '1,260p' plan/11-roadmap-near-term.md
sed -n '1,220p' plan/41-save-load-checkpoint-roadmap.md
sed -n '1,260p' samples/README.md
sed -n '1,220p' samples/alpha/cut-save-load/README.md
sed -n '1,220p' samples/alpha/cut-save-load/*.mir
sed -n '1,220p' samples/alpha/cut-save-load/*.expected.json
nl -ba specs/15-cut-save-load-checkpoint.md | sed -n '1,260p'
nl -ba specs/16-runtime-package-adapter-hotplug.md | sed -n '60,110p'
nl -ba specs/17-mirrorea-spaces-alpha-scope.md | sed -n '45,95p'
nl -ba plan/41-save-load-checkpoint-roadmap.md | sed -n '1,220p'
nl -ba samples/alpha/cut-save-load/README.md | sed -n '1,220p'
nl -ba progress.md | sed -n '80,95p'
nl -ba tasks.md | sed -n '132,170p'
date '+%Y-%m-%d %H:%M:%S %Z'
git branch --show-current
```

## Evidence / outputs / test results

- Review findings were derived from line-by-line comparison between `specs/15`, `plan/41`, sample scaffold rows, and touched snapshot summaries.
- No executable save/load checker/runtime exists yet in the reviewed scope.
- Validation executed after adding this report:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- The draft is mostly disciplined about not overclaiming distributed durability.
- The main semantic weakness is not overclaiming completion, but under-specifying closure classes beyond receive/observe/witness/hot-plug, especially around capability/auth provenance and stale membership/witness resurrection.
- The roadmap currently weakens one normative invariant around in-flight representation and still contains at least one stale repo-state statement.

## Open questions

- Should capability-grant / capability-use and `AuthEvidence` create/use be first-class consistent-cut closure obligations in the Alpha-0 checker, or should they be explicitly deferred? Current text suggests the former but does not carry it through.
- For stale lease / stale witness / stale membership on load, is the intended behavior hard rejection, selective invalidation, or load-with-expired-marker? Current scaffold leaves this partially ambiguous.
- Should `CUT-12` remain in the sample table now, or move behind an explicit deferred/optional subsection because communication-induced checkpointing is still later work?

## Suggested next prompt

“Address the Alpha-0 save/load review findings by tightening `specs/15`, `plan/41`, and the `samples/alpha/cut-save-load/` scaffold so that auth/capability closure, resurrection coverage, and in-flight message requirements are explicit and non-ambiguous.”

## Plan update status

`plan/` 更新不要。review-only task であり、repository memory の normative/roadmap judgmentは変更していない。

## Documentation.md update status

`Documentation.md` 更新不要。今回の task は review-only で、front-door summary wording の修正は未実施。

## progress.md update status

`progress.md` 更新不要。current status snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要。task map 自体の判断は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要。sample dashboard status は変更していない。

## Reviewer findings and follow-up

1. `specs/15` declares `capability_grant -> capability_use` and `auth_evidence_create -> auth_evidence_use` as minimum causal edges, but the consistent-cut consequences, save-state inventory, proof obligations, roadmap, and sample rows do not carry those closures through. This leaves a checker-sized hole for authorization/capability resurrection.
2. `plan/41` weakens the in-flight message rule with “explicit channel-state/in-flight representation if modeled,” which conflicts with the normative requirement that in-flight message treatment be explicit for multi-Place snapshots.
3. The load-resurrection rule covers stale lease, witness, and membership, but the CUT scaffold only exercises lease resurrection; witness/membership resurrection is still uncovered.
4. `plan/41` still says the repo lacks the Alpha-0 save/load sample family even though `samples/alpha/cut-save-load/` already exists.

## Skipped validations and reasons

- No full sample/Cargo/runtime validation was run because this task was review-only and did not modify executable code.
- No save/load runtime behavior was exercised because the reviewed package is still scaffold-only and current runners do not execute it.

## Commit / push status

- Commit: not yet created at report write time
- Push: not yet performed at report write time

## Sub-agent session close status

- No sub-agent session used.
