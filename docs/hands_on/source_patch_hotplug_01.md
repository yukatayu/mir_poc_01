# Source Patch Hot-Plug 01

## purpose

This guide explains the Surface Mir source patch hot-plug boundary from
`specs/42-source-patch-hotplug-semantics.md`.

`P-SURF-06` actualizes a narrow alpha evidence lane. The commands below produce
reports for parse/typecheck/elaborate/compatibility/admission, HotPlugRequest,
HotPlugVerdict, Core IR diff, and activation_cut rows. They do not define a
final hot-plug ABI.

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

## CLI

```bash
mirrorea-alpha check-source patch.mir
mirrorea-alpha parse-source patch.mir --format json
mirrorea-alpha elaborate-source patch.mir --format json
mirrorea-alpha patch-source session#id patch.mir --format json
mirrorea-alpha export-core-ir patch.mir --format json
```

Representative sample commands:

```bash
python3 scripts/surface_mir_samples.py run PATCH-01 --format json
python3 scripts/surface_mir_samples.py run PATCH-02 --format json
python3 scripts/surface_mir_samples.py check-all --format json
cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture
cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture
```

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

- undeclared generated failure row (`PATCH-02`).
- self-grant of server authority (`PATCH-03`).
- private state write without capability remains a later widened row.
- attempt to alter an already finalized `atomic_cut` prefix.
- activation against a different membership frontier than the admitted one.
- redaction / retention weakening.

## non-claims

- no final public hot-plug ABI.
- no direct eval.
- no distributed activation ordering protocol.
- no durable migration completion.
