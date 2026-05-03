# practical-alpha1 expected summaries

These JSON files describe the current practical alpha-1 expected artifacts.

- `src-*.expected.json` files are test expectations for the narrow `package.mir.json` loader floor.
- `chk-*.expected.json` files are test expectations for the first practical checker floor.
- Positive checker proof uses explicit `accepted_obligations`.
- Negative checker proof uses explicit `rejected_rows` and `diagnostics`.
- They are not public API schemas.
- They do not imply runtime execution or public checker/API freeze.
