---
id: meta/proposal-018
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, adr/ADR-0012, adr/ADR-0014]
summary: owner が Mir Theory v0 と I1+ の有限な完走 scope、bounded autonomy、単一 milestone/report/frontier 運用、proof-status 更新条件を承認した記録。
open_items: []
---

# PROPOSAL-018 - Mir v0 / I1+ autonomous execution

## Owner disposition

Recorded on 2026-08-03: **accepted as an owner-level direction**.

The owner authorizes one bounded program whose completion target is:

```text
Mir Theory v0 + Mir I1+ deterministic reference system
```

The program follows Milestones 0--10 from repository and agent bootstrap,
through a concise Constitution, T0/G0 closeout, evaluation/materialization and
relation calculi, a shared formal model, Surface/checker/elaborator/runtime,
typed auth and verification extensions, and final conformance. The active
semantic frontier is one milestone. Each milestone closes normative rules,
executable behavior, positive and negative evidence, formal evidence, one
independent review, validation, and commit/push before the next semantic
milestone begins.

## Delegated authority

Within this accepted program, the orchestrator may autonomously update theory,
specification, scenarios, Gate/Phase state, the proof ledger, Lean evidence,
Rust implementation, tests, diagnostics, roadmaps, status documents, agent
configuration, and governance wording when required by the accepted
Constitution and v0/I1+ acceptance criteria. The orchestrator may select
internal carriers, judgments, algorithms, proof decomposition, provisional
syntax, tests, bounded model abstractions, and conformance details.

This delegation is evidence-gated rather than file-label-gated. Every semantic
change receives one independent review. A proof-ledger entry may become
`lean-proved` only when the corresponding statement compiles under the trusted
profile, contains no hidden axiom or placeholder proof, and corresponds to the
implementation target. Bounded search remains `model-checked-bounded`.

## Operating constraints

- One current execution roadmap and one active semantic frontier.
- One report per milestone by default. Registration, metadata, snapshot sync,
  or closeout alone does not create a report.
- A new `WRK-####` requires a direct consumer, reduction of a current blocker,
  a reason the milestone report cannot hold it, an explicit falsifier, and an
  adoption/discard rule. Frozen or closed WRKs are not reopened to manufacture
  progress.
- Compare at most the current proposal and one minimal viable alternative.
- Read only reports directly referenced by the current Canon/roadmap/status.
- Production Rust has one implementer writer by default; tests, planning, and
  review retain distinct ownership.

## Owner-reserved boundary

The orchestrator stops only if the work requires changing the North Star,
weakening authority/privacy/redaction/no-stale-resurrection guarantees,
promoting domain vocabulary such as World or Avatar into Core, making a v0
non-goal such as distributed transactions mandatory, irreversibly freezing a
final public API/ABI/wire format, beginning production deployment or external
publication, choosing between irreconcilable tied irreversible alternatives,
or risking destruction/exposure of current user data or secrets.

## Non-effects

This decision does not weaken any safety invariant, authorize a final public
contract, add production deployment, make distributed transaction/exactly-once
semantics a v0 requirement, or make LAB evidence normative. The Canon remains
the only normative source. ADR-0014 remains the default route for research
outside this explicitly delegated program.
