# typed external boundary / adapter plan 01

## 目的

phase 9 `Typed external boundary / adapter` を、repo-local current layer で
安全に前進させるための docs-first sample ladder を置きます。

ここで固定するのは **planned sample ID / evidence anchor / stop line** です。
final public adapter API や real transport 実装ではありません。

## current evidence anchors

- clean near-end `05_delegated_rng_service`
  - `provider_boundary`
  - `provider_receipt`
  - `provider_boundary_redacted_flow`
  - `provider_boundary_dispatch`
- Sugoroku helper `03_roll_publish_handoff`
  - `local_queue`
  - `message_envelopes`
  - `visualization_views`
  - `telemetry_rows`

## planned family

- `EXT-01` `LogText` adapter local console
- `EXT-02` `ShowFloatingText` world overlay
- `EXT-03` `SendRoomMessage` local queue
- `EXT-04` adapter failure typed result
- `EXT-05` debug visualization label restriction

これらは current repo の working sample ID です。
exact host schema / final adapter contract は `OPEN` のままです。

## current rule

- standard I/O は Mir core primitive にしない
- adapter boundary で external world へ接続する
- transport / auth / membership / capability / witness / visualization を separate lane に保つ
- adapter failure は typed result / explicit failure reason として表す

## 次

次の promoted package は projection / placement です。
