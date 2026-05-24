# ELAB-02 Cross-Place Write Positive

Surface source authority: `main/src/cross-place-write-positive.mir`.

Expected evidence: a role-authored nested `S { ... }` write does not switch ambient authority; it elaborates to an owner-directed remote write request.
