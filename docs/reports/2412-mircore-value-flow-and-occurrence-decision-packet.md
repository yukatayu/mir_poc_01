# Report 2412 - MirCore value-flow and occurrence decision packet

## Objective

Turn the two unresolved interfaces isolated by Plan 186 into a precise,
non-selecting owner decision packet: read-result value flow and the occurrence
identity of successful service/admission effects.

## Scope and assumptions

`mirrorea_canon/` is normative. This package may create a Canon design memo
but may not select a Core primitive, result carrier, runtime configuration,
event/occurrence schema, theorem, OBL, Gate, Phase, scenario, transport, or
public contract. LAB source and Oracle output are evidence only.

## Start state / dirty state

The worktree was clean at `43e77378`, equal to `origin/main`. The task-scoped
Discord baseline was recorded before substantive work. No pre-existing user
changes were present.

## Documents consulted

- Canon: README, MAP, meta/agent-instructions, meta/style-guide, theory/00
  through 05 where relevant, ADR-0002, ADR-0003, ADR-0005, PROPOSAL-008, and
  PROPOSAL-009.
- LAB: Plans 184 through 186, the affected source elaborator/admission/runtime
  boundaries, `Documentation.md`, `docs/project-status.md`, `progress.md`,
  `tasks.md`, and `samples_progress.md`.
- Primary sources: Moggi (1991); Flanagan et al. (1993); Felleisen and Hieb
  (1992); Fournet and Gonthier (2000); Abadi, Fournet, and Gonthier (2000).
- Advisory input: code mapper, primary-literature researcher, planner, final
  semantic reviewer, and one GPT-5.6 Sol Pro temporary Oracle consultation.

## Actions taken

1. Confirmed that LAB elaboration dependency rows contain neither a dynamic
   value/result carrier nor a read-dependent evaluation relation, and that the
   separate LAB admission/runtime helpers do not supply a shared occurrence
   model.
2. Compared established value-flow and distributed-protocol semantic families
   without treating them as Mir decisions.
3. Created PROPOSAL-012 with four independently recordable owner dispositions:
   `V` value flow, `R` successful read receipt, `S` successful served write,
   and `A` admission identity.
4. Created Plan 187 as LAB rationale, obligation map, literature comparison,
   recommendation, and stop line; registered it in both documentation catalogs.
5. Applied Oracle and final-review corrections: an evaluation frame is not a
   no-semantics alternative; OPEN-010 remains open; receipt obligations are
   explicit; served-write option names do not collide with S1/S2 strata; and
   decomposed/composite obligations are aligned with their options.
6. Synchronized the reader-facing status snapshots and current task map.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md`
- `mirrorea_canon/INDEX.json`
- `plan/187-mircore-value-flow-and-occurrence-decision-packet.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `docs/reports/2412-mircore-value-flow-and-occurrence-decision-packet.md`

## Commands run

- ordered Canon/LAB/source reads and line-numbered interface extraction
- one browser-backed `ask-chatgpt-pro-temp` review with ten exact Canon/LAB
  attachments; it completed in about 14 minutes
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check` from
  `mirrorea_canon/`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `make check`
- `git diff --check`

The first index command was accidentally started from repo root and returned
`canon root not found`; it made no file change. The corrected command from the
Canon root succeeded and its `--check` verification passed.

## Evidence / outputs / test results

The primary-source comparison supports the following lower-bound reading of the
current grammar: a non-fused read that supplies a later write needs a formal
value-flow relation, while independently observable distributed phases cannot
be hidden in one occurrence without an explicit abstraction/refinement rule.
These are constraints on a later choice, not published Mir minimality theorems.

The Oracle recommends a restricted, locus-bound one-shot result-binding
contract as the reference research model, rejects an unstated evaluation context
as semantic closure, and requires explicitly typed occurrence facets only for
genuinely atomic roles. It identifies pending control, `Gamma`/`Delta`
linearity, result correlation, save/load, read receipt, served-write identity,
and admission grant/witness mapping as the missing design obligations.

Final validation passed: Canon index check found 104 files; documentation
validation found 1,566 numbered reports; source hierarchy found all 737
required paths; `make check` completed, including `cargo check`; and the full
documentation unit suite passed 87 tests in 925.641 seconds after this report
was added. Whitespace validation also passed.

## What changed in understanding

The immediate gate is more precise than “define read semantics.” There are four
separable decisions: make dependent value flow formal; decide whether a read
receipt is explicit or abstracted with proof; decide served-write identity; and
decide whether admission creates typed facts atomically or through separate
occurrences. An evaluation frame can still be useful, but only as an explicit
representation of the restricted contract, not as a way to keep the missing
relation outside the calculus.

## Open questions

- `V`: select the restricted result-binding contract or defer it.
- `R`: select typed reply/receipt, refined abstraction, or defer it.
- `S`: select atomic `ServedWrite`, decomposed service/write, or defer it.
- `A`: select decomposed admission, atomic typed admission/verdict, or defer it.
- `PROPOSAL-008` BND-001 totality and `PROPOSAL-009` static Core/write
  correspondence remain independent owner boundaries.

## Suggested next prompt

Review `mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md` and record `V`, `R`, `S`, and `A` dispositions. The current research recommendation is `V1`, `R1`, `SW1`, and conditional `A2`; accepting any item authorizes only its later bounded design package.

## Plan update status

`plan/` 更新済み: Plan 187 records the source boundary, literature comparison,
four-way decision decomposition, later obligations, recommendations, and stop
line. `plan/00-index.md` links it.

## Documentation.md update status

更新済み: the concise frontier note now points to PROPOSAL-012/Plan 187 and
distinguishes recommendation from Canon adoption.

## docs/project-status.md update status

更新済み: the reader control view now lists V/R/S/A as the owner-facing
semantic boundary and preserves PROPOSAL-008/009 independence.

## progress.md update status

更新済み: the logical-specification snapshot and the dated recent log record
the decision packet without claiming a phase, proof, or implementation move.

## tasks.md update status

更新済み: the current task map names PROPOSAL-012 as the next owner packet and
records the precise non-selecting scope.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, validation command, debug surface,
or retained sample evidence classification changed.

## Reviewer findings and follow-up

The planner recommended one proposal with independently recordable decisions.
The Oracle challenged the initial alternative matrix and supplied the
restricted-binding/pending-control constraints. The final reviewer reported
seven actionable issues: OPEN-010 wording, a pre-selected V2 presentation,
reversed service obligations, missing receipt obligations, an inconsistent
three/four decision count, S1/S2 label collision, and an overbroad event-syntax
statement. All seven are corrected in the final packet. No reviewer selected a
Canon answer.

## Skipped validations and reasons

No new Lean model, executable sample, runtime path, or distributed protocol was
created. Such an artifact would necessarily choose the disputed semantic
carrier/occurrence mapping and would violate the proposal's non-selecting
boundary. No existing admitted executable lane supplies the shared semantics.

## Commit / push status

Validation is complete. The package is ready to commit with `--no-gpg-sign`
and push immediately.

## Sub-agent session close status

The code mapper, literature researcher, planner, and final reviewer completed
read-only work and are closed. The temporary Oracle consultation completed. No
sub-agent edited repository files.
