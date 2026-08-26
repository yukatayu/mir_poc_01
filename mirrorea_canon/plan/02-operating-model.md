---
id: plan/02-operating-model
status: L1-fixed
maturity: reviewed
depends_on: [adr/ADR-0012, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026]
summary: active I2 Systems Foundation、closed Mir v0/I1+、通常L3 researchを分離し、goal statement、単一frontier/roadmap、direct-consumer/report/WRK/candidate/review/evidence規律を定める。
open_items: []
---

# 02 — Operating model

## Authority profiles

Three profiles are deliberately separate.

1. **Mirrorea I2 Systems Foundation execution** follows ADR-0026 and
   PROPOSAL-029. The owner approved the parent capability, SYS-0--SYS-7 order,
   decision priority, bounded semantic/implementation writes, non-effects, and
   stop line. Evidence and independent review gate integration.
2. **Mir v0/I1+ M0--M10** followed ADR-0015 and PROPOSAL-018. It is closed;
   its accepted cuts and Plan 247 remain immutable history/regression baseline
   and grant no successor authority.
3. **Research outside an active bounded program** follows ADR-0014. Its Canon write surface
   remains a reversible `working/WRK-####` L3 record; L2 promotion remains
   fail-closed while the owner-authenticated trust anchor is absent.

Neither profile changes `canon > LAB`. An agent's filesystem write capability
is not semantic authority.

## Roles and single-writer surfaces

- **owner**: supplies the North Star and program authorization; decides only
  ADR-0026 owner-reserved escalations during the active program.
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

For the active ADR-0026 program this is
`LAB:plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`; closed Plan
247 and older numbered plans are not current queues.

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

Before each ADR-0026 milestone begins, its current roadmap records:

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

Within ADR-0026, stop only at its owner-reserved condition. Otherwise close the
current SYS milestone, send a concise progress checkpoint, and continue without
waiting for owner confirmation. Official T1, deferred general obligations,
open public contracts, incomplete later phases, or unoptimized performance are
not stop conditions. Outside an active bounded program, use ADR-0014's
escalation and fail-closed rules.
