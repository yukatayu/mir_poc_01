# Surface Mir Elaboration Samples

This directory contains P-SURF-03 source-first samples for the narrow Surface-to-Core elaboration floor.

Current executable rows:

- `ELAB-01`: cross-locus indexed read generates an explicit remote read request and observe edge.
- `ELAB-02`: nested foreign place write generates an owner-directed remote write request.
- `ELAB-04`: generated remote request is rejected when the surrounding `when` failure row is underdeclared.
- `ELAB-05`: generated Core IR carries source spans for transitions, requests, and generated edges.
- `ELAB-06`: unsupported Surface statements are rejected instead of silently dropped.
- `ELAB-07`: generated write requests reject underdeclared failure rows.
- `ELAB-08`: nested place read blocks generate owner-directed read request evidence.

These rows do not claim final public grammar, runtime execution, role-admission capability grants, or auto-communication completion.
