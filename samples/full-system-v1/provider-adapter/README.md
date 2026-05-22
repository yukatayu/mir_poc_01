# Full System V1 Provider Admission Samples

This root actualizes `P-ENG-02` as bounded provider admission evidence over the accepted local role-split floor.

Current scope:

- `viewer-diagnostic-positive/` admits a viewer/diagnostic provider contract as inventory-only typed boundary evidence.
- `over-capability-negative/` rejects a provider that asks for capability outside the matched projection boundary.
- `missing-rollback-negative/` rejects a provider that omits rollback/replay/cut policy.
- `native-disabled-negative/` rejects native execution under the default-disabled policy.
- `wasm-inventory-positive/` keeps the WASM provider row inventory-only with explicit deferred sandbox admission.

Commands:

```bash
python3 -m unittest scripts.tests.test_provider_admission_samples
python3 scripts/provider_admission_samples.py matrix --format json
python3 scripts/provider_admission_samples.py check-all --format json
cargo test -p mir-runtime --test provider_admission -- --nocapture
```

Stop lines:

- no arbitrary native package execution
- no arbitrary WASM execution
- no final engine adapter ABI or SDK
- no renderer-owned world semantics
