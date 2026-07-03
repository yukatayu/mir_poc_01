# ELAB-14 MissingCapability Singleton Failure Row Negative

Surface source authority:
`main/src/missing-capability-singleton-failure-row-negative.mir`.

Expected evidence: a generated remote write request with exactly
`MissingCapability` omitted remains `E-ROW-001` no-repair evidence.

This row is LAB evidence for the repair-widening boundary. It does not emit
`suggested_repair[]`, does not prove OBL-025, and does not change the final
diagnostic / repair ABI.
