# network transport helper-local canaries

この directory は、phase 13 `Network transport` の **active helper-local canary landing page** です。

- 規範正本ではありません。
- final public transport ABI でもありません。
- 実際の source sample は主に `samples/clean-near-end/sugoroku-world/` にあり、
  ここではそれらを process-boundary / reconnect / typed failure / redacted route trace の
  helper-local transport evidence として読み替えます。

## active commands

`check-all` is the executable canary anchor for `NET-02..05`.
`closeout` is inventory evidence only; it records process-boundary scope, parity anchors,
non-collapse lanes, kept-later gates, and validation-floor wording.

```bash
python3 scripts/network_transport_samples.py list
python3 scripts/network_transport_samples.py run NET-02 --debug route-trace
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json
python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --transport loopback_socket --format json
```

## current sample IDs

- `NET-02`
  two-process attach / roll / publish / handoff canary。
  `scripts/sugoroku_world_samples.py` を child process として 2 回呼び、envelope / witness lanes が
  JSON bridge をまたいでも落ちないことだけを確認します。
  これは stitched helper-local trace であり、単一の continuous transported session を証明するものではありません。
- `NET-03`
  reconnect epoch guard canary。
  stale `membership_epoch` / `member_incarnation` を hidden repair せず reject する helper-local read です。
- `NET-04`
  typed transport failure family canary。
  timeout / queue-full / route-not-found / detach-after-send を retryable / terminal に分けて明示します。
- `NET-05`
  observer-safe redacted route trace canary。
  route trace を typed visualization として出し、auth / capability payload をそのまま漏らさないことを確認します。

## relation to other paths

- active Sugoroku evidence:
  `samples/clean-near-end/sugoroku-world/`
- active runner:
  `scripts/network_transport_samples.py`
- docs-first backlog:
  `samples/not_implemented/network-transport/`

## current closeout inventory

- helper closeout は `transport_scope = helper_local_process_boundary` を返す
- helper closeout は `process_boundary_canaries = NET-02..05` を返す
- helper closeout は `loopback_parity_sources = 01_runtime_attach_game / 03_roll_publish_handoff / 04_non_owner_roll_rejected` を返す
- helper closeout は `non_collapse_lanes = transport / auth / membership / capability / witness / visualization` を返す
- helper closeout は `kept_later_gates = real_socket_or_broker / crypto_session_protocol / durable_replay_commit / continuous_shared_runtime_state / final_public_transport_abi` を返す
- helper closeout は `validation_floor` を返し、helper-local canaries + `loopback_socket` parity だけを current line として読む

## stop line

- real socket / broker / QUIC / WebRTC choice
- cryptographic session protocol
- production reconnect policy
- durable distributed commit
- final public transport / telemetry contract
