# plan/61 — PoseGraph runtime roadmap

## purpose

This document is repository memory for `specs/37-posegraph-runtime-semantics.md`.

## current state

Current PoseGraph evidence is now split across two layers:

- `samples/product-alpha1/posegraph/` remains the helper-backed comparison floor with one same-client same-observation-snapshot accepted row, one split-frame `violation_export` row, and seven planned rows.
- `samples/full-system-v1/avatar-pose/` is now the bounded source-first runtime floor with 8 executable rows and 1 planned save/load row.
- `crates/mir-runtime::posegraph_runtime`, `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`, and `scripts/posegraph_runtime_samples.py check-all --format json` now prove Transform / PoseVersion / AnchorBinding / AnchorSwitch / fallback state / reacquire requirement runtime behavior.

This is useful first-floor evidence. It is not pose-aware save/load completion, devtools panel completion, or renderer integration.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-POSE-03` | runtime PoseGraph | session state includes Transform, PoseVersion, Anchor, AnchorBinding, full AnchorSwitch fields, fallback/reacquire state, and positive/negative no-split-frame checks |
| `P-POSE-04` | pose save/devtools | PoseGraph state enters save/load carrier with positive and negative admissibility rows, and observer-safe panels expose runtime evidence |
| `P-ENG-03` | renderer pose backend demo | renderer receives a pose snapshot but does not own semantics |

## planned rows

Runtime rows should cover:

- avatar head transform.
- anchored object.
- sparkle fallback anchor.
- no-split-frame positive.
- split-frame negative.
- save/load roundtrip.
- stale anchor after membership advance.
- anchor-switch frontier negative.
- stale-anchor reacquire required.

## validation target

Current validation:

```bash
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
python3 scripts/posegraph_runtime_samples.py check-all --format json
```

Existing helper anchor:

```bash
python3 scripts/posegraph_samples.py check-all --format json
```

## stop lines

- Do not claim global simultaneity.
- Do not claim VRM / VRChat / Unity compatibility.
- Do not treat renderer state as semantic owner.
- Do not call helper-only rows runtime completion.
