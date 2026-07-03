# Surface Mir Elaboration Samples

This directory contains P-SURF-03 / P-SURF-04 source-first samples for the narrow Surface-to-Core elaboration floor and the generated communication floor.

Current executable rows:

- `ELAB-01`: cross-locus indexed read generates an explicit remote read request and observe edge.
- `ELAB-02`: nested foreign place write generates an owner-directed remote write request.
- `ELAB-03`: private field auto-publish / observe is rejected.
- `ELAB-04`: generated remote request is rejected when the surrounding `when` failure row is underdeclared.
- `ELAB-05`: generated Core IR carries source spans for transitions, requests, and generated edges.
- `ELAB-06`: unsupported Surface statements are rejected instead of silently dropped.
- `ELAB-07`: generated write requests reject underdeclared failure rows.
- `ELAB-08`: nested place read blocks generate owner-directed read request evidence.
- `ELAB-09`: visible field write generates MessageEnvelope, publish, and observe rows.
- `ELAB-10`: visible communication rejects an underdeclared `VisibilityDenied` failure row.
- `ELAB-11`: SCN-01-shaped visible same-field assignment records an RHS dependency row without claiming runtime dispatch.
- `ELAB-12`: SCN-02-shaped attack assignment records target/self RHS dependency rows without freezing read materialization.
- `ELAB-13`: non-visibility singleton `MissingWitness` omission carries LAB-only `E-ROW-001` repair evidence.
- `ELAB-14`: non-visibility singleton `MissingCapability` omission carries LAB-only `E-ROW-001` repair evidence.
- `ELAB-15`: non-visibility singleton `RouteUnavailable` omission carries LAB-only `E-ROW-001` repair evidence.
- `ELAB-16`: non-visibility singleton `StaleMembership` omission carries LAB-only `E-ROW-001` repair evidence.

These rows do not claim final public grammar, runtime MessageEnvelope dispatch, role-admission capability grants, source patch activation, or TypeMismatch typechecker discharge.

G1 LAB-only E-ROW diagnostic detail evidence is present for `ELAB-04`,
`ELAB-07`, `ELAB-10`, and `ELAB-13..16` as non-final
`lab_diagnostic_details`. It preserves legacy
`generated_failure_not_declared` output, now includes request / failure-row
context, emits `E-ROW-002` / `VisibilityDenied`-only `suggested_repair[]`
evidence for `ELAB-10`, emits `E-ROW-001` non-visibility singleton
`suggested_repair[]` evidence for `ELAB-13..16` under the `plan/93` gate, and
keeps `ELAB-04/07` as mixed / multi-missing no-repair fences. This does not
freeze a diagnostic / repair ABI or claim OBL-025.
