# Report 2456 — WRK-0028 R0 evidence linkage

- Date: 2026-07-28 08:31 JST
- Author / agent: Codex
- Scope: Append the exact retained R0 evidence commit and artifact digest to
  WRK-0028 without changing its pre-registration or result boundary.
- Decision levels touched: Canon `working/` L3 evidence metadata and index/MAP
  metadata only.

## Objective

Bind the source-local R0 manifest to its owning evidence commit so downstream
research can verify provenance without treating the LAB artifact as semantics.

## Scope and assumptions

The R0 evidence commit is
`2b4a89801b3d30442426926d6aff96b1d709874a`; its manifest SHA-256 is
`23c7668615d35f8ee82c85db8f5e73f779badeb7db57f7b94990c63a3bc8e478`.
All question, alternative, falsifier, rollback, execution cut, and non-claim
text in WRK-0028 remains unchanged.

## Start state / dirty state

Started clean and equal to `origin/main` at the pushed R0 evidence commit
`2b4a89801b3d30442426926d6aff96b1d709874a`.

## Documents consulted

- WRK-0028, its MAP row, ADR-0014, and working annex evidence rules.
- R0 manifest, Report 2455, and the current Git evidence commit.

## Actions taken

1. Verified the exact evidence commit and manifest SHA-256.
2. Appended only Results-and-review evidence fields in WRK-0028.
3. Updated the MAP summary to describe the retained, not-promoted result.
4. Regenerated Canon index metadata before validation.

## Files changed

- `mirrorea_canon/working/WRK-0028-r0-common-cut-fact-manifest.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2456-wrk0028-r0-evidence-linkage.md`

## Commands run

- `git rev-parse HEAD`, `git rev-parse origin/main`, and `sha256sum` for the
  retained manifest.
- Canon index regeneration/check, documentation validation, diff/secret checks
  before the metadata commit.

## Evidence / outputs / test results

The evidence commit contains only the R0 LAB manifest, its plan index entry,
current LAB snapshots, and Report 2455. The appended artifact snapshot names
that exact commit and matches its SHA-256. No new semantic evidence is added.

## What changed in understanding

R0 is now reproducible as provenance evidence. It remains L3 and
not-promoted; it cannot be consumed as a selected C0/C2 solution.

## Open questions

- C0-A and C2-A remain separate eligibility screens and must not inherit an
  unselected semantic carrier from R0.

## Suggested next prompt

Screen C0-A source authority and C2-A equality vocabulary independently under
ADR-0014, stopping at any reserved-boundary requirement.

## Plan update status

`plan/` 更新不要: the evidence artifact and plan index were retained in the
preceding evidence commit; this task only links their exact provenance.

## Documentation.md update status

`Documentation.md` 更新不要: navigation did not change.

## docs/project-status.md update status

更新不要: evidence linkage does not change the project status established by
Report 2455.

## progress.md update status

`progress.md` 更新不要: no new milestone beyond the retained R0 result.

## tasks.md update status

`tasks.md` 更新不要: next candidates remain C0-A and C2-A.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample or command changed.

## Reviewer findings and follow-up

No additional independent review was needed. The linkage is a mechanical
append-only check against the already pushed evidence commit.

## Skipped validations and reasons

No Lean, runtime, parser, or sample execution applies to metadata-only
evidence linkage.

## Commit / push status

Pending at report write. This metadata-only commit will be pushed and checked
for `HEAD == origin/main`.

## Sub-agent session close status

No callable sub-agent session was available. The completed Oracle session is
already distilled in Reports 2454/2455.
