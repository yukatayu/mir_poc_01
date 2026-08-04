---
id: meta/proposal-019
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-018, adr/ADR-0015, root/north-star]
summary: owner-approved Mir v0/I1+ program の M1 として、短い Design Constitution と既知の Canon alignment を採択する記録。
open_items: []
---

# PROPOSAL-019 — Design Constitution adoption

## Owner disposition

The owner direction recorded in PROPOSAL-018 and delegated by ADR-0015 is
applied for M1: adopt `root/design-constitution` as the concise cross-cutting
decision filter for Mir Theory v0 and deterministic I1+. It is subordinate to
the North Star and ADRs; it does not create a parallel specification or a
public contract.

The owner direction also requires the existing SCN-02 elaboration explanation
to distinguish requester authority origin from S-side evaluation, and requires
semantic fallback to remain distinct from consumer-local presentation fallback.
These narrow alignment effects are recorded by ADR-0016.

## Scope and non-effects

This proposal authorizes only the M1 Constitution, its supporting Canon
alignment, the M1 report/evidence, and the ordinary proposal/ADR/changelog/index
procedure. It does not move T0/G0/T1, discharge or change an OBL status, claim
SCN conformance, implement runtime behavior, freeze a final grammar/API/ABI or
wire format, authorize transport/deployment, or make a new permanent v0
non-goal.

Detailed calculus/carrier choices remain M3--M5; Surface syntax remains M6;
implementation and conformance remain M7--M10.
