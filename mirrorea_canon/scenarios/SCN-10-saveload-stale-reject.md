---
id: scenarios/SCN-10
status: L0-frozen
maturity: draft
depends_on: [scenarios/SCN-04]
summary: save/load。consistent cut での保存、stale membership・失効 lease の復活拒否。
open_items: []
---

# SCN-10 — Save/load stale rejection

**Purpose**: saving is cut-backed; loading refuses to resurrect the dead
(THM-003).

Script: run SCN-03 world to a state with participants A, B and a live chain
(SCN-08). Take save S1 at a consistent cut. Then: A leaves; the avatar_session
lease expires; take save S2. Attempt three loads:

1. **Load S1 into a fresh session**: succeeds; restored config is well-formed;
   A is live *in the restored world's own frontier* (this is a past world, not
   the current one); provenance connected.
2. **Load S1 "into" the current session as if merging** (i.e. treating S1's
   membership as current): must be refused — stale membership epoch /
   incarnation resurrection (E-CUT-002).
3. **Doctored S2′** with the expired lease flipped live, or with a receive-occ
   whose send is outside the cut: refused — expired-lease resurrection /
   Consistent(cut) violation (E-CUT-001/002).

**Expected additionally**: the save/load timeline panel shows S1, S2, the
refusals with reasons; reacquire after load is a new occurrence (new epoch /
witness), never hidden repair. Distributed variants (multi-locus checkpoint
graphs with Z-cycles) are C-distributed extensions: a checkpoint on a zigzag
cycle is inadmissible for a recoverable global cut (OBL-014).
**Refs**: THM-003, theory/04, ADR-0004.
