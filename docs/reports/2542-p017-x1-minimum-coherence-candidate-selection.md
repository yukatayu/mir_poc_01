# Report 2542: P017 X1 Minimum Coherence Candidate Selection

- Date: 2026-07-30 00:37 JST
- Author: Codex
- Scope: LAB selection record for one possible ADR-0014 L3 package; no source,
  implementation, or Canon semantic amendment.
- Decision level: LAB research preparation; Canon remains unchanged.

## Objective

Select or reject the next independent research package after Plan 227 without
reopening the closed P017 X1 fixture-only line or selecting an ordinary Canon
design.

## Scope and assumptions

P017 X1 is an owner-recorded direction for V1/R1 cross-locus reads only. The
selection assumes P017 and ADR-0014 remain the normative boundary. A temporary
Oracle candidate screen is advisory and was checked against those pinned
sources. The next possible L3 result will be candidate-local and erasable.

## Start state / dirty state

`main...origin/main` was clean at `9b18ba9c319dd6ca03a4312af5959613283890ff`.
Plan 227 was the latest LAB decision-preparation record. No new WRK, Lean
source, helper, schema, runtime, or sample existed at task start.

## Documents consulted

Read Canon root/Map/working process, ADR-0014, P012, P013, P017, theory/01--05
and theory/07, current Core/runtime boundary, Plans 222, 224--227,
`Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and
the current documentation validator. The temporary Oracle screen was used as
an advisory independent challenge only.

## Actions taken

Recorded the candidate screen in Plan 228. Selected only the integrated,
candidate-local minimum X1 coherence experiment with external rejection and no
observation surface. Separated this selection commit from the later WRK
registration because the authoritative validator permits a registration commit
to contain only the WRK, exact operational metadata, and a direct report.

## Files changed

- `plan/228-p017-x1-minimum-coherence-candidate-selection.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `docs/reports/2542-p017-x1-minimum-coherence-candidate-selection.md`

## Commands run

Ran clean-state inspection; read the required Canon/LAB inputs; computed and
checked the parent-cut SHA-256 digests; inspected the working-annex and report
validator rules; then ran the documentation validation commands recorded below.

## Evidence / outputs / test results

No Lean source or runtime was executed in this package. The source cut supports
one independent candidate: it uses P017's required integration rows as its
consumer and has an explicit stop/falsifier family. The fixture-only candidate
line remains closed by Plan 225, and the restore-quantifier candidate remains
duplicate by Plan 226. Documentation validation passed before commit.

## What changed in understanding

The next autonomous step is not another fixture detector and not an ordinary
Canon proposal. A reversible minimum coherence presentation can be screened in
the existing Lean lane only if it treats every required P017 row as a joint
candidate-local condition, keeps rejection external, makes no observation
claim, and stops at any surface that would become a semantic commitment.

## Open questions

Whether the registered candidate can express all required rows without a
reserved surface is untested. The actual relation schema, branch/failure
semantics, causal generators, persistence encoding, observation projection,
source convenience, and runtime remain open ordinary design work.

## Suggested next prompt

Register the selected ADR-0014 L3 record at the pinned cut, push it, and only
then materialize its single bounded existing-lane experiment.

## Plan update status

Updated: added Plan 228 and registered it in `plan/00-index.md`.

## Documentation.md update status

Updated: added the Plan 228 reader entry without claiming that a WRK or result
already exists.

## docs/project-status.md update status

Updated: recorded the selected but unregistered coherence-candidate boundary.

## progress.md update status

Updated: synchronized the current logical-specification boundary and recent
LAB log.

## tasks.md update status

Updated: replaced the next P017 X1 action with the one selected pre-registration
and stated its stop line.

## samples_progress.md update status

更新不要: no runnable sample path, command, debug surface, or blocker changed.

## Reviewer findings and follow-up

The prior independent temporary Oracle screen found this to be the only
non-duplicate candidate at the cut. Local review confirmed its important
constraint: Plan 225's fixture-only closure must not be bypassed by relabeling
another finite detector. The next package therefore needs an integrated
candidate-native model and pre-registration before any outcome source exists.

## Skipped validations and reasons

Skipped Lean execution deliberately: no evidence source may exist before the
next WRK registration is committed and pushed. No sub-agent tool is available
in this environment; the independent Oracle screen and local source review
were used instead.

## Commit / push status

Pending at report creation; this package will be committed with
`--no-gpg-sign`, pushed, and checked against `origin/main` before the next
package begins.

## Sub-agent session close status

No callable sub-agent session exists in this environment. The advisory Oracle
consultation had already completed; no active external session remains.
