---
id: meta/proposal-029
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0014, adr/ADR-0015, adr/ADR-0025]
summary: accepted M10 semanticsをper-locus executable artifactsとprocess-internal generated dispatchへ進める、owner-directed Mirrorea I2 Systems Foundation bounded program。
open_items: []
---

# PROPOSAL-029 — Mirrorea I2 Systems Foundation bounded program

## Owner disposition

Recorded on 2026-08-26: **accepted as an owner-level direction**.

The owner authorizes one bounded successor program whose parent goal is:

> From one ordinary `.mir` source, deterministically generate
> meaning-preserving per-locus executable artifacts and a generated
> communication plan, execute those artifacts as independent locus runtimes
> inside one operating-system process, and retain typed correspondence from
> source through runtime occurrence in a local Mirrorea toy fabric.

The program is named **Mirrorea I2 Systems Foundation**. It preserves the
accepted M10 cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1` as an immutable
implementation/validation regression baseline. ADR-0015 and LAB Plan 247
remain the closed authority and execution record for M0--M10; they are not
reopened or reinterpreted as this program's authority.

## Fixed capability sequence

The single semantic frontier advances in this order:

```text
SYS-0 baseline and goal alignment
→ SYS-1 runtime-kernel/conformance separation and internal carrier boundary
→ SYS-2 concurrency, memory, and effect-handler refinement
→ SYS-3 per-locus projection and executable artifact generation
→ SYS-4 in-process generated dispatch runtime
→ SYS-5 minimal typed devtools and local virtual-space vertical slice
→ SYS-6 I2 assurance, conformance, and lifecycle closeout
→ SYS-7 I3 entry contract only
```

The sole current LAB roadmap is
`LAB:plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`. It records
the parent goal, one active milestone, completed and next goals, blockers,
acceptance evidence, and deferred scope. LAB Plan 247 stays a closed baseline.

## Goal-driven operating protocol

Before each milestone begins, the roadmap records a Goal Statement with:
goal id and capability sentence, North Star link, user-visible outcome,
preserved semantic invariants, direct consumer, non-goals, primary falsifier,
exit evidence, and stop condition. A goal that only says to research or write
documents is invalid.

Every new subtask, research record, lemma, carrier, or report needs a named
direct consumer, a current blocker it reduces, and an acceptance use. For one
design question, compare only the current/smallest design and one viable
alternative. Close a question when the accepted design preserves the
Constitution, runs its positive case, detects its representative falsifier,
is usable by the direct consumer, remains conservatively extensible, avoids a
public compatibility freeze, and has no major independent-review
counterexample.

There is one report per milestone by default. Registration, metadata,
evidence attachment, snapshot synchronization, and closeout stay in that
report unless material counterevidence requires a forward-only record. Each
milestone closes its applicable normative rule, executable behavior, positive
and negative evidence, proof/model/runtime classification, source-to-
implementation correspondence, independent review, fresh validation, and
commit/push parity before the next semantic milestone begins.

## Delegated authority

Within SYS-0--SYS-7, the orchestrator may autonomously update Canon theory,
specification, architecture, ADR, plan, proof ledger, implementation, tests,
Lean/model-check evidence, CLI, internal carriers, projection IR, per-locus
artifact format, process-internal dispatch, runtime kernel, effect-handler
contract, concurrency backend profile, minimal typed devtools, agent
configuration, reports, roadmap, and derived status. Gate/Phase state may
change only when its actual Canon exit criteria and an authorized acceptance
record are satisfied.

The program may resolve or narrowly fix OPEN-030 as an **internal bounded
semantic contract**. It may not silently turn that carrier into a final public
API, ABI, or wire format. Internal request/reply/receipt data, transport
delivery metadata, authority evidence, and semantic mutation remain separate.

The program uses this decision priority: meaning preservation; authority,
privacy, and safety; ordinary simple Surface; no hidden communication,
failure, effect, or transaction; small orthogonal Core; deterministic and
inspectable semantics; finite decidability; modular proofability;
conservative extensibility; implementation simplicity; performance.

## Preserved invariants and owner-reserved stop line

The program preserves communication as a projection of checked meaning;
owner-side mutation; explicit authority, failure, effect, observation,
lifetime, and lineage; no stale resurrection; relation-first late projection;
designated non-reexecution; patch-DAG discipline; and separability of Mir,
Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform.

Stop and return an owner decision bundle only if work requires changing the
North Star; weakening authority/privacy/redaction/no-stale-resurrection;
promoting World/Avatar or other domain vocabulary into Core; introducing a
hidden multi-owner transaction; irreversibly freezing a public API/ABI/wire;
selecting or implementing real transport in this program; production
deployment or external publication; risk to user data, secrets, or paid
resources; an irreversible observable tie the Constitution cannot order; or
a reproducible contradiction between the parent goal and North Star.

Official lifecycle state, deferred general obligations, final grammar/public
contract, later I3+ work, performance, and unreviewed historical LAB coverage
are not stop reasons by themselves.

## Non-effects

Program activation is not broad PHASE-I1 exit, I2 entry acceptance, or I2
exit acceptance. At SYS-0 the official theory lifecycle remains T1 and the
broad I1 carrier-freeze residual remains open. This direction does not change
the accepted M10 profile or proof ledger, freeze a public grammar/API/ABI/wire,
choose or implement real socket/multi-process transport, provide durable
distributed save/load, deploy or publish a product, complete a browser/View
renderer, prove arbitrary relation-DAG or scheduler theorems, require
lock-free execution, or start I3 implementation. SYS-7 creates only an
inactive I3 goal and entry contract.
