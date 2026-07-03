---
id: scenarios/SCN-02
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-01]
summary: attack。ブログ由来の規範例。cross-locus read+write、failure row 包含、THM-001 の主試験。
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

**Expected (C-static)**: request edge (→ S, write player[target].hp);
dependency rows for player[target].hp and player[self].atk (cross-locus
reads); generated failure set exactly ⊆ declared fails; nested `S { }` is not
ambient authority (owner-directed, authorized from the actor locus).
**Expected (C-runtime)**: with a granted write capability, target hp 100→90;
without it, explicit MissingCapability failure occurrence, store unchanged.
**Negative variants**: (a) drop `MissingCapability` from fails ⇒ E-ROW-001;
(b) claim the nested block as local write in an implementation ⇒ fails
C-static (edge must exist); (c) attack after target leave ⇒ StaleMembership.
**Refs**: THM-001, THM-004, ADR-0005, mental-model/02.
