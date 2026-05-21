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
- pose-aware save/load admissibility
- PoseGraph devtools panel family
- anchor switch / stale-anchor reacquire negative rows

Current repo now actualizes bounded helper evidence for:

- no-split-frame positive same-snapshot acceptance
- no-split-frame negative mismatch `violation_export`

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-POSE-01` | Transform / PoseGraph theory and scaffold | `specs/29`, `plan/54`, planned sample matrix, no-split-frame non-claims |
| `P-POSE-02` | avatar head + anchored object no-split-frame sample | positive same-snapshot evidence and negative mismatch evidence |

## current sample matrix

`P-POSE-01` actualizes the scaffold family. `P-POSE-02` then promotes two rows to bounded helper evidence while keeping the rest planned:

- `samples/product-alpha1/posegraph/avatar-head-transform/`
- `samples/product-alpha1/posegraph/anchored-object/`
- `samples/product-alpha1/posegraph/sparkle-fallback-anchor/`
- `samples/product-alpha1/posegraph/no-split-frame-positive/`
- `samples/product-alpha1/posegraph/split-frame-negative/`
- `samples/product-alpha1/posegraph/save-load-roundtrip/`
- `samples/product-alpha1/posegraph/stale-anchor-after-membership-advance/`
- `samples/product-alpha1/posegraph/anchor-switch-frontier-negative/`
- `samples/product-alpha1/posegraph/stale-anchor-reacquire-required/`

Current helper actualized in `P-POSE-02`:

- `scripts/posegraph_samples.py`

Current validation anchors:

```bash
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json
```

These commands validate matrix classification, accepted same-snapshot evidence, and negative `violation_export` behavior. They do not prove full PoseGraph runtime evidence.

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

`P-POSE-01` records the scaffold and future hooks. `P-POSE-02` actualizes the minimal helper-backed no-split-frame evidence, while save/load and devtools hooks stay later.

## observed closeout

`P-POSE-02` closed on 2026-05-21 with:

- `samples/product-alpha1/posegraph/no-split-frame-positive/package.mir.json`
- `samples/product-alpha1/posegraph/split-frame-negative/package.mir.json`
- `samples/product-alpha1/posegraph/matrix.json` updated to `mixed`
- `scripts/posegraph_samples.py` returning `accepted` for `pose-04` and `violation_export` for `pose-05`
- `scripts/tests/test_posegraph_samples.py` locking the accepted / violation / planned split

This closeout is intentionally bounded. It does not claim workflow-ready PoseGraph runtime, save/load admissibility, or devtools panel completion.

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
