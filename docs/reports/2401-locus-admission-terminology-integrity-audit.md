# Report 2401 - Locus/admission terminology integrity audit

- Date: 2026-07-23 22:05 JST
- Author / agent: Codex
- Scope: cross-source audit and owner decision packet for one overview sentence
- Decision levels touched: no settled semantic decision; new L3-open Canon proposal only

## Objective

Determine whether the `child locus` / `admission path` sentence in the Canon
overview exposes a missing Core invariant or an undefined subject, and record
the smallest owner decision surface without manufacturing a hierarchy or proof
obligation.

## Scope and assumptions

Canon remains normative. This package reads the existing admission, membership,
and ordering definitions together; it does not infer a global provenance rule
from selected operational rules. The proposal is a decision request, not an
accepted theory change.

## Start state / dirty state

Started at pushed `9878757c` on `main` with no tracked worktree diff and no
local/remote divergence. Ignored local webhook and helper state remains
preserved and is excluded from evidence. At 2026-07-23 22:05 JST, the root
filesystem had approximately 70 GiB free; this documentation-only package did
not create a heavy artifact or install a toolchain.

## Documents consulted

Read Canon README, MAP, `root/glossary`, `theory/00-overview`,
`theory/01-mircore-v0`, `theory/04-ordering-and-cuts`,
`theory/05-authority`, `theory/11-metatheory-ledger`, ADR-0012, ADR-0014, the
proposal and Canon-process instructions, and the relevant gates/phases. Read LAB plans 163, 180, 181, and 182; reports
2285, 2395, 2399, and 2400; `Documentation.md`, the current status snapshots,
and the report template. Consulted an independent planner, a read-only
reviewer, and a temporary external advisory review; no external transcript is
repository state.

## Actions taken

1. Cross-checked the noun and actor of the overview sentence against the
   Glossary, `join` elaboration, admission carriers, configuration `M`, and
   causal ordering.
2. Found no basis to infer or propose a global membership-provenance OBL: no
   current source defines the alleged child relation, an admission-path object,
   unique paths, or exhaustive membership-update provenance.
3. Added PROPOSAL-010 with three bounded wording/extension alternatives and
   explicit non-effects.
4. Updated the concise reader map, owner-decision view, and LAB snapshots.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-010-locus-admission-subject-clarification.md`
- `mirrorea_canon/INDEX.json`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- targeted Canon/LAB source reads and exact terminology searches
- `date '+%Y-%m-%d %H:%M JST'`, `df -h .`, `free -h`, `du -sh . .git target .cargo .lake`, `lsblk -f`, and `findmnt -T .`
- independent planner and reviewer sub-agent consultations
- one temporary Oracle attempt that failed before prompt submission, followed by one focused temporary Oracle wording review
- `cd mirrorea_canon && python3 meta/build-index.py`
- `make check`, `git diff --check`, and focused report/index/line-budget checks

## Evidence / outputs / test results

`CON-001` says a Locus is not a Participant, while `CON-002`/`CON-003` make
the Principal and its Participant incarnation distinct. `[JOIN]` elaborates a
principal's request, and `theory/05` gives that request and its verdict
carriers. `theory/04` gives the request/verdict/activation causal family but
does not define hierarchy or universal provenance. The resulting issue is an
overview wording scope leak, not evidence for a new Core primitive, theorem,
or OBL.

The first temporary Oracle attempt failed before the prompt appeared, with no
answer and no repository effect. The focused temporary review independently
recommended replacing the sentence with an existing admission statement or
deleting it; its guidance is distilled in PROPOSAL-010 rather than copied as a
transcript. `make check` passed the Canon index check, source hierarchy,
documentation validation, and `cargo check`.

## What changed in understanding

The Canon already has a coherent admission model for principals, participants,
verdicts, epochs, grants, and membership-dependent dispatch. The overview
sentence changes the subject to an undefined `child locus`; treating that as a
missing invariant would silently introduce the hierarchy that needs proof.
PROPOSAL-010 separates the wording correction from any future hierarchy design.

## Open questions

- Does the owner accept A, B, or C in PROPOSAL-010, or defer with no current
  Canon change?
- Does the owner accept A, B, or C in the independent PROPOSAL-009 OBL-001
  statement-interface decision?
- Any future locus hierarchy or universal membership-provenance claim needs a
  separately scoped Canon proposal.

## Suggested next prompt

Record `A accepted` for PROPOSAL-010 to replace the undefined sentence with
the existing principal/admission summary, select B/C with the intended scope,
or defer without a current Canon change. This does not require deciding a
hierarchy or an OBL.

## Plan update status

`plan/` 更新不要: plans 163 and 180 through 182 already distinguish existing
proof interfaces from Core additions. A duplicate plan would not clarify this
single owner wording decision.

## Documentation.md update status

`更新済み:` the reader map now links the separate admission-subject decision.

## docs/project-status.md update status

更新済み: the owner-decision table now distinguishes the overview wording
question from membership semantics and proof obligations.

## progress.md update status

`更新済み:` the logical-specification snapshot and dated log now record the
bounded terminology decision request without changing readiness.

## tasks.md update status

`更新済み:` the current task map now lists PROPOSAL-010 separately from the
OBL-001 statement-interface decision.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample source, runnable command, or
workflow/evidence classification changed.

## Reviewer findings and follow-up

The planner classified the issue as a wording ambiguity, not a missing global
membership invariant. The reviewer listed possible broader audit candidates,
but the local source check found no basis to promote them: they are either
already known boundaries or lack the relation needed for a new obligation. The
focused advisory review agreed that leaving the undefined wording unchanged is
not a sound default. Final package-diff review found five documentation/scope
issues: an ambiguous pronoun, an incomplete validation history, missing defer
output, a stale status timestamp, and non-ID Canon references. All five are
corrected before the final revalidation.

## Skipped validations and reasons

No Lean proof, runtime, distributed, or non-index generated-artifact workflow
ran because the package changes only Canon proposal metadata and documentation.
Canon index regeneration and the repository-required `cargo check` did run
through `make check`; no targeted Cargo test is needed for this non-executable
change.

## Commit / push status

Pending at report write. The package will be validated, committed with
`--no-gpg-sign`, pushed, and then recorded in a closeout update.

## Sub-agent session close status

The planner and reviewer completed read-only work without edits. They remain
available until the final diff review, then will be closed.
