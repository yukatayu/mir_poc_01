# Post-WRK-0011 corrected candidate selection (R-2343)

- Date: 2026-07-22 10:17 JST
- Author / agent: Codex, with read-only planner, source explorer, reviewer, and temporary Oracle advisory review.
- Scope: Correct the candidate-selection rule after WRK-0011, choose the next bounded L3 pre-registration target, and synchronize LAB repository memory.
- Decision levels touched: LAB priority disposition only; no Canon theory, OBL, Gate, Phase, conformance, or implementation decision.

## Objective

Correct the earlier root-scope reading and select the highest-value standing-
eligible next L3 candidate without creating its working record or running its
outcome commands.

## Scope and assumptions

The screen is pinned to clean `main`
`0969a52cdfa139e3f7b10beece4f0a40feffec87`. The corrected reading is that a
future WRK declares its own existing documented LAB lane(s); it is not limited
to roots retained by an older WRK. Bounded non-production source/test changes
are allowed in that declared lane, while new helper families, schemas, CI/Make,
evidence lanes, public interfaces, and production implementation remain
prohibited. This package creates no working record, candidate fixture, runtime
source change, or outcome evidence; documentation validators are updated only
to register this new numbered LAB memory file.

## Start state / dirty state

Started clean at pushed `main` `0969a52cdfa139e3f7b10beece4f0a40feffec87`,
equal to `origin/main`. The initial local draft was uncommitted when its final
review found two priority-one errors in its lane/root rule; it was replaced
before this package was validated or committed.

## Documents consulted

Canon README/MAP, ADR-0014, working README, theory/11, WRK-0007 through
WRK-0011, `plan/158`, `plan/160`, `plan/162`, `plan/165`, `plan/166`,
`plan/167`, `plan/168`, `plan/169`, the Product Alpha computational README and
two fixed P-COMP-03 manifests, the Product Alpha direct-package example and
runtime route, `Documentation.md`, `docs/project-status.md`, `progress.md`,
`tasks.md`, `samples_progress.md`, `plan/00-index.md`, the report template,
and the two plan-path registries were consulted.

## Actions taken

1. Performed local candidate mapping and requested independent planner,
   source-explorer, reviewer, and Oracle critiques.
2. Accepted the review finding that prior WRK permitted roots are not a global
   whitelist, and that bounded fixture/source material can be retained in a
   newly declared existing lane.
3. Compared candidate families and selected a fixed P-COMP-03 direct-carrier
   cut for the next pre-registration: one accepted control-flow row and one
   rejected variables/scope row, each represented only by a sidecar manifest in
   its existing Product Alpha row directory.
4. Recorded the corrected disposition in `plan/170` and synchronized current
   LAB reader/status/task snapshots. No `WRK-0012` was created and no evidence
   command was run.

## Files changed

- `plan/170-post-wrk0011-candidate-selection.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2343-post-wrk0011-candidate-selection.md`

## Commands run

Read-only canon/LAB/source inspection, `oracle status`, and a temporary Oracle
consultation were used for the screen. The corrected draft passed `git diff
--check`, `python3 scripts/validate_docs.py`, `python3
scripts/check_source_hierarchy.py`, and `(cd mirrorea_canon && python3
meta/build-index.py --check)`. The validators reported 1,497 numbered reports,
720/720 required hierarchy paths, and 91 indexed Canon files.

## Evidence / outputs / test results

The corrected reviews converge on a distinct candidate: the checked-in
P-COMP-03 fixtures are helper rows, while a separate existing `world` package
shape and `run-local` route can carry `runtime_input.mir_compute`. The selected
cut asks whether two fixed rows can use that existing direct carrier without an
implementation change. The expected falsifier is a schema, execution, or
classification mismatch that would require a prohibited change. No sample,
Lean, Rust, or evidence command was run before a working record exists.

## What changed in understanding

The apparent lack of a next candidate came from treating previous WRK roots as
permanent. The standing route is instead record-local: a future record can use
another existing documented lane when it declares exact permitted locations and
stays within the non-production boundary. The direct-carrier question is not a
claim that the helper corpus is already directly executable.

## Open questions

The future record must fix exact sidecar paths and expected CLI exit/JSON
outcomes before execution. It must also stop rather than repair any schema,
helper, runtime, CLI, or public-carrier defect discovered by the experiment.
No owner decision is required for this L3 registration, but L2 promotion and
all Canon/public consequences remain outside it.

## Suggested next prompt

Pre-register `WRK-0012-pcomp03-direct-carrier.md` with its two exact existing
Product Alpha directories and existing command set, commit and push it, then
create and evaluate only the registered sidecar fixtures.

## Plan update status

`plan/` 更新済み: `plan/170` now records the corrected standing rule, comparison,
selected P-COMP-03 cut, falsifier, and stop line; `plan/00-index.md` indexes it.

## Documentation.md update status

`Documentation.md` 更新済み: it identifies the selected but unregistered
two-row P-COMP-03 direct-carrier candidate and its non-claims.

## docs/project-status.md update status

更新済み: the control view now distinguishes the selected candidate from an
opened record and states the corrected per-record existing-lane rule.

## progress.md update status

`progress.md` 更新済み: its logical, macro, feature, and dated recent-log
snapshots now show selected-but-unregistered P-COMP-03 research without a
phase, gate, or workflow-status claim.

## tasks.md update status

`tasks.md` 更新済み: package 29 is the corrected closed selection and package
30 is the ready self-driven registration, with the Full System line corrected
to a conditional reserve rather than a globally excluded root.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface,
or workflow classification changed; the selected fixture work is not yet
registered or created.

## Reviewer findings and follow-up

Initial reviewer `Ptolemy` found two priority-one problems: the first draft
wrongly made prior WRK roots a global whitelist and categorically prohibited
fixture/test changes. The corrected draft removes both claims. Planner `Hooke`,
source explorer `Rawls`, and the temporary Oracle review identified the
P-COMP-03 direct-carrier candidate. Final reviewer `Harvey` found no material
issue and requested three low-risk clarifications: make fixture re-encoding
conditional, distinguish the candidate source boundary from registry-script
updates, and distinguish future retained fixture changes from pinned unmodified
matrix/Rust execution machinery. All three were applied; the focused checks
were then rerun and passed.

## Skipped validations and reasons

No evidence test, Lean compilation, or sample command was run: this package has
not committed the required pre-registration, and executing an outcome command
now would violate the candidate lifecycle. The detached full documentation-unit
wrapper is not accepted as completion evidence because its outer wrapper does
not provide a final status.

## Commit / push status

Not committed at report write. After final focused validation and review, this
package will use `git commit --no-gpg-sign`, push to `origin/main`, and verify a
clean remote-tracking head.

## Sub-agent session close status

Planner `Einstein`, explorer `Maxwell`, reviewer `Ptolemy`, corrected planner
`Hooke`, and corrected explorer `Rawls` completed read-only work and were
closed. No sub-agent edited this package. One final reviewer is requested after
the corrected validation diff exists.
