# samples/product-alpha1/computational

This root is the planned-only Mir-owned computation sample line for `P-COMP-01`.

## Current Status

- The root exists as scaffold actualization only.
- All rows are `planned_only`.
- No `package.mir.json` in this root is executable yet.
- The current `AddOne` non-claim remains explicit: the existing alpha `AddOne` lane is host-boundary evidence, not Mir-owned computation completion.

## Current Validation Anchor

Use the helper classification commands:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
```

`run comp-02-pure-add-one` must reject as `planned_only` until `P-COMP-02`.

## Rows

- `comp-02-pure-add-one`
- `comp-03-variables-scope`
- `comp-03-arrays-bounds`
- `comp-03-records-vec3`
- `comp-03-control-flow`
- `comp-03-imports-functions`
- `comp-04-host-io-internal-transform`

Representative `.mir` files are explanatory sketches only. They are not final grammar and are not current executable input.
