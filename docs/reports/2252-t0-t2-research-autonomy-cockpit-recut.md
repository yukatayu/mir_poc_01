# Report 2252 — T0-T2 research autonomy cockpit recut

- Date: 2026-07-16
- Author / agent: Codex
- Scope: LAB management and research-operation documentation only
- Decision levels touched: no canon decision level changed

## Objective

Make the current theory-stage work understandable and self-operable without
turning LAB evidence into a canon lifecycle or proof claim.

## Scope and assumptions

The owner authorized careful autonomous research in the existing T0-T2 scope.
This task records a LAB operating envelope only. Canon package close, Gate/
Phase exit, ADR effectivity, L0/L1 decisions, and `theory/11` proof status stay
outside the scope.

## Start state / dirty state

`main...origin/main` was clean. Canon read `T0/G0 rebaseline`; ADR-0013 had
recorded the one-off T0 profile evidence, with G0-D3 deferred.

## Documents consulted

- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/121-g1-minimal-vertical-slice-candidate-map.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/154-project-control-cockpit.md`
- `plan/155-t0-g0-governance-profile-proposal.md`
- `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`
- advisory Oracle consultation on T0-T2 autonomy boundaries and stop triggers

## Actions taken

- Recorded the owner-authorized LAB research selection, stop, and
  decision-bundle protocol in `plan/156`.
- Recut the concise reader view into canon state, runnable LAB evidence,
  decision queue, and research roadmap panels.
- Replaced historical accumulation in `Documentation.md`, `progress.md`, and
  `tasks.md` with current snapshots that point to detailed `plan/` memory.
- Updated the two Mermaid maps to show the system layers and the distinction
  between research evidence and canon progression.
- Registered the new numbered plan in both required-scaffold lists.

## Files changed

- `Documentation.md`
- `docs/diagrams/layer-stack.mmd`
- `docs/diagrams/workflow.mmd`
- `docs/project-status.md`
- `plan/00-index.md`
- `plan/154-project-control-cockpit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

- `df -h .`
- `free -h`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`

## Evidence / outputs / test results

- Root filesystem had 17 GiB available before the light documentation work;
  no heavy artifact or global installation was created.
- `python3 scripts/validate_docs.py` passed and reported 1,405 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed with 704 required paths
  present and none missing.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 52 tests.
- `git diff --check` passed.

An initial attempt to name a nonexistent `scripts.tests.test_check_source_hierarchy`
module failed at unittest import. Inspection showed that source-hierarchy
coverage is intentionally included in `test_validate_docs.py`; the corrected
test command above passed. No code or test fix was needed.

## What changed in understanding

The safe autonomy boundary is not an unofficial T1 promotion. It is the ability
to choose and finish an existing-lane research work unit as LAB evidence.
Research completion, a canon package close, and a proof-status movement are
different events. Deferred G0-D3 is a dormant selection guard, not a recurring
prompt or a default next task.

## Open questions

- T-RESEARCH-001 must still determine which concrete premises make the three
  existing statement drafts non-vacuous theorem candidates.
- Any semantic premise not derivable from canon must be returned as a
  `decision-ready` bundle; no such request is made by this management recut.

## Suggested next prompt

Continue T-RESEARCH-001: reproduce the existing Surface and Lean anchors, run
the disposable finite countermodels, and record the bounded result.

## Plan update status

`plan/` 更新済み: added `plan/156`, updated the plan index, and recut
`plan/154` to the four-panel control view.

## Documentation.md update status

`Documentation.md` 更新済み: replaced the legacy accumulation with the short
reader entry point and source hierarchy.

## docs/project-status.md update status

更新済み: `docs/project-status.md` now separates canon state, runnable LAB
evidence, dormant decisions, and the bounded research route.

## progress.md update status

`progress.md` 更新済み: replaced the historical accumulation with the current
three-axis snapshot, macro map, and recent log.

## tasks.md update status

`tasks.md` 更新済み: rewrote the task map around T-RESEARCH-001, selection
rules, decision gates, and research-discovery items.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface,
or classification changed.

## Reviewer findings and follow-up

The Oracle advisory review found that autonomous work may close only as
research evidence unless the canon's own close condition is met. It also
required counterexample-first selection and explicit stop triggers. A local
planner sub-agent was attempted but its nested sandbox could not start; it
returned no findings and is not relied on as review evidence.

## Skipped validations and reasons

No build/runtime workflow was changed. Mermaid rendering was reviewed as source
only because this repository has no configured Mermaid renderer in the local
validation surface. Full runnable sample suites were not rerun because their
inputs and classifications did not change.

## Commit / push status

Pending at report write. The management recut and this report will be committed
with `--no-gpg-sign` and pushed before starting T-RESEARCH-001 execution.

## Sub-agent session close status

Oracle advisory result was received and incorporated as non-normative input.
The local planner attempt ended without output because its nested sandbox could
not initialize; no running sub-agent remains.
