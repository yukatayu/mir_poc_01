---
id: scenarios/SCN-07
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-01]
summary: gradient 観測。可視性水準の勾配、fields 絞り込み、private 遮断、redaction 単調性。
open_items: []
---

# SCN-07 — Gradient observation

**Purpose**: observation is a typed effect with levels; the same state exposes
different surfaces to different observers, monotonically redacted.

```mir
module Scn.Gradient
place World
record Player { position: Int64, hp: Int64, inventory_note: Text }

World {
  state player[p: Participant]: Player
    visible observer_safe fields { position }
}
```

Admin observers additionally hold admin_debug authority.

**Expected**: owner-local writes to position generate observer_safe publish
rows; hp and inventory_note generate none. The observer_safe export contains
position rows only; the admin_debug view may contain hp but **still no raw
witness/auth payloads**. Retention marked session_local in the profile.
Redaction order admin_full ≥ redacted_admin ≥ observer_safe ≥ public_summary
is machine-checked on the export.
**Negative variants**: (a) marking inventory_note visible observer_safe while
a policy labels it private-like ⇒ E-VIS-002; (b) a layer that widens
observation without ContractUpdate fails the overlay condition (theory/02);
(c) an export row lacking a subject occurrence ⇒ conformance failure (every
row derives from H).
**Refs**: THM-005, theory/07, CON-035.
