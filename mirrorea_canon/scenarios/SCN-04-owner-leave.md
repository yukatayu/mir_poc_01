---
id: scenarios/SCN-04
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-03]
summary: 退出と incarnation 引退。stale key 拒否、tombstone、savepoint による compaction 阻止。
open_items: []
---

# SCN-04 — Owner leave / stale key

**Purpose**: leave retires an incarnation; indexed entries tombstone rather
than vanish; stale access is an explicit failure; compaction respects
retained references.

Setup: SCN-03 world with participants A, B admitted. Script: A leaves
(epoch+1, incarnation retired, player[A] tombstoned); B calls
`attack(A)`-style write to player[A].

**Expected**: the request is rejected with StaleMembership (explicit
occurrence; store unchanged); devtools shows the tombstoned key and the
rejection row. A savepoint taken before A left blocks compaction of
player[A] (retained-savepoint blocker); after the savepoint is released and
no witness/in-flight reference remains, compaction may drop the entry.
A rejoin by A creates a **new** incarnation; old grants do not revive
(write with pre-leave capref ⇒ rejected).
**Negative variant**: an implementation that silently reinitializes
player[A] on rejoin fails (hidden repair; must be a fresh init occurrence
under the new incarnation).
**Refs**: theory/05 lifecycle, THM-003 lemmas OBL-011..013, E-IDX-003.
