# 0967 P13 helper-local process-boundary closeout

## Objective

`P13` `network transport minimal alpha` の current first-cut closeout を、helper-local `process_boundary`
inventory と `loopback_socket` parity evidence までで正確に閉じる。

## Scope and assumptions

- scope は helper/test/docs closeout hardening に限定する。
- real socket / broker、crypto session、durable replay / commit、continuous shared runtime state、
  final public transport ABI は kept-later gate に残す。
- crate-side runtime ownership cut は進めず、`scripts/network_transport_samples.py` と snapshot / roadmap /
  reader docs の同期に留める。
- `P13` close 後の queue wording は `P14 promoted next / P15 reopen next` に揃える。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `samples/clean-near-end/network-transport/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `scripts/network_transport_samples.py`
- `scripts/tests/test_network_transport_samples.py`

## Actions taken

1. `scripts/tests/test_network_transport_samples.py` に closeout inventory regression test を追加し、
   `process_boundary_canaries` 不在で RED を確認した。
2. `scripts/network_transport_samples.py` の closeout に次を追加した。
   - `process_boundary_canaries`
   - `loopback_parity_sources`
   - `non_collapse_lanes`
   - `kept_later_gates`
   - `validation_floor`
3. network transport docs を `process_boundary` closeout inventory に同期した。
4. snapshot / roadmap docs を `P13 close / P14 promoted next / P15 reopen next` に同期した。
5. `plan/11` の stale near-term queue を current line に更新した。
6. `docs/research_abstract/hotplug_attachpoint_plan_01.md` と
   `samples/clean-near-end/sugoroku-world/README.md` の hot-plug 説明を、
   runtime ownership claim ではなく package-manager first-cut wording に寄せた。

## Files changed

- `scripts/network_transport_samples.py`
- `scripts/tests/test_network_transport_samples.py`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `samples/clean-near-end/network-transport/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`

## Evidence / outputs / test results

### RED/GREEN

- RED:
  - `python3 -m unittest scripts.tests.test_network_transport_samples`
  - failure reason: `KeyError: 'process_boundary_canaries'`
- GREEN:
  - `python3 -m unittest scripts.tests.test_network_transport_samples`
  - result: `Ran 11 tests in 2.245s` / `OK`

### Fresh validation

- `python3 scripts/network_transport_samples.py list`
  - `NET-02..05` sample set present
- `python3 scripts/network_transport_samples.py run NET-02 --debug route-trace --format json`
  - `transport_scope = helper_local_process_boundary`
  - `bridge_kind = subprocess_json_bridge`
  - `attach_request#1` / `roll_request#1` / `handoff_notice#1` parity evidence present
- `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json`
  - `reason_family = stale_membership_epoch`
- `python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json`
  - typed failure family `timeout / queue_full / route_not_found / detach_after_send`
- `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
  - observer-safe redacted route trace
  - `retention_scope = helper_local_ephemeral`
- `python3 scripts/network_transport_samples.py check-all --format json`
  - `passed = [NET-02, NET-03, NET-04, NET-05]`
- `python3 scripts/network_transport_samples.py closeout --format json`
  - `process_boundary_canaries = NET-02 / NET-03 / NET-04 / NET-05`
  - `loopback_parity_sources = 01_runtime_attach_game / 03_roll_publish_handoff / 04_non_owner_roll_rejected`
  - `non_collapse_lanes = transport / auth / membership / capability / witness / visualization`
  - `kept_later_gates = real_socket_or_broker / crypto_session_protocol / durable_replay_commit / continuous_shared_runtime_state / final_public_transport_abi`
  - `validation_floor = helper-local canaries plus loopback_socket Sugoroku parity; not real socket/broker/session/replay runtime`
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json`
  - `attach_request#1` parity anchor present
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json`
  - `roll_request#1` / `handoff_notice#1` parity anchors present
- `python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --transport loopback_socket --format json`
  - reject parity anchor present
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- `P13` の honest completion line は real transport alpha ではなく、
  helper-local `process_boundary` closeout inventory で十分に切れる。
- `loopback_socket` parity を source sample 側に明示すると、
  `NET-02..05` canary family が same-process preview と subprocess JSON bridge の両方に anchored される。
- `P14` next line は runtime crate actualization ではなく、
  helper/test/docs closeout hardening を先に切る方が repo current line と整合する。

## Open questions

- `P14` package-manager first cut で `scripts/sugoroku_world_samples.py` closeout に
  どこまで package inventory field を追加するか。
- rollback wording を `deferred` と `UNRESOLVED` のどちらで統一するか。
- `P15` emitted place-specific program family の first artifact minimum を
  helper preview と generated reserve のどこで切るか。

## Suggested next prompt

`P14` hot-plug package-manager tranche の current first cut を進め、helper/test/docs closeout hardening として
`hotplug_lifecycle` / attach-detach canary / package-manager inventory wording / activation cut / migration stop line を
同期してください。runtime crate 側の migration engine / rollback / final public hot-plug ABI は deferred のままにしてください。
