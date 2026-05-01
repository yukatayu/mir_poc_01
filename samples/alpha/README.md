# alpha samples

この root は、Mirrorea Spaces Alpha-0 line の sample matrix scaffold を置く。

- active runnable root ではない
- current package では planned skeleton / expected-verdict sidecar を置く
- `lifetime-fallback/` と `contract-variance/` と `cut-save-load/` には、selected rows について non-public checker floor 用の `expected_static.checked_reason_codes` を追加済み
- active runnable evidence は引き続き `samples/clean-near-end/` と related helpers にある
- `local-runtime/` には current package line で first Rust local-runtime floor が入り、`layer-insertion/` には first Rust layer-insertion floor が入るが、いずれも non-public sample-ID keyed runner であり、active sample root への昇格ではない
- `hotplug-runtime/` と `contract-variance/` の overlapping rows は引き続き planned/sample-mirror authority であり、current attach-time runtime floor は `layer-insertion/` 側に置く
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
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
```

## stop line

- do not mark these files active/runnable merely because they exist
- do not treat `.expected.json` sidecars as proof of implementation
- do not treat synthetic-artifact checker helpers as parser/runtime implementation
- do not treat the current Rust local-runtime floor as hot-plug/package/avatar/network completion
- do not treat the current Rust layer-insertion floor as completed lifecycle / detach / migration / public ABI completion
- do not silently move existing `samples/not_implemented/` residual families into active roots
