# Report 2301 - WRK-0003 OBL-021 projection-extensionality pre-registration

- Date: 2026-07-21 20:06 JST
- Author / agent: Codex
- Scope: Select and commit a bounded L3 countermodel candidate before running its outcome checks.
- Decision levels touched: L3 only. No L0/L1, theory/11, contract, SCN, Gate, Phase, proof, implementation, or public-state movement.

## Objective

Pre-register a small Lean countermodel that distinguishes per-projection
non-vacuity from joint Result extensionality in the existing LAB OBL-021
statement draft, without selecting any final equality or result relation.

## Scope and assumptions

Canon is authoritative. Canon theory/03 fixes the intended tuple/function
contract, but the current LAB statement interface supplies neither a direct
Result relation nor a law connecting its abstract projections to Result
identity. The candidate uses only the existing `plan` and `samples/lean` lanes
and creates no helper family, schema, CI/Make target, or public interface.

## Start state / dirty state

Started from pushed, clean `main` at `45403b68`. WRK-0002's source evidence is
manifested as L3 `not-promoted`; no source for the WRK-0003 countermodel exists
and no outcome command has run.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014,
  `working/README.md`, and `plan/01-phases.md` / `02-operating-model.md`.
- `mirrorea_canon/theory/01-mircore-v0.md`, `03-elaboration.md`,
  `10-diagnostics.md`, `11-metatheory-ledger.md`, and
  `architecture/02-boundary-contracts.md`.
- WRK-0002, `plan/wrk-0002-projection-vacuity-countermodel.md`, `plan/126`,
  `plan/143`, `plan/158`, and `plan/159`.
- The existing OBL-021 statement draft and projection-vacuity countermodel.
- A temporary Oracle advisory review and a read-only sub-agent Canon audit.

## Actions taken

- Compared the Oracle finding that empty projections are not the only possible
  degeneracy with the Canon audit of BND-001's tuple/function wording.
- Separated the established empty-projection countermodel from the next,
  strictly stronger question: whether total/unique projection witnesses plus
  equality comparisons are sufficient without joint extensionality.
- Created WRK-0003 with pinned inputs, alternative, falsifier, rollback, exact
  commands, and explicit non-claims.

## Files changed

- `mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2301-wrk-0003-preregistration.md`

## Commands run

- Read-only Canon/LAB inspection with `sed`, `rg`, pinned SHA-256 snapshots,
  and `git rev-parse HEAD`.
- One temporary Oracle boundary review, read after completion.
- One read-only sub-agent Canon audit; it made no edits and was closed after
  returning its findings.
- No WRK-0003 Lean compile, missing-file red check, source audit, or outcome
  command.

## Evidence / outputs / test results

- Oracle confirmed WRK-0002 as a valid countermodel to the bare LAB draft but
  warned that all-true component equivalences are an independent degeneracy.
- The independent Canon audit found BND-001's concrete tuple and function
  wording, but no abstract result-projection predicate, totality/uniqueness
  law, or direct bridge in the LAB interface. This is a constrained inference,
  not a Canon change.
- WRK-0003 is only pre-registered. Its target Lean source does not yet exist
  and no positive or negative outcome is claimed.

## What changed in understanding

The next useful question is not whether to select totality, equality, or a
direct relation. It is whether totality and equality alone can be falsified as
an insufficient repair. A positive countermodel would isolate the need for an
unselected joint-extensionality or direct-relation bridge; a failed construction
would identify an existing constraint that deserves narrower study.

## Open questions

- Whether the registered total/unique-projection countermodel compiles and
  establishes its planned insufficiency remains untested until after this
  commit.
- The final Result equality, joint extensionality form, direct Result relation,
  diagnostic equivalence, and all OBL-021 status decisions remain unresolved.

## Suggested next prompt

Run WRK-0003's registered red/green Lean commands, retain the countermodel only
if its exact totality/uniqueness and equality claims compile, and preserve the
L3-only non-claims.

## Plan update status

更新不要: registration changes only Canon working metadata and current task
selection; the detailed LAB evidence plan belongs after the registration commit.

## Documentation.md update status

更新不要: the high-level reader route did not change.

## docs/project-status.md update status

更新済み: the reader status now distinguishes WRK-0003 pre-registration from
unrun evidence.

## progress.md update status

更新済み: the recent log records the L3 registration, its Canon/LAB reading,
and the no-result boundary.

## tasks.md update status

更新済み: premise-gap triage is closed and WRK-0003 evidence is the current
package.

## samples_progress.md update status

更新不要: no active sample status, validation command, or dashboard row changed
before evidence execution.

## Reviewer findings and follow-up

Oracle's advisory review and the sub-agent's Canon audit agree with the local
reading: Canon imposes the intended tuple/function contract but leaves the LAB
projection bridge undefined. Their result is recorded as advisory/evidence
context only; it neither creates a Canon law nor selects the next law.

## Skipped validations and reasons

The registered Lean outcome commands are intentionally deferred until after
this commit. Running them first would violate the committed outcome order.
Documentation/index/source-hierarchy validation is run for the registration
itself.

## Commit / push status

This pre-registration package is committed and pushed after documentation and
Canon-index validation, before any outcome evidence is generated.

## Sub-agent session close status

One read-only sub-agent completed the Canon audit without edits and was closed.
The temporary Oracle review completed; its raw temporary output is not
repository state.
