---
id: meta/proposal-042
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-037, meta/proposal-041, adr/ADR-0034, adr/ADR-0038, plan/05-i3-entry-contract, arch/09-i3-private-adapter]
summary: I3-2のsource-first two-process private QUIC runtimeをbounded evidenceとして受理し、program executionをowner pauseする。
open_items: []
---

# PROPOSAL-042 — Mirrorea I3-2 multi-process runtime

Direct consumer: I3-3 actual network-failure, retry and ordering harness after
an explicit owner/user resume instruction.

Blocker reduced: accepted per-locus images and generated communication could
cross a real stream, but had not yet executed a remote owner operation in two
independent operating-system processes with exact source/Core/runtime
correspondence and bounded child cleanup.

Acceptance use: I3-3 may inject faults at a real process/transport seam without
reconstructing route, owner, authority, state, expected result or semantic
occurrence from deployment or transport configuration.

## Disposition

The bounded-program owner direction accepts this proposal at L1 for the finite
I3-2 profile at source/evidence cut
`19c5b386613d6adb1f0b934e6ced81acb327d245`. Execution of the still-authorized
ADR-0034 / Plan 250 program is owner-paused after this acceptance. No semantic
milestone is active; I3-3 is next in the fixed sequence and remains inactive
until explicit resume. This state is neither blocked nor stale, does not close
the program and does not enter or exit the official I3 lifecycle.

## Accepted bounded contract

One supervisor reads and checks ordinary Mir source once, admits the checked
project once and derives the two process images, retained start bindings,
deployment and generated owner-request/reply edges. It then starts two actual
`exec` children in the bounded Linux x86_64 localhost profile:

```text
process-a: ParticipantA, ViewerC
process-b: WorldAuthority, ParticipantB
```

Each child receives only its tainted encoded image plus a separately delivered,
one-shot trusted start/control record. A process starts only when those two
records agree. Children do not receive ordinary source or a global authority
object, do not reparse/recheck source and own distinct process-local stores.
Deployment maps logical loci to process endpoints; it does not supply a route,
operation, owner, authority, capability, witness, result or occurrence.

The selected private QUIC adapter owns the live mutually authenticated
connection and its single reliable bidirectional stream, checks the exact peer
leaf Subject Public Key Info (SPKI) reference and reciprocal run/image/cohort
preface, then alone reaches crate-private decoded carrier admission. Transport,
certificate, connection, session and preface are delivery evidence, never Mir
authority; receiver-side sealed M9/owner/pending-request validation remains
independent. QUIC datagrams are not enabled.

The positive path carries a source-generated owner request from process A to
the `WorldAuthority` owner runtime in process B, performs the owner-local serve
and write, returns the generated reply, and creates the requester-local receipt.
There is no cross-process store handle and no network receipt phase. Four child-
reported delivery records—request send/receive and reply send/receive—are
equality-joined to the generated source/Core/artifact/edge contracts and to
carrier, semantic-request, network and runtime occurrence references. These are
observer-safe references, not raw source, state, payload, credential,
capability or witness material.

Success requires both child processes to be naturally reaped with zero exit
status and no force kill. The process-execution lifecycle uses one absolute
post-preflight main-deadline-plus-reaper-allowance bound. Natural zero exits
observed after that deadline fail closed; undersized reaper allowance is
rejected before source read or child spawn. Forced cleanup never becomes clean
success. Synchronous source/build/admission/cohort/credential preflight is
outside this finite child-lifecycle deadline.

## Falsifiers and assurance

The accepted executable falsifiers cover image/control binding swap, reciprocal
preface mismatch, a CA-valid but exact-SPKI-wrong peer, bootstrap/setup/reaper
stall, `Completed` followed by nonzero exit or hang, asymmetric child terminal
reports, undersized reaper allowance and late supervisor observation of natural
zero exits. Failures retain both child terminal records and actual admission /
mutation totals where available; they do not use a single rejecting child to
claim whole-run nonmutation.

Assurance classification is `runtime-monitored` for this finite process,
transport, correspondence and lifecycle evidence. No new `lean-proved`,
`lean-stated` or `model-checked-bounded` result is asserted; theory/11 is
unchanged.

## Explicit non-claims

This acceptance does not complete the plan/05 failure matrix, retry,
reconnect, ambiguous-delivery or abstract-order refinement. A generic failure
after remote admission is not yet classified as nonmutation and must be treated
conservatively by I3-3. It does not accept SCN-01/02/03/06 as C-distributed,
relation/designated pressure slices, network devtools, finite I3 conformance,
durability, Browser/Host/provider integration, WAN/production support or a
public wire/API/ABI/platform. It does not claim exactly-once, real-time return
under host suspension, bounded synchronous preflight, arbitrary verifier
configuration or a general proof.
