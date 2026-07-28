# 2517: C2-B/C3 Relation-State Proof-Obligation Audit

- Date: 2026-07-29
- Author / agent: Codex
- Scope: conditional LAB audit of Plan 219's recommended semantic-residence option.
- Decision levels touched: none; Canon remains unchanged.

## Objective

Check that the minimal relation-state recommendation does not hide a proof,
delivery assumption, type/failure policy, or persistence rule, and record the
definitions and falsifiers a later selected model must close.

## Scope and assumptions

The audit treats option A as a proposed decision envelope, not as a selected
carrier. It changes neither Canon nor the existing request semantics. A current
history occurrence can anchor a relation within that history; no source, wire,
session, or cross-load global identity is assumed.

## Start state / dirty state

Started from clean, remote-equal commit
`59221216ceeef0223614080d71eb0859b247138b`.

## Documents consulted

`AGENTS.md`; Canon README/MAP; ADR-0014; theory/01--05 and theory/07;
P012 V1/R1; P013 M1; `OPEN-010`; `OPEN-011`; spec/04; Plans 199, 200,
209, 210, and 215--219; `Documentation.md`; `docs/project-status.md`;
`progress.md`; `tasks.md`; and the reporting template.

## Actions taken

Re-read the recommendation against P012 R1 and the core/save/load/observation
boundaries. Corrected Plan 219 so that it has explicit pending state, at-most-
one terminal branch rather than forced termination, semantic rather than raw
receipt, redaction, and causal/channel closure on save. Added Plan 220's
conditional definition/proof/falsifier matrix for M1, branch typing,
causality, linearity, observation, load, exclusions, and future elaboration.

## Files changed

- `plan/219-c2b-c3-minimal-semantic-residence-options.md`
- `plan/220-c2b-c3-relation-state-proof-obligation-audit.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `docs/reports/2517-c2b-c3-relation-state-proof-obligation-audit.md`

## Commands run

Read targeted Canon/LAB sources, calculated source digests, checked `git diff
--check`, inspected focused diffs, ran `make docs`, and committed/pushed the
evidence change before this report closeout.

## Evidence / outputs / test results

P012 records V1/R1 as the owner direction but explicitly leaves carrier,
history schema, transition rule, proof, and implementation unselected. R1
requires separate typed owner-result/requester-receipt causal steps, redaction,
duplicate/stale/wrong-locus treatment, provenance, failure behavior, and no
raw-value history leak. Plan 220 maps each of these to a required future
definition and a decisive falsifier; it does not mark any row proved.

The prior completed Oracle carrier-neutral review remains advisory evidence at
SHA-256 `d496ba61d986013e25177e065cc1444365884de50421a20156adc2ad6967d502`.
No new Oracle review was submitted here because the preceding task recorded
three browser failures before prompt submission; no unavailable external result
is treated as evidence. `make docs` passed after this report was present:
Canon index check reported 126 indexed files, source hierarchy reported 761
required paths with none missing, and documentation validation found 1,671
numbered reports.

## What changed in understanding

The relation-state route remains the smallest currently credible design
envelope, but it is not a proof and not a carrier selection. Its nontrivial
obligations are not merely correlation: they include pending/terminal status,
typed receipt rejection policy, causal closure, exact linear disposition,
redaction, and admissible save/load reconstruction. Limiting one-shotness to
accepted consumption avoids accidentally requiring at-most-once raw delivery.

## Open questions

Owner/Canon selection of the relation state and semantic receipt transition is
still required. The selected proposal must then choose only the M1 record,
failure-receipt policy, provenance/redaction relation, `Gamma`/`Delta`
disposition, and fallback scope needed to discharge Plan 220. No exact grammar,
wire protocol, scheduler, or implementation is selected.

## Suggested next prompt

Review an ordinary Canon proposal for Plan 219 option A against every Plan 220
obligation and falsifier before accepting any Core/Config amendment.

## Plan update status

`plan/` 更新済み: Corrected Plan 219, added Plan 220, and synchronized Plans
199/200 and the plan index with the obligation boundary.

## Documentation.md update status

`Documentation.md` 更新済み: Added the Plan 220 index entry and concise role.

## docs/project-status.md update status

更新済み: Kept the recommendation explicitly non-Canon and added the remaining
未解決 branch/redaction/load proof boundary to the concise status view.

## progress.md update status

`progress.md` 更新済み: Recorded the obligation audit, corrected current
logical-specification boundary, and added the timestamped work log.

## tasks.md update status

`tasks.md` 更新済み: Made the next normal Canon decision depend on the full
obligation matrix rather than the carrier choice alone.

## samples_progress.md update status

`samples_progress.md` 更新不要: No runnable sample, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

The audit incorporates the earlier completed Oracle review only for its
carrier-neutral caution. Local Canon text, especially P012 R1 and theory/04/
theory/07, supplied the concrete corrections. No callable sub-agent facility
was available; no new Oracle answer exists for this package.

## Skipped validations and reasons

Lean and runtime suites are not applicable because no executable semantics or
sample changed. No applicable validation was intentionally skipped.

## Commit / push status

Evidence change committed as
`0aa49b74406a75cfdeac535c56f5f3bc0669f8c2` (`docs: audit C2-B/C3
relation-state obligations`), pushed to `origin/main`, then verified equal to
the fetched `origin/main`. This closeout report is committed and pushed in the
following report-only closeout commit with the same verification.

## Sub-agent session close status

No callable sub-agent facility was available in this environment; no sub-agent
session was opened or left active.
