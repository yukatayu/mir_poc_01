---
id: scenarios/SCN-09
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-03]
summary: patch の受理(DebugLamp)と拒否(self-grant)。拒否は無変異、activation は frontier 束縛。
open_items: []
---

# SCN-09 — Patch accept / reject

**Purpose**: hot-plug as capstone: an accepted patch adds visible state under
an activation cut bound to the admission frontier; a malicious patch is
rejected with zero mutation (THM-006).

Positive patch:

```mir
module Patch.DebugLamp
import Scn.Join
place World
record DebugLamp { enabled: Bool }

World {
  state lamp[p: Participant]: DebugLamp
    init DebugLamp { enabled: true }
    visible observer_safe fields { enabled }
}
```

**Expected**: pipeline rows parse→check→elaborate→compat→admission→
HotPlugRequest→Verdict(accepted)→activation_cut; entries initialized for the
active participants **of the checked frontier**; lamp.enabled appears in
observer_safe devtools; no new authority appears. If membership changes
between admit and activation ⇒ verdict flips to deferred (no silent
activation).

Negative patch: same module but adding
`grant ServerAuthority(self)` inside World.
**Expected**: Verdict(rejected) with E-PATCH-003 (self-grant); configuration
unchanged except lifecycle rows (machine-checkable: store/membership/grant
hashes equal before and after).
**Second negative**: patch writing player.hp without declared capability ⇒
E-PATCH-002 / rejected.
**Refs**: THM-006, ADR-0006, theory/08.
