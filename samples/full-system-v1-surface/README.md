# Full System V1 Surface Mir Samples

This root is the Surface Mir source-authority line under `specs/39..43` and
`plan/64..68`.

Current state after `P-SURF-99`: bounded Surface alpha audit closed. These
roots are runnable evidence, not final Surface runtime/transport or final
viewer/telemetry ABI.

- `syntax/` is actualized for `P-SURF-01` as a narrow parser evidence lane for
  canonical `S { ... }` place blocks, role-instance blocks, indexed state
  declarations, record literals, and expected parser rejections.
- `indexed-state/` is actualized for `P-SURF-02` as a narrow semantic checker
  evidence lane for S-owned Participant-indexed state, key-not-authority
  rejection, stale-key rejection, retained-savepoint compaction rejection, and
  nested-place ambient-authority rejection.
- `elaboration/` is actualized for `P-SURF-03` / `P-SURF-04` as a narrow
  Surface-to-Core elaboration and generated communication evidence lane for
  cross-locus read/write remote requests, MessageEnvelope rows, visible field
  publish/observe rows, generated edges, RHS indexed-read dependency rows,
  source spans, obligations, underdeclared generated failure-row rejection, and
  private/non-visible field rejection. `ELAB-11/12` are later G1 LAB
  dependency-gap rows, and `ELAB-17` is a later G1 LAB exact SCN-01
  `VisibilityDenied` negative row. These rows do not claim C-static
  conformance, runtime request serving, or runtime read materialization.
- `role-admission/` is actualized for `P-SURF-05` as a narrow report-level role
  admission evidence lane for role claims, join admission requests, accepted
  verdicts, capability grants, admission witnesses, missing-grant write
  rejection, stale membership rejection, and optional package/runtime hash
  metadata.
- `source-patch/` is actualized for `P-SURF-06` as a narrow source patch
  hot-plug evidence lane for parse/typecheck/elaborate/compatibility/admission,
  HotPlugRequest, HotPlugVerdict, Core IR diff, activation_cut, no-direct-eval,
  and rejection-without-mutation rows.
- `devtools/` is actualized for `P-SURF-08` as static observer-safe devtools
  diagnostics evidence for Surface source, generated Core IR,
  semantic-checker-backed indexed-state map, generated communication,
  role/admission, redacted patch lifecycle, and source-span panels.
- `world-core/`, `membership-chat/`, `sugoroku-world/`, `portal-worldlink/`,
  `two-shard-hard-boundary/`, and `gradient-observation/` are actualized for
  `P-SURF-07` as source-first operational evidence roots for
  `E2E-SURF-01..12`.
- `operational-matrix.json` records which alpha checks are required per
  operational row and keeps `.mir` files as source authority.

Commands:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture
cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture
cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture
```

Stop lines:

- no final public grammar / ABI / SDK.
- no runtime MessageEnvelope dispatch or final transport completion yet.
- no final source patch hot-plug ABI or distributed durable migration planner.
- no final Surface operational runtime / transport completion yet.
- no production identity provider / hardware attestation / WAN admission.
- no generated package artifact authority.
