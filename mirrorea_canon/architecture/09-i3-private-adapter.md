---
id: arch/09-i3-private-adapter
status: L1-fixed
maturity: reviewed
depends_on: [arch/04-runtime-carriers, adr/ADR-0038, meta/proposal-041, adr/ADR-0041, spec/16-i3-owner-admission-budget]
summary: I3-1のbounded private adapter/encoding mapping。I3-2が直接consumeする。
open_items: []
---

# 09 — I3-1 private adapter mapping

Direct consumer: I3-2 actual two-or-more-process generated-artifact runtime.

Blocker reduced: the accepted I2 internal carrier lacked a checked private
bytes/stream encode, decode and admission mapping over the selected QUIC seam.

Acceptance use: I3-2 may transport generated edges without reconstructing or
inventing owner, authority, state, route, occurrence or other semantics.

ADR-0038 accepts this bounded, non-public mapping for direct consumption by
I3-2. Six exhaustive static carrier families provide twelve checked-Core-derived
edges: owner request/reply-receipt, designated input/receipt, relation
projection publication, and designated result delivery. `AbsoluteValueStream`
is rejected. Source/Core/artifact identity, authority, effect/failure,
visibility/redaction and lifecycle identity remain in exact receiver-retained
snapshots.

The provisional boundary is marker/version → bounded `u32` frame → strict JSON
object → complete-frame typed admission. Unknown or duplicate fields,
malformed/truncated/oversized frames and unsupported markers/versions fail
closed before semantic admission. The selected seam is bounded Linux x86_64
localhost QUIC reliable bidi-stream. Transport, connection, certificate and
session are correlation evidence, never authority; observer evidence is
reference-only and redacted.

This is not a public wire/codec/API, a general proof or coverage-guided fuzz
claim, retry/reconnect semantics, durability, production support, or official
I3 lifecycle entry.

## Subsequent bounded owner-outcome contract

ADR-0041 / spec/16 select a typed declared-owner-failure alternative on the
existing generated owner reply edge for explicitly opted-in source. It must
preserve exact retained bindings and reject unknown/malformed variants; it
cannot be represented by a success receipt or transport failure. The I3-1
accepted cut above remains historical evidence, not evidence for this new
producer or a public compatibility promise.
