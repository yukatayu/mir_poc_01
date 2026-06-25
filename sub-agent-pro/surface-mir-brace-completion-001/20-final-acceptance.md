# 20 — Final Acceptance Criteria

After P-SURF-99, it must be true that:

1. A developer can write `.mir` source using `S { ... }` place blocks.
2. `S[ ... ]` is rejected with a clear diagnostic.
3. Indexed state works:

```mir
S { state player[p: Participant]: Player }
```

4. Cross-place read/write elaborates to Core IR and visible devtools traces.
5. Auto publish/observe works for declared visible fields.
6. Private field auto publish is rejected.
7. Browser role admission works through admission root / capability grant.
8. Role spoofing does not grant authority.
9. Source patch hot-plug works through parse/typecheck/elaborate/admit/activation_cut.
10. Product Alpha / Operational Suite compatibility anchors still pass.

This is near-production-flow alpha, not final production.

Still non-claims:

- final public grammar / ABI / SDK.
- full Rust-like language completion.
- LLVM/native codegen.
- production WAN/federation.
- distributed durable save-load R3/R4.
- arbitrary native/WASM/Unity/UE provider execution.
