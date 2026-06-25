# 09 — Source Patch Hot-Plug

## 1. Goal

Throw `.mir` source patch into a running system and safely modify the virtual space.

## 2. Pipeline

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

## 3. No direct eval

Do not eval source directly.

## 4. CLI target

```bash
mirrorea-alpha check-source patch.mir
mirrorea-alpha parse-source patch.mir --format json
mirrorea-alpha elaborate-source patch.mir --format json
mirrorea-alpha patch-source session#id patch.mir --format json
mirrorea-alpha export-core-ir patch.mir --format json
```

## 5. Compatibility

Patch must declare:

- provided surfaces.
- required capabilities.
- effect row.
- failure row.
- observation policy.
- retention policy.
- state additions / migrations.

## 6. Activation

Accepted patch produces activation cut.
Rejected patch must not mutate runtime.
Deferred patch must be visible in lifecycle.

## 7. Surface patch example

```mir
module Patch.AddDebugLamp

import Game.WorldCore

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

## 8. Runtime expectations

- state map added at S.
- entries initialized for active participants.
- visible field appears in devtools.
- no hidden authority added.

## 9. Negative examples

- patch writes private state without capability.
- patch introduces undeclared failure row.
- patch tries to alter already-finalized atomic_cut prefix.
- patch grants ServerAuthority to itself.

## 10. Devtools

Show:

- patch source span.
- Core IR diff.
- compatibility verdict.
- capability check.
- activation cut.
- state migration summary.
