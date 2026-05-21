# 14 — Operational Samples

## Canonical sample chain

```text
WorldCore
  -> MembershipChat
  -> SugorokuWorld
  -> PortalWorldLink
  -> TwoShardHardBoundary
  -> TwoShardGradientObservation
```

## Add new source-first variants

For each operational package, add textual Mir source beside package manifest.

```text
samples/full-system-v1/
  world-core/
    src/world-core.mir
    package.mir.json
  membership-chat/
    src/membership-chat.mir
    package.mir.json
  sugoroku-world/
    src/sugoroku-world.mir
    package.mir.json
  avatar-pose/
    src/avatar-pose.mir
    package.mir.json
  portal-worldlink/
    src/portal-worldlink.mir
    package.mir.json
  two-shard-hard-boundary/
    src/two-shard-hard-boundary.mir
    package.mir.json
```

## OPS sample definitions

### OPS-FS-01 WorldCore

Must show:

- world declaration
- membership registry
- observation policy
- Place definitions
- check/run/devtools

### OPS-FS-02 MembershipChat

Must show:

- import WorldCore
- join/leave
- ChatText implemented as Mir-owned transform where possible
- stale membership reject
- rate-limit failure row

### OPS-FS-03 SugorokuWorld

Must show:

- import MembershipChat
- roll
- publish
- witness
- handoff
- stale membership reject
- save/load/quiescent-save

### OPS-FS-04 AvatarPose

Must show:

- Transform
- PoseVersion
- head/shoulder
- sparkle anchor
- fallback
- no-split-frame

### OPS-FS-05 PortalWorldLink

Must show:

- portal resolve
- source handoff
- admission witness
- destination admit
- fallback if unavailable

### OPS-FS-06 TwoShardHardBoundary

Must show:

- owner shard
- handoff offer/prepare/commit
- old-owner write reject
- missing witness reject
- stale config reject

### OPS-FS-07 GradientObservation

Must show:

- authority hard boundary
- observation overlap
- write reject in observer-only region
- stale view drop

## Required negative samples

- undeclared effect
- undeclared failure
- missing capability
- stale membership
- missing witness
- split-frame pose violation
- array out of bounds
- record missing field
- old shard owner write
- portal admission denied

## Sample commands

Every sample needs:

```bash
mirrorea-alpha check <sample> --format json
mirrorea-alpha run-local <sample> --format json
mirrorea-alpha export-devtools <session> --out <dir> --format json
mirrorea-alpha view <dir> --check --format json
```

If relevant:

```bash
mirrorea-alpha attach ...
mirrorea-alpha save ...
mirrorea-alpha quiescent-save ...
mirrorea-alpha transport --mode local|docker ...
mirrorea-alpha build-native-bundle ...
```
