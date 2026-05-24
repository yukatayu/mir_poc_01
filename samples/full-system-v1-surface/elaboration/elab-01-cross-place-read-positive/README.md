# ELAB-01 Cross-Place Read Positive

Surface source authority: `main/src/cross-place-read-positive.mir`.

Expected evidence: a role-local read of `player[self].hp` elaborates to an explicit remote read request and generated observe edge against owner place `S`.
