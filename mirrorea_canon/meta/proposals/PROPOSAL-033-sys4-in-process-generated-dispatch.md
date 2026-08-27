---
id: meta/proposal-033
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0026, adr/ADR-0027, adr/ADR-0028, adr/ADR-0029, arch/03-toolchain, arch/04-runtime-carriers, theory/13-evaluation-materialization, spec/12-sys3-per-locus-projection]
summary: SYS-4のgenerated-plan-only in-process locus endpoints、finite ST/OW1 dispatch、typed failure、ST cut/restore、designated-only checked patchをcut 22196f93で有限受理する提案。
open_items: []
---

# PROPOSAL-033 — SYS-4 in-process generated dispatch

## Owner disposition and selected capability

Under ADR-0026, accept the smallest SYS-4 runtime fabric that executes the
owned artifacts and communication plan produced by SYS-3 without reparsing
source, selecting a fixture plan, or inventing a route. The selected boundary
is crate-private and process-local:

```text
GlobalProjectionResult + complete sealed M9 admission + typed initial state
  -> independent locus runtimes + generated endpoints
  -> staged request / dispatch / receive / serve or typed failure
  -> source/Core/artifact/edge/runtime-occurrence correspondence
```

Each locus runtime owns its artifact, local store, incoming and outgoing
mailboxes, endpoint records, and local observation state. ST uses distinct M8
sessions for the logical loci that own semantic runtime state. The accepted
four-locus pressure profile contains two independent semantic owners and does
not expose a global cross-locus mutable store. A locus may mutate only state
owned by its projected artifact.

Bootstrap validates exact checked-program identity, projection fingerprint,
complete final M9 admission, projected authority families, locus inventory,
state schemas, indices, and fields before a live fabric exists. Initial state
is a typed seed for already projected local schemas; it cannot add Core,
owners, operations, edges, membership, capability, witness, or authority.

## Generated endpoint and dispatch contract

The runtime instantiates endpoints only by iterating the SYS-3 communication
plan. An external action may name a source-derived handler invocation and its
ordinary arguments, a declared tick, or a bounded fault injection. It cannot
choose a semantic target, add an edge, replace carrier provenance, inject an
expected result, or mint authority.

For each staged carrier, send, transport, receive, dequeue, M9 revalidation,
M8 execution, reply/publication, and terminal receipt or quarantine remain
separate observable states. Endpoint records retain exact source/Core,
fragment, edge, envelope, route, occurrence, visibility, redaction, authority-
generation, and result/publication identity. Queue position is neither request
identity nor authority. A message crosses the source outbox and target inbox;
the target never obtains a direct handle to the source store.

The selected owner lifecycle preserves owner-side RMW and request-origin
authority. The designated lifecycle preserves source-owner read, evaluator
decision, versioned publication, and explicitly named consumer as separate
steps. The evaluator-to-consumer endpoint imports the exact accepted M8
publication into the consumer partition before consumption. It never copies
the evaluator expression or raw remote input into the consumer artifact.

The accepted implementation realizes `[E-CONSUME]` through one stable source/
Core-bound semantic-consumption identity. The first accepted delivery performs
exactly one M8 semantic consume. An exact retry by the same named consumer and
same binding returns the stored typed decision without a second M8 consume;
any changed consumer, publication, frontier, policy, provenance, visibility,
redaction, or binding digest rejects. This is application-level finite
idempotent return, not transport exactly-once, hidden retry, or a multi-consumer
protocol. The accepted M10 duplicate-delivery rejection remains unchanged.

## Backend, failure, and observation boundary

ST executes every accepted finite projection. The selected OW1 correspondence
profile runs the same generated artifact only when ADR-0028's exactly-one
combined semantic owner/source-owner eligibility holds. The coordinator owns
endpoints and carrier state while the worker exclusively owns its M8 session.
The selected source/owner result and generated-dispatch observations agree with
ST. A four-locus/two-owner artifact rejects OW1 with the precise projected
ineligibility reason rather than sharing or duplicating owner state.

Failures are typed and fail closed. The bounded matrix includes unavailable
route, wrong target, stale membership or authority lineage, missing capability
or witness, missing producer/evaluator authority, malformed or duplicate
result, stale receipt/publication, split-frame frontier or policy mismatch,
payload loss, provenance/visibility/redaction corruption, and post-dequeue M8
rejection. Invalid carriers are rejected or terminally quarantined without
head blocking, unintended semantic mutation, cache consumption, authority
minting, or fabricated M8 success evidence.

Observer projection is evidence, never authority. Raw credential, capability,
witness, private payload, and internal M8 identity material do not enter the
observer-safe partition view. In OW1, a worker snapshot failure is a typed
`ObserverSnapshotUnavailable`, distinct from genuine absence. It does not turn
an already committed semantic operation into failure, reuse stale observation
state, or replay the operation. Recovery obtains a fresh exact worker snapshot.

## Process-local cut and bounded patch

Accept one ST-only whole-fabric local cut. It retains the exact projected
program and M9 generation, per-locus M8 cuts, local stores and traces, endpoint
mailboxes and send/receive records, pending carriers and typed faults, completed
receipts and consumption/cache/publication bindings, request/occurrence
counters, causality, M9 admitted-validation counters and observer-safe audit
maps, and patch lifecycle/frontier.
Restore validates all cross-record identities, causal and raw-M8 dependencies,
counter floors, endpoint symmetry, authority-generation live floor, and patch
lifecycle before mutation. It can continue one retained in-flight owner or
designated delivery exactly once. OW1 cut/restore remains typed
`BackendIneligible` until an acknowledged worker-cut protocol exists.

Accept one ST-only, quiescent, designated-expression patch boundary. The
runtime receives only a `Sys4CheckedPatchCandidate` already checked, projected,
and completely M9-admitted outside the fabric. It receives no source text,
AST, manual edge, expected result, or raw grant. The candidate must bind the
exact base program/projection/activation frontier and exact M9 authority
lineage; retain topology, state schema, owner routes and RMW Core, relations,
non-designated fragments/edges/handlers, authority lineages and tombstones;
and change only the admitted designated material. Installation is prepared on
a clone and becomes visible only after the shared M9 live floor is atomically
rebound. Accepted activation advances the patch generation and invalidates
old designated caches. Rejected or stale candidates append only a typed
lifecycle row and leave semantic and authority state unchanged. OW1 patch
activation remains typed `BackendIneligible`.

This is a finite designated-only patch pressure case, not general hot-plug,
arbitrary compatibility, durable upgrade, or distributed activation.

## Falsifiers, evidence, and stop condition

The contract is falsified if execution reparses source or uses fixture names;
an external schedule supplies a route, Core, authority, or expected result; a
target mutates a foreign store; an accepted Core edge is missing or an extra
edge runs; the same semantic consume reaches M8 twice; a corrupt retry returns
cached success; a typed fault mutates semantic state or mints authority; an
observer failure becomes absence or stale evidence; ST and eligible OW1 differ
on the selected result; restore admits a forged/stale/asymmetric cut; or a
rejected patch changes anything beyond patch lifecycle evidence.

The accepted source/evidence cut is
`22196f93b0112b8fd2987ec078021c8865b71651`. Evidence includes 99 focused
SYS-4 tests and the full 179-test `mir-runtime` library suite, preserved M10
source/CLI/conformance regression, formatting, warnings-denied Clippy, diff
validation, and independent semantic/code review. OBL-061 classifies only this
finite executable correspondence as `runtime-monitored`. No Lean statement,
bounded model result, or general theorem is added.

```text
Direct consumer: SYS-5 minimal typed devtools and local virtual-space slice
Blocker reduced: SYS-3 artifacts and generated communication were static plans;
  no independent locus runtime actually crossed a generated endpoint
Acceptance use: SYS-5 causal viewer and toy-world execution, then the SYS-6
  finite source-to-runtime conformance profile
```

Close SYS-4 at this cut. Reopen only for a reproducible generated-edge omission
or invention, direct cross-locus mutation, source/fixture-plan reconstruction,
authority or redaction loss, duplicate semantic consume, selected ST/OW1
semantic mismatch, stale/forged cut acceptance, rejected-patch mutation, or a
SYS-5 direct consumer unable to reconstruct the accepted causal line from the
typed runtime evidence.

## Non-effects

This proposal does not define or expose a CLI, public API/ABI/wire/JSON schema,
deployment mapping, real transport, socket, retry/exactly-once protocol,
durable persistence, multi-process recovery, OW1 cut/patch, multi-owner OW
execution, general scheduler/data-race/projection/dispatch/save/patch theorem,
production relation DAG, arbitrary patch, browser/View, or final devtools
surface. It does not change theory T1 or accept broad PHASE-I1, official I2
entry, or I2 exit. SYS-5 is the sole next active milestone; SYS-6 follows it.
