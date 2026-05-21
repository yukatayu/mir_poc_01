# samples/product-alpha1/posegraph

This root is the planned-only Transform / PoseGraph sample line for `P-POSE-01`.

## Current Status

- The root exists as scaffold actualization only.
- All rows are `planned_only`.
- No PoseGraph row is executable yet.
- The current no-split-frame reading stays explicit: same client session, same observation snapshot, same pose version.
- Renderer / engine state is not treated as semantic owner.

## Current Validation Anchor

Use the helper classification commands:

```bash
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json
```

Both `run` commands must reject as `planned_only` until `P-POSE-02`.

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

Representative `.mir` files are explanatory sketches only. They are not final grammar and are not current executable input.
