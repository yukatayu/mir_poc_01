# 37 — PoseGraph Runtime Semantics

## role

This document fixes the Full System V1 runtime target for Transform / PoseGraph semantics.

It extends `specs/29` by moving from helper evidence toward session-runtime state, save/load relation, and devtools panels.

## decision level

- `L1`
  - Transform and PoseGraph state are Mir / Mirrorea-owned semantic state.
  - Renderer / Unity / Unreal / WASM / native providers are not semantic owners.
  - No-split-frame means same-client same-observation-snapshot coherence, not global simultaneity.
- `L2`
  - PoseGraph runtime carriers, devtools panels, and save/load hooks are Full System V1 implementation targets.

## runtime carriers

Required runtime-visible state:

```text
PoseGraph = {
  nodes,
  anchor_bindings,
  pose_snapshot_frontier,
  pose_versions,
  anchor_switch_log: AnchorSwitch[],
  reacquire_required,
  fallback_state
}
```

`anchor_switch_log` must retain the semantically required `AnchorSwitch` fields from `specs/29`: `from_anchor`, `to_anchor`, `reason`, `required_capability`, `membership_epoch`, `owner_epoch`, `sequence`, and `pose_snapshot_frontier`. If a later runtime uses another carrier, that carrier must be named and must preserve those fields before save/load or no-split-frame completion can be claimed.

Required row families:

- `TransformNode`.
- `PoseSnapshot`.
- `PoseVersion`.
- `Anchor`.
- `AnchorBinding`.
- `AnchorSwitch`.
- `FallbackReason`.
- `AnchorFreshness`.
- `ReacquireRequired`.

## no-split-frame runtime check

For one client session and one observation frame:

```text
NoSplitFrame(session, frame, anchored, target) :=
  anchored.snapshot_ref == target.snapshot_ref
  and anchored.pose_version == target.pose_version
```

Runtime must either accept the row with evidence or emit a typed rejection / violation row. A split-frame mismatch must not be stored as accepted stable state.

## fallback and reacquire

Fallback extends access-path availability. It does not extend object lifetime or ownership.

Runtime fallback requires:

- declared same-lineage evidence.
- fallback reason.
- monotone degradation.
- visibility witness or explicit loss reason.
- freshness / membership epoch check.
- explicit reacquire when returning to the original anchor.

Stale anchor resurrection and hidden repair are forbidden.

## save/load relation

PoseGraph state must enter the `SaveObject` carrier when runtime-visible state crosses save/load boundaries.

Load admissibility must reject or force explicit reacquire for:

- stale anchor witness.
- stale fallback position.
- incoherent anchor component snapshot.
- membership epoch advance invalidating anchor evidence.
- owner epoch / sequence mismatch.

## devtools panels

Full System V1 PoseGraph devtools must expose:

- PoseGraph node list.
- anchor edges.
- pose snapshot timeline.
- pose version per observation snapshot.
- no-split-frame accepted / violation rows.
- fallback degradation.
- stale/reacquire events.
- observer-safe redacted transform summary.

Admin/debug richer views remain authority-gated and audited.

## stop line

- Do not claim VRM / VRChat / Unity compatibility.
- Do not claim renderer-owned world semantics.
- Do not claim global simultaneous coordinates.
- Do not claim distributed durable pose save/load before R3/R4 evidence exists.
- Do not promote current helper-only rows to runtime completion.
