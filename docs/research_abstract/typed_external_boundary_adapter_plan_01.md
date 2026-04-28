# typed external boundary / adapter plan 01

## 目的

phase 9 `Typed external boundary / adapter` を、repo-local current layer で
安全に前進させるための docs-first sample ladder を置きます。

ここで固定するのは **planned sample ID / synthetic preview helper subset / evidence anchor / stop line** です。
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

## current executable cut

- synthetic preview helper subset は `EXT-03` / `EXT-04`
- residual planned family は `EXT-01` / `EXT-02` / `EXT-05`
- helper entrypoint:
  `python3 scripts/typed_external_boundary_samples.py`
- debug modes:
  `summary`
  `envelopes`
  `visualization`
  `failures`
- scenario label は helper-local working name であり、final effect 名ではない

## current rule

- standard I/O は Mir core primitive にしない
- adapter boundary で external world へ接続する
- effect boundary / transport / auth / membership / capability / witness / visualization を separate lane に保つ
- adapter failure は typed result / explicit failure reason として表す

## residual planned family matrix

| Sample | Current status | Current indirect anchor | Reopen condition | Kept-later gate |
|---|---|---|---|---|
| `EXT-01` | residual planned | clean near-end `provider_boundary` / `provider_boundary_dispatch` | typed adapter request / receipt を `provider_boundary` 上に載せても stdio builtin 誤読にならないことを `summary` / `envelopes` で示せる | `final_console_schema`, final public adapter API |
| `EXT-02` | residual planned | Sugoroku `visualization_views` / `telemetry_rows` + projection preview floor | label / authority / redaction を保った overlay route を host family split なしで記述できる | `final_host_schema`, browser / network / VR family split, final projection public API |
| `EXT-05` | residual planned | `EXT-03` `visualization_view` + Sugoroku `visualization_views` | standalone sample が `EXT-03` では示せない redaction / authority case を増やす | `final_visualization_schema`, public visualization service contract |

## 次

この文書の current role は、phase 9 planned family、`EXT-03` / `EXT-04` synthetic preview helper subset、evidence anchor、residual reopen criterion を保つことです。
具体的な promoted queue は `tasks.md` と `docs/research_abstract/mirrorea_future_axis_01.md` を参照してください。
