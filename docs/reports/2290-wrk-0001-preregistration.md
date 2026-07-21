# Report 2290 - WRK-0001 finite-index pre-registration

- Date: 2026-07-21 17:09 JST
- Author / agent: Codex
- Scope: Select and commit the first bounded L3 pilot before running its outcome checks.
- Decision levels touched: L3 only. No L0/L1, theory/11, contract, SCN, Gate, Phase, proof, implementation, or public-state movement.

## Objective

Pre-register a small, reversible first pilot that tests the new autonomous research ratchet without converting a helper-local Lean model into Mir semantics.

## Scope and assumptions

The selected candidate is a reproduction of the existing `CurrentL2FiniteIndexFirstLayer.lean` against theory/02's already-permitted finite index families. It deliberately excludes unresolved OBL formalization, authority semantics, runtime behavior, and source-language design. No Lean outcome command is run before this registration commit.

## Start state / dirty state

Started from pushed, clean `main` at `032a0ac2`, after Package A closeout. There was no active WRK record and no candidate outcome evidence from this package.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014, theory/02, and `working/README.md`.
- `plan/156`, `plan/158`, `plan/159`, `tasks.md`, `progress.md`, and `samples/lean/README.md` as LAB operating evidence.
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`, its explanation, and `samples/lean/manifest.json`.

## Actions taken

- Compared a finite-index fragment, an IFC authority toy model, and abstract OBL statement drafts against the standing eligibility predicate.
- Chose the finite-index fragment because theory/02 explicitly names its three finite families, its file already contains both positive and rejecting helper-local lemmas, and it needs no new source, helper, or canonical interpretation.
- Created WRK-0001 with pinned canon/LAB blobs, an alternative, a falsifier, rollback rule, commands, non-claims, and no outcome evidence.

## Files changed

- `mirrorea_canon/working/WRK-0001-finite-index-boundaries.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2290-wrk-0001-preregistration.md`

## Commands run

- `git rev-parse HEAD`
- `sha256sum` over the pinned theory/02 and LAB source/manifest inputs
- `lean --version`
- Candidate source/document reads and an asynchronous Oracle candidate-selection advisory request

## Evidence / outputs / test results

- The pre-registration pins commit `032a0ac2`, theory/02 SHA-256, and the existing Lean source/manifest SHA-256 values.
- Lean 4.29.1 is installed. The target Lean file itself has not yet been compiled in this package, and no result is claimed.
- The Oracle advisory request is still running at report write and is not relied on for the pre-registration decision.

## What changed in understanding

A useful first autonomous pilot need not select an unresolved OBL carrier. A source-grounded reproduction with explicit rejecting lemmas is enough to test whether the governance route preserves scope, falsifiability, and evidence provenance.

## Open questions

- Whether the pre-registered Lean fragment compiles and contains all named rejecting lemmas remains untested until after this commit.
- Oracle may identify a safer candidate; any change to this pre-registration requires a successor or escalation rather than rewriting it.

## Suggested next prompt

Run WRK-0001's exact positive and negative commands, record their output in an existing LAB plan, manifest the evidence commit, and preserve the L3 non-claims.

## Plan update status

`plan/` 更新不要 at registration: the current autonomous protocol is already in `plan/158`/`plan/159`; the experiment result and comparison belong in a post-registration existing-lane evidence update.

## Documentation.md update status

`Documentation.md` 更新不要: the high-level reader route did not change.

## docs/project-status.md update status

更新済み: the human-facing status now distinguishes committed WRK-0001 pre-registration from unrun evidence.

## progress.md update status

`progress.md` 更新済み: the recent log records the L3 registration and its non-result status.

## tasks.md update status

`tasks.md` 更新済み: triage/pre-registration is closed and the pilot experiment becomes current.

## samples_progress.md update status

`samples_progress.md` 更新不要: sample status, command catalog, and blocker classification did not change before evidence execution.

## Reviewer findings and follow-up

Package A's planner/Oracle recut required exact pre-registration before result execution. A new Oracle candidate-selection request is running but advisory; it cannot amend the committed pre-registration by itself.

## Skipped validations and reasons

The target Lean compile and negative source audit are intentionally skipped until after this pre-registration commit; running them first would violate the registered outcome order.

## Commit / push status

Pending validation of the committed pre-registration, then commit and push.

## Sub-agent session close status

No new sub-agent result was used. The Oracle consultation is running asynchronously and will be mirrored only as advisory evidence if relevant.
