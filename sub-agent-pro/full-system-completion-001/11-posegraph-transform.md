# 11 — PoseGraph / Transform

## Goal

Mir should own semantic transform relations in VR space.

Renderer/Unity/UE should not be semantic owner.

## Core types

```mir
record Vec3 { x: Float64, y: Float64, z: Float64 }
record Quat { x: Float64, y: Float64, z: Float64, w: Float64 }
record Transform {
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
  pose_version: UInt64,
}
```

## Anchor relation

```text
Anchor = TransformRef + fallback chain + observation snapshot rule
```

Example:

```text
Sparkle.anchor = Avatar[Alice].head
  fallback Avatar[Alice].shoulder
  fallback World.origin
```

## No split-frame invariant

Within one client session and one observation snapshot:

```text
If object O is anchored to transform T,
then O and T are rendered from the same pose_version.
```

This does not mean all clients see the same wall-clock frame.

It means each client does not render dependent objects from mismatched pose snapshots.

## Required runtime states

- PoseGraph
- TransformNode
- AnchorBinding
- ObservationSnapshot
- PoseVersion
- AnchorSwitch
- ReacquireRequired

## Samples

- avatar head transform
- anchored object
- sparkle fallback anchor
- no-split-frame positive
- split-frame negative
- save/load roundtrip
- stale-anchor after membership advance
- anchor-switch frontier negative
- stale-anchor reacquire required

## Devtools panels

- PoseGraph node list
- anchor edges
- pose version per observation snapshot
- no-split-frame checks
- stale/reacquire events

## Engine boundary

Renderer receives a pose snapshot.
Renderer does not decide world authority.
