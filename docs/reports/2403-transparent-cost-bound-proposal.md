# Report 2403 - Transparent cost-bound proposal

## Title and identifier

Report 2403 - Transparent cost-bound proposal.

## Objective

Separate the missing `cost_bound` condition in the Canon transparent-overlay
rule into an owner decision packet without changing the Contract rule,
cost algebra, runtime, OBL-026, patch carrier, or public status.

## Scope and assumptions

- Canon remains normative; this report and `plan/183` are LAB evidence.
- The current all-must-hold wording does not state whether its list is
  exhaustive or how omitted Contract fields are treated. The package records
  that ambiguity; it does not assert a hidden rule.
- `OPEN-013` remains open. No final cost representation or comparison is
  selected here.

## Start state / dirty state

The worktree was clean at `1eca4f61`, with `main` equal to `origin/main`. The
latest source-cut priority screen had selected no new WRK, but it did not amend
ADR-0014 or prohibit a later independent candidate.

## Documents consulted

- Canon: `README.md`, `MAP.md`, `GLOSSARY.md`, `theory/02-types-effects-failures.md`,
  `theory/08-patch-hotplug.md`, `theory/11-metatheory-ledger.md`,
  `plan/03-risks.md`, `adr/ADR-0012.md`, and the Canon proposal process.
- LAB: `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `plan/00-index.md`, `plan/40-layer-compatibility-freeze-roadmap.md`,
  and the cited clean-near-end source/runtime evidence.
- Process: `AGENTS.md`, `ADR-0014`, and the repository's report requirements.

## Actions taken

1. Compared the Contract field list with the named transparent-overlay
   conditions and isolated the omitted `cost_bound` question.
2. Checked its relationship to transparent patch overlays, OBL-026, OPEN-013,
   the risk register, and the active clean-near-end LAB evidence.
3. Added a non-normative owner decision packet with four alternatives and
   explicit non-effects.
4. Added a concise LAB memory memo and synchronized reader/status snapshots.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-011-transparent-cost-bound-substitutability.md`
- `mirrorea_canon/INDEX.json`
- `plan/183-transparent-cost-bound-substitutability-decision.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `docs/reports/2403-transparent-cost-bound-proposal.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- targeted Canon/LAB source reads, literal searches, source digest capture, and
  independent planner/reviewer consultation
- Canon index rebuild/check and `make check` (first run stopped because the new
  numbered plan was unregistered; second run stopped on the stale snapshot
  timestamp; both metadata omissions were corrected before the next run)
- final `make check` and `git diff --check` after the review corrections
- focused final package-diff re-review
- `git worktree add --detach /tmp/mir-poc-2403-audit-9e0901c8 9e0901c8`
- `make docs` in that clean disposable worktree
- `git worktree remove /tmp/mir-poc-2403-audit-9e0901c8`

## Evidence / outputs / test results

The pre-edit source cut is `1eca4f61`. `theory/02` contains `cost_bound` in
the Contract tuple but does not name it in the following all-must-hold
conditions. The text does not define whether those conditions are complete or
an old/new-bound preservation relation. `OPEN-013` describes a current simple
numeric comparison while deferring final algebra/runtime semantics. The active
LAB CostBudget evidence uses named, pointwise counters and rejects
`remote_calls 1 <= 0`; it is not a Canon Contract/layer implementation.

The final `make check` passed: Canon index check, source-hierarchy check (733
required paths), docs validation (1,557 numbered reports), and `cargo check`.
The final whitespace check passed. The clean disposable worktree at committed
`9e0901c8` was clean before and after `make docs`; that audit passed the Canon
index, source-hierarchy, and docs validations with the same 733/1,557 counts.

## What changed in understanding

The repository does have a concrete semantic clarification worth elevating,
but it is owner-reserved rather than an ADR-0014 working-record target: the
Canon does not state how a changed `cost_bound` participates in transparent
overlay. The owner may select an explicit treatment or defer interpretation;
the package does not require a general cost calculus now.

## Open questions

- Which PROPOSAL-011 alternative does the owner select?
- If A is accepted later, which finite comparison fragment and outside-fragment
  behavior are selected without claiming a universal cost algebra?
- Does a later patch-specific carrier need to name cost separately, or can it
  continue to reference the selected whole-Contract layer law?

## Suggested next prompt

Record an A/B/C/D/defer disposition for PROPOSAL-011 when the Contract rule is
ready to change. Independently, continue only ADR-0014-eligible LAB research
that has its own committed pre-registration and does not decide this question.

## Plan update status

`plan/` 更新済み: `plan/183` records the source relation, LAB evidence limit,
owner packet, and safe reopen boundary; `plan/00-index.md` now registers it.

## Documentation.md update status

`Documentation.md` 更新済み: the concise reader map now names PROPOSAL-011 and
the LAB decision memory.

## docs/project-status.md update status

更新済み: the proposal inventory now includes the Contract cost overlay decision
without treating it as an accepted rule.

## progress.md update status

更新済み: the current logical-specification snapshot and dated log distinguish
the new decision packet from a theory/OBL/workflow change.

## tasks.md update status

更新済み: the current task map records PROPOSAL-011, its alternatives, and the
separation from future ADR-0014 research.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or evidence classification changed.

## Reviewer findings and follow-up

An independent targeted reviewer identified the omitted Contract field and
recommended a scoped non-weakening option. The broader reviewer found three
wording defects: treating the listed conditions as exhaustive, treating an
existing numeric check as a defined old/new relation, and asserting that a
field must receive an explicit treatment. The proposal, memo, and snapshots
were corrected so all four owner options remain live. The same reviewer
classifies cost scope as an OPEN-013 contract decision rather than an L3 repair.
The temporary Oracle review does not select a new L3 target at its supplied
source cut and likewise does not treat its advice as Canon. A planner located a
separate active-cost sample candidate; it remains subject to independent
pre-registration and does not decide PROPOSAL-011.

The focused re-review of the corrected package returned no blocking findings.

## Skipped validations and reasons

No cost-runtime, layer evaluator, Lean proof, or candidate outcome command ran:
this package is a decision packet and must not manufacture evidence that a
later working record has not pre-registered.

## Commit / push status

The validated package was committed with `--no-gpg-sign` as `9e0901c8`
(`docs: add contract cost decision packet`) and pushed to `origin/main`. This
audit/closeout update is committed and pushed immediately afterward as a
separate status commit so the report records the first package commit without
self-referential history.

## Sub-agent session close status

The planner and both reviewers completed read-only consultation, including the
focused corrected-diff re-review. The temporary Oracle consultation completed.
No sub-agent made repository edits.
