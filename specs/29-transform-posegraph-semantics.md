# 29 — Transform / PoseGraph Semantics

## role

この文書は、VR / shared-space state を renderer-owned hidden state ではなく、Mir / Mirrorea-owned semantic state として扱うための Transform / PoseGraph boundary を置く。

Current avatar preview / fallback rows are useful evidence, but they do not yet define full PoseGraph semantics. This document creates the docs-first normative target for that next line.

## decision level

- `L1`
  - avatar head transform, object anchor, pose version, UI state, world state, sync policy, fallback, capability, observation, and save/load are Mir / Mirrorea-owned semantic state.
  - Unity / Unreal Engine / renderer / WASM / native library are providers or backends, not semantics owners.
  - no-split-frame is same-client same-observation-snapshot coherence, not global simultaneous coordinates.
- `L2`
  - `Transform`, `PoseSnapshot`, `AnchorBinding`, `AnchorSwitch`, and pose devtools panels are proposed next-line carriers.
  - PoseGraph samples are planned as a separate semantics line before any promotion into the current product operational suite.

## core vocabulary

Required semantic carriers:

```text
Vec3 = { x: Float64, y: Float64, z: Float64 }
Quat = { x: Float64, y: Float64, z: Float64, w: Float64 }

Transform = {
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
  pose_version: PoseVersion,
  snapshot_ref: PoseSnapshotRef
}

AnchorBinding = {
  anchored_object,
  target,
  lineage,
  fallback_chain,
  authority,
  freshness
}
```

Additional rows:

- `PoseSnapshot`
- `PoseVersion`
- `AnchorSwitch`
- `FallbackReason`
- `AnchorFreshness`
- `PoseSnapshotFrontier`

## no-split-frame invariant

The conformance point is an observation snapshot exported to one client session.

```text
NoSplitFrame(session, frame, anchored, target) :=
  anchored.snapshot_ref == target.snapshot_ref
  and anchored.pose_version == target.pose_version
```

This means that, within one client observation frame, anchored object and target are resolved from the same pose snapshot. It does not claim global cross-client simultaneity, continuous spatial federation, or zero-latency renderer synchronization.

## fallback anchor admissibility

Fallback is guarded access-path availability extension, not object lifetime extension.

Fallback chain such as:

```text
head -> shoulder -> world_origin
```

is admissible only when it carries:

- same-lineage declared evidence
- explicit fallback reason
- monotone degradation
- visibility witness or explicit loss reason
- freshness / membership epoch check
- reacquire gate when the original target returns

Hidden repair, stale anchor resurrection, and implicit reacquire are not allowed.

## save/load interaction

PoseGraph state must enter the existing save/load carrier. A future `SaveObject` widening may include:

- `anchor_graph`
- `pose_snapshot_frontier`
- `pose_versions`
- `anchor_switch_state`
- fallback anchor state

Load admissibility must include:

- no stale anchor witness
- no stale fallback position
- anchor component snapshot coherence
- new witness / new epoch when saved anchor state cannot be resumed safely

## observability interaction

Future observer-safe devtools panels should expose:

- pose snapshot timeline
- anchor graph
- anchor switch rows
- split-frame violation rows
- redacted observer-safe transform summary

Admin/debug views may show richer state, but they remain authority-gated and audited.

## sample line

`P-POSE-01` closes only when:

- this spec and `plan/54` exist and are indexed.
- `Transform`, `Quat`, `Anchor`, `PoseVersion`, and no-split-frame are normative.
- sample rows are named and classified as planned.
- the non-claim "not global cross-client simultaneity" is explicit.

`P-POSE-02` closes only when:

- one positive sample shows target and anchored object using the same `pose_version` in one observation snapshot.
- one negative sample is statically rejected, runtime rejected, model-check counterexampled, or exported as a machine-readable devtools violation row.

Planned samples include:

- `avatar-head-transform`
- `anchored-object`
- `sparkle-fallback-anchor`
- `no-split-frame-positive`
- `split-frame-negative`
- `save-load-roundtrip`
- `stale-anchor-after-membership-advance`

## operational suite boundary

PoseGraph is not automatically part of `samples/product-alpha1/operational/`. It starts as a separate semantics line. Promotion into the operational product suite requires a later package with runnable check / run / observe / negative evidence and updated suite docs.

## non-claims

This document does not claim:

- VRM / VRChat / Unity compatibility
- renderer-owned world semantics
- global simultaneous coordinates
- continuous spatial sync
- WAN/federation
- distributed durable pose save/load
- active PoseGraph runtime samples in the current tree

