---
id: meta/proposal-043
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-037, meta/proposal-042, adr/ADR-0034, adr/ADR-0039, plan/05-i3-entry-contract]
summary: owner instruction により ADR-0034 / Plan 250 の固定順を再開し、I3-3 を唯一の active milestone とする。
open_items: []
---

# PROPOSAL-043 — Mirrorea I3 program resume

Direct consumer: I3-3 actual network-failure, retry and ordering harness.

Blocker reduced: the owner pause no longer blocks the already-authorized
I3-3 frontier; the existing Plan 250 / plan/05 criteria remain the applicable
acceptance boundary.

Acceptance use: I3-3 may proceed against those existing criteria. This resume
record adds no acceptance and does not change their semantics.

Authority baseline: owner instruction at source cut `648425f6bd4304d003d36bc04d346ddf0e78c058`.

## Owner disposition

The owner explicitly resumes the entire original ADR-0034 / Plan 250 fixed
sequence through NEXT-0. The program remains bounded by ADR-0034 and its
preserved invariants. Only I3-3 is active now; I3-4 through NEXT-0 remain
inactive until their fixed predecessors close.

ADR-0039's accepted I3-2 cut remains unchanged historical acceptance. Its
owner-pause exception is satisfied by this explicit instruction; it is not
rewritten or treated as a new acceptance. The official I3 lifecycle remains
unentered, theory remains T1, and the broad I1 residual remains unchanged.

## Scope and non-effects

I3-3 may now implement and validate the direct-consumer failure, retry,
reconnect, ambiguous-delivery, and ordering harness required by Plan 250 and
plan/05. This proposal does not decide new network semantics, promote a
transport fact into Mir authority, add exactly-once behavior, or accept any
milestone. It does not alter the accepted QUIC private adapter, the deferred
TLS-over-TCP baseline, or the exclusion of QUIC datagrams.

The program remains subject to the general owner-pause exception: after any
accepted milestone, execution may again pause with no active semantic
milestone until an explicit resume. All existing ADR-0034 owner-reserved
stops, direct-consumer requirements, falsifiers, validation, and review gates
remain in force.
