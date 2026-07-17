# Report 2283 - Surface v0 grammar scope pre-decision audit

## Objective

Reduce the Surface v0 grammar-closure question to the smallest owner decision
by testing which keyspace and expression forms are actually required by canon
and active LAB evidence. Prepare a non-effective canon design memo without
selecting syntax, changing semantics, or claiming parser readiness.

## Scope and assumptions

Canon remains normative; LAB is evidence only. The task compares existing
sources and writes a decision-request proposal under the allowed canon process.
It does not amend L1 grammar/spec text, implement a parser, promote a LAB
grammar, alter SCN expectations, or decide the proposal on the owner's behalf.

## Start state / dirty state

The worktree was clean and synchronized at `fe31622c` after Report 2282. This
task recorded its own Discord baseline before reading. No tracked source,
sample, runner, or generated artifact had changed.

## Documents consulted

- Canon `README`, `MAP`, `CHANGELOG`, agent/source-hierarchy rules,
  ADR-0001/0002/0005/0008/0009, spec/01--03, theory/01--03, and SCN-01--10
- LAB `specs/34`, `specs/39`, `specs/40`, `specs/43`, active Surface sample
  corpus, `plan/156`, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, and `samples_progress.md`
- Existing canon proposal patterns PROPOSAL-001--003 and Oracle operations

## Actions taken

- Counted indexed-state keyspaces in canonical scenario source fences and
  active Full System V1 `.mir` sources.
- Checked current canonical and active Surface sources for signed numeric
  literals and non-Participant indexed-state declarations.
- Compared the existing Core abstraction with the Surface and historical alpha
  boundaries rather than treating Core generality as a Surface requirement.
- Prepared PROPOSAL-004 with A/B/C alternatives, a narrow A candidate, explicit
  non-effects, and a requested owner disposition.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md`
- `docs/reports/2283-surface-v0-grammar-scope-predecision-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `mirrorea_canon/INDEX.json` after index regeneration

## Commands run

- focused canonical/LAB `rg`, `sed`, and scenario-source extraction
- read-only Python keyspace and Surface-form inventories
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check`
- focused Surface regression commands, documentation/source-hierarchy checks,
  and Git checks listed below

## Evidence / outputs / test results

- All six canonical indexed-state declarations use `Participant`; none uses a
  custom keyspace. All 47 indexed-state declarations found in the active Full
  System V1 Surface corpus also use `Participant`; none is non-Participant.
- `LAB:specs/40` describes alpha role keyspace `Participant` and classifies
  object/Avatar keyspaces as later. This supports, but does not normatively
  decide, a narrow v0 Surface fragment.
- No signed numeric literal appeared in canonical scenarios or the active
  Surface/current-L2 corpus. The existing `-?` lexical form and unary-minus
  precedence nevertheless require an exact decision for an unambiguous grammar.
- The Core finite-keyspace abstraction is preserved under every proposal option;
  exposing only `Participant` in Surface v0 does not reduce Core generality.
- PROPOSAL-004 recommends A: close only the current vocabulary and defer custom
  keyspaces. It has no effect until an owner records A, B, or C.

## What changed in understanding

The earlier coherence audit correctly found a closure gap but had insufficient
scope evidence to prefer custom keyspaces. The source inventory reverses that
advisory preference: current evidence favors the smaller Participant-only
Surface v0, while keeping the Core theory abstract for later extension. The
remaining owner choice is compact and does not require a decision about
World/Game, transport, authority, events, or a runtime ABI.

## Open questions

- Does the owner accept PROPOSAL-004 option A, B, or C?
- If A is accepted, should the required L1 grammar amendment use an amendment
  to ADR-0008 or a narrowly scoped additional ADR?
- If B is accepted, which declaration form and diagnostic behavior belong to a
  separate custom-keyspace design package?

## Suggested next prompt

Review PROPOSAL-004 and record `A accepted`, `B accepted`, `C deferred`, or
`return for clarification`. After A, apply and validate the closed grammar in
one canon package without broadening the language.

## Plan update status

Updated: plan/156 now records the Participant-only evidence, replaces the
earlier advisory inclination, and links the owner decision to PROPOSAL-004.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points and user commands
did not change.

## docs/project-status.md update status

更新済み: the human status view now identifies PROPOSAL-004, its A/B/C owner
choice, and the evidence-backed Participant-only recommendation.

## progress.md update status

Updated: the current research snapshot and dated log now include
T-RESEARCH-030 and the proposal boundary.

## tasks.md update status

Updated: the task map identifies PROPOSAL-004 as the active grammar owner
decision and distinguishes its evidence from a normative edit.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample, command, or
evidence classification changed.

## Reviewer findings and follow-up

Local review found that the historical alpha restriction and the current
scenario/corpus inventory agree on Participant-only v0. A fresh temporary
Oracle review was attempted as
`surface-v0-grammar-predecisio-review`; it failed before prompt submission
because the browser profile had no usable cookies. The default remains
`ask-chatgpt-pro-temp` for a later one-off review after login is repaired. No
local sub-agent service was available.

## Skipped validations and reasons

The proposal is intentionally non-effective, so parser conformance cannot be
claimed. Runtime, distributed, product, proof, and Gate validation do not apply
to a proposal-only package. Repository regression/doc checks validate only the
recording and existing LAB evidence routes.

## Commit / push status

The proposal package was committed with `--no-gpg-sign` as `0f73e2b6`
(`Prepare Surface v0 grammar closure proposal`) and pushed to `origin/main`.
Immediately after that push, `git status --short` was empty and
`git rev-list --left-right --count HEAD...@{upstream}` returned `0 0`.

## Sub-agent session close status

No local sub-agent service was available; no session was opened or requires
closure.
