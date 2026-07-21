# Report 2332 - WRK-0009 e5 proof-skeleton identity preregistration

- Date: 2026-07-22 06:25 JST
- Author / agent: Codex
- Scope: next standing-eligible L3 target selection and immutable pre-registration
- Decision levels touched: L3 working-record pre-registration only; no Canon theory or LAB implementation change

## Objective

Select one non-duplicative standing-eligible research target after WRK-0008 and
pre-register its alternative, falsifier, no-effect boundary, existing commands,
and stop condition before any result is interpreted.

## Scope and assumptions

The starting revision is clean pushed `main` at
`e36c804b9149e048c6e92bec2b55d21956354f2f`. `mirrorea_canon/` remains
normative. The selected question is strictly literal-transcription evidence:
the e5 tuple in the existing Lean foundation versus the tuple emitted by the
existing current-L2 static route. It does not decide what either tuple means.

## Start state / dirty state

Started clean with `HEAD == origin/main ==
e36c804b9149e048c6e92bec2b55d21956354f2f`. No user changes were present,
reverted, or overwritten. Resource check before the prospective disposable
run found 13 GiB free on `/` and 9.6 GiB memory available; Lean 4.29.1 is
installed. `/mnt/mirrorea-work` remains unavailable, so generated output will
remain in a unique disposable `/tmp` directory.

## Documents consulted

- Canon README/MAP, ADR-0014, working README, theory/06, and theory/11.
- WRK-0001 through WRK-0008, `plan/158`, `plan/165`, and plan/73.
- The current-L2 source corpus, formal-hook and theorem-stub support/tests,
  theorem-stub pipeline, and Lean foundation/sync documentation.
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `plan/00-index.md`, and the report template.

## Actions taken

1. Rejected the direct OBL-001/024/025 variants already classified as
   duplicate or carrier-dependent by plan/165.
2. Asked a read-only planner to rank new existing-lane targets and performed
   a temporary Oracle challenge review. The planner ranked literal e5 tuple
   fidelity first. Oracle agreed it is eligible if kept literal, and noted
   that an upstream-bundle audit would be preferable only if upstream records
   exposed the missing relation components.
3. Inspected the existing detached-bundle schema. It retains event kinds,
   terminal outcome, non-admissible metadata, narrative text, and step count,
   but no occurrence/locality/order/frontier/rollback-effect relation. The
   upstream audit therefore has no existing discriminating record and is not
   opened; adding one is forbidden by ADR-0014.
4. Pinned WRK-0009 to the current Canon/LAB blobs and registered only the
   existing Lean, current-L2, Cargo, and regression commands.
5. Added the immutable Canon working record and MAP entry, then regenerated
   the Canon index. No evidence command has yet run.

## Files changed

- `mirrorea_canon/working/WRK-0009-current-l2-e5-skeleton-identity.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json` (mechanically regenerated)
- this report

## Commands run

- Read-only Canon/LAB source inspection and candidate-triage searches.
- `df -h .`, `free -h`, `lean --version`, `sha256sum`, and Git base checks.
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check` will
  run before commit, followed by source-hierarchy/document checks, focused
  diff review, `make check`, the full documentation suite, and push
  verification at close.

## Evidence / outputs / test results

This is a pre-registration, not evidence execution. The established input
mismatch is sufficient to define a falsifiable tuple-identity question, but is
not yet a retained result. The bounded evidence command will compile and print
the foundation's `obligationName`, `mkLeanStub`, and concrete `e5ReviewUnits`
source slices, run the existing theorem-stub test, emit the existing e5
pipeline into `/tmp`, print its normalized review/stub tuples, and run the
existing current-L2 regression. The displayed source derives the foundation
triple literally; comparison is against the displayed pipeline triple. The
pipeline's internal review-to-stub agreement is a separate control only.

## What changed in understanding

The strongest next task is not an attempted repair of formal-hook attribution.
The current upstream detached bundle cannot distinguish the relation that would
make such an audit useful, while the Lean foundation makes an explicit,
checkable structural identity claim. The latter can be audited without treating
any helper-local label as Canon semantics.

## Open questions

- Does the existing e5 pipeline reproduce the foundation tuple exactly?
- If not, is an explicit lossless mapping or intentional synthetic-role
  statement present in the registered sources?
- A future decision whether to repair a mismatch remains outside WRK-0009.

## Suggested next prompt

Execute the committed WRK-0009 existing-lane command, retain only its literal
tuple-comparison evidence, and stop before any mapping, helper, or semantic
repair is proposed.

## Plan update status

`plan/` 更新不要（この登録コミット）: working-record policy restricts the
registration commit to the new WRK and exact operational metadata. The next
immediate, separate snapshot commit will replace plan/165's no-selection
disposition before any evidence command executes.

## Documentation.md update status

`Documentation.md` 更新不要: no current project state changed before evidence.

## docs/project-status.md update status

更新不要: no Canon/LAB lifecycle, readiness, or behavior changed before
evidence.

## progress.md update status

`progress.md` 更新不要（この登録コミット）: a pre-registered question is not a
research result. The next immediate, separate snapshot commit will record that
target selection has closed and WRK-0009 evidence is pending.

## tasks.md update status

`tasks.md` 更新不要（この登録コミット）: package 24 remains in progress until the
registration exists at `HEAD`. The next immediate, separate snapshot commit
will record its selected target and evidence-pending state.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, command, runnable workflow, or
dashboard classification changed.

## Reviewer findings and follow-up

Read-only planner `Mencius` found three distinct candidates and ranked the e5
literal identity audit first. Oracle initially proposed an upstream-projection
audit conditionally; local source inspection showed that its prerequisite
record is absent. Oracle's compact retry agreed that WRK-0009 is eligible only
as literal-transcription evidence and must not infer semantic equivalence.
Final reviewer `Avicenna` found that the first draft did not print the
foundation tuple being compared. The command now displays `obligationName`,
`mkLeanStub`, and `e5ReviewUnits`, which derive the foundation triple before
the pipeline triple is printed. The reviewer also confirmed that existing
command implementations at the immutable base need not widen permitted LAB
roots, and required immediate post-registration snapshot synchronization,
which is recorded above. No reviewer edited the workspace.

## Skipped validations and reasons

No evidence command, Lean compilation, Cargo test, or pipeline execution ran
before pre-registration because their result must not be relied on before the
WRK is committed. No helper/schema/fixture/test/runtime work is in scope.

## Commit / push status

Pending at report write. This pre-registration will be committed with
`git commit --no-gpg-sign`, pushed, and checked against `origin/main` before
the registered evidence command executes.

## Sub-agent session close status

`019f8679-df0c-7cf1-b766-57f8799e5ff2` (`Mencius`) completed the read-only
candidate ranking and was closed. `019f8694-303e-7e73-8d78-efcd0064c11f`
(`Avicenna`) completed the preregistration review and narrow re-reviews, then
was closed. No sub-agent edited the workspace. Oracle consults were advisory
only; one attachment upload timed out, and its compact retry completed.
