# 0999 handoff path normalization and docs freshness

## Objective

Normalize the 2026-04-30 Codex handoff path to the filename that the handoff itself expects, remove stale wording around `sub-agent-pro/` and `plan/10`, and sync the current maintenance snapshot without opening any new promoted implementation line.

This is a docs / repository-memory maintenance package. It is not a `U1` decision, not a public-surface freeze, and not a hot-plug completion claim.

## Scope and assumptions

- Normative judgments remain in `specs/`.
- Long-lived repository memory remains in `plan/`.
- `progress.md` and `tasks.md` remain snapshots, not normative sources.
- `sub-agent-pro/` is handoff / directive material, not normative source.
- `U1` actual commitment remains open.
- This package is limited to docs freshness, handoff path normalization, and queue-temperature cleanup.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
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
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Normalized the 2026-04-30 handoff path from `sub-agent-pro/02_codex_mir_poc_01_research_handoff_2026-04-30.md` to `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`.
- Updated the handoff front matter from `想定配置先` to `配置先` so the file no longer describes a different intended location than its actual path.
- Rewrote `sub-agent-pro/README.md` so the directory is described as containing both question bundles and working handoffs, with explicit non-normative status and reading / mirroring rules.
- Added the new 2026-04-30 handoff to `specs/00-document-map.md`.
- Cooled `plan/10-roadmap-overall.md` so Lane B and Lane C no longer read as if there is still a promoted self-driven implementation queue after `P21`; the current reading is maintenance plus `U1` gate.
- Updated `progress.md` and `tasks.md` timestamps and maintenance notes to record this closeout without changing the actual blocker map.
- Created this report.

## Files changed

- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- `sub-agent-pro/README.md`
- `specs/00-document-map.md`
- `plan/10-roadmap-overall.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0999-handoff-path-normalization-and-docs-freshness.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | observed dirty state | initial state showed untracked `sub-agent-pro/02_codex_mir_poc_01_research_handoff_2026-04-30.md` |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `6a84925 Refresh docs freshness and dashboards` |
| `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .` | pass | `Task baseline recorded.` |
| `date '+%Y-%m-%d %H:%M JST'` | pass | `2026-04-30 13:27 JST` |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 997 numbered report(s).` |
| `git diff --check` | pass | no output |

Additional evidence:

- `rg -n 'mir_poc_01_research_handoff_2026-04-30|02_codex_mir_poc_01_research_handoff_2026-04-30' -S .` showed the naming mismatch only in the handoff file itself before normalization.
- The handoff body and current snapshot agreed that the next actual reopen point is `U1` actual commitment, so the path normalization was maintenance-only and did not require a roadmap rewrite.
- Reviewer-oriented subagents confirmed the same current line:
  no new promoted self-driven implementation package exists after `P21`; maintenance lane remains the only safe autonomous path without `U1`.

## What changed in understanding

- The most immediate stale-doc risk was not semantic drift in `specs/`, but path / role drift in `sub-agent-pro/` and temperature drift in `plan/10`.
- `plan/10` needed explicit wording that it is lane memory, not queue authority, once `progress.md` / `tasks.md` read the promoted workline as maintenance-only.
- The 2026-04-30 handoff is materially useful current-line guidance, but only once its basename stops disagreeing with its own declared placement.

## Open questions

- Should `plan/02-system-overview-and-positioning.md`, `plan/07-parser-free-poc-stack.md`, and `plan/09-helper-stack-and-responsibility-map.md` also be cooled to reduce “current next-step” misreads, or is the current distinction between snapshot authority and repository memory sufficient?
- When `U1` is actually committed, which of the provisional recommendations should become fixed first: packaging, host target, shipped surface, or catalog breadth?

## Suggested next prompt

Proceed with the next maintenance package or with actual `U1` commitment. If maintenance continues, audit remaining `plan/` temperature drift (`plan/02`, `plan/07`, `plan/09`) without inventing any new promoted queue.

## plan/progress/tasks/samples updates

- `plan/`: updated `plan/10-roadmap-overall.md`
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- Full validation floor was not run. This package changed only handoff / snapshot / repository-memory docs, so the docs-focused floor was sufficient.
- No Cargo tests were run because no Rust or sample-runner behavior changed.
- No storage audit or cleanup command was run because this package did not touch storage policy or heavy artifacts.

## Commit / push status

- Initial package close commit: `dee1636 Normalize handoff path and sync docs`
- `git push` succeeded: `main -> main`

## Sub-agent session close status

- `docs_researcher` session for `plan/01..11` completed and was closed after its summary was integrated.
- `docs_researcher` session for `plan/18..38` and the 2026-04-30 handoff completed and was closed after its summary was integrated.
- `reviewer` session was requested for final stale-wording / overclaim review, but it timed out twice without returning findings and was shut down. Final closeout therefore relies on local diff inspection plus fresh docs-floor verification evidence.
