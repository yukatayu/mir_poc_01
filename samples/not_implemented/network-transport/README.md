# network-transport planned family

この directory は phase 13 `Network transport` の **planned skeleton family** です。

- current parser / runner ではまだ扱いません。
- final public transport ABI、real socket / broker 実装、cryptographic session protocol を意味しません。
- current repo では、Sugoroku helper の `local_queue` envelope と clean near-end `provider_boundary` を
  docs-first evidence anchor として使います。

## planned sample IDs

- `NET-01` loopback attach / envelope parity
- `NET-02` two-process roll / publish / handoff
- `NET-03` reconnect with membership epoch guard
- `NET-04` typed transport failure / timeout / queue-full
- `NET-05` redacted route trace / observer-safe network debug

これらの name は **working sample ID** です。
規範正本ではなく、exact transport schema / final command surface はまだ `OPEN` です。

## current evidence anchors

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
```

## intended reading

- `NET-01`
  current `local_queue` envelope から loopback transport seam へ widening する最小線
- `NET-02`
  process boundary をまたいでも publish / witness / handoff の順序と evidence path を崩さない線
- `NET-03`
  reconnect 後も `membership_epoch` / `member_incarnation` を hidden repair しない線
- `NET-04`
  timeout / queue-full / route-not-found / detach-after-send を typed failure family として返す線
- `NET-05`
  route trace / telemetry を observer-safe redaction 付きで出す線

## stop line

- concrete socket / broker / QUIC / WebRTC choice
- final public transport ABI / FFI
- cryptographic session / signature protocol
- multi-server consensus / durable distributed commit
