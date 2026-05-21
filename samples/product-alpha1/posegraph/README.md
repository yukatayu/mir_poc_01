# samples/product-alpha1/posegraph

This root is the bounded Transform / PoseGraph helper evidence line for `P-POSE-02`.

## Current Status

- The root keeps the scaffold actualization from `P-POSE-01`.
- `pose-04-no-split-frame-positive` is helper-executable and must return `accepted`.
- `pose-05-split-frame-negative` is helper-executable and must return `violation_export`.
- The remaining 7 rows stay `planned_only`.
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

These commands must show one accepted row, one violation row, and seven planned rows. They do not claim full PoseGraph runtime completion.

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

Representative `.mir` files are explanatory sketches only. `package.mir.json` for `pose-04` and `pose-05` is helper-only executable input, not final grammar and not direct product-alpha CLI input.
