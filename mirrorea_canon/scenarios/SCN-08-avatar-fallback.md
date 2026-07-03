---
id: scenarios/SCN-08
status: L0-frozen
maturity: draft
depends_on: [scenarios/readme]
summary: avatar fallback。chain 宣言、lease 失効による単調劣化、write-after-expiry、明示的再取得。
open_items: []
---

# SCN-08 — Avatar fallback

**Purpose**: the graceful-degradation law: a reference to a possibly-shorter-
lived pose degrades along a declared chain and recovers only by explicit
reacquire (THM-002 in the flesh).

```mir
module Scn.AvatarFallback
place World
record Pose { x: Float64, y: Float64 }

World {
  state live_pose[p: Participant]: Pose visible observer_safe fields { x, y }
  state room_anchor: Pose init Pose { x: 0.0, y: 0.0 }
}

chain view_pose: Pose =
  live on live_pose cap Read lease avatar_session
  > anchor on room_anchor cap Read lease room_epoch @ lineage
  > frozen on default_pose cap Read lease static @ lineage
```

**Expected**: while avatar_session is live, reads through view_pose resolve
`live`. On session lease expiry: chain position advances to `anchor`
(non-admissible subreason lease-expired in audit; no dedicated occurrence);
further reads never return to `live` on this lineage. If a write-capable
option expired and no later write-capable option exists ⇒ request-level
Reject. Explicit reacquire (new session witness, new epoch) starts a new
lineage that may again resolve `live`. `try`/rollback restores state but not
the chain position.
**Negative variants**: (a) chain edge without `@ lineage` ⇒ E-DECL-001;
(b) later option requiring Write when predecessor had Read ⇒ E-LIN-003;
(c) an implementation that re-promotes after rollback fails C-runtime.
**Refs**: THM-002, ADR-0004, theory/06, mental-model/03 gap 1.
