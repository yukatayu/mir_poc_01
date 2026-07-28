# 2518: C2-B/C3 Relation-State Owner Proposal Preparation

- Date: 2026-07-29
- Author / agent: Codex
- Scope: prepare and critically review a bounded Canon owner decision request
  for the C2-B/C3 relation-state envelope.
- Decision levels touched: no decision level changed; PROPOSAL-017 remains an
  unadopted L3-open decision-request artifact.

## Objective

Turn the accepted direction of continued C2-B/C3 investigation into a precise,
reviewable owner decision request without silently selecting a Core/Config rule,
runtime carrier, source syntax, proof, or implementation.

## Scope and assumptions

The proposal is limited to V1/R1 cross-locus read requests and M1 validation
context. The user asked that eventual ergonomic inference be considered, but
this task treats source omission as a separate later decision rather than an
immediate consequence. No owner disposition is inferred from the preparation.

## Start state / dirty state

Started from clean, remote-equal commit
`1d84ab4a637527ff837ce3a8df4b41dff5575d55`.

## Documents consulted

`AGENTS.md`; Canon README/MAP; ADR-0012 and ADR-0014; style guide;
theory/01--05 and theory/07; PROPOSAL-012 and PROPOSAL-013; Plans 199, 200,
219, and 220; `Documentation.md`; `docs/project-status.md`; `progress.md`;
`tasks.md`; the Oracle operating manual and repo-local operating note; and the
reporting template.

## Actions taken

Created PROPOSAL-017 with an X1/XD choice. Asked an independent Oracle review
to challenge the first draft. Applied every blocking correction from that
review: narrowed scope to cross-locus reads, required an explicit unique
pending binding, split owner-service outcome from receipt/use state, bounded
receipt rejection, removed a premature `pure`/zero-occurrence selection,
mapped causality to existing theory/04 relations or explicit later amendments,
completed the save/load state matrix, kept validation provenance non-authority,
made observation conditional, prohibited immediate source convenience, and
kept the review matrix outside the OBL ledger.

Added Plan 221 as LAB memory of the proposal preparation, including the review
corrections, and synchronized reader-facing/current-state documents. Regenerated
the Canon index.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-017-c2b-c3-relation-state-envelope.md`
- `mirrorea_canon/INDEX.json`
- `plan/221-c2b-c3-canon-proposal-preparation.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `docs/reports/2518-c2b-c3-relation-state-owner-proposal-preparation.md`

## Commands run

Read focused Canon/LAB files; ran `oracle status`; ran one new
`ask-chatgpt-pro-temp` review with nine attached source files; inspected its
completed output critically; ran `git diff --check`; attempted
`python3 mirrorea_canon/meta/build-index.py` from repository root (expected
working directory error: `canon root not found`); reran correctly as
`cd mirrorea_canon && python3 meta/build-index.py`; ran `make docs`; then ran a
second, corrected-text final Oracle review; committed/pushed the proposal
evidence; and checked remote equality before this report closeout.

## Evidence / outputs / test results

The independent review completed with verified GPT-5.6 Sol + Pro selection in
14m24s. It found that X1 was a viable decision family but the initial proposal
was too broad and premature. The corrected text limits X1 to the already
directed V1/R1 read boundary and makes every later semantic addition explicit.
It preserves P012's non-effects: no successful-write acknowledgement, transport
guarantee, public wire/API, source grammar, or implementation consequence.

The Canon index regeneration succeeded with 127 indexed files. `git diff
--check` passed after the update. `make docs` passed: Canon index check found
127 files, source hierarchy found all 761 required paths, and documentation
validation found 1,672 numbered reports. A second final Oracle review completed
in 7m27s with verified GPT-5.6 Sol + Pro selection and returned `PASS`.

## What changed in understanding

The required owner choice is smaller and cleaner than the first draft suggested:
whether to authorize a bounded investigation of a relation-state envelope for
cross-locus reads. It is not a choice of a final exchange schema or an
operational receipt rule. A unique requester-side pending binding and a
service-outcome/receipt-use product state are necessary constraints on any
later model; they do not yet define its fields or transitions. Ergonomic
surface omission belongs after a semantic and elaboration proposal, not here.

## Open questions

The owner must still record `X1`, `XD`, or a clarification request. After an
X1 record, the next package must select the smallest state/transition model and
then test Plan 220's falsifiers. The exact relation schema, receipt-rejection
policy, consumption presentation, SaveObject amendment, observation projection,
and future source syntax remain unresolved.

## Suggested next prompt

Review PROPOSAL-017 as the owner decision request and record `X1`, `XD`, or
`return for clarification`; X1 authorizes only the next bounded design and
falsification package.

## Plan update status

`plan/` 更新済み: Added Plan 221 and synchronized Plans 199/200 plus the plan
index with the unadopted proposal boundary and review corrections.

## Documentation.md update status

`Documentation.md` 更新済み: Added P017 and Plan 221 to the reader-facing
index and current C2-B/C3 explanation.

## docs/project-status.md update status

更新済み: Replaced the broad carrier-choice description with the smaller,
unadopted P017 X1/XD decision request and its exact non-effects.

## progress.md update status

`progress.md` 更新済み: Recorded the reviewed decision request, current macro
position, and timestamped work log without marking any semantic rule complete.

## tasks.md update status

`tasks.md` 更新済み: Made P017's X1/XD record the immediate C2-B/C3 owner gate
and preserved the distinction from post-decision autonomous research.

## samples_progress.md update status

`samples_progress.md` 更新不要: No runnable sample, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

Two completed Oracle temporary consultations were used as advisory review. The
first identified ten blocking issues; the corrected text passed the second
final review. Neither review creates a Canon decision or replaces local source
checking. No callable sub-agent facility was available in this environment.

## Skipped validations and reasons

Lean, runtime, and sample suites are not applicable because no executable
semantics, source implementation, or sample changed. No applicable validation
was intentionally skipped.

## Commit / push status

Proposal evidence committed as
`4a2b858d695e0be4426498fb7c76dbfacde96252` (`docs: prepare C2-B/C3
relation-state decision`), pushed to `origin/main`, then verified equal to the
fetched `origin/main`. This closeout report is committed and pushed in the
following report-only closeout commit with the same verification.

## Sub-agent session close status

No callable sub-agent facility was available in this environment; no sub-agent
session was opened or left active.
