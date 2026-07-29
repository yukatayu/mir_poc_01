# 2520: P017 X1 Owner Disposition

- Date: 2026-07-29
- Author / agent: Codex
- Scope: record the owner's accepted X1 disposition for the bounded C2-B/C3
  relation-state research direction and synchronize the current LAB views.
- Decision levels touched: the human owner's disposition in `PROPOSAL-017`;
  no theory, specification, OBL, scenario, Gate, Phase, implementation, or
  public decision changed.

## Objective

Record the owner's acceptance of `X1 relation-state envelope` precisely and
open only the next ADR-0014-eligible L3 research package, without turning the
decision into a Core, Config, runtime, or source-language change.

## Scope and assumptions

`mirrorea_canon/` is normative. The user explicitly accepted X1 after reading
the decision's scope, alternatives, and non-effects. X1 applies only to V1/R1
cross-locus reads and M1 validation context. It neither decides the final
relation schema nor authorizes a direct theory or implementation edit.

## Start state / dirty state

Started clean and remote-equal at
`b08de379cc24485866975305476bc6702b409338`.

## Documents consulted

`AGENTS.md`; Canon README/MAP; ADR-0012; ADR-0014; `working/README`; Canon
style guide; PROPOSAL-012, PROPOSAL-013, and PROPOSAL-017; Plans 219--221;
`Documentation.md`; `docs/project-status.md`; `progress.md`; `tasks.md`; the
report template; and the prior P017 final-review/frontier-screen reports.

## Actions taken

Recorded X1 in PROPOSAL-017 in the same owner-disposition form used by prior
accepted proposals. Added a changelog entry and regenerated the Canon index.
Updated the proposal-preparation plan and the concise reader/current-state
documents so that they distinguish the newly authorized L3 model from any
unselected Canon carrier or operational rule.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-017-c2b-c3-relation-state-envelope.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `plan/221-c2b-c3-canon-proposal-preparation.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2520-p017-x1-owner-disposition.md`

## Commands run

Recorded the Discord task baseline; read the Canon change procedure, ADR-0014
standing predicate, P017, prior C2-B/C3 evidence, and current snapshots;
checked repository state and resources; ran `cd mirrorea_canon && python3
meta/build-index.py`, `python3 meta/build-index.py --check`, `git diff --check`,
`make docs`, and a direct `python3 scripts/validate_docs.py` confirmation.

## Evidence / outputs / test results

The regenerated Canon index contains 127 files. The index check passed.
`make docs` passed its Canon-index and source-hierarchy stages: all 761 required
paths are present. The direct documentation validator completed without an
error. `git diff --check` passed.

Resource preflight before documentation work found 25 GiB free on `/`, 6.3 GiB
available memory, and a 5.9 GiB repository dominated by the existing 5.9 GiB
`target/` directory. This documentation-only package created no heavy artifact.

## What changed in understanding

The former C2-B/C3 stop was an owner-selection boundary. It is now a bounded
research frontier: an L3 record may model and falsify the relation-state
envelope, but it must still satisfy ADR-0014 independently and stop before any
new primitive, contract, failure row, causal generator, or implementation is
assumed. X1 is not evidence that its eventual model is viable.

## Open questions

The next package must choose neither a final schema nor an operational rule in
advance. It must test the Plan 220 definition, branch/type, causality,
one-shot, observation, and save/load frontiers and stop on a reserved surface.
Receipt-rejection semantics, consumption presentation, exact causal mapping,
SaveObject placement, source ergonomics, C0/C1/C4/C5/C6/C7, and all Canon
amendments remain open.

## Suggested next prompt

Continue the X1-bounded ADR-0014 pre-registration, then execute its registered
model and falsifiers without changing Core or implementation surfaces.

## Plan update status

`plan/` 更新済み: Plan 221 now records the X1 disposition and the exact
pre-registration/stop boundary.

## Documentation.md update status

`Documentation.md` 更新済み: The reader index now identifies X1 as recorded
and limits its effect to the first L3 research package.

## docs/project-status.md update status

更新済み: Replaced the obsolete P017 decision wait with the X1-authorized L3
research frontier while retaining all theory and implementation non-effects.

## progress.md update status

`progress.md` 更新済み: Updated the semantic-kernel frontier, self-drive
status, and timestamped recent log.

## tasks.md update status

`tasks.md` 更新済み: Promoted the first bounded L3 model to the next autonomous
package and removed X1/XD as the current blocking choice.

## samples_progress.md update status

`samples_progress.md` 更新不要: No runnable sample, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

No new advisory review was needed for the mechanical recording of an explicit
owner decision. The prior independent P017 final review remains evidence only
for the proposal's boundedness. No callable sub-agent facility was available in
this environment.

## Skipped validations and reasons

Lean, runtime, and sample suites are not applicable because this package changes
only decision and status documents. No applicable documentation validation was
skipped.

## Commit / push status

Decision record committed as
`d7db0e6e097de23319ac9b872c69f20354c1b3d0` (`docs: record P017 X1
disposition`), pushed to `origin/main`, and verified equal to fetched
`origin/main`. This report-closeout update is committed and pushed next.

## Sub-agent session close status

No callable sub-agent facility was available in this environment; no sub-agent
session was opened or left active.
