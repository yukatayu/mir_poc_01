# comp-04-host-io-internal-transform

This root now carries the `P-COMP-04` pure/effect split rows.

- `positive/` is a direct Product Alpha-1 `run-local` row.
- `negative-undeclared-effect/` rejects at `check` because `host_output.effect_ref` is not declared in package effects.
- `negative-undeclared-failure/` rejects at `check` because `mir_compute.failure_tag` is not declared in the active computational contract `failure_row`.
- `negative-missing-capability/` rejects at `check` because `mir_compute.required_capabilities` is not declared in package capabilities.

The boundary stays explicit:

- host input reads through `typed_host_io.read_int`
- Mir-owned computation performs the internal transform
- host output writes through `typed_host_io.write_int`

Representative `.mir` files are explanatory only. Current executable input remains `package.mir.json`.
