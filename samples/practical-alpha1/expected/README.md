# practical-alpha1 expected summaries

These JSON files describe the current practical alpha-1 expected artifacts.

- `src-*.expected.json` files are test expectations for the narrow `package.mir.json` loader floor.
- `chk-*.expected.json` files are test expectations for the first practical checker floor.
- `run-*.expected.json` files are exact expected reports for the first practical local-runtime floor.
- `hp-a1-*.expected.json` files are exact expected reports for the first practical hot-plug floor.
- `tr-a1-*.expected.json` files are exact expected reports for the first practical transport floor.
- `vis-a1-*.expected.json` files are exact expected bundles for the first practical devtools export floor.
- Positive checker proof uses explicit `accepted_obligations`.
- Negative checker proof uses explicit `rejected_rows` and `diagnostics`.
- They are not public API schemas.
- `run-*.expected.json` does not imply Docker/package/save-load/devtools completion or public runtime/API freeze.
- `hp-a1-*.expected.json` may include attach-time stale-membership reject, attach-time missing-witness reject, narrow object package attach preview evidence, and an explicit deferred detach minimal contract boundary, but does not imply final object package attach completion, detach runtime lifecycle completion, Docker transport, save/load, or final public package/hot-plug ABI.
- `tr-a1-*.expected.json` may include local TCP / Docker Compose TCP accepted path, stale-membership reject, missing-capability reject, missing-witness reject, observer-safe route trace, and auth-lane separation, but does not imply WAN/federation, save/load, devtools export, product prototype, or final public transport ABI.
- `vis-a1-*.expected.json` may include event DAG export, publication / witness / handoff relation export, observer-safe route trace export, and redacted observer view with auth-lane separation, but does not imply full devtools completion, membership timeline, hot-plug lifecycle, fallback degradation, retention/on-demand trace, save/load, product prototype, or final public viewer / telemetry ABI.
