# Source Patch Hot-Plug 01

## purpose

This guide explains the planned Surface Mir source patch hot-plug boundary from
`specs/42-source-patch-hotplug-semantics.md`.

It is docs/spec rebaseline only. The CLI commands below are target commands for
later packages.

## patch pipeline

Source patch hot-plug is:

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

It is not direct eval.

## planned CLI

```bash
mirrorea-alpha check-source patch.mir
mirrorea-alpha parse-source patch.mir --format json
mirrorea-alpha elaborate-source patch.mir --format json
mirrorea-alpha patch-source session#id patch.mir --format json
mirrorea-alpha export-core-ir patch.mir --format json
```

Until implemented, these surfaces should return explicit unsupported /
not-yet-implemented diagnostics rather than silent success.

## example patch

```mir
module Patch.DebugLamp

import Surface.WorldCore

record DebugLamp {
  enabled: Bool
}

World {
  state lamp[p: Participant]: DebugLamp
    init DebugLamp { enabled: true }
    visible observer_safe fields { enabled }
}
```

Expected future runtime meaning:

- add an indexed state map owned by `World`.
- initialize entries for active participants.
- expose `enabled` through observer-safe devtools.
- do not add hidden authority.
- emit an activation cut on accepted patch.
- bind admission and activation to the checked membership / witness frontier;
  reject or defer if the frontier drifts before activation.

## required negative cases

- undeclared failure row.
- private state write without capability.
- attempt to alter an already finalized `atomic_cut` prefix.
- self-grant of server authority.
- activation against a different membership frontier than the admitted one.
- redaction / retention weakening.

## non-claims

- no final public hot-plug ABI.
- no direct eval.
- no distributed activation ordering protocol.
- no durable migration completion.
