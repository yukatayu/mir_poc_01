# Report 2407 - WRK-0021 CostBudget countermodel registration

## Title and identifier

Report 2407 - WRK-0021 CostBudget countermodel registration.

## Objective

Pre-register one bounded ADR-0014 L3 countermodel that tests only whether an
explicit scalar-total relation reflects the existing three-counter LAB
CostBudget pointwise bound.

## Scope and assumptions

- Canon remains normative. PROPOSAL-011 remains owner-pending and unchanged.
- The target is one fixed finite LAB implication, not a Contract old/new
  relation, transparent-overlay rule, or general cost-algebra result.
- Registration adds no outcome source. The Lean foundation amendment and plan
  memo are reserved for a later post-push evidence commit.

## Start state / dirty state

The worktree was clean at `bfa1e809`, equal to `origin/main`, after the
WRK-0020 closeout and its final successful `make check`.

## Documents consulted

- Canon: `README`, `MAP`, ADR-0014, `working/README`, theory/02,
  theory/11, and PROPOSAL-011.
- LAB: plan/183, the active CostBudget index and rejected cost sample, the
  existing finite-index Lean foundation, current snapshots, and Report 2403.
- Process: `AGENTS.md`, `samples/lean/README.md`, and validator rules.

## Actions taken

1. Rejected the lower-value `G_e` literal audit after local, planner, reviewer,
   and temporary Oracle assessment found no current non-reserved decision.
2. Verified that no matching scalar-total/pointwise countermodel exists and
   that plan/183 expressly permits a separate L3 projection test.
3. Fixed a one-pair scalar-reflection question, alternative, falsifier,
   permitted lanes, exact future commands, and non-effects in WRK-0021.
4. Kept the eventual finite source in the existing `samples/lean/foundations/`
   lane, avoiding a new runner, helper family, schema, or API.

## Files changed

- `mirrorea_canon/working/WRK-0021-costbudget-scalar-projection.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2407-wrk0021-costbudget-registration.md`

## Commands run

- ordered Canon/LAB source reads, duplicate searches, and fresh input digest
  capture
- `lean --version`, which reported Lean 4.29.1
- resource audit: `df -h .` and `free -h`
- pending: Canon index rebuild/check, diff review, commit, push, and
  committed-head `make check`

## Evidence / outputs / test results

No WRK-0021 marker check, Lean countermodel compilation, or clean-near-end
runner command has run before registration. The checked source facts are only
selection inputs: the active index names `cpu_steps`, `remote_calls`, and
`writes` with `pointwise_natural_bound`; plan/183 records no scalar-total
policy and explicitly allows a separately pre-registered projection test.

The fixed future pair is `(0, 1, 0)` versus `(1, 0, 0)`. If Lean establishes
equal scalar totals and failed pointwise comparison, the retained result can
only reject that exact scalar-reflection implication in the finite LAB model.

## What changed in understanding

The current cost source supplies a concrete, bounded model suitable for an L3
countermodel without supplying a Canon cost algebra. The safe consumer is the
LAB candidate implication itself, not the owner-pending Contract question.

## Open questions

- Does the registered source amendment compile and establish the fixed finite
  countermodel after push?
- Which PROPOSAL-011 alternative, if any, will the owner select later?

## Suggested next prompt

After this registration is pushed, run only WRK-0021's exact marker, Lean, and
baseline commands; retain or freeze the finite LAB result without selecting a
cost policy.

## Plan update status

`plan/` 更新不要: registration adds no outcome artifact. The future unnumbered
WRK-0021 memo and its index entry are reserved for evidence.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing workflow or capability changed.

## docs/project-status.md update status

更新済み: the reader status distinguishes the unrun WRK-0021 L3 registration
from PROPOSAL-011 and any cost-policy decision.

## progress.md update status

更新済み: the snapshot and dated log record the unrun scalar-projection
countermodel boundary.

## tasks.md update status

更新済み: the task map separates the pending exact evidence sequence from the
owner-reserved cost decision.

## samples_progress.md update status

`samples_progress.md` 更新不要: no Lean source, runnable sample, command, or
sample evidence classification has changed at registration.

## Reviewer findings and follow-up

The planner rejected the `G_e` route as a low-value textual replay and proposed
this stronger existing-lane countermodel. A temporary Oracle consultation
confirmed that literal row taxonomy would need an explicit live retain/reject
decision. The focused CostBudget reviewer found no matching countermodel,
confirmed that the runtime's map comparison is pointwise baseline evidence
only, and recommended the existing finite-index Lean foundation rather than a
new lane. These are advisory inputs; the pinned source and ADR-0014 boundary
control this registration.

## Skipped validations and reasons

The registered marker check, Lean compile, sample runner, and source audit are
deliberately deferred until the registration commit is pushed. No runtime
implementation or Contract/layer test applies because no such surface changed.

## Commit / push status

Pending registration commit and immediate push with `--no-gpg-sign`; outcome
commands are prohibited until then.

## Sub-agent session close status

The source explorer, strategic planner, and focused CostBudget reviewer
completed read-only work and were closed. No sub-agent made repository edits.
