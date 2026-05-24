# Surface Mir Alpha 01

## summary

`P-SURF-00B` rebaselined the next promoted line around Surface Mir as the
user-facing source layer. `P-SURF-01` adds the first parser floor.

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

## current parser evidence

- `crates/mir-ast::surface_alpha`
- `crates/mir-ast/examples/surface_mir_alpha_parse.rs`
- `samples/full-system-v1-surface/syntax/`
- `scripts/surface_mir_samples.py`

Actualized rows:

- `SURF-01`: `S { ... }` accepted.
- `SURF-02`: `S[ ... ]` rejected with `bracket_place_scope_not_supported`.
- `SURF-03`: record literal accepted.
- `SURF-04`: ambiguous brace construct rejected.
- `SURF-05`: role-instance block accepted.

## next package

```text
P-SURF-02 indexed-state semantics
```

Close condition:

- owner/keyspace/value type semantics for `state player[p: Participant]:
  Player`.
- key access does not grant authority.
- stale/leave access rejection row exists.
- parser floor remains compatible with `SURF-01..09`.

## non-claims

- final public grammar / ABI / SDK is not fixed.
- Surface runtime/helper implementation beyond the parser floor is not present
  yet.
- `samples/full-system-v1-surface/syntax/` is parser evidence, not
  workflow-ready runtime evidence.
- LLVM/native codegen, production WAN/federation, distributed durable save-load,
  and arbitrary native/WASM provider execution remain later.
