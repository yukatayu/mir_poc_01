# plan/54 — Transform / PoseGraph roadmap

## purpose

This document is repository memory for the Transform / PoseGraph semantics line defined by `specs/29-transform-posegraph-semantics.md`.

It keeps current avatar preview and fallback evidence useful, but does not reinterpret that evidence as full PoseGraph completion.

## current recognition

Current repo evidence covers:

- placeholder / custom avatar preview boundary
- fallback degradation export
- FAIRY-style same-lineage fallback and stale-membership reject concepts
- non-final observer-safe devtools export

Current repo defines the docs/spec carriers for:

- `PoseSnapshot`
- `pose_version`
- `Anchor`
- `AnchorBinding`
- `AnchorSwitch`
- no-split-frame invariant

Current repo does not yet actualize these as runnable evidence:

- anchor graph carrier
- no-split-frame positive / negative evidence
- pose-aware save/load admissibility
- PoseGraph devtools panel family
- anchor switch / stale-anchor reacquire negative rows

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-POSE-01` | Transform / PoseGraph theory and scaffold | `specs/29`, `plan/54`, planned sample matrix, no-split-frame non-claims |
| `P-POSE-02` | avatar head + anchored object no-split-frame sample | positive same-snapshot evidence and negative mismatch evidence |

## planned sample matrix

Planned roots, not yet present or runnable in `P-COMP-00`:

- `samples/product-alpha1/posegraph/avatar-head-transform/`
- `samples/product-alpha1/posegraph/anchored-object/`
- `samples/product-alpha1/posegraph/sparkle-fallback-anchor/`
- `samples/product-alpha1/posegraph/no-split-frame-positive/`
- `samples/product-alpha1/posegraph/split-frame-negative/`
- `samples/product-alpha1/posegraph/save-load-roundtrip/`
- `samples/product-alpha1/posegraph/stale-anchor-after-membership-advance/`
- `samples/product-alpha1/posegraph/anchor-switch-frontier-negative/`
- `samples/product-alpha1/posegraph/stale-anchor-reacquire-required/`

Planned helper, not yet present:

- `scripts/posegraph_samples.py`

Future validation anchors may include:

```bash
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-02-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-03-split-frame-negative --format json
```

These are future anchors, not current runnable validation.

## sample success criteria

`POSE-01 avatar-head-transform`:

- head publish/update remains session-bound event evidence.
- observer-safe export includes `pose_version` and `pose_snapshot_ref`.

`POSE-02 anchored-object`:

- target and anchored object reference the same observation snapshot.
- anchor graph is exported.

`POSE-03 fallback-anchor`:

- fallback chain has explicit lineage and explicit reason.
- visibility loss and stale membership do not trigger hidden repair.

`POSE-04 no-split-frame positive`:

- machine-readable evidence has `target_pose_version == anchored_pose_version` within the same session observation snapshot.

`POSE-05 split-frame negative`:

- mismatch is not accepted as stable state.
- one rejection or violation mode is fixed and reproducible.

`POSE-06 save-load roundtrip`:

- saved anchor component returns to a coherent saved frontier, or requires new witness / new epoch.

`POSE-07 stale-anchor-after-membership-advance`:

- stale membership is rejected and fallback is visible.

`POSE-08 anchor-switch-frontier-negative`:

- concurrent or stale anchor switch is ordered by owner epoch / sequence or rejected.
- `PoseSnapshotFrontier` and `membership_epoch` are visible.

`POSE-09 stale-anchor-reacquire-required`:

- fallback visibility does not extend ownership.
- reacquire requires explicit witness / epoch advance before the anchored object becomes stable again.

## save/load and devtools dependencies

Future PoseGraph packages should update:

- `specs/20-cut-save-load-semantics.md` for pose carrier and load admissibility.
- `specs/22-observability-devtools-semantics.md` for pose panels.
- `plan/47-operational-alpha09-devtools-roadmap.md` for roadmap-level panel inventory.

Future runtime work must make `PoseSnapshotFrontier`, `AnchorSwitch`, reacquire, membership-epoch advance, and concurrent switch ordering observable in helper output before claiming PoseGraph runtime completion.

`P-COMP-00` only records these as future hooks.

## operational suite promotion

PoseGraph starts as a separate semantics line. Promotion into `samples/product-alpha1/operational/` requires:

- runnable check / run / observe path
- positive and negative rows
- save/load and devtools carrier synchronization
- updated suite docs

## stop lines

- Do not claim global cross-client simultaneity.
- Do not claim Unity / VRM / VRChat compatibility.
- Do not treat renderer state as semantic owner.
- Do not promote planned PoseGraph roots into workflow-ready dashboard rows.
