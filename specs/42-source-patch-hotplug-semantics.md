# 42 — Source Patch Hot-Plug Semantics

## role

This document fixes the Surface Mir source patch hot-plug pipeline.

A source patch is not direct eval. It must pass parse, typecheck, elaboration,
compatibility/admission, and activation-cut checks before mutating a runtime
session.

## decision level

- `L1`
  - source patch hot-plug preserves directed acyclic graph discipline and
    explicit activation cuts.
  - direct eval of `.mir` source into a live runtime is disallowed.
  - rejected patches do not mutate active runtime state.
  - patch authority is a capability/admission decision, not a role claim.
- `L2`
  - alpha patch CLI command family.
  - compatibility carrier fields.
  - source/Core/devtools diff shape.

## pipeline

```text
patch.mir
  -> parse
  -> typecheck
  -> elaborate to Core Mir
  -> compatibility check
  -> capability/admission check
  -> HotPlugRequest
  -> HotPlugVerdict
  -> activation_cut
  -> runtime mutation
  -> devtools trace
```

Rejected patch:

```text
parse/typecheck/elaborate/admit failure
  -> HotPlugVerdict(rejected)
  -> no active runtime mutation
  -> devtools lifecycle row
```

Deferred patch:

```text
HotPlugVerdict(deferred)
  -> no active runtime mutation until later activation
  -> visible lifecycle row
```

## no direct eval

The runtime must not execute new source text as an untyped script. Every patch
must enter through the source pipeline above.

This is required even for development-only patches.

## alpha CLI

```bash
mirrorea-alpha check-source patch.mir
mirrorea-alpha parse-source patch.mir --format json
mirrorea-alpha elaborate-source patch.mir --format json
mirrorea-alpha patch-source session#id patch.mir --format json
mirrorea-alpha export-core-ir patch.mir --format json
```

P-SURF-06 actualizes these commands as alpha report surfaces. They must not be
read as final hot-plug ABI.

## compatibility carrier

Patch compatibility must declare:

```text
provided_surfaces
required_capabilities
effect_row
failure_row
observation_policy
redaction_policy
retention_policy
state_additions
state_migrations
save_load_interaction
rollback_replay_cut_policy
checked_membership_epoch
checked_member_incarnations
required_membership_witness_refs
required_capability_witness_refs
```

Generated communication / publish / observe rows introduced by a patch must
still satisfy the failure/capability/visibility checks in
`specs/39-surface-mir-placement-elaboration.md`.

## source patch example

```mir
module Patch.AddDebugLamp

import Surface.WorldCore

role Participant
place S

record DebugLamp {
  enabled: Bool
}

S {
  state lamp[p: Participant]: DebugLamp
    init DebugLamp { enabled: true }
    visible observer_safe fields { enabled }
}
```

Runtime expectations:

- state map is added at `S`.
- entries are initialized for active participants.
- visible field appears in observer-safe devtools.
- no hidden authority is added.
- patch lifecycle includes activation cut.
- activation is bound to the checked membership / witness frontier used during
  admission.

## negative patch examples

Reject:

- patch writes private state without declared capability.
- patch introduces undeclared failure row.
- patch tries to alter already-finalized `atomic_cut` prefix.
- patch grants `ServerAuthority` to itself.
- patch imports unresolved or incompatible source.
- patch silently weakens redaction / retention.

## activation cut

Accepted patch activation emits an activation cut. If activation is included in
a save/load cut, the request, verdict, membership/capability frontier, and
package/source version information must also be present.

Patch admission and activation must carry the checked membership epoch, member
incarnations, and required witness refs used to validate state additions and
migrations. If membership or witness frontier drifts between `admit` and
`activation_cut`, the patch is rejected or deferred; it is not silently activated
against a different active-participant set.

Patch activation is not distributed durable migration and is not final product
hot-plug ABI completion.

## devtools requirements

Devtools must show:

- patch source span.
- parsed / typed source status.
- Core IR diff.
- compatibility verdict.
- capability checks.
- activation cut.
- state migration summary.
- generated communication / publish / observe rows.
- rejected/deferred patch lifecycle rows.

Observer-safe views must redact private capability, witness, and auth payloads.

## required alpha sample rows

- `PATCH-01`: source patch adds visible state.
- `PATCH-02`: patch undeclared failure rejected.
- `PATCH-03`: patch self-grants server authority rejected.
- `PATCH-04`: patch lifecycle devtools export accepted.

## non-claims

This document does not claim:

- final public hot-plug ABI.
- direct source eval.
- distributed activation ordering protocol.
- durable migration completion.
- arbitrary native / WASM execution through patches.
