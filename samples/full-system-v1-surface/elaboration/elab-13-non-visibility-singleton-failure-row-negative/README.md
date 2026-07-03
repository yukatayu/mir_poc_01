# ELAB-13 Non-Visibility Singleton Failure Row Negative

Surface source authority:
`main/src/non-visibility-singleton-failure-row-negative.mir`.

Expected evidence: a generated remote write request with exactly one missing
non-visibility generated failure remains `E-ROW-001` no-repair evidence.

This row is LAB evidence for the repair-widening boundary. It does not emit
`suggested_repair[]`, does not prove OBL-025, and does not change the final
diagnostic / repair ABI.
