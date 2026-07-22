# WRK-0013 retained-reproduction selection (R-2350)

## Title and identifier

R-2350 records the post-WRK-0012 source screen and selects only the next
registration target. It is not WRK-0013 and authorizes no outcome command.

## Objective

Determine whether an existing, admissible retention path supports a genuinely
forward L3 retained-reproduction question after WRK-0012 froze.

## Scope and assumptions

WRK-0012 remains frozen. Its two sidecars at
`2242901a44d3feb7708f82ff535d91bff4fbe143` are possible future inputs only;
R-2347 output is historical metadata and cannot become successor evidence.

## Start state / dirty state

`main` and `origin/main` were clean at
`98f1ccc9945d74d1a9ccb12a890864f0f1cc65bd`. Task 31 required a source screen
before another record could be registered.

## Documents consulted

Canon README and MAP, ADR-0014, working README, frozen WRK-0012,
`plan/00-index.md`, `plan/170`, existing unnumbered WRK evidence plans,
R-2347/R-2348, validators and their tests, `Documentation.md`,
`docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
were consulted. Canon remains normative.

## Actions taken

Inspected the numbered-plan rule, allowed LAB roots, evidence-delta boundary,
and existing unnumbered plan convention. A disposable unnumbered-plan probe
passed unchanged documentation and source-hierarchy validators. Planner Euler
and a temporary Oracle advisory independently reviewed the narrowed proposal.
Selected WRK-0013 as retained reproduction only, then updated plan memory and
current snapshots. No WRK-0013 file or outcome command was created or run.

## Files changed

- `plan/wrk-0013-retained-reproduction-selection.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2350-wrk0013-retained-reproduction-selection.md`

## Commands run

Read working-annex and validator source, inspected committed sidecar ownership,
ran the disposable unnumbered-plan probe with documentation/source-hierarchy
validation, and ran focused diff/documentation/index validation before commit.

## Evidence / outputs / test results

The disposable unnumbered plan file passed `python3 scripts/validate_docs.py`
and `python3 scripts/check_source_hierarchy.py` unchanged; it remains outside
the repository. Source inspection confirmed that only numbered filenames are
statically registered and that existing `plan/wrk-0008` through `wrk-0011`
files are indexed. Planner and Oracle both conditionally recommended a fresh
retained-reproduction record, provided it pins old sidecars only as inputs,
runs after registration, and owns a new result commit.

## What changed in understanding

The available next question is operational provenance, not a second claim that
the carrier works. An unnumbered, already-indexed plan convention supplies a
potential retention artifact without repairing WRK-0012 or changing validators.

## Open questions

WRK-0013 has not tested this path. A future registration must still pin exact
input hashes, declare its full falsifier, and prove the actual evidence delta
under unchanged validators. Numbered-plan policy remains a separate escalation.

## Suggested next prompt

Register WRK-0013 as a retained reproduction with no outcome command or
plan/index edit, then run only its new registered command sequence.

## Plan update status

`plan/` 更新済み: unnumbered selection memory and its `plan/00-index.md` entry
record the narrowed question, artifact path, stop line, and commit choreography.

## Documentation.md update status

`Documentation.md` 更新済み: the reader guide now identifies WRK-0013
registration, not a source screen, as the next action.

## docs/project-status.md update status

更新済み: the current LAB view now distinguishes frozen WRK-0012 from selected
but unregistered WRK-0013 retained reproduction.

## progress.md update status

`progress.md` 更新済み: current milestone, macro phase, feature boundary, and
dated log now identify registration as the next self-driven boundary.

## tasks.md update status

`tasks.md` 更新済み: task 31 is closed selection and task 32 is registration.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface, or
workflow status changed.

## Reviewer findings and follow-up

Planner Euler and a temporary Oracle advisory independently required the same
guardrails: frame this as retained reproduction; do not list the old sidecar
commit as evidence; run only after registration; and freeze rather than repair
if the exact unnumbered memo/index route fails. Their advice is distilled here;
no external transcript is normative state.

## Skipped validations and reasons

No P-COMP command, WRK-0013 creation, sidecar modification, validator change,
numbered plan, or policy escalation was attempted because this package selects
only the next registration target.

## Commit / push status

Pending fresh validation, commit, push, and remote-head verification.

## Sub-agent session close status

Planner Euler completed read-only work and is closed. No sub-agent changed
files.
