# network transport canaries 01

2026-04-27 current cut では、phase 13 `Network transport` を **helper-local executable canary** として追います。

ここで確認するのは、

- process boundary をまたいでも envelope / witness lane を見失わないこと
- reconnect が stale epoch / stale incarnation を hidden repair しないこと
- transport failure と domain rejection を分けて読めること
- route trace が typed / redacted visualization として出せること

であり、real network transport や final public ABI ではありません。

## まず実行するコマンド

```bash
python3 scripts/network_transport_samples.py run NET-02 --debug route-trace
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect
python3 scripts/network_transport_samples.py run NET-04 --debug failures
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace
python3 scripts/network_transport_samples.py check-all --format json
```

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
