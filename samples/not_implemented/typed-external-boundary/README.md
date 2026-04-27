# typed-external-boundary planned family

この directory は phase 9 `Typed external boundary / adapter` の **planned skeleton family** です。

- current parser / runner ではまだ扱いません。
- final public adapter API、browser/network/VR host schema、real transport 実装を意味しません。
- current repo では、`provider_boundary` と Sugoroku helper の `local_queue` carrier を
  docs-first evidence anchor として使います。

## planned sample IDs

- `EXT-01` `LogText` adapter local console
- `EXT-02` `ShowFloatingText` world overlay
- `EXT-03` `SendRoomMessage` local queue
- `EXT-04` adapter failure typed result
- `EXT-05` debug visualization label restriction

これらの name は 2026-04-24 handoff に基づく **working sample ID** です。
規範正本ではなく、exact host schema / final adapter contract はまだ `OPEN` です。

## current evidence anchors

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
```

## intended reading

- `EXT-01`
  typed `LogText` effect を local console adapter に route する最小線
- `EXT-02`
  host/world overlay 側へ floating text を出すが、visualization label / authority / redaction を保つ線
- `EXT-03`
  room-level message publication を local queue の message carrier として扱う線
- `EXT-04`
  adapter failure を typed result / explicit failure reason として返す線
- `EXT-05`
  debug / visualization 出力でも label restriction を破らない線

## stop line

- concrete browser/network/VR adapter 実装
- final public serialization schema
- final public adapter ABI / FFI
- multi-process / real transport widening
