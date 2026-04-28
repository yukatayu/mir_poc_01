# network transport canaries 01

2026-04-28 current closeout では、phase 13 `Network transport` を **helper-local executable canary + process-boundary closeout** として追います。

ここで確認するのは、

- process boundary をまたいでも envelope / witness lane を見失わないこと
- reconnect が stale epoch / stale incarnation を hidden repair しないこと
- transport failure と domain rejection を分けて読めること
- route trace が typed / redacted visualization として出せること

であり、real network transport や final public ABI ではありません。

## まず実行するコマンド

```bash
python3 scripts/network_transport_samples.py list
python3 scripts/network_transport_samples.py run NET-02 --debug route-trace
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect
python3 scripts/network_transport_samples.py run NET-04 --debug failures
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --transport loopback_socket --format json
```

## current closeout inventory

- `transport_scope`
  `helper_local_process_boundary`
- `process_boundary_canaries`
  `NET-02 / NET-03 / NET-04 / NET-05`
- `loopback_parity_sources`
  `01_runtime_attach_game / 03_roll_publish_handoff / 04_non_owner_roll_rejected`
- `non_collapse_lanes`
  `transport / auth / membership / capability / witness / visualization`
- `kept_later_gates`
  `real_socket_or_broker / crypto_session_protocol / durable_replay_commit / continuous_shared_runtime_state / final_public_transport_abi`
- `validation_floor`
  helper-local canaries plus `loopback_socket` Sugoroku parity。real socket / broker / session / replay runtime ではない。

## sample ごとの読み

### `NET-02`

- source:
  `01_runtime_attach_game`, `03_roll_publish_handoff`
- execution:
  Sugoroku helper を child process として起動し、JSON bridge で route trace を集約する
- proves:
  attach / roll / publish / handoff の envelope と witness が process boundary canary を越えて残る
- does not prove:
  continuous shared state、real socket、durable replay

### `NET-03`

- source:
  `06_leave_non_owner`
- execution:
  stale `stale_roll_after_leave#1` envelope を reconnect attempt として再解釈する
- proves:
  `membership_epoch` と `member_incarnation` は transport success と別 lane で reject できる
- does not prove:
  production reconnect handshake

### `NET-04`

- source:
  `09_detach_todo` と helper-local failure matrix
- execution:
  timeout / queue-full / route-not-found / detach-after-send を typed row で並べる
- proves:
  retryable failure と terminal failure を transport lane で分けて扱える
- does not prove:
  real scheduler / backpressure tuning

### `NET-05`

- source:
  `03_roll_publish_handoff`, `09_detach_todo`
- execution:
  envelope 群から observer-safe redacted route trace を組み立てる
- proves:
  visualization も typed effect として authority / redaction を伴って扱える
- does not prove:
  final public visualization schema
