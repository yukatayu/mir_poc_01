# Surface Mir Alpha 01

## summary

`P-SURF-00B` rebaselined Surface Mir as the
user-facing source layer. `P-SURF-01` adds the first parser floor,
`P-SURF-02` adds the indexed-state semantic checker floor, `P-SURF-03`
adds the Surface-to-Core elaboration evidence floor, `P-SURF-04` adds the
generated communication evidence floor, and `P-SURF-05` adds report-level role
admission / capability grant evidence. `P-SURF-06` adds source patch hot-plug
pipeline evidence. `P-SURF-07` adds source-first operational evidence roots.
`P-SURF-08` adds static observer-safe devtools diagnostics evidence.
`P-SURF-99` closes the bounded final validation / claim-non-claim audit.

The central syntax decision is:

```text
canonical place scope: S { ... }
rejected syntax:       S[ ... ]
```

`S[ ... ]` is not sugar. `[]` remains available for arrays, maps, role-instance
heads, and indexed state access.

## what is decided

- `.mir` source files are semantic source authority.
- `package.mir.json` remains alpha compatibility / generated package artifact.
- Surface Mir is user-facing; Core Mir is elaboration target.
- generated communication / publish / observe must appear in Core IR and
  devtools.
- `state player[p: Participant]: Player` is an S-owned Participant-indexed map.
- key is not authority.
- role claim is not authority; authority is a capability grant.
- capability refs are valid only with matching admission, principal, target,
  epoch, and incarnation lineage.
- source patch hot-plug is parse/typecheck/elaborate/admit/activation_cut, not
  eval.
- backend/provider surfaces do not own world semantics.

## LAB evidence / repository-memory docs

- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`

## repository memory

- `plan/64-surface-mir-placement-roadmap.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## current evidence

- `crates/mir-ast::surface_alpha`
- `crates/mir-ast/examples/surface_mir_alpha_parse.rs`
- `crates/mir-semantics::surface_indexed_state`
- `crates/mir-semantics/examples/surface_indexed_state_check.rs`
- `crates/mir-semantics::surface_to_core_elaboration`
- `crates/mir-semantics/examples/surface_to_core_elaborate.rs`
- `crates/mir-semantics::surface_role_admission`
- `crates/mir-semantics/examples/surface_role_admission_check.rs`
- `crates/mir-runtime::surface_source_patch_hotplug`
- `samples/full-system-v1-surface/syntax/`
- `samples/full-system-v1-surface/indexed-state/`
- `samples/full-system-v1-surface/elaboration/`
- `samples/full-system-v1-surface/role-admission/`
- `samples/full-system-v1-surface/source-patch/`
- `samples/full-system-v1-surface/devtools/`
- `samples/full-system-v1-surface/world-core/`
- `samples/full-system-v1-surface/membership-chat/`
- `samples/full-system-v1-surface/sugoroku-world/`
- `samples/full-system-v1-surface/portal-worldlink/`
- `samples/full-system-v1-surface/two-shard-hard-boundary/`
- `samples/full-system-v1-surface/gradient-observation/`
- `scripts/surface_mir_samples.py`

Actualized rows:

- `SURF-01`: `S { ... }` accepted.
- `SURF-02`: `S[ ... ]` rejected with `bracket_place_scope_not_supported`.
- `SURF-03`: record literal accepted.
- `SURF-04`: ambiguous brace construct rejected.
- `SURF-05`: role-instance block accepted.
- `IDX-01`: S-owned Participant-indexed state accepted.
- `IDX-02`: key write without authority rejected.
- `IDX-03`: stale key rejected.
- `IDX-04`: retained-savepoint compaction rejected.
- `IDX-05`: nested place block rejected as an ambient authority switch.
- `ELAB-01`: cross-locus indexed read generates a remote read request and observe edge.
- `ELAB-02`: nested foreign place write generates an owner-directed remote write request.
- `ELAB-03`: private/non-visible field auto communication is rejected.
- `ELAB-04`: underdeclared generated failure row is rejected; current LAB
  branch inventory keeps it no-repair because it mixes base remote-request
  failures with `VisibilityDenied` and lacks diagnostic ownership / branch
  association / ranking semantics.
- `ELAB-05`: generated Core IR carries source spans.
- `ELAB-06`: unsupported statements reject instead of being silently dropped.
- `ELAB-07`: write-side underdeclared generated failure row is rejected; current
  LAB gate review keeps it no-repair until set-insertion atomicity or bundle
  semantics are explicit, and the executable preflight records the future
  atomic set-insertion / whole-gap coverage tests without widening output.
- `ELAB-08`: nested foreign place read generates an owner-directed read request.
- `ELAB-09`: visible write generates MessageEnvelope, publish, and observe rows.
- `ELAB-10`: underdeclared `VisibilityDenied` failure is rejected.
- `ROLE-01`: BrowserClient join is accepted through admission, and the grant
  authorizes a World-owned indexed-state write.
- `ROLE-02`: role claim without grant cannot write server state.
- `ROLE-03`: stale membership message and post-stale write are rejected.
- `ROLE-04`: package/runtime hash binding is metadata, not safety proof.
- `PATCH-01`: visible-state source patch is accepted and emits an activation cut.
- `PATCH-02`: undeclared generated failure row is rejected without mutation.
- `PATCH-03`: self-grant of ServerAuthority is rejected without mutation.
- `PATCH-04`: patch lifecycle/devtools positive row is accepted.
- `DEV-01..02`: required Surface source, generated Core IR,
  semantic-checker-backed indexed-state map, generated communication,
  role/admission, redacted patch lifecycle, and source-span panels are visible
  in static diagnostics; private-field diagnostics do not expose raw private
  payloads.
- `E2E-SURF-01..12`: WorldCore, MembershipChat, Sugoroku, PortalWorldlink,
  TwoShardHardBoundary, and GradientObservation positive/negative source rows
  pass their required alpha checks.

## current audit state

```text
P-SURF-99 final surface alpha audit: closed
```

Closeout evidence:

- validation and compatibility anchors rerun across P-SURF-01..08.
- Surface alpha claim / non-claim wording audited.
- `.mir` files remain semantic source authority and generated reports remain
  evidence only.
- parser, indexed-state checker, elaboration, generated communication,
  role-admission, source-patch, source operational, and static devtools floors remain compatible
  with `SURF-01..09`, `IDX-01..05`, `ELAB-01..10`, `ROLE-01..04`,
  `PATCH-01..04`, `DEV-01..02`, and `E2E-SURF-01..12`.

## non-claims

- final public grammar / ABI / SDK is not fixed.
- Surface runtime/helper implementation beyond the parser, indexed-state
  checker, elaboration, generated communication, role-admission, source-patch,
  source operational, and static devtools diagnostics evidence floors is not
  present yet.
- P-SURF-08 is not final viewer/telemetry ABI or runtime devtools completion.
- `samples/full-system-v1-surface/syntax/` is parser evidence, not
  workflow-ready runtime evidence.
- `samples/full-system-v1-surface/indexed-state/` is semantic checker evidence,
  not workflow-ready runtime or elaboration evidence.
- `samples/full-system-v1-surface/elaboration/` is elaboration and generated
  communication evidence, not workflow-ready runtime or role-admission evidence.
- `samples/full-system-v1-surface/role-admission/` is report-level admission
  evidence, not production identity, hardware attestation, WAN admission, or
  runtime membership lifecycle evidence.
- `samples/full-system-v1-surface/source-patch/` is source patch pipeline
  evidence, not a final hot-plug ABI, distributed durable migration planner, or
  production patch registry.
- `samples/full-system-v1-surface/world-core/` and sibling operational roots
  are source operational evidence, not final runtime/transport or final catalog.
- LLVM/native codegen, production WAN/federation, distributed durable save-load,
  and arbitrary native/WASM provider execution remain later.
