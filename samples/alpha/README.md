# alpha samples

この root は、Mirrorea Spaces Alpha-0 line の sample matrix scaffold を置く。

- active runnable root ではない
- root 全体としては mixed alpha-local scaffold であり、family ごとに
  - scaffold-only / checker-seed family
  - helper-local scoped evidence family
  - shared-helper-backed runtime-mirror family
  - runner-backed non-public floor
  を併存させる
- `lifetime-fallback/` と `contract-variance/` には、selected negative rows（現在は `LIF-01/05..08` と `VAR-02/03/05/07/09/10/15`）について non-public checker floor 用の `expected_static.checked_reason_codes` を追加済み
- `lifetime-fallback/` の `LIF-02/03/04` と `contract-variance/` の `VAR-01/04/06` には、selected positive rows 用の helper-local synthetic acceptance floor `expected_acceptance.checked_acceptance_rows` を追加済み
- `lifetime-fallback/` の `LIF-13` には、selected snapshot-selected positive row 用の helper-local synthetic snapshot floor `expected_snapshot.checked_snapshot_rows` を追加済み
- `lifetime-fallback/` の `LIF-11` には、selected anchor-handoff positive row 用の helper-local synthetic anchor-handoff floor `expected_anchor_handoff.checked_anchor_handoff_rows` を追加済み
- `contract-variance/` の `VAR-08/11/13` には、selected runtime-sensitive positive rows 用の `runtime_mirror.scope = alpha-runtime-mirror-floor` を追加済みであり、source authority は `layer-insertion/` の `LI-04/01/03` runtime-floor sidecars に残す
- `reason_codes_scope = alpha-static-floor`、`acceptance_scope = alpha-acceptance-floor`、`snapshot_scope = alpha-snapshot-selected-floor`、`anchor_handoff_scope = alpha-anchor-handoff-floor`、`runtime_mirror.scope = alpha-runtime-mirror-floor` は別 carrier であり、negative-static reject row、helper-local positive acceptance row、snapshot-selected row、anchor-handoff row、runtime-mirror row を混同しない
- `LIF-15` は planned-only `remote_observe_scope = alpha-remote-observe-floor` に split 済みであり、place identity / target identity / membership epoch / participant incarnation / frontier / label-redaction / covariance の future carrier inventory を要求する
- `VAR-14` は planned-only `adapter_transform_scope = alpha-adapter-transform-floor` に split 済みであり、source-target contract / pre-post preservation / effect-failure containment / provided surface / observation-redaction-retention / fallback representation の future carrier inventory を要求する
- `cut-save-load/` には selected negative rows の checker floor に加えて、`scripts/alpha_cut_save_load_samples.py` が `CUT-04` local-only save/load bridge、`CUT-17` stale-membership rejection bridge、`CUT-11` checker-backed Z-cycle inadmissibility row を actualize している
- `local-runtime/` には `scripts/alpha_local_runtime_samples.py` が `LR-01/02` dedicated runner と current-scope Stage B closeout surface を追加し、`CUT-04/17` は supporting local-only save/load subset としてだけ再利用される
- `scripts/alpha_hotplug_lifecycle_samples.py` は `layer-insertion/` の `LI-01/02/03/04/05` と `avatar-runtime/` / `hotplug-runtime/` の `AV-01/02/06/08/09` / `HP-11/12/15` を current-scope Stage D closeout surface として束ねる
- active runnable evidence は引き続き `samples/clean-near-end/` と related helpers にある
- `local-runtime/` には current-scope Stage B closeout surface、`layer-insertion/` と `avatar-runtime/` / `hotplug-runtime/` には current-scope Stage D lifecycle closeout surface over the existing Rust layer/package/avatar floors、`network-docker/` には current-scope Stage C transport closeout surface over the existing Rust network floor + Docker Compose runner が入るが、いずれも non-public sample-ID keyed runner であり、active sample root への昇格ではない
- `visualization/` には thin runner `scripts/alpha_visualization_samples.py` が `VIS-01/02/03/05/06/07/08/10/11` を actualize したが、`VIS-04/09/12` は planned-only のままであり、Stage E / Stage F completion claim には使わない
- `e2e/` には thin integrated bridge runner `scripts/alpha_e2e_samples.py` が `E2E-01/02/03/04/05/06/07/09/10` を actualize したが、`E2E-08` は planned-only のままであり、Stage F completion claim には使わない
- `hotplug-runtime/` と `contract-variance/` の overlapping rows は引き続き planned/sample-mirror authority であり、current attach-time runtime floor は `layer-insertion/` 側に置く
- `P-A0-18` は `contract-variance/` に parser/runtime bridge を追加せず、`VAR-08/11/13` を existing `layer-insertion/` runtime floor への mirror evidence としてだけ actualize した
- `P-A0-20` は `LIF-13` だけを actualize し、`alpha-acceptance-floor` を広げずに `snapshot_scope = alpha-snapshot-selected-floor` の helper-local synthetic snapshot-selected carrier を追加した
- `P-A0-21` は `LIF-11` だけを actualize し、`alpha-acceptance-floor` や `alpha-snapshot-selected-floor` を広げずに `anchor_handoff_scope = alpha-anchor-handoff-floor` の helper-local synthetic anchor-handoff carrier を追加した
- `P-A0-22` は `LIF-15` と `VAR-14` を actualize せず、planned-only `alpha-remote-observe-floor` と `alpha-adapter-transform-floor` の blocker split として固定した
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
  scripts.tests.test_alpha_lifetime_fallback_acceptance \
  scripts.tests.test_current_l2_family_snapshot_support \
  scripts.tests.test_alpha_lifetime_fallback_snapshot \
  scripts.tests.test_current_l2_family_anchor_handoff_support \
  scripts.tests.test_alpha_lifetime_fallback_anchor_handoff \
  scripts.tests.test_alpha_contract_variance_acceptance \
  scripts.tests.test_current_l2_family_runtime_mirror_support \
  scripts.tests.test_alpha_contract_variance_runtime_mirror \
  scripts.tests.test_alpha_cut_save_load_checker
cargo test -p mirrorea-core --test runtime_substrate
cargo test -p mir-runtime --test alpha_cut_save_load_runtime
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
cargo test -p mir-runtime --test alpha_local_runtime
python3 scripts/alpha_local_runtime_samples.py check-all --format json
python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout
cargo test -p mir-runtime --test alpha_network_runtime
python3 scripts/alpha_network_docker_e2e.py check-all --format json
python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json
cargo test -p mir-runtime --test alpha_avatar_runtime
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json
python3 scripts/alpha_visualization_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py run E2E-06 --format json
python3 scripts/alpha_e2e_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py closeout --format json
python3 -m unittest \
  scripts.tests.test_alpha_lifetime_fallback_checker \
  scripts.tests.test_alpha_contract_variance_checker \
  scripts.tests.test_alpha_lifetime_fallback_acceptance \
  scripts.tests.test_current_l2_family_snapshot_support \
  scripts.tests.test_alpha_lifetime_fallback_snapshot \
  scripts.tests.test_current_l2_family_anchor_handoff_support \
  scripts.tests.test_alpha_lifetime_fallback_anchor_handoff \
  scripts.tests.test_alpha_contract_variance_acceptance \
  scripts.tests.test_current_l2_family_runtime_mirror_support \
  scripts.tests.test_alpha_contract_variance_runtime_mirror \
  scripts.tests.test_alpha_cut_save_load_checker \
  scripts.tests.test_alpha_cut_save_load_samples \
  scripts.tests.test_alpha_visualization_samples \
  scripts.tests.test_alpha_e2e_samples
```

## stop line

- do not mark these files active/runnable merely because they exist
- do not treat `.expected.json` sidecars as proof of implementation
- do not treat synthetic-artifact checker helpers as parser/runtime implementation
- do not treat helper-local synthetic acceptance rows as parser/runtime implementation or final public checker verdict
- do not treat the current Rust local-runtime floor as hot-plug/package/avatar/network completion
- do not treat the current Rust layer-insertion floor as completed lifecycle / detach / migration / public ABI completion
- do not treat the current Stage D hot-plug lifecycle closeout surface as detach runtime / migration / native execution / public ABI completion
- do not treat the current Rust/Docker Stage-C network floor as production transport / WAN federation / final public transport ABI completion
- do not treat the current runtime-private avatar/package floor as final avatar API / native execution / final runtime package ABI completion
- do not treat the current visualization subset runner or integrated `e2e/` bridge runner as Stage E or Stage F completion
- do not silently move existing `samples/not_implemented/` residual families into active roots
