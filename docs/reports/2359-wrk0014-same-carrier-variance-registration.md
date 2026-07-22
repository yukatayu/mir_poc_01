# Report 2359 — WRK-0014 same-carrier variance registration

- Date: 2026-07-22 17:58 JST
- Author / agent: Codex
- Scope: pre-registration of a parameter-only relation-variance Lean probe
- Decision levels touched: L3 only; no L0/L1 decision

## Objective

Register the smallest new theory experiment justified by the correspondence
audit without defining a Canon carrier or treating a generic lemma as an OBL
result.

## Scope and assumptions

The Canon remains normative. The new record is limited to an existing
`samples/lean/lab-statements/obl020` LAB lane and asks only about relation
inclusion on identical abstract carriers. The Oracle consultation is advisory;
it selected no Canon semantics.

## Start state / dirty state

`main...origin/main` was clean at `486507ad`. The prior correspondence audit
had closed with no selected L3 record; this package adds registration only and
runs no Lean outcome command.

## Documents consulted

Read Canon README/MAP, ADR-0014, working annex, theory/01, theory/03,
theory/11, BND-001, WRK-0005/0006/0007, plan/171, the post-WRK-0013
disposition, current reports/status/tasks, and the existing OBL-020/021 Lean
boundary experiments. A temporary Oracle formal-semantics review was compared
against those sources.

## Actions taken

Registered WRK-0014 before any outcome evidence. It fixes one generic
same-carrier question: safety/coherence transfer needs Canon-to-model
inclusion, while outcome-existence transfer needs model-to-Canon realization.
It also pre-registers exact semantic-neutrality falsifiers and the allowed
evidence paths.

## Files changed

- `mirrorea_canon/working/WRK-0014-same-carrier-variance.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- Canon/LAB source inspection with `sed`, `rg`, `sha256sum`, and `git show`
- `oracle status theory-kernel-route-20260722`
- `python3 mirrorea_canon/meta/build-index.py`
- registration validation commands recorded in this report's evidence package

## Evidence / outputs / test results

No result-producing Lean command has run. The registration pins authority and
LAB input digests at `486507ad`, declares `Evidence commits: none`, and limits
the future source to a standalone non-importing file. The only current result
is that the L3 record is eligible once this registration commit is pushed.

## What changed in understanding

The prior audit identified not another local counterexample but a polarity
problem: universal claims such as safety/coherence can move from an
over-approximating model to the intended relation, whereas outcome existence
needs witnesses realizable by the intended relation. This is a conditional
proof-hygiene observation, not an actual correspondence proof.

## Open questions

- Which future proof-facing model, if any, can establish actual coverage or
  witness-realization against Canon?
- How should a direct Core/write reading be represented for THM-001?
- Does PROPOSAL-008 place outcome totality anywhere in the Canon?

## Suggested next prompt

Run only WRK-0014's registered Lean command, then retain or freeze its
conditional result after an independent review. Do not select a Canon carrier
or totality interpretation through the experiment.

## Plan update status

`plan/` 更新不要: this is pre-registration only. The future evidence commit
may add the declared plan/172 result memo and index entry.

## Documentation.md update status

`Documentation.md` 更新不要: the concise reading map does not need a
registration-only link before outcome evidence exists.

## docs/project-status.md update status

更新済み: the human control view now identifies WRK-0014 as pre-registration
only and preserves the owner-reserved proof-facing boundaries.

## progress.md update status

`progress.md` 更新済み: the logical, macro-phase, feature, and recent-log
snapshots distinguish the new relation-polarity probe from evidence.

## tasks.md update status

`tasks.md` 更新済み: task 36 records the exact post-push command boundary and
its freeze condition.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command,
debug surface, or workflow classification changed.

## Reviewer findings and follow-up

The completed planner and literature reviews plus the temporary Oracle review
agree that a concrete minimal semantics would cross the reserved boundary.
This registration therefore excludes concrete carriers and tests only generic
relation direction. A final independent reviewer will inspect the evidence
delta after the registered command.

## Skipped validations and reasons

The registered Lean command and broad build/release checks are intentionally
skipped until this pre-registration has been committed and pushed. No runtime
source changed; root storage remains constrained, so no unrelated heavy build
is justified.

## Commit / push status

Pending at report write. The registration package will be committed with
`--no-gpg-sign` and pushed before any outcome command.

## Sub-agent session close status

The current package uses completed advisory outputs only. Three stale completed
sub-agent sessions were closed before registration; no active sub-agent owns a
registration or evidence file.
