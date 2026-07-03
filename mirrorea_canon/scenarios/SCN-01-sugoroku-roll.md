---
id: scenarios/SCN-01
status: L0-frozen
maturity: draft
depends_on: [scenarios/readme]
summary: 双六の roll。非 owner からの普通の代入が owner 宛 request + publish に翻訳される最小例。
open_items: []
---

# SCN-01 — Sugoroku roll

**Purpose**: the smallest end-to-end shape of the axis: ordinary assignment at
a non-owner locus becomes an owner-directed request; a visible field publishes.

```mir
module Scn.Sugoroku
role BrowserClient
place World
record Player { position: Int64 }

World {
  state player[p: Participant]: Player
    init Player { position: 0 }
    visible observer_safe fields { position }
}

BrowserClient[self] {
  when roll(draw: Int64)
    fails MissingCapability, MissingWitness, RouteUnavailable,
          StaleMembership, VisibilityDenied {
    World { player[self].position = player[self].position + draw }
  }
}
```

**Expected (C-static)**: elaboration yields one request edge
(BrowserClient[self] → World, write player[self].position), one dependency row
(read of same field), one publish row (observer_safe, field position); spans
on all; obligations include cap-write(player).
**Expected (C-runtime)**: after admission (as in SCN-03) and `roll(3)`,
World store has position = 3; H contains request ≺ serve ≺ publish.
**Negative variant**: remove `VisibilityDenied` from fails ⇒ E-ROW-002 at the
assignment span (publish-generated failure undeclared).
**Refs**: THM-001, ADR-0002, ADR-0003.
