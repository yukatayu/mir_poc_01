# alpha samples

この root は、Mirrorea Spaces Alpha-0 line の sample matrix scaffold を置く。

- active runnable root ではない
- root 全体としては mixed alpha-local scaffold であり、family ごとに
  - scaffold-only / checker-seed family
  - runner-backed non-public floor
  を併存させる
- `lifetime-fallback/` と `contract-variance/` と `cut-save-load/` には、selected rows について non-public checker floor 用の `expected_static.checked_reason_codes` を追加済み
- active runnable evidence は引き続き `samples/clean-near-end/` と related helpers にある
- `local-runtime/` には first Rust local-runtime floor、`layer-insertion/` には first Rust layer-insertion floor、`network-docker/` には first Rust Stage-C network floor + Docker Compose runner、`avatar-runtime/` には first runtime-private package/avatar admission floor + thin runner が入るが、いずれも non-public sample-ID keyed runner であり、active sample root への昇格ではない
- `e2e/` には thin integrated bridge runner `scripts/alpha_e2e_samples.py` が `E2E-01/02/03/04/05/07/09/10` を actualize したが、`E2E-06/08` は planned-only のままであり、Stage F completion claim には使わない
- `hotplug-runtime/` と `contract-variance/` の overlapping rows は引き続き planned/sample-mirror authority であり、current attach-time runtime floor は `layer-insertion/` 側に置く
- `hotplug-runtime/` の `HP-11/12/15` は `avatar-runtime/` family と共用する runtime-private native-policy subset として actualize 済みだが、family 全体の runnable promotion ではない
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
cargo test -p mir-runtime --test alpha_network_runtime
python3 scripts/alpha_network_docker_e2e.py check-all --format json
cargo test -p mir-runtime --test alpha_avatar_runtime
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py check-all --format json
```

## stop line

- do not mark these files active/runnable merely because they exist
- do not treat `.expected.json` sidecars as proof of implementation
- do not treat synthetic-artifact checker helpers as parser/runtime implementation
- do not treat the current Rust local-runtime floor as hot-plug/package/avatar/network completion
- do not treat the current Rust layer-insertion floor as completed lifecycle / detach / migration / public ABI completion
- do not treat the current Rust/Docker Stage-C network floor as production transport / WAN federation / final public transport ABI completion
- do not treat the current runtime-private avatar/package floor as final avatar API / native execution / final runtime package ABI completion
- do not treat the current integrated `e2e/` bridge runner as Stage E or Stage F completion while `E2E-06` and dedicated alpha visualization/devtools remain later
- do not silently move existing `samples/not_implemented/` residual families into active roots
