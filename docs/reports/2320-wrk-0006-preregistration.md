# Report 2320 - WRK-0006 familywise/global preservation pre-registration

- Date: 2026-07-22 01:06 JST
- Author / agent: Codex with two temporary Oracle reviews and one independent reviewer
- Scope: commit a bounded L3 experiment before running any outcome evidence
- Decision levels touched: L3 only; no L0/L1, theory ledger, contract, SCN, Gate, Phase, proof, implementation, or public-state movement

## Objective

Pre-register a narrowly bounded Lean experiment about the relationship between
the existing abstract OBL-020 aggregate preservation draft and its separate
family-qualified wrapper, without selecting a Canon step taxonomy, coverage
policy, theorem interface, or semantic carrier.

## Scope and assumptions

Canon is authoritative. The current LAB draft remains a compile-check-only
statement shape. This package changes only the reversible `working/` annex,
its canonical index/map metadata, and this report. Current LAB snapshots are
deliberately deferred to the later evidence package because the working-annex
registration contract permits only exact operational metadata with the new WRK.
The target evidence file does not exist and no conclusion from its intended Lean
checks is relied upon.

## Start state / dirty state

Started at clean pushed commit `5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60`.
WRK-0001 through WRK-0005 were manifested `not-promoted` L3 evidence. The
prior checkpoint had no active candidate under its stricter LAB
distinct-live-branch priority test.

## Documents consulted

- `AGENTS.md`, `CANON.md`, `mirrorea_canon/README.md`, and `mirrorea_canon/MAP.md`
- `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`, and
  `mirrorea_canon/theory/01-mircore-v0.md` / `11-metatheory-ledger.md`
- `mirrorea_canon/meta/proposals/PROPOSAL-001-obl020-g1-statement-scope-review.md`
  and `PROPOSAL-003-obl020-formalization-boundary-review.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` and its README
- `plan/126`, `plan/134`, `plan/144`, `plan/156`, `plan/158`, and `plan/161`
- `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`,
  and Reports 2259, 2260, and 2319

## Actions taken

1. Rechecked the Canon OBL-020 direction, the abstract LAB statement, and the
   earlier 65-cell source-adequacy result.
2. Obtained independent review that found no circularity and no full Canon
   binding in the draft.
3. Distinguished the prior checkpoint's LAB priority heuristic from
   ADR-0014's standing eligibility predicate.
4. Created WRK-0006 with pinned source inputs, alternative, expected
   falsifier, rollback trigger, command plan, and no-effect boundary.
5. Prepared current LAB snapshot wording for the later evidence package, but
   deliberately excluded it from this registration commit to preserve the
   working-annex registration boundary.

## Files changed

- `mirrorea_canon/working/WRK-0006-obl020-familywise-global-boundary.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2320-wrk-0006-preregistration.md`

## Commands run

- Canon/LAB source inspection with `sed`, `rg`, and `nl`
- `df -h .` and `free -h`
- direct Lean compile of the existing OBL-020 statement draft
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/validate_docs.py` and
  `python3 scripts/check_source_hierarchy.py --format json`
- temporary Oracle reviews `mirrorea-theory-reopen-20260722` and
  `obl020-theory-audit-20260722`
- independent read-only review of the same boundary

## Evidence / outputs / test results

Lean 4.29.1 compiled the existing OBL-020 draft. The Lean synchronization suite
passed all 21 tests. Documentation validation found 1,473 reports before this
report and source hierarchy validation found 711 required paths present.

The two Oracle reviews and the independent reviewer agree on the core facts:
the aggregate draft is not circular, has the ordinary preservation shape, and
does not bind its abstract vocabulary to the Canon configuration or complete
operational relation. They also agree that neither family coverage nor a
family-indexed final theorem can be inferred. One review recommends deferral
because the candidate has low impact; the other shows that ADR-0014 nevertheless
permits this exact existing-lane conditional experiment. The registration adopts
only that limited L3 route.

## What changed in understanding

The prior no-candidate result remains a useful priority assessment, but its
distinct-live-branch test is LAB triage rather than a Canon eligibility rule.
The candidate is worth testing only because it can clarify how a future proof
package must compose aggregate and family-local reasoning. It is not a route to
full OBL-020, a replacement for the 65-cell audit, or a reason to choose
coverage as semantics.

## Open questions

1. Whether the registered Lean implications and non-vacuous separation model
   compile without hidden semantic assumptions is intentionally untested.
2. Whether any future Canon proof should use familywise decomposition remains
   unselected and is owner-reserved when it requires a theorem interface or
   coverage mechanism.
3. Concrete transition, history, frame, state/membership, authority-record,
   and chain premises for rule-level OBL-020 remain open.

## Suggested next prompt

Run the committed WRK-0006 red/green Lean evidence plan. Retain only its
experiment-local composition result, and freeze/escalate if the proof needs a
Canon carrier, step taxonomy, coverage rule, or final theorem interface.

## Plan update status

更新不要: the prepared `plan/161` snapshot update is deferred to the evidence
package because it is not registration metadata.

## Documentation.md update status

更新不要: high-level reader entry points and public claims did not change.

## docs/project-status.md update status

更新不要: the prepared lifecycle snapshot update is deferred to the evidence
package because it is not registration metadata.

## progress.md update status

更新不要: the prepared pending-evidence snapshot is deferred to the evidence
package because it is not registration metadata.

## tasks.md update status

更新不要: the prepared task-map snapshot is deferred to the evidence package
because it is not registration metadata.

## samples_progress.md update status

更新不要: no sample, validation command, debug surface, or workflow
classification has changed.

## Reviewer findings and follow-up

The independent reviewer and one temporary Oracle review recommend leaving the
existing statement draft unchanged and treating family coverage as an adequacy
gap rather than a present defect. A second temporary Oracle review found that
the exact global/familywise relation remains standing-eligible under ADR-0014
even though it is low-impact under the earlier LAB priority heuristic. The
registered question adopts the common safe boundary: it tests no Canon carrier,
coverage policy, step taxonomy, or OBL status.

## Skipped validations and reasons

The target Lean source, absence check, source audit, and theorem/countermodel
checks are intentionally deferred until after this pre-registration commit.
Running them first would violate ADR-0014's required outcome ordering. The
first local registration attempt was rejected by `make check` before push
because it included LAB snapshots outside the registration contract. The local,
unpushed commit is amended so it contains only the WRK, allowed metadata, and
this direct report; the retained snapshot edits move to the evidence package.
Runtime, distributed, conformance, and product validations do not apply to this
non-production theory registration.

## Commit / push status

The initial local registration commit was not pushed after its contract failure.
It is being amended with `--no-gpg-sign`, then will be revalidated and pushed
before the evidence package begins.

## Sub-agent session close status

The independent read-only reviewer completed and was closed without workspace
edits. Both temporary Oracle sessions completed without repository edits. No
sub-agent remains active for the registration package.
