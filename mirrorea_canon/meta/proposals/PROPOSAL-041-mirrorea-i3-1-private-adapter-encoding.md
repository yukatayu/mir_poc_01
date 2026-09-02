---
id: meta/proposal-041
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-037, adr/ADR-0034, meta/proposal-040, adr/ADR-0037, plan/05-i3-entry-contract]
summary: I3-1のprivate transport-neutral adapter/encodingとQUIC reliable-bidi seamをbounded evidenceとして受理し、I3-2へ進める。
open_items: []
---

# PROPOSAL-041 — Mirrorea I3-1 private adapter and encoding

Direct consumer: I3-2 actual two-or-more-process generated-artifact runtime.

Blocker reduced: the accepted I2 internal carrier lacked a checked private
bytes/stream encode, decode and admission mapping over the selected QUIC seam.

Acceptance use: I3-2 may transport generated edges without reconstructing or
inventing owner, authority, state, route, occurrence or other semantics.

## Disposition

Owner direction in the bounded program accepts this proposal at the reviewed
L1 level. It is a finite implementation/evidence boundary, not a public
compatibility commitment and not official I3 lifecycle entry.

## Accepted bounded contract

I3-1 closes six exhaustive static carrier families and their twelve generated
edges: owner request/reply-receipt, designated input/receipt, relation
projection publication, and designated result delivery. `AbsoluteValueStream`
is rejected as an unselected semantic carrier. Source-owner facts, checked
source/Core/artifact identities, authority, effect/failure, visibility and
redaction are mapped exhaustively into static snapshots; no handwritten route
or source-free authority is admitted.

The private provisional codec uses an explicit marker/version, bounded `u32`
frame length and strict JSON objects. Admission is complete-frame-only and
fail-closed for limits, unknown or duplicate fields, malformed/truncated
frames, unknown markers/versions, and oversized input. The receiver retains an
exact snapshot for admission; it does not reconstruct semantics from a hash
or transport identity. The selected real seam is localhost QUIC reliable
bidi-stream on the bounded Linux x86_64 profile.

Transport, connection, certificate and session are evidence/correlation only;
they cannot mint authority, ownership, membership or capability. Observer
evidence is reference-only and redacted. The finite property/mutation tests
are not coverage-guided fuzz evidence and do not establish a general proof.
Accepted finite execution/codec/admission evidence is `runtime-monitored`; no
new Lean or bounded-model result is claimed. General proof, coverage-guided
fuzzing, platform/live authority, retry/reconnect and I3-2 runtime are
`intentionally-deferred`, and theory/11 is unchanged.

## Scope and consumer

The direct consumer is I3-2's two-or-more-process generated-artifact runtime.
TLS-over-TCP remains a deferred replacement baseline and QUIC datagrams remain
excluded. Runtime retry/reconnect, complete failure matrix, durability,
production security, supported-platform matrix, public wire/API/ABI/package
format, browser/provider integration and official I3 entry/exit remain open.
