# Report 2410 - Cost-bound substitutability primary-literature audit

## Title and identifier

Report 2410 - Cost-bound substitutability primary-literature audit.

## Objective

Create bounded, primary-source decision support for PROPOSAL-011 without
turning the open Contract cost question into an inferred Canon rule.

## Scope and assumptions

- Canon is normative; this is a LAB literature comparison and current-state
  synchronization task.
- The audit may describe necessary premises for alternatives A/B/C/D, but may
  not select an alternative, a cost carrier, an order, runtime accounting, or
  an OBL premise.
- Frozen WRK-0021 remains historical. Its Lean `Decidable` failure is not
  repaired, replayed, or used as a source of a general cost conclusion.

## Start state / dirty state

The worktree was clean at `19625f86`, equal to `origin/main`, after the
post-WRK-0021 autonomous frontier triage.

## Documents consulted

- Canon: README, MAP, theory/02, theory/12, ADR-0014, PROPOSAL-011, and the
  WRK operating rules.
- LAB: plans 156, 171, 179 through 184, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Primary literature: Liskov/Wing on behavioral subtyping; Atkey on amortised
  resource logic; Das/Hoffmann/Pfenning on resource-aware session types; and
  Sharf/Besselink/Johansson on compositional contract refinement.
- Process: AGENTS.md and the repository-local Oracle operating notes.

## Actions taken

1. Re-read the exact Canon mismatch: `cost_bound` is in Contract, but the
   transparent-overlay list gives no old/new bound relation.
2. Compared four primary sources for their actual resource/substitutability
   assumptions and separated those facts from Mir-specific choices.
3. Mapped the resulting necessary premises to PROPOSAL-011 A/B/C/D without
   selecting a proposal option.
4. Re-ran three existing Full System V1 boundary tests to keep the current
   checker-before-executor implementation evidence distinguishable from the
   unresolved Contract rule.
5. Registered the numbered plan in the two existing validation catalogs and
   synchronized the human-facing LAB snapshots.

## Files changed

- `plan/185-cost-bound-substitutability-primary-literature-audit.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `docs/reports/2410-cost-bound-substitutability-primary-literature-audit.md`

## Commands run

- ordered Canon/LAB reads and repository status/source searches
- `df -h .` and `free -h` before reviewing or rerunning build-dependent work
- three focused `cargo test` commands for Full System V1 static/runtime/effect
  boundaries
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- two focused numbered-plan catalog alignment unit tests
- `make check`, `cargo check`, and `git diff --check`

## Evidence / outputs / test results

All three focused Full System V1 tests passed. They establish only that the
current LAB runner invokes checking before execution and exposes static versus
runtime rejection in its bounded implementation path. They do not define a
Contract-level `cost_bound` substitutability relation.

The literature comparison establishes a shared requirement for an explicit
resource carrier/fragment, comparison direction, unknown/incomparable policy,
and composition premise before a transparent resource rule can be relied on.
It supplies no source for choosing those Mir terms.

Final validation passed: documentation scaffold validation found 1,564
numbered reports, source hierarchy reported all 735 required paths present,
both focused catalog-alignment tests passed, and `make check`, `cargo check`,
and whitespace validation completed successfully.

## What changed in understanding

The project does not need to halt theory work because the cost question is
open. The precise safe outcome is narrower: autonomous research can document
the premises a future decision must state, while only the owner/canon process
may classify cost as transparent, explicitly updated, advisory, or deferred.

## Open questions

- Is `cost_bound` intended to be an observable Contract guarantee at the
  transparent-overlay boundary?
- If so, which interim carrier/fragment, relation direction, unknown-value
  policy, and composition law should the Canon record?
- If no interim comparison is desired, should the project explicitly choose B
  or leave the interpretation at D?

## Suggested next prompt

Continue the independent literature taxonomy for elaboration totality and
determinism, while preserving the PROPOSAL-009 and PROPOSAL-008 owner/canon
boundaries.

## Plan update status

`plan/` 更新済み: Plan 185 records the primary-source comparison and the
non-selecting A/B/C/D decision support; `plan/00-index.md` links it.

## Documentation.md update status

`Documentation.md` 更新済み: the current-plan list now includes the two latest
frontier and literature records without claiming a new capability.

## docs/project-status.md update status

更新済み: the concise LAB view now says that the literature audit identifies
required comparison premises but selects no cost policy.

## progress.md update status

更新済み: the logical-specification snapshot and dated recent log distinguish
the completed decision support from a Canon cost-rule decision.

## tasks.md update status

更新済み: the task map records the closed literature audit and refines the
PROPOSAL-011 decision recommendation without selecting it.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, validation command, debug surface,
or retained sample evidence classification changed.

## Reviewer findings and follow-up

An independent planner recommends this bounded literature audit rather than a
new WRK. The completed temporary Oracle consultation agrees that no cost rule
may be inferred from the field or the frozen CostBudget route; it additionally
recommends a separate read-only canonical interface-closure audit for
elaboration results, `G_e`, and per-step occurrence deltas. That follow-up is
not folded into this cost audit: each reported canonical discrepancy requires
its own direct source verification before it becomes an LAB finding or an
owner escalation. The completed independent semantic reviewer independently
identifies OBL-026's missing comparison/composition premises and several
separate potential Canon inconsistencies. None changes this audit's
non-selecting disposition. The separate claims are inputs to the next
read-only source audit, not accepted findings or a reason to amend Canon.

## Skipped validations and reasons

No new Lean, countermodel, sample, or runtime outcome command was created for
cost substitution: none can choose the owner-pending Contract rule, and
reworking WRK-0021 would violate its frozen record boundary.

## Commit / push status

Pending final validation. The completed package will be committed with
`--no-gpg-sign` and pushed immediately.

## Sub-agent session close status

The planner and semantic reviewer completed read-only work and were closed
after integration. No sub-agent edited repository files.
