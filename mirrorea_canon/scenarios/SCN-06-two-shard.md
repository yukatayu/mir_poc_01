---
id: scenarios/SCN-06
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-05]
summary: two-shard。硬い境界。route 不在は宣言された明示的失敗であり、透過の限界を可視化する。
open_items: []
---

# SCN-06 — Two shards, hard boundary

**Purpose**: placement transparency has declared limits: when no route exists,
the failure is explicit and typed — never a hang or silent drop.

Setup: places ShardEast, ShardWest with no declared route between them
(topology is part of Π, the projection context). Participant P at East holds
a reference to state owned by West via a declared observe grant, and a
handler attempts a cross-shard write.

**Expected (C-static)**: the write elaborates to a request whose generated
failure set includes RouteUnavailable; if the handler's fails row omits it ⇒
E-ROW-001 (the boundary is visible at compile time).
**Expected (C-runtime / C-distributed)**: with the route absent, the request
yields an explicit RouteUnavailable occurrence at the requester; store
unchanged; devtools route-trace shows the fail-closed row. When a route is
later patched in (SCN-09 machinery), the same source succeeds without edits —
the fails row already covered the world where routes fail.
**Negative variant**: an implementation that blocks indefinitely or retries
silently fails conformance (explicit failure required within the profile's
turn budget).
**Refs**: theory/03, ADR-0007, BND-005/006.
