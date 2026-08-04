---
id: scenarios/SCN-02
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-01]
summary: attack。ブログ由来の規範例。authority origin と S-owner RMW、failure row 包含、THM-001 の主試験。
open_items: []
---

# SCN-02 — Attack

**Purpose**: the project's canonical example (origin: the founding blog):
another participant's state is written with ordinary code; a cross-locus read
feeds it.

```mir
module Scn.Attack
role BrowserClient
place S
record Player { hp: Int64, atk: Int64 }

S { state player[p: Participant]: Player
      init Player { hp: 100, atk: 10 } }

BrowserClient[self] {
  when attack(target: Participant)
    fails StaleMembership, MissingCapability, MissingWitness, RouteUnavailable {
    S { player[target].hp = player[target].hp - player[self].atk }
  }
}
```

**Expected (C-static)**: one owner-directed request edge (→ S) whose source
operation is an S-evaluated RMW; S-local dependency rows for
player[target].hp and player[self].atk; generated failure set exactly ⊆
declared fails; and source spans. `BrowserClient[self]` is the authority
origin, while S is the evaluation locus. Nested `S { }` is not ambient
authority and does not return S-private values to the actor.
**Expected (C-runtime)**: with a granted write capability, one attack changes
target hp 100→90. Two accepted attacks queued at S yield 100→90→80 because
each RHS is evaluated at owner service. Without the capability, an explicit
MissingCapability failure occurrence leaves the store unchanged.
**Negative variants**: (a) drop `MissingCapability` from fails ⇒ E-ROW-001;
(b) read `player[self].atk` at the requester or blind-write a requester
precomputed hp ⇒ fails C-static; (c) treat same-owner RMW as cross-owner
atomicity ⇒ reject/defer rather than infer a transaction; (d) attack after
target leave ⇒ StaleMembership.
**Refs**: THM-001, THM-004, ADR-0003, ADR-0005, ADR-0016, ADR-0018,
theory/13, mental-model/02.
