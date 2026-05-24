# Surface Mir Alpha 01

## summary

`P-SURF-00B` rebaselined the next promoted line around Surface Mir as the
user-facing source layer. `P-SURF-01` adds the first parser floor,
`P-SURF-02` adds the indexed-state semantic checker floor, `P-SURF-03`
adds the Surface-to-Core elaboration evidence floor, and `P-SURF-04` adds the
generated communication evidence floor.

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

## new normative docs

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
- `samples/full-system-v1-surface/syntax/`
- `samples/full-system-v1-surface/indexed-state/`
- `samples/full-system-v1-surface/elaboration/`
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
- `ELAB-04`: underdeclared generated failure row is rejected.
- `ELAB-05`: generated Core IR carries source spans.
- `ELAB-06`: unsupported statements reject instead of being silently dropped.
- `ELAB-07`: write-side underdeclared generated failure row is rejected.
- `ELAB-08`: nested foreign place read generates an owner-directed read request.
- `ELAB-09`: visible write generates MessageEnvelope, publish, and observe rows.
- `ELAB-10`: underdeclared `VisibilityDenied` failure is rejected.

## next package

```text
P-SURF-05 role admission capability grant
```

Close condition:

- role claim / join / admission request / capability grant rows are explicit.
- role claim alone does not grant authority.
- stale membership and spoofed/admission-less grant attempts reject.
- parser, indexed-state checker, elaboration, and generated communication
  floors remain compatible with `SURF-01..09`, `IDX-01..05`, and
  `ELAB-01/02/03/04/05/06/07/08/09/10`.

## non-claims

- final public grammar / ABI / SDK is not fixed.
- Surface runtime/helper implementation beyond the parser, indexed-state
  checker, elaboration, and generated communication evidence floors is not
  present yet.
- `samples/full-system-v1-surface/syntax/` is parser evidence, not
  workflow-ready runtime evidence.
- `samples/full-system-v1-surface/indexed-state/` is semantic checker evidence,
  not workflow-ready runtime or elaboration evidence.
- `samples/full-system-v1-surface/elaboration/` is elaboration and generated
  communication evidence, not workflow-ready runtime or role-admission evidence.
- LLVM/native codegen, production WAN/federation, distributed durable save-load,
  and arbitrary native/WASM provider execution remain later.
