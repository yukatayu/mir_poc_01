# 11 — PoseGraph / Transform Roadmap

## 1. Goal

Mir manages VR semantic state:

- avatar head / hand / shoulder transform.
- object anchors.
- fallback anchors.
- pose version.
- same-snapshot no-split-frame guarantee.

Renderer / Unity / UE only render or provide bounded backend effects.

## 2. Core types

```mir
record Vec3 {
  x: Float64,
  y: Float64,
  z: Float64,
}

record Quat {
  x: Float64,
  y: Float64,
  z: Float64,
  w: Float64,
}

record Transform {
  position: Vec3,
  rotation: Quat,
  pose_version: UInt64,
}
```

## 3. Anchor example

```mir
Avatar[self] {
  state head: Transform
  state shoulder: Transform
}

World {
  state sparkle[p: Participant]: Anchor
}
```

Possible syntax later:

```mir
World {
  state sparkle_anchor[p: Participant]: Anchor
    init anchor Avatar[p].head
      fallback Avatar[p].shoulder
      fallback World.origin
}
```

## 4. No-split-frame invariant

```text
Within one client observation frame,
all values dependent on an anchor must use the same pose_version.
```

Positive:

```text
head@v and sparkle anchored to head@v
```

Negative:

```text
head@v and sparkle anchored to head@v-1
```

## 5. Runtime requirements

- pose snapshot.
- pose version.
- anchor binding.
- fallback/reacquire rule.
- stale anchor rejection.
- save/load admissibility.
- devtools pose panel.

## 6. Samples

```text
POSE-SURF-01 avatar head transform
POSE-SURF-02 object anchored to head
POSE-SURF-03 sparkle fallback anchor
POSE-SURF-04 no-split-frame positive
POSE-SURF-05 split-frame negative
POSE-SURF-06 pose save/load
POSE-SURF-07 devtools panel
```
