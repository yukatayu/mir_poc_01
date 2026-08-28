---
id: meta/proposal-036
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0026, adr/ADR-0032, arch/04-runtime-carriers, theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, spec/06-conformance]
summary: SYS-7のtransport-neutral inactive I3 entry contractを採用し、transport未選定のままADR-0026 programを閉じる提案。
open_items: [OPEN-032]
---

# PROPOSAL-036 — SYS-7 inactive I3 entry contract and program close

## Owner disposition and selected capability

Under ADR-0026, accept one inactive entry contract for a future bounded I3
program:

> Map the accepted I2 per-locus artifacts and generated communication plan
> across at least two operating-system processes using a real transport while
> preserving semantic authority, typed failure, source/Core provenance,
> redaction, and Mir abstract ordering, and close a finite C-distributed
> profile.

SYS-7 records only the conditions under which that future goal may start. It
does not activate I3, select or implement transport, define a public wire, or
change the accepted I2 runtime. The exact direct consumer is a new
owner-authorized bounded program that does not yet exist.

## Transport-neutral boundary and candidate limit

The accepted boundary is one replaceable transport adapter below the I2
internal carrier and above an unresolved external byte/stream representation.
Only two candidates are retained for future evaluation:

| Candidate | Admitted future evaluation scope | Current disposition |
|---|---|---|
| A | TLS-over-TCP framed reliable-stream adapter | **UNSELECTED** |
| B | QUIC reliable-stream adapter | **UNSELECTED**; datagram mode is not admitted or evaluated |

Neither candidate is preferred or adopted by SYS-7. The future program must
evaluate both against the same semantics/failure/order profile before OPEN-032
can be decided. No third candidate is opened here. No protocol version, codec,
frame schema, wire field, implementation library, certificate format, port,
connection strategy, or deployment topology is chosen.

The smallest viable alternative was to select one reliable-stream stack now.
It is rejected because no C-distributed execution evidence exists and a
premature selection would make an implementation convenience look like
semantic authority or a public compatibility commitment.

## Authority, identity, and public-wire separation

Transport, connection, stream, session, address, process identifier,
certificate, TLS peer identity, QUIC connection identifier, retry token, and
route metadata are not Mir authority. They cannot mint Core, locus ownership,
membership, epoch/incarnation, capability, witness, policy verdict, semantic
state, expected result, or observation permission.

The accepted I2 internal carrier remains typed and non-public. A future public
wire is a separate representation with an explicit meaning-preserving mapping
to that carrier. It cannot add or omit semantic fields, collapse request /
serve / result / receipt occurrences, or turn cryptographic peer
authentication into membership/capability authority. Version negotiation,
codec, compatibility, and wire redaction remain unresolved.

## Required failure and retry discipline

The future profile must fail closed and type at least these failure families:

- route/endpoint unavailable, wrong target, partition, and connection refusal;
- transport handshake or peer-admission failure without authority minting;
- partial write, split/truncated/malformed/oversized frame, and stream reset;
- disconnect before remote admission and disconnect after admission but before
  result/receipt, with ambiguous delivery made visible;
- reconnect or connection migration with a new non-authoritative session;
- duplicate request, result, receipt, or replayed stale occurrence;
- reordering across streams, connections, reconnects, and control/data paths;
- stale membership/epoch/incarnation, missing or revoked capability/witness,
  and policy-layer rejection;
- wrong owner/locus/artifact/operation or source/Core provenance mismatch;
- backpressure/queue capacity, timeout/lease expiry, and external effect
  failure; and
- visibility/redaction violation plus patch/save/cut interaction with in-flight
  traffic.

No failure may become a hang, silent drop, hidden retry, hidden authority
transfer, or hidden multi-owner transaction. Retry is an explicit future
policy or source-derived operation. A duplicate may reuse an already decided
result or fail with a typed duplicate/stale outcome only where the accepted
semantic operation permits it. This is not a global exactly-once guarantee.

## Ordering refinement requirement

The future adapter/runtime must show how concrete network occurrences refine
the accepted high-level edges, including:

```text
program order
request -> send -> receive/admit -> serve
serve -> result send -> result receive -> receipt/consume
publish -> observe
membership update -> dependent dispatch
capability/witness grant or revoke -> dependent use
verdict -> activation cut
save/cut quiescence -> later transition
```

Reliable-stream byte order is not sufficient for Mir semantic order,
especially across multiple streams or reconnects. Request and occurrence
identity remain source/Core-bound and distinct from queue/stream position.
Owner-local mutation remains serialized at the semantic owner. Old-session
messages after revocation, membership advance, patch activation, or cut must
be rejected or placed by an explicit admitted dependency; they cannot be
resurrected by reconnect. General fairness, arbitrary network scheduling, and
lock-free refinement remain deferred.

## Future C-distributed acceptance gates

The future program must make ordinary-source C-distributed paths for frozen
SCN-01, SCN-02, SCN-03, and SCN-06 direct acceptance gates:

- SCN-01: owner-directed write and observer-safe publish cross a process
  boundary; route/visibility failure is typed and non-mutating.
- SCN-02: owner-side RMW retains owner evaluation and two-request seriality;
  missing/revoked authority and ambiguous delivery do not double-mutate.
- SCN-03: admission verdict, epoch/incarnation, grant, and dependent dispatch
  retain order; reconnect/replay cannot replay authority.
- SCN-06: absent/partitioned route produces explicit `RouteUnavailable` within
  a bounded turn budget, never an indefinite block or silent retry.

Each gate needs a positive execution, representative transport/failure
falsifier, source/Core/artifact/carrier/network/runtime occurrence
correspondence, observer-safe diagnostics, exact evidence classification, and
independent review. Existing C-static/C-runtime evidence is a baseline, not
C-distributed evidence.

## Direct consumer, stop condition, and program consequence

```text
Direct consumer: a future owner-authorized Mirrorea I3 bounded program
Blocker reduced: I2 lacked a transport-neutral, authority-preserving,
  failure-complete and ordering-explicit network entry contract
Acceptance use: constrain future transport evaluation and C-distributed gates
  without selecting an implementation or public wire
```

The representative falsifier is a contract that grants authority from a
connection/certificate/session, hides duplicate/retry/failure, assumes stream
order is semantic order, selects a transport/wire, or activates I3.

Close SYS-7 when plan/05 and the accepted decision record contain this exact
boundary and independent review finds no major counterexample. Then close the
ADR-0026 SYS-0--SYS-7 program, leave no active roadmap or goal, keep I3
inactive and OPEN-032 unresolved, and require new owner direction for any
future I3 work.

## Non-effects

This proposal does not modify Rust, Lean, Core, Surface, runtime artifacts,
tests, model checking, OBL/THM status, or SCN expectations. It does not select
TLS/TCP or QUIC; admit QUIC datagrams; choose protocol versions, codec, wire,
library, certificate scheme, port, or deployment topology; implement sockets
or processes; freeze public API/ABI/wire; claim exactly-once, general ordering,
fairness, durability, production security, deployment, browser/View product,
or public completion; accept broad PHASE-I1; move theory T1; or activate I3.
