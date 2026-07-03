# ELAB-16 StaleMembership Singleton Failure Row Negative

Surface source authority:
`main/src/stale-membership-singleton-failure-row-negative.mir`.

Expected evidence: a generated remote write request with exactly
`StaleMembership` omitted emits a LAB-only `E-ROW-001`
`add-to-fails-row` repair payload.

This row is LAB evidence for the singleton repair prototype. It does not prove
OBL-025, does not add repair ranking or multi-edit support, and does not change
the final diagnostic / repair ABI.
