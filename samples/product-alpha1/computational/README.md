# samples/product-alpha1/computational

This root is the Mir-owned computation sample line defined by `specs/28` / `plan/53`.

## Current Status

- `comp-02-pure-add-one` is now executable through Product Alpha-1 `run-local`.
- The remaining `P-COMP-03` / `P-COMP-04` rows are still `planned_only`.
- The legacy alpha `typed_host_io.add_one` lane remains host-boundary evidence. It is preserved alongside the new Mir-owned lane and is not reinterpreted.

## Current Validation Anchor

Use the computational helper and runtime commands:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
```

`run comp-02-pure-add-one` now proves one bounded Mir-owned row:

```text
ReadInt(41) -> add_one -> WriteInt(42)
```

The remaining rows still reject as `planned_only`.

## Rows

- `comp-02-pure-add-one`
- `comp-03-variables-scope`
- `comp-03-arrays-bounds`
- `comp-03-records-vec3`
- `comp-03-control-flow`
- `comp-03-imports-functions`
- `comp-04-host-io-internal-transform`

Representative `.mir` files are explanatory sketches only. They are not final grammar and are not current executable input.
