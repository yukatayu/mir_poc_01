# samples/full-system-v1/avatar-pose

This root is the source-first Full System V1 PoseGraph runtime line for `P-POSE-03` / `P-POSE-04`.

## Current Status

- All 9 rows are runtime-executable through `crates/mir-runtime::posegraph_runtime`.
- `pose-06-save-load-roundtrip` is the bounded accepted save/load row, while `pose-07` through `pose-09` also carry explicit load-inadmissible negative evidence.
- The current runtime reading stays explicit: same-client same-observation-snapshot no-split-frame coherence, typed stale-anchor rejection, explicit anchor-switch frontier checks, explicit reacquire requirement after fallback-only state, bounded save/load admissibility, and observer-safe PoseGraph/devtools export.
- Renderer / engine state is not treated as semantic owner.

## Current Validation Anchor

```bash
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
python3 scripts/posegraph_runtime_samples.py matrix --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
```

These commands prove bounded runtime PoseGraph evidence only. They do not claim distributed durable pose save/load, final devtools panels, or renderer compatibility.

## Rows

- `pose-01-avatar-head-transform`
- `pose-02-anchored-object`
- `pose-03-sparkle-fallback-anchor`
- `pose-04-no-split-frame-positive`
- `pose-05-split-frame-negative`
- `pose-06-save-load-roundtrip`
- `pose-07-stale-anchor-after-membership-advance`
- `pose-08-anchor-switch-frontier-negative`
- `pose-09-stale-anchor-reacquire-required`
