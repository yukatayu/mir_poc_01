---
id: plan/02-operating-model
status: L1-fixed
maturity: reviewed
depends_on: [adr/ADR-0012, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026, adr/ADR-0033, adr/ADR-0034, adr/ADR-0035, adr/ADR-0036, adr/ADR-0037]
summary: active ADR-0034 program/ADR-0035三軸map/ADR-0036 trust boundary、program外L3 research、goal/frontier/review規律。
open_items: []
---

# 02 — Operating model

## Authority profiles

Four profiles are deliberately separate.

1. **Mirrorea I3 Distributed Foundation** follows ADR-0034 and
   PROPOSAL-037. Its fixed sequence is ALIGN-0--2, I3-0--6, NEXT-0; Plan 250
   is its sole current roadmap. ALIGN-0/1/2 and I3-0 are completed, I3-1 is the
   sole active frontier, and I3-2 is next/inactive under ADR-0037.
2. **Mirrorea I2 Systems Foundation SYS-0--SYS-7** followed ADR-0026 and
   PROPOSAL-029. It is closed by ADR-0033; its accepted cuts and Plan 249 are
   immutable history/regression baseline and grant no I3 or successor authority.
3. **Mir v0/I1+ M0--M10** followed ADR-0015 and PROPOSAL-018. It is closed;
   its accepted cuts and Plan 247 remain immutable history/regression baseline
   and grant no successor authority.
4. **Research outside the active bounded program** follows ADR-0014. Its Canon write surface
   remains a reversible `working/WRK-####` L3 record; L2 promotion remains
   fail-closed while the owner-authenticated trust anchor is absent.

ADR-0033 / plan/05 remains the accepted I3 entry boundary consumed by
PROPOSAL-037 / ADR-0034. Program activation is not official I3 entry or exit.

Neither profile changes `canon > LAB`. An agent's filesystem write capability
is not semantic authority.

## Roles and single-writer surfaces

- **owner**: supplies the North Star and any future program authorization;
  decides reserved semantic/public/transport/production boundaries.
- **parent/orchestrator**: owns milestone state, Canon integration, writer
  delegation, final evidence judgment, commit, push, and remote parity.
- **planner**: single writer for the delegated current roadmap, dependency map,
  milestone criteria, decision queue, and derived status snapshots. It may
  apply and validate those edits. It does not normally write production Rust,
  tests, Lean proofs, or normative semantics unless the parent delegates an
  exact file surface.
- **theory/formalization writer**: owns a delegated Canon/Lean surface.
- **implementer**: normally the single production Rust/source writer.
- **test author**: owns test/fixture/model-check harness surfaces.
- **reviewer**: read-only and independent from the author of the same change.
- **status reporter**: updates current status only at milestone close when that
  surface has not been delegated to the planner.

Agents share a worktree: each writer preserves other writers' edits and stays
inside its assigned surface. The orchestrator serializes overlapping changes.

## One semantic frontier and one roadmap

The active semantic work-in-progress limit is one milestone. Independent
implementation, tests, formalization, and review may run concurrently only when
they test the same semantic candidate. Do not open competing candidate families
as separate active frontiers.

One LAB document is designated the **current execution roadmap**. It contains:

- active and next milestone;
- direct blocker and direct consumer;
- acceptance criteria and validation gates;
- current owner boundary; and
- deferred scope.

ADR-0034 designates
`LAB:plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`. Plan 249,
Plan 247, and older numbered plans are not current queues.

`progress.md`, `tasks.md`, `docs/project-status.md`, and `Documentation.md` are
derived snapshots. Older numbered plans remain repository memory and are not a
queue. `plan/01-phases` remains the official Gate/Phase lifecycle source; the
LAB roadmap sequences work but cannot invent a lifecycle fact.

## Milestone close unit

Each milestone closes the relevant subset of:

```text
normative rule
Lean definition/statement/theorem
executable reference behavior
positive case
negative/counterexample case
independent review
focused validation
commit + push + remote parity
```

A later layer must not redefine an earlier accepted semantic rule. Where a
feature adds an extension, prove or test its local preservation condition and
compose extensions only after those local conditions close.

## Goal Statement and direct-consumer protocol

Before each bounded-program milestone begins, its current roadmap records:

```text
Goal ID and capability sentence
North Star link
User-visible outcome
Semantic invariants
Direct consumer
Non-goals
Primary falsifier
Exit evidence
Stop condition
```

The goal sentence names a system capability or a semantic contract directly
required by it; “research” or “write documents” is not sufficient. Every new
subtask/research/lemma/carrier/report records its direct consumer, blocker
reduced, and acceptance use. Do not start it unless it advances per-locus
generation/dispatch or closes a required meaning/authority/failure/memory
boundary, and can close through a positive case plus falsifier.

## Report policy

Create **one report per milestone by default**. The report combines objective,
decisions, model, implementation, tests, proof evidence, review, corrections,
non-claims, commit/push, and next milestone.

Do not create a separate report merely for registration, evidence attachment,
metadata linking, snapshot synchronization, configuration wording, path fixes,
or closeout. Add a second report only when material counterevidence requires a
forward record without rewriting the original milestone report. Trivial work
is recorded in the open milestone report or commit and does not create a report.
Report count is never a progress metric.

## WRK admission and candidate limit

A new `WRK-####` is allowed only when all are true:

1. a named current-milestone direct consumer exists;
2. it reduces a real current blocker;
3. the milestone report cannot contain the investigation;
4. an explicit falsifier exists; and
5. an adoption/discard rule exists.

Do not reopen frozen or closed WRKs to create progress. ADR-0014 adds its
standing eligibility requirements for research outside the program.

For one design question compare at most:

```text
current proposal
one smallest viable alternative
```

If both are falsified, integrate their causes before proposing one successor.
Do not enumerate a third candidate speculatively.

Close the question when the accepted choice preserves the Constitution, runs
the positive case, detects the representative falsifier, is usable by the
direct consumer, has a conservative extension path, freezes no public
compatibility boundary, and independent review has no major counterexample.

## Review and correction

Each milestone receives one independent review after author self-check and
focused validation. Review prioritizes hidden communication/authority,
source/Core mismatch, lost update, stale resurrection, fallback re-promotion,
information leaks, rejected-patch mutation, projection loss, fake evidence,
theorem/implementation mismatch, and unexplained residual obligations.

Apply one correction cycle. A second review/correction is justified only by new
material counterevidence. Style-only preference does not reopen a closed
semantic decision.

## Evidence and proof claims

Classify every proof obligation as one of:

```text
lean-proved
lean-stated
model-checked-bounded
runtime-monitored
intentionally-deferred
```

`lean-proved` requires trusted compilation, an axiom/placeholder scan, and
theorem-to-implementation correspondence. `sorry`, `admit`, a `True` stub, or a
bounded enumeration is not a general proof. Run only validations actually
claimed and record skipped validations with reasons.

Hash, manifest, release anchor, and artifact identity support reproducibility;
they do not become the semantic/runtime goal or force runtime architecture to
depend on conformance/release orchestration.

## Reading and record discipline

Read Canon first and resolve locations through `INDEX.json`. Read only reports
directly referenced by current Canon, roadmap, or status; do not reconstruct the
entire report chronology. LAB keeps implementation, tests, artifacts, reports,
and historical plans. Canon keeps current normative rules, decisions, assurance
status, and lifecycle state.

Heavy disposable artifacts use a verified configured external workdir. Never
assume a mount exists. Preserve user changes, history, old T0 artifacts, secrets,
and source data; do not force-push or rewrite history.

## Stop condition

ADR-0026 is closed; its old continuation rule no longer authorizes work.
ADR-0034 now authorizes only the fixed Plan 250 program. Outside that scope,
use ADR-0014's standing L3, escalation, and fail-closed rules. Official T1, deferred general
obligations, open public contracts, incomplete later phases, or unoptimized
performance do not by themselves create an active goal.
