---
id: scenarios/SCN-03
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-01]
summary: 後から join。admission verdict、epoch 更新、grant 前の書き拒否、可視履歴は過去。
open_items: []
---

# SCN-03 — Late join

**Purpose**: role claim ≠ authority; admission mints grants; a late joiner
sees visible history as past, not as replayed authority.

```mir
module Scn.Join
role BrowserClient { supports renderer.pose_v1 }
place World
place WorldAdmission
record Player { hp: Int64 }

World { state player[p: Participant]: Player init Player { hp: 1 } }

WorldAdmission {
  when admit(req: JoinRequest) {
    require valid_principal(req.principal)
    grant Member(World, req.principal)
    grant WritePlayer(World, req.principal)
    publish participant_joined(req.principal) produces witness admission_w
  }
}

BrowserClient[self] {
  when start fails MissingCapability {
    join World as BrowserClient via WorldAdmission
    World { player[self].hp = 5 }
  }
}
```

**Expected**: admitreq ≺ verdict(accepted, epoch e+1, incarnation i, grants) ≺
write-request served. The write is valid only because its capref lineage
matches the verdict. A second joiner later observes position history via
publish rows marked as past occurrences (no authority replay).
**Negative variants**: (a) write before verdict ordering ⇒ E-AUTH-002 /
runtime MissingCapability; (b) role claim `ServerRole` without grant cannot
write (anti-spoofing, theory/05); (c) replayed capref from another principal ⇒
rejected, severed lineage.
**Refs**: THM-004, ADR-0005, CON-005..009.
