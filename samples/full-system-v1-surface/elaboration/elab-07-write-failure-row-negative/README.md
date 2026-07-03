# ELAB-07 Write Failure Row Negative

Surface source authority: `main/src/write-failure-row-negative.mir`.

Expected evidence: generated remote write requests are rejected when the
surrounding `when` failure row is underdeclared. The LAB diagnostic detail now
carries one non-final `E-ROW-001` `set_insertion` repair payload for this exact
row: insert `MissingWitness`, `RouteUnavailable`, and `StaleMembership` into
the existing `when_fails_row` while preserving the non-goal that no capability,
witness, route, membership, or runtime success is supplied.
