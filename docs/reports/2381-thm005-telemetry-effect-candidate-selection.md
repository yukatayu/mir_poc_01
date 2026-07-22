# Report 2381 - THM-005 telemetry-effect candidate selection

- Date: 2026-07-23 02:17 JST
- Author / agent: Codex with two independent read-only subagents and temporary Oracle review
- Scope: LAB candidate selection and project-state synchronization only
- Decision levels touched: L3 LAB selection; no L0/L1/L2/Canon decision

## Objective

Select at most one new, non-duplicative, standing-eligible L3 research package
after the broad no-candidate disposition, without changing Canon semantics.

## Scope and assumptions

The working-annex route in ADR-0014 is available only for a reversible,
existing-lane experiment with a pre-registered falsifier and reserved surfaces
excluded. Canon, `theory/11`, scenarios, grammar, Gate/Phase, public claims,
and implementation remain read-only or out of scope.

## Start state / dirty state

Started clean at `a0ca235fd3cb1ef4c7ef7f43f447f26812139c53`, matching
`origin/main`. Root storage had about 7.0 GiB free; no heavy build or generated
artifact was created.

## Documents consulted

`mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, theory/01, theory/04,
theory/06, theory/07, theory/11, ADR-0014, BND-008, spec/02, SCN-08,
`mirrorea_canon/working/README.md`, plans 156, 158, 171, 172, 175, and 176,
the current IFC Lean foundation, `progress.md`, `tasks.md`, and
`docs/project-status.md`.

## Actions taken

1. Re-read the Canon/LAB source cuts and separated literal issues from reserved
   repair decisions.
2. Closed two independent read-only subagent audits after incorporating their
   findings.
3. Obtained one temporary Oracle ranking consultation. It selected the
   telemetry-effect question; the raw external transcript was not committed.
4. Recorded the selection and owner escalation boundary in plan/status views.

## Files changed

- `plan/177-thm005-telemetry-effect-boundary-selection.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2381-thm005-telemetry-effect-candidate-selection.md`

## Commands run

- Canon/LAB literal searches with `rg`, `sed`, `find`, `sha256sum`, and Git
  state checks.
- `oracle status mirrorea-l3-candidate-rank-20260723`
- `make docs`
- direct Python assertion that plan 177 is registered in both documentation
  scaffolds.
- `python3 -m unittest scripts.tests.test_validate_docs` (final aggregate
  output was not recoverable from the detached command session; no pass claim
  is made for it).

## Evidence / outputs / test results

- THM-002 history-maximum variants duplicate the bounded state/trace limits
  already recorded by T-RESEARCH-005 and T-RESEARCH-011; no new L3 route was
  opened.
- SCN-08's scalar `room_anchor` declaration is incompatible with the current
  indexed `StateDecl` spelling. Resolving it would alter a frozen scenario or
  grammar and is therefore owner-reserved.
- THM-005's `occurrence DAG or a declared telemetry effect` wording admits one
  new finite dependency question. Its selected adverse control tests only a
  model premise boundary, not the Canon theorem.
- No Lean source, runnable sample, outcome command, WRK record, Canon/OBL,
  Gate/Phase, implementation, or workflow state changed in this selection
  package.
- `make docs` passed: Canon index checked 96 files, source hierarchy found all
  727 required paths, and documentation validation found 1,535 numbered
  reports. The direct registration assertion passed.

## What changed in understanding

The post-plan-176 state is not a permanent no-candidate condition. The precise
telemetry source alternative yields a distinct, narrowly testable dependency
boundary, whereas the fallback route is known evidence and the SCN issue is a
reserved specification decision.

## Open questions

- Does the registered finite telemetry model compile without selecting a new
  label, effect, provenance, or export interface?
- Which owner disposition should resolve SCN-08 scalar-state syntax when exact
  parser/scenario conformance becomes active?

## Suggested next prompt

Continue with the pre-registered THM-005 telemetry-effect L3 experiment, then
freeze or retain only its bounded evidence before selecting another route.

## Plan update status

`plan/` 更新済み: added plan 177 and indexed it.

## Documentation.md update status

`Documentation.md` 更新済み: the reader entry now points to the focused selection as well as the
historical broad disposition.

## docs/project-status.md update status

更新済み: records the 未登録 L3 pre-registration candidate and separates the SCN-08
owner issue.

## progress.md update status

`progress.md` 更新済み: records the focused selection without claiming an outcome.

## tasks.md update status

`tasks.md` 更新済み: adds the 未登録 L3 package and owner decision item.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface, or blocker
changed in this selection-only package.

## Reviewer findings and follow-up

Two subagents found no immediate theorem contradiction but identified the
THM-002 duplicate route, SCN-08 grammar mismatch, and reserved authority/core
questions. The focused temporary Oracle review independently ranked only the
THM-005 telemetry candidate as suitable for a new bounded L3 record. Follow up
by pre-registering it; do not resolve the owner-reserved findings here.

## Skipped validations and reasons

Lean and runtime checks were skipped because this package changes no executable
source. The full `test_validate_docs` process ran for about ten minutes, but
its final aggregate output was unavailable after command-session detachment;
it is intentionally not reported as passing. `make docs`, source hierarchy,
index, and direct registration checks passed.

## Commit / push status

Pending at report write; this selection package will be committed with
`--no-gpg-sign` and pushed before opening the separate WRK registration.

## Sub-agent session close status

Both independent read-only subagents were closed after reporting. The temporary
Oracle session completed; its advisory output remains external and uncommitted.
