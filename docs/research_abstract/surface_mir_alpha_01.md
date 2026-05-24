# Surface Mir Alpha 01

## summary

`P-SURF-00B` rebaselines the next promoted line around Surface Mir as the
user-facing source layer.

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

## next package

```text
P-SURF-01 surface brace parser
```

Close condition:

- `S { ... }` parses.
- `S[ ... ]` rejects with a clear diagnostic.
- record literal and place block ambiguity is resolved through namespace and
  context.
- role-instance block syntax is represented without treating arbitrary indexed
  expressions as place scopes.

## non-claims

- final public grammar / ABI / SDK is not fixed.
- Surface parser/runtime/helper implementation is not present yet.
- `samples/full-system-v1-surface/` is planned, not workflow-ready.
- LLVM/native codegen, production WAN/federation, distributed durable save-load,
  and arbitrary native/WASM provider execution remain later.
