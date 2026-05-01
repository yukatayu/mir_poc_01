# alpha samples

この root は、Mirrorea Spaces Alpha-0 line の sample matrix scaffold を置く。

- active runnable root ではない
- current package では planned skeleton / expected-verdict sidecar を置く
- `lifetime-fallback/` と `contract-variance/` と `cut-save-load/` には、selected rows について non-public checker floor 用の `expected_static.checked_reason_codes` を追加済み
- active runnable evidence は引き続き `samples/clean-near-end/` と related helpers にある
- `local-runtime/` には current package で first Rust local-runtime floor が入るが、non-public in-memory runner keyed by sample ID であり、active sample root への昇格ではない
- `samples/not_implemented/` は pre-existing residual planned families を保持する legacy planned root として残す

## current families

- `lifetime-fallback/`
- `contract-variance/`
- `cut-save-load/`
- `local-runtime/`
- `layer-insertion/`
- `network-docker/`
- `hotplug-runtime/`
- `avatar-runtime/`
- `visualization/`
- `e2e/`

## validation anchor for the scaffold package

```bash
find samples/alpha -maxdepth 3 -type f | sort
python3 -m unittest \
  scripts.tests.test_alpha_lifetime_fallback_checker \
  scripts.tests.test_alpha_contract_variance_checker \
  scripts.tests.test_alpha_cut_save_load_checker
cargo test -p mir-runtime --test alpha_local_runtime
```

## stop line

- do not mark these files active/runnable merely because they exist
- do not treat `.expected.json` sidecars as proof of implementation
- do not treat synthetic-artifact checker helpers as parser/runtime implementation
- do not treat the current Rust local-runtime floor as hot-plug/package/avatar/network completion
- do not silently move existing `samples/not_implemented/` residual families into active roots
