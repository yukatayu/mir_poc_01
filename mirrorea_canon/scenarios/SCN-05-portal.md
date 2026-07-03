---
id: scenarios/SCN-05
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-01]
summary: portal。二つの place を跨ぐ移動と可視性。private link は VisibilityDenied。
open_items: []
---

# SCN-05 — Portal / world link

**Purpose**: "worlds" are relative (ADR-0001): a portal is ordinary
cross-locus semantics between two declared places, plus membership movement.

Setup: places WorldA, WorldB, shared admission; participant P admitted to
WorldA. WorldA declares `state portal_link[p: Participant]: LinkInfo visible
observer_safe fields { destination }` plus a private field `secret_key`.
Handler `travel` at P: requests leave-A/join-B via admission (two verdicts,
epochs bump independently per world), then writes spawn state at WorldB.

**Expected**: travel = admission choreography, not teleport magic: verdict_A
(leave) ≺ verdict_B (join) ≺ spawn write at B; P's WorldA entries tombstone;
grants for A do not apply at B (per-target lineage). Observers of WorldA see
destination via publish; `secret_key` never publishes.
**Negative variants**: (a) reading portal_link.secret_key cross-locus ⇒
VisibilityDenied (runtime) and E-ROW-002 if undeclared; (b) using WorldA
capref at WorldB ⇒ rejected (wrong-target lineage).
**Refs**: ADR-0001, theory/05, theory/07.
