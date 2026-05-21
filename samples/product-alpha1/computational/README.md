# samples/product-alpha1/computational

This root is the Mir-owned computation sample line defined by `specs/28` / `plan/53`.

## Current Status

- `comp-02-pure-add-one` is executable through Product Alpha-1 `run-local`.
- `comp-03-*` now has ten helper-executable rows:
  five `accepted` rows and five `runtime_rejection` rows.
- `comp-04-host-io-internal-transform` remains `planned_only`.
- The legacy alpha `typed_host_io.add_one` lane remains host-boundary evidence. It is preserved alongside the new Mir-owned lane and is not reinterpreted.

## Current Validation Anchor

Use the computational helper and runtime commands:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
python3 scripts/mir_computational_samples.py run comp-03-control-flow-positive --format json
python3 scripts/mir_computational_samples.py run comp-03-variables-scope-negative --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
```

`run comp-02-pure-add-one` proves one bounded Mir-owned direct runtime row:

```text
ReadInt(41) -> add_one -> WriteInt(42)
```

`run comp-03-*` proves helper-executable accepted/runtime-rejection first-floor rows. Only `comp-04-host-io-internal-transform` still rejects as `planned_only`.

## Rows

- `comp-02-pure-add-one`
- `comp-03-variables-scope-positive`
- `comp-03-variables-scope-negative`
- `comp-03-arrays-bounds-positive`
- `comp-03-arrays-bounds-negative`
- `comp-03-records-vec3-positive`
- `comp-03-records-vec3-negative`
- `comp-03-control-flow-positive`
- `comp-03-control-flow-negative`
- `comp-03-imports-functions-positive`
- `comp-03-imports-functions-negative`
- `comp-04-host-io-internal-transform`

Representative `.mir` files are explanatory sketches only. They are not final grammar and are not current executable input.
