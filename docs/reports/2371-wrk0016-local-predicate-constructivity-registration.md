# Report 2371 - WRK-0016 local-predicate constructivity registration

- Date: 2026-07-22 23:48 JST
- Author / agent: Codex
- Scope: reversible L3 pre-registration only
- Decision levels touched: none; no Canon theory, ledger, gate, phase, or evidence outcome change

## Objective

Register a narrow existing-LAB Lean experiment that can test whether the exact
two-constructor `Capability` carrier admits a constructive all-input
`captureSubset` decision term without turning that local result into a MirCore
or OBL claim.

## Scope and assumptions

`mirrorea_canon/` remains normative. `plan/173` is a pinned LAB input, not a
result artifact. This package creates no Lean source, runs no candidate outcome
command, and selects no generic finite-carrier interface, typeclass instance,
Core rule, grammar, checker behavior, or formal obligation disposition.

## Start state / dirty state

`main...origin/main` was clean at
`e24f78d485f30932740d410f10526b0a1a8e9f33`, matching `origin/main`. The
Discord task baseline was already recorded. During prior command inspection,
the current-L2 sync script was accidentally invoked with `--help`; it ignored
the flag, made no tracked change, and is neither registration evidence nor a
candidate outcome command.

## Documents consulted

Read Canon README/MAP, ADR-0014, the working-annex README, theory/01,
theory/02, theory/11, WRK-0001, WRK-0014, `plan/173`, the exact Lean foundation
and explanation, `scripts/validate_docs.py`, current project snapshots, and
Report 2370.

## Actions taken

Created WRK-0016 with fixed Canon/LAB snapshot hashes, permitted locations,
three named future decision terms, an opaque-domain adverse probe, lexical
stop conditions, rollback, and non-claims. Added its pointer to the Canon map
and marked the current LAB task/status snapshots as registration-only.

## Files changed

- `mirrorea_canon/working/WRK-0016-local-predicate-constructivity.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- targeted `git`, `rg`, and `sed` inspections of the Canon boundary, current
  Lean foundation, working-record validator, report template, and snapshots
- `date` for the recorded timestamp
- `python3 mirrorea_canon/meta/build-index.py`
- registration diff/surface inspection and post-commit documentation validation

## Evidence / outputs / test results

No candidate Lean command ran. The record is `L3-open` with Reliance status
`not-promoted`, `Evidence artifacts: none`, and `Evidence commits: none`.
The exact authority/input cut is `e24f78d485f30932740d410f10526b0a1a8e9f33`.
The registered future source tail must be non-instance, local to the existing
two-constructor carrier, and reject `Fintype`, `Finset`, `Classical`, imports,
definitions, helpers, and admissions. The separately declared arbitrary-domain
probe must fail without an explicit finite interface.

An initial index-builder invocation from the repository root correctly stopped
with `canon root not found` and made no write; rerunning it from
`mirrorea_canon/` succeeded with 95 indexed files. The pre-commit
documentation validator and source-hierarchy check passed; the required fresh
post-commit validation remains the push gate.

## What changed in understanding

The useful result of registration is procedural rather than semantic: the
experiment now distinguishes a closed-carrier local proof from a generic
decidability mechanism before its outcome is known. This keeps a positive Lean
compilation from being misread as an OBL-003, checker, or language-core result.

## Open questions

- Does the exact local term compile under the registered no-helper/no-instance
  constraints?
- Does the opaque arbitrary-domain control fail as expected on the installed
  Lean version?
- Can the resulting source and explanation be manifested without changing the
  runnable sample dashboard or any public interface?

## Suggested next prompt

After this registration commit is pushed, execute exactly the WRK-0016 command
plan, retain the result only if every stop condition holds, and independently
review the evidence boundary.

## plan/ update status

`plan/` update unnecessary: `plan/173` is an immutable LAB input to this
registration, and editing it in the registration commit would violate the
working-record package boundary.

## Documentation.md update status

`Documentation.md` update unnecessary: its existing current-position link
already points to plan 173. The new working-record pointer belongs in the Canon
map and current status snapshots; registration control-file limits deliberately
exclude a reader-map rewrite.

## docs/project-status.md update status

Updated: the concise control view now distinguishes committed WRK-0016
registration from an unexecuted Lean outcome and from OBL-003 progress.

## progress.md update status

Updated: the logical snapshot, Macro 1 row, and dated log record the
registration-only state and post-push execution condition.

## tasks.md update status

Updated: package 43 closes the registration and leaves the evidence package as
the next autonomous action.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample command, debug
surface, catalog entry, or workflow readiness changed at registration.

## Reviewer findings and follow-up

The prior selection package used an independent foundation mapper, feasibility
planner, adversarial review, final review, narrow re-review, and temporary
Oracle advice. Their accepted constraints are preserved here: use only
`captureSubset` as the candidate; retain `outlives` and `remoteCallAllowed` as
controls; include `plan` and `samples/lean` as permitted locations; manifest
any retained evidence by append-only full commit; and make no OBL-003 claim.
No new outcome review is requested until registered evidence exists.

## Skipped validations and reasons

The registered Lean commands, runtime suites, distributed checks, sample sync,
and Oracle consultation were intentionally skipped: they would either be an
outcome command before immutable registration or do not apply to a
documentation/registration package. `samples_progress.md` is unchanged for
the same reason. Post-commit documentation validation is required before push.

## Commit / push status

Pending at report write. The registration will be committed with
`--no-gpg-sign`, documentation-validated after the commit, and pushed
immediately when that validation passes.

## Sub-agent session close status

No new sub-agent was opened for the mechanical registration. The selection
package's mapper, planner, adversarial reviewer, final reviewer, and narrow
re-reviewer are closed; its temporary Oracle consultation remains advisory and
outside repository state.
