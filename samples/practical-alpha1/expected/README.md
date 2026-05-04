# practical-alpha1 expected summaries

These JSON files describe the current practical alpha-1 expected artifacts.

- `src-*.expected.json` files are test expectations for the narrow `package.mir.json` loader floor.
- `chk-*.expected.json` files are test expectations for the first practical checker floor.
- `run-*.expected.json` files are exact expected reports for the first practical local-runtime floor.
- `hp-a1-*.expected.json` files are exact expected reports for the first practical hot-plug floor.
- `tr-a1-*.expected.json` files are exact expected reports for the first practical transport floor.
- `vis-a1-*.expected.json` files are exact expected bundles for the first practical devtools export floor.
- `sl-a1-*.expected.json` files are exact expected reports for the first practical local save/load floor.
- `av-a1-*.expected.json` files are exact expected reports for the first practical avatar preview companion floor.
- `pe2e-a1-*.expected.json` files are exact expected bundles for the first practical product-preview floor.
- Positive checker proof uses explicit `accepted_obligations`.
- Negative checker proof uses explicit `rejected_rows` and `diagnostics`.
- They are not public API schemas.
- `run-*.expected.json` does not imply Docker/package/save-load/devtools completion or public runtime/API freeze.
- `hp-a1-*.expected.json` may include attach-time stale-membership reject, attach-time missing-witness reject, narrow object package attach preview evidence, and an explicit deferred detach minimal contract boundary, but does not imply final object package attach completion, detach runtime lifecycle completion, Docker transport, save/load, or final public package/hot-plug ABI.
- `tr-a1-*.expected.json` may include local TCP / Docker Compose TCP accepted path, stale-membership reject, missing-capability reject, missing-witness reject, observer-safe route trace, and auth-lane separation, but does not imply WAN/federation, save/load, devtools export, product prototype, or final public transport ABI.
- `vis-a1-*.expected.json` may include event DAG export, publication / witness / handoff relation export, exact save-load membership timeline export, observer-safe route trace export, exact-report hot-plug lifecycle export, exact rejected-source fallback degradation export, and redacted observer view with auth-lane separation, but does not imply full devtools completion, distributed durable membership timeline, detach runtime lifecycle execution, retention/on-demand trace, save/load, product prototype, native execution, unsupported-runtime execution success, or final public viewer / telemetry ABI.
- `sl-a1-*.expected.json` may include a distinct save-load plan scope, saved local frontier summary, local-only roundtrip resume, and stale-membership non-resurrection, but does not imply distributed durable save/load, stale witness/stale lease non-resurrection completion, queue/channel/transport persistence, product prototype, or final public save-load ABI.
- `av-a1-*.expected.json` may include placeholder/custom/fallback avatar preview rows over exact hot-plug source reports, but does not imply native execution, final avatar package ABI, same-session product runtime completion, or VRM / VRChat / Unity compatibility.
- `pe2e-a1-*.expected.json` may include preview-manifest references, exact practical runtime / hot-plug / transport / save-load reports, exact devtools bundles, exact avatar preview companion reports, and a non-final static HTML viewer-openability preview, but does not imply native execution, same-session runtime attach/detach execution, unsupported-runtime execution success, full product prototype completion, or final public CLI / viewer / transport / save-load / package-avatar ABI.
