# practical-alpha1 expected summaries

These JSON files describe the current practical alpha-1 expected artifacts.

- `src-*.expected.json` files are test expectations for the narrow `package.mir.json` loader floor.
- `chk-*.expected.json` files are test expectations for the first practical checker floor.
- `run-*.expected.json` files are exact expected reports for the first practical local-runtime floor.
- `hp-a1-*.expected.json` files are exact expected reports for the first practical hot-plug floor.
- Positive checker proof uses explicit `accepted_obligations`.
- Negative checker proof uses explicit `rejected_rows` and `diagnostics`.
- They are not public API schemas.
- `run-*.expected.json` does not imply Docker/package/save-load/devtools completion or public runtime/API freeze.
- `hp-a1-*.expected.json` may include attach-time stale-membership reject, attach-time missing-witness reject, narrow object package attach preview evidence, and an explicit deferred detach minimal contract boundary, but does not imply final object package attach completion, detach runtime lifecycle completion, Docker transport, save/load, or final public package/hot-plug ABI.
