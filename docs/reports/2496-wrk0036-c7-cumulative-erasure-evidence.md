# Report 2496 - WRK-0036 C7 cumulative-erasure evidence

**Identifier:** `LAB-REPORT-2496`
**Date:** 2026-07-28 15:39 JST
**Status:** evidence package prepared; commit/push pending

## Objective

Execute the committed WRK-0036 outcome procedure and retain the one permitted
fixed finite countermodel: individually fiber-constant local erasures can have
a common coarsening that loses a paired observation.

## Scope and assumptions

The evidence is limited to one fenced, artifact-local Lean block in `plan/`.
Its local types and functions are mathematics only. It creates no Mir source
transformation, omitted fact, grounds/provenance relation, elaborated artifact,
acceptance algorithm, general composition law, interface, or implementation.

## Start state / dirty state

Start point was clean `main` at committed and pushed registration cut
`0c98c762f01702d9a0175e8477df42f986c4217b`, equal to `origin/main`.
WRK-0036 was already registered and its allowed artifact path did not yet
exist.

## Documents consulted

- `AGENTS.md`, Canon README/MAP, ADR-0014, and `working/WRK-0036`
- theory/03, P012, Plans 199, 200, 204, 205, and 206; WRK-0035
- reports 2494 and 2495, plus `plan/00-index.md`

## Actions taken

1. Rechecked all registered authority/LAB input files and their SHA-256
   digests against the WRK-0036 execution cut.
2. Ran the registered broad duplicate query. Its C7-relevant matches are this
   selection, registration, planned artifact, and their reports; no earlier
   statement-equivalent multi-erasure/common-coarsening countermodel was found.
3. Confirmed the disposable RED attempt fails at the intended unprovable
   assertion that the cumulative collision is fiber-constant.
4. Retained an import-free finite Lean proof of two individual
   fiber-constancy results, both explicit common-coarsening equations, a
   paired-observation collision, and the resulting negated fiber constancy.
5. Extracted the fenced source to a disposable temporary file and executed the
   registered `lean --trust=0` command.

## Files changed

- `plan/wrk-0036-c7-cumulative-erasure-countermodel.md`
- `plan/00-index.md`
- this report

## Commands run

- Registered input presence and SHA-256 checks
- Registered duplicate query over Canon and LAB search roots
- Disposable RED check: `lean --trust=0` on the failed assertion
- Registered fenced-source extraction and `lean --trust=0` command
- `#print axioms` for all six retained theorems
- Registered forbidden-token scan and `git diff --check`

## Evidence / outputs / test results

All ten authority/LAB input digests match the WRK-0036 registration cut. The
retained artifact digest before commit is
`21f7b1ab6dc5618d9ccb4050ad0358ffb3f428a146ad0f57aee78dfc04937687`.

The RED source failed as expected at `rfl` for
`not FiberConstant eraseAB pairedObserve`; this confirms the intended negative
claim was not assumed by the test harness. The extracted GREEN source passed
`lean --trust=0`. `#print axioms` reported no axioms for
`commonCoarseningA`, `commonCoarseningB`, `individualA`, `individualB`,
`cumulativeCollision`, and `cumulativeNotFiberConstant`. The forbidden-token
scan was clean and `git diff --check` passed.

The evidence establishes only this finite countermodel: two separately valid
local observations do not entail validity after their common coarsening when
the observation is considered as a pair.

## What changed in understanding

The future C7 inference/desugaring matrix needs a direct check for any final
cumulative representation. Separate approval of local factorization checks is
not a compositional shortcut. This is still a negative guard for a future
design, not a rule for Mir source syntax or elaboration.

## Open questions

- No actual C7 source representation, grounds model, or acceptance matrix has
  been selected.
- A future matrix may check every cumulative representation directly, in which
  case this countermodel remains a boundary test rather than a new decision.
- Ergonomic inference is deferred: only a future source-level design can show
  whether an omitted fact is uniquely reconstructible and whether cumulative
  checking is relevant.

## Suggested next prompt

Append outcome metadata to WRK-0036 without changing its pre-registration,
then synchronize LAB reader-facing snapshots and re-screen the autonomous
research frontier.

## Plan update status

Updated: the retained LAB evidence is indexed in `plan/00-index.md`; no
roadmap or normative decision changed.

## Documentation.md update status

更新不要: reader-facing status is synchronized only in the later snapshot
package, outside this declared evidence scope.

## docs/project-status.md update status

更新不要: official reader-facing status is synchronized only in the later
snapshot package, outside this declared evidence scope.

## progress.md update status

更新不要: current status is synchronized only in the later snapshot package,
outside this declared evidence scope.

## tasks.md update status

更新不要: the next package and blocker map are synchronized only in the later
snapshot package, outside this declared evidence scope.

## samples_progress.md update status

更新不要: no active sample root, runnable workflow, validation command, or
debug surface changed.

## Reviewer findings and follow-up

The earlier advisory review required a concrete Plan 199 consumer, a
current-cut duplicate check, and a finite countermodel rather than a general
law; all three constraints remain satisfied. No independent review is
required for this L3 evidence. No callable sub-agent session was available.

## Skipped validations and reasons

No source/elaboration, grounds, runtime, sample, or end-to-end validation was
run because WRK-0036 excludes those surfaces. No general composition theorem
was attempted because the registered result class is one countermodel.

## Commit / push status

Pending evidence-only commit, push, fetch, and `HEAD == origin/main`
verification.

## Sub-agent session close status

No callable sub-agent session was opened.
