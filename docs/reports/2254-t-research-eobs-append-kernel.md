# Report 2254 — T research [E-OBS] append kernel

- Date: 2026-07-17
- Author / agent: Codex
- Scope: bounded LAB calculus experiment under `plan/156`
- Decision levels touched: no canon decision level changed

## Objective

Select and complete one further concrete T0-T2 research unit after
T-RESEARCH-002: test whether a restricted `[E-OBS]` one-occurrence graph
extension preserves occurrence-DAG acyclicity and publication ancestry without
claiming a complete rule or a canonical graph-update definition.

## Scope and assumptions

The canon remains `T0/G0 rebaseline`. The experiment is disposable Lean work
under `/tmp/mirrorea-t-research-003/`, not a tracked Lean lane or a MirCore
runtime model. The source cut is `mirrorea_canon/theory/01-mircore-v0.md` for
the well-formedness clauses, one-occurrence append, and `[E-OBS]`; and
`mirrorea_canon/theory/04-ordering-and-cuts.md` for causal generators and
transitive closure. The work may close only as `research-complete`.

## Start state / dirty state

The worktree was clean at `7d15d108`. T-RESEARCH-001 and T-RESEARCH-002 were
already recorded as LAB-only research-complete evidence. No canon package was
promoted and no successor work unit was preselected.

## Documents consulted

- `mirrorea_canon/README.md` and `mirrorea_canon/MAP.md`
- `mirrorea_canon/plan/01-phases.md`, `plan/02-operating-model.md`, and
  `plan/03-risks.md`
- `mirrorea_canon/theory/00-overview.md`, `01-mircore-v0.md`,
  `03-elaboration.md`, `04-ordering-and-cuts.md`, `07-observation.md`, and
  `11-metatheory-ledger.md`
- `mirrorea_canon/meta/agent-instructions.md` and `source-hierarchy.md`
- `plan/121-g1-minimal-vertical-slice-candidate-map.md`,
  `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`,
  `plan/147-g1-next-line-promotion-boundary-audit.md`, and `plan/156`
- `samples/lean/lab-statements/obl001`, `obl020`, and `obl021`
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and `.docs/progress-task-axes.md`

## Actions taken

- Re-read the canon and current LAB authority boundary, then reproduced the
  existing Lean and Surface anchors.
- Compared an OBL-020 `[E-OBS]` candidate with OBL-021 `[READ-CROSS]` in an
  Oracle continuation. The latter crosses OPEN-014 materialization semantics
  and was not selected.
- Wrote a deliberately weak finite graph model first. The desired acyclicity
  proof failed as expected because the fresh node could point back to an old
  publication.
- Replaced that red scratch test with a passing finite countermodel and a
  separate generic conditional kernel. The kernel uses `OldEvent + Unit`,
  preserves old generators, admits only old-to-fresh generators, and proves
  selected graph invariants with `Relation.TransGen`.
- Requested a second Oracle follow-up that reviewed the actual scratch files
  and source traceability. Its conclusion is distilled below; raw Oracle output
  remains outside the repository.

## Files changed

- `docs/project-status.md`
- `docs/reports/2254-t-research-eobs-append-kernel.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`
- `tasks.md`

## Commands run

- `df -h .` and `free -h`
- `lean --trust=0 samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `lean --trust=0 samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `lean --trust=0 samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `lean --trust=0 /tmp/mirrorea-t-research-003/EObsAppendWeakCountermodel.lean`
- `lean --trust=0 /tmp/mirrorea-t-research-003/EObsAppendKernel.lean`
- `ask-chatgpt-pro-followup` for candidate selection and actual-result review

## Evidence / outputs / test results

- The three existing statement drafts compiled with `lean --trust=0`.
- The existing Lean statement sync suite passed: 21 tests.
- The Surface check reported 53 samples, 53 passed, 0 failed, and
  `workflow_ready=False`; it remains bounded LAB evidence only.
- The initial red scratch proof failed with `Missing cases`, confirming that
  acyclicity cannot follow from the weaker two-edge extension.
- `EObsAppendWeakCountermodel.lean` subsequently compiled at `--trust=0` and
  proves an acyclic old history, a direct old-publication-to-fresh edge, and a
  fresh-to-old edge forming a two-edge cycle.
- `EObsAppendKernel.lean` compiled at `--trust=0` and proves, for arbitrary
  old event types, that the stated fresh/prefix-preserving/incoming-only
  construction preserves acyclicity and kind-level publication ancestry.
- Scratch files occupied 16 KiB. No new global tool, build artifact, tracked
  implementation, helper, runner, or evidence lane was created.

## What changed in understanding

For the selected two graph conjuncts, `append` needs more than the existence
of a new observation and a publication predecessor if arbitrary new-to-old
causal edges are permitted. The experiment supplies one sufficient local
construction, not a proof that it is the weakest or canonical construction.
The direct publication edge proves kind-level ancestry only; it says nothing
about semantic publication matching, observer authority, visibility, or
redaction.

## Open questions

- Canon does not yet give an extensional equation for `H + occurrence`; a
  later package must not equate it with this experiment's `PostGen` without a
  human/canon semantic act.
- OBL-021 `[READ-CROSS]` determinism remains blocked by OPEN-014's unresolved
  materialization and result-equivalence boundary.
- Further OBL-020 work needs a new concrete rule/clause with an exact canon
  cut and falsification criterion; none is selected by this report.

## Suggested next prompt

Run a bounded validation/drift audit, or explicitly name a new canon-grounded
rule-local research question. Do not infer a next calculus experiment from the
completed `[E-OBS]` result alone.

## Plan update status

`plan/` 更新済み: `plan/156` records T-RESEARCH-003, its conditional result,
source traceability boundary, Oracle review, and reopen trigger.

## Documentation.md update status

`Documentation.md` 更新不要: the reader entry point and project authority
boundary did not change.

## docs/project-status.md update status

更新済み: the concise control view now distinguishes the conditional
`[E-OBS]` evidence from a canonical append definition.

## progress.md update status

`progress.md` 更新済み: it now records the third research-complete result and
the absence of a preselected successor without changing lifecycle status.

## tasks.md update status

`tasks.md` 更新済み: it lists T-RESEARCH-003 as completed and leaves the next
work unit unselected under the `plan/156` selection rule.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command,
debug surface, or workflow classification changed.

## Reviewer findings and follow-up

Oracle first recommended the `[E-OBS]` kernel over `[READ-CROSS]`: the latter
would choose an OPEN-014 materialization/equivalence policy. The follow-up
review of both scratch files found the formal result valid as a conditional
kernel and the weak model valid for the weaker premise package. It required
the report to say that incoming-only is sufficient but not proven necessary,
minimal, unique, or canonically required; that the direct predecessor is
stronger than arbitrary ancestry; and that no complete `[E-OBS]` or runtime
claim follows. It advised against manufacturing a second calculus experiment.
Both Oracle sessions completed; no local sub-agent remains active.

## Skipped validations and reasons

No runtime, Cargo, product, conformance, or sample implementation changed, so
their broad suites were not rerun for this proof-boundary package. The scratch
model deliberately excludes Config, queues, stores, membership, grants,
witnesses, authority, visibility, redaction, scheduling, and transport; those
omissions are the experiment boundary, not unverified success claims.

## Commit / push status

Pending at report write. The report and synchronized LAB snapshots will be
committed with `--no-gpg-sign` and pushed at this package close.

## Sub-agent session close status

The two Oracle continuation sessions completed and were critically distilled
into this report. No local sub-agent session was available or remains active.
