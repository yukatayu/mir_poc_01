# Report 2408 - WRK-0021 CostBudget falsifier

## Title and identifier

Report 2408 - WRK-0021 CostBudget falsifier.

## Objective

Execute the registered post-push WRK-0021 sequence once, classify its first
Lean falsifier without repair, restore transient source, and freeze the record
before any CostBudget conclusion.

## Scope and assumptions

- The pushed registration at `4ac08f77` is immutable. No theorem, tactic,
  relation encoding, source procedure, or command is revised in this package.
- The failure establishes only that the registered proof did not compile with
  its declared source. It is not a countermodel or a decision about CostBudget.
- Canon remains normative; PROPOSAL-011 remains owner-pending and unchanged.

## Start state / dirty state

The worktree was clean and equal to `origin/main` at pushed registration
`4ac08f77`. The registered transient foundation, explanation, and generated
manifest modifications were present only while executing the declared command;
they are restored in this package.

## Documents consulted

- Canon: `README`, `MAP`, ADR-0014, `working/README`, WRK-0021, theory/02,
  theory/11, and PROPOSAL-011.
- LAB: plan/183, the active CostBudget index and rejected sample, the existing
  Lean foundation, `samples/lean/README.md`, current snapshots, and Report 2407.
- Process: `AGENTS.md`, the Discord reporting skill, and the verification rule.

## Actions taken

1. Ran the exact pre-source marker check after the registration push; it passed.
2. Added only the declared transient source to the existing Lean foundation.
3. Ran the registered Lean and baseline sequence once. Lean failed before a
   proof could be established because it could not synthesize two `Decidable`
   propositions used by `decide`.
4. Froze WRK-0021 without modifying the theorem to make it pass, restored the
   transient foundation/explanation/manifest changes, and retained only this
   failure memo and status record.

## Files changed

- `mirrorea_canon/working/WRK-0021-costbudget-scalar-projection.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `plan/wrk-0021-costbudget-scalar-projection-falsifier.md`
- `plan/00-index.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2407-wrk0021-costbudget-registration.md`
- `docs/reports/2408-wrk0021-costbudget-falsifier.md`

## Commands run

- `make check` after the pushed registration, passing Canon index, hierarchy,
  docs, and Cargo checks
- the registered pre-source marker check, which passed
- `lean --version`, reporting Lean 4.29.1
- the exact registered `lean --trust=0` proof, source audit, clean-near-end
  typing runner, Lean manifest sync, and manifest diff check sequence once
- focused failure inspection and restoration diff review
- frozen evidence commit `fbfbc31a8ec0ac4a085b704a21400326b7b96952` and
  metadata manifest commit `6628784a3b6e77e85b74fb61bbb63bdafe5e93f2`
- clean detached-worktree `make docs` at `6628784a`, passing Canon index,
  source hierarchy, and documentation validation

## Evidence / outputs / test results

Lean failed at the two `decide` proofs because it could not synthesize
`Decidable (scalarTotalLeq scalarCandidate scalarReference)` or
`Decidable ¬pointwiseBudgetLeq scalarCandidate scalarReference`. This is
WRK-0021's explicit Lean falsifier. The marker was absent before source was
added. The command's later semicolon-separated baseline steps ran after the
failure, but are not countermodel evidence; the manifest was restored.

No finite countermodel, pointwise/scalar policy, Contract rule, final algebra,
runtime accounting rule, OBL result, or public behavior is established.

## What changed in understanding

The current bounded formulation cannot be retained merely because the intended
arithmetic pair appears straightforward. Its actual registered proof requires
decidability structure that the record neither stated nor allowed to be added.
The falsifier guard therefore prevents an implementation convenience from
becoming an unstated cost-model choice.

## Open questions

- Does a distinct future CostBudget question have a live non-reserved consumer,
  or would it only repeat this frozen route?
- Which PROPOSAL-011 alternative, if any, will the owner select later?

## Suggested next prompt

Treat WRK-0021 only as a frozen proof-procedure boundary, then select a new
non-duplicative research target rather than repairing its Lean formulation.

## Plan update status

`plan/` 更新済み: the new unnumbered WRK-0021 memo records the exact failure,
restoration, and prohibited repair; `plan/00-index.md` links it.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing workflow or capability changed.

## docs/project-status.md update status

更新済み: WRK-0021 is frozen at its registered Lean falsifier, without a
CostBudget or Contract conclusion.

## progress.md update status

更新済み: the snapshot and dated log distinguish the failed Lean procedure from
any scalar/pointwise result.

## tasks.md update status

更新済み: the task map closes WRK-0021 and forbids a repair/retry route.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface, or
retained sample evidence classification changed; the transient source was
restored.

## Reviewer findings and follow-up

A focused reviewer confirmed that the two Lean errors meet WRK-0021's exact
falsifier, that no semicolon-separated command after the first failure is
retainable as an outcome, and that the tentative source plus manifest must be
restored. The review also identified stale unrun wording in the record, MAP,
snapshots, and Report 2407; this package updates those locations. No reviewer
converts the direct Lean failure into a semantics conclusion.

## Skipped validations and reasons

The failed theorem is deliberately not repaired or retried. No modified Lean
proof, alternate relation encoding, Contract/layer test, runtime test, or
countermodel command is run because each would be a new experiment outside this
record.

## Commit / push status

Registration `4ac08f77f0ef718803ab0628abce7ac85eebff43`, frozen evidence
`fbfbc31a8ec0ac4a085b704a21400326b7b96952`, and metadata manifest
`6628784a3b6e77e85b74fb61bbb63bdafe5e93f2` were each committed with
`--no-gpg-sign` and pushed immediately. This report closeout is committed and
pushed immediately as a separate documentation-only commit.

## Sub-agent session close status

The pre-registration source explorer, planner, and focused CostBudget reviewer
were closed. The focused falsifier reviewer completed after its findings were
recorded and was closed. No sub-agent made repository edits.
