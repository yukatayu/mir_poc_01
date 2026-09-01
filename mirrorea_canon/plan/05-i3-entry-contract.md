---
id: plan/05-i3-entry-contract
status: L1-fixed
maturity: reviewed
depends_on: [adr/ADR-0033, adr/ADR-0034, arch/04-runtime-carriers, theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, theory/08-patch-hotplug, spec/06-conformance, scenarios/SCN-01, scenarios/SCN-02, scenarios/SCN-03, scenarios/SCN-06]
summary: ADR-0034 programがconsumeするI3 goal、transport-neutral adapter、failure/order refinement、C-distributed gates。
open_items: [OPEN-032]
---

# 05 — I3 entry contract

## Status and future goal

ADR-0033 accepted this contract as an inactive entry boundary. PROPOSAL-037 /
ADR-0034 now consumes it for the active bounded program, whose parent goal is:

> Execute accepted I2 per-locus artifacts and generated communication across
> at least two operating-system processes using a real transport while
> preserving semantic authority, typed failure, source/Core provenance,
> redaction, and Mir abstract ordering, and close a finite C-distributed
> profile.

Program activation is not official I3 lifecycle entry or exit. LAB Plan 250 is
the sole current roadmap; ALIGN-0 is completed and ALIGN-1 is active. The two transport candidates
remain unselected and OPEN-032 remains unresolved.

## Accepted input boundary

The active ADR-0034 program starts from the accepted I2 boundary, not from source-free
message schemas:

```text
ordinary checked source
  -> checked global Core
  -> per-locus executable artifacts
  -> generated CommunicationPlan / typed internal carrier
  -> transport adapter (future)
  -> remote locus runtime (future)
```

Deployment may map a logical locus to a process/host. It may not invent
communication edges, owner operations, capabilities, witnesses, effect rows,
failures, state, expected results, or semantic occurrences.

## Candidate inventory

Only the following future evaluation candidates are admitted:

| ID | Candidate | Required shape | Current status |
|---|---|---|---|
| A | TLS-over-TCP framed reliable-stream adapter | explicit framing above a reliable byte stream; partial read/write and head-of-line consequences remain visible | **UNSELECTED** |
| B | QUIC reliable-stream adapter | reliable streams only; cross-stream/reconnect ordering remains explicit | **UNSELECTED** |

QUIC datagrams are not admitted or evaluated. The contract does not choose a
protocol version, codec, framing/wire fields, implementation library,
certificate format, port, process topology, connection pooling, retry policy,
or deployment environment. Future comparison uses the same failure/order/
authority gates for A and B. OPEN-032 remains unresolved.

## Authority and identity invariants

The following are non-authoritative metadata:

```text
host/address/port
process or operating-system identity
transport or adapter identity
connection/stream/session identity
TLS certificate or authenticated peer identity
QUIC connection identifier / retry token / migration state
queue/stream position and delivery timing
```

None can mint or transfer Core, locus ownership, membership,
epoch/incarnation, capability, witness, auth/policy verdict, semantic state,
observation permission, or expected result. Cryptographic peer authentication
may be an input to a separately admitted policy, but is never that policy's
grant or Mir authority itself.

Request identity remains source/Core-bound and distinct from every network and
runtime occurrence. Reconnect or connection migration does not create a new
semantic request, renew a revoked grant, or resurrect an old incarnation.

## Internal carrier and public wire

The accepted I2 internal carrier is typed and non-public. A future public wire
is a separate representation with a checked mapping:

```text
internal semantic carrier
  -> future versioned encoding (unresolved)
  -> transport adapter bytes/streams
  -> checked decoding/admission
  -> internal semantic carrier
```

Encoding/decoding may not add or omit semantic meaning, collapse request /
serve / result / receipt, hide failure/effect/visibility, or weaken redaction.
Public versioning, compatibility, schema evolution, limits, codec, and wire
diagnostic spelling require separate decisions. No public freeze follows from
this entry contract.

## Required failure matrix

Every future candidate/profile must exercise and type the following cases.
Diagnostic spellings remain internal/unresolved; semantic families and
non-mutation requirements are fixed.

| Condition | Required future result | Preserved invariant / falsifier |
|---|---|---|
| no declared route, partition, endpoint absent/refused | explicit route/unavailable failure within bounded profile turn | no hang, silent drop, or route invention |
| transport handshake or peer-admission failure | typed transport/admission failure before semantic admission | certificate/session is not authority |
| wrong target locus/owner/artifact/operation | typed rejection before owner mutation | generated plan and owner preservation |
| source/Core/artifact/carrier provenance mismatch | typed admission rejection | no source-free operation/state |
| partial write/read or split frame | buffer until one checked frame or reject typed incomplete frame | no partial semantic request |
| truncated/malformed/oversized frame | typed fail-closed rejection before carrier admission | no parser confusion or hidden truncation |
| stream reset/disconnect before remote admission | typed unavailable/cancelled outcome with no remote mutation | request not assumed served |
| disconnect after remote admission before result/receipt | explicit ambiguous-delivery state joined to request identity | no blind retry or false success |
| reconnect/new connection/migration | new non-authoritative session; old request/grant lineage revalidated | no stale resurrection |
| duplicate request | operation-specific stored-result/no-new-consume or typed duplicate rejection | no second semantic mutation; no global exactly-once claim |
| duplicate/stale result or receipt | typed duplicate/stale rejection or exact already-decided observation | receipt is not authority/transfer |
| reorder across streams/connections/control-data paths | explicit dependency buffering or typed stale/order rejection | stream order is not Mir order |
| stale membership/epoch/incarnation | `StaleMembership`-family rejection before mutation | monotone retirement |
| missing/revoked capability or witness | `MissingCapability` / `MissingWitness`-family rejection | no reconnect authority renewal |
| auth/policy layer unavailable/rejected/revoked | declared typed failure before activation/use | transport authentication does not collapse policy |
| backpressure/queue capacity | typed capacity/backpressure outcome | no silent loss or unbounded hidden buffering |
| timeout/lease/clock advance | explicit external-time/lease outcome | schedule/clock cannot mint semantic facts |
| external effect/provider failure | declared effect failure with provider non-authority | no hidden effect success |
| visibility/redaction mismatch | `VisibilityDenied`-family rejection/redacted diagnostic | no credential/private-state leak |
| patch activation or save/cut with in-flight traffic | explicit quiescence/admission rule or typed rejection | no stale post-cut/patch resurrection |

Retry is never implicit. A future profile must record who initiated retry, why
it is permitted, its relation to the original request identity, and whether
the semantic operation specifies stored-result return or duplicate failure.
Neither reliable stream candidate supplies exactly-once semantics.

## Network ordering refinement

For each accepted operation, future evidence maps concrete occurrences to the
Mir abstract trace:

```text
local program/order eligibility
  -> request occurrence
  -> adapter send admission
  -> frame/stream transmission
  -> remote complete-frame receive
  -> carrier admission and authority revalidation
  -> owner/effect serve linearization
  -> typed result/failure send
  -> result receive
  -> receipt or designated consume
```

The mapping must preserve at least:

- local program order and owner-local modification/coherence order;
- request -> serve and send -> receive;
- serve -> result and result -> receipt/consume;
- publish -> observe;
- witness create -> use and capability grant/revoke -> use;
- membership update -> dependent dispatch;
- verdict -> activation cut;
- semantic fallback -> later access; and
- cut/save quiescence -> later local transition.

Within-stream byte order cannot justify cross-stream, cross-connection, or
post-reconnect semantic order. Explicit dependency/frontier/provenance fields
must do so. Required visibility edges must dominate dependent admission/use.
Late old-session traffic is rejected or placed by an explicit current
dependency; it cannot become a fresh occurrence. The future profile records
operation linearization, reads-from/coherence where needed, and a bounded
scheduler assumption. Arbitrary fairness, WAN liveness, hardware memory, and
lock-free refinement remain deferred.

## I3 C-distributed gates

Frozen ordinary-source scenarios remain unchanged. The active ADR-0034 program
must add C-distributed execution evidence for:

| Gate | Positive path | Representative falsifier |
|---|---|---|
| SCN-01 | requester and World owner in distinct processes; request/serve/write/publish/observe preserve source lineage | route or visibility failure hangs, drops, mutates, or leaks |
| SCN-02 | owner evaluates same-owner RMW remotely; two accepted requests serialize and produce 100->90->80 | requester precomputes private value, missing/revoked cap mutates, or ambiguous delivery double-applies |
| SCN-03 | remote admission verdict/epoch/incarnation/grant dominates dependent write and visible history is observation only | pre-verdict write, replayed cap, reconnect, or certificate replays authority |
| SCN-06 | missing/partitioned route returns explicit `RouteUnavailable` within the finite turn budget; later admitted route can serve the same source | indefinite block, silent retry/drop, or transport-created route |

Each gate requires:

1. ordinary source as semantic authority;
2. generated artifacts/communication only;
3. actual two-or-more-process transport execution;
4. positive and typed negative/fault cases;
5. source/Core/artifact/carrier/network/runtime correspondence;
6. observer-safe trace and diagnostic output;
7. exact `lean-proved` / `lean-stated` / `model-checked-bounded` /
   `runtime-monitored` / `intentionally-deferred` classification; and
8. independent semantic/security/concurrency review.

Existing I2 evidence is the regression baseline. It is not counted as actual
C-distributed execution.

## Entry, stop, and activation rules

An I3 program may start only after owner direction names:

- its bounded parent goal and sole current roadmap;
- the retained or revised at-most-two candidate set;
- the exact transport comparison and selection authority;
- the finite C-distributed profile and failure/order evidence;
- public-wire non-freeze or separately authorized freeze boundary; and
- owner-reserved production/security/deployment stop line.

PROPOSAL-037 / ADR-0034 satisfy these program-activation inputs and designate
Plan 250. They do not select transport or apply official I3 entry/exit. Those
lifecycle transitions remain explicit I3-6 acceptance actions after actual
evidence and independent review. OPEN-032 remains unresolved.

## Non-claims

This contract does not select/implement TLS-over-TCP or QUIC, admit/evaluate
QUIC datagrams, choose version/codec/wire/library/certificate/port/deployment,
freeze public compatibility, claim exactly-once or hidden retry, implement
multi-process runtime, prove general network ordering/fairness/security/
durability, accept broad PHASE-I1, change theory T1 or OBL status, apply
official I3 lifecycle entry/exit without I3-6 acceptance, deploy production,
or complete the public Mirrorea product.
