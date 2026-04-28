# plan/25 — typed external boundary executable roadmap

## 目的

phase 9 `Typed external boundary / adapter` を、
**repo-local synthetic preview helper widening** と
**final public host-facing gate** に分けて読むための repository memory を置く。

ここで固定するのは、current synthetic preview helper subset、evidence anchor、
debug surface、residual planned family の reopen criterion、stop line である。
final public adapter API、exact host schema、browser / engine / VR family への配分は固定しない。

2026-04-28 note:
`P2` residual planned family review では、
`EXT-01` / `EXT-02` / `EXT-05` を active executable sample に昇格させるのではなく、
**indirect anchor / reopen criterion / kept-later gate** を repository memory と helper closeout に固定する。
これは host-facing adapter implementation closeout ではない。

2026-04-28 P12 first-cut note:
current repo では、typed external helper subset / closeout に
`host_boundary_scope = helper_local_synthetic_preview`、
`host_boundary_lanes = request / receipt / failure / visualization`、
`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、
`host_family_gates = final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`、
`host_boundary_inventory`
を actualize してよい。
これは helper-local inventory hardening であり、crate-side adapter ABI / FFI actualization ではない。

## current executable cut

- current helper-local executable widening は `scripts/typed_external_boundary_samples.py`
  を thin facade として置く。
- この helper は新しい意味論を足さず、planned source stub と既存 anchor を phase 9 wording で束ねる synthetic preview helper である。
- current synthetic preview subset は `EXT-03` / `EXT-04` だけである。
  - `EXT-03`
    `SendRoomMessage` local queue synthetic preview lane。
    effect boundary、transport envelope、auth evidence、witness refs の non-collapse を helper self-consistency と anchor comparison で確認する。
  - `EXT-04`
    typed adapter failure synthetic preview lane。
    adapter failure を domain rejection や hidden retry に潰さず、typed failure reason を explicit に残す。
- current residual planned family は `EXT-01` / `EXT-02` / `EXT-05` である。
  - `EXT-01`
    local console scenario。
    stdio builtin 誤読を避けるため current helper cut では residual planned に保つ。
    - current indirect anchor:
      clean near-end `05_delegated_rng_service` の `provider_boundary` / `provider_boundary_dispatch`
    - reopen criterion:
      typed adapter request / receipt evidence を `provider_boundary` anchor 上に載せても、
      Mir core standard I/O primitive と誤読されないことを summary / envelopes で示せること
    - kept-later gate:
      `final_console_schema`、`final_public_adapter_api`
  - `EXT-02`
    world overlay scenario。
    projection / visualization split と結びつくため current helper cut では residual planned に保つ。
    - current indirect anchor:
      Sugoroku `03_roll_publish_handoff` の `visualization_views` / `telemetry_rows` と projection preview floor
    - reopen criterion:
      label / authority / redaction を保った overlay route を、
      final host family split や emitted-program contract を固定せずに記述できること
    - kept-later gate:
      `final_host_schema`、browser / network / VR host family split、final projection public API
  - `EXT-05`
    standalone visualization restriction scenario。
    current helper cut では `EXT-03` の visualization view に吸収し、standalone sample としては residual planned に保つ。
    - current indirect anchor:
      `EXT-03` の `visualization_view` と Sugoroku `visualization_views`
    - reopen criterion:
      standalone sample にすることで `EXT-03` では示せない redaction / authority case が増えること
    - kept-later gate:
      `final_visualization_schema`、public visualization service contract

## current host-boundary preview inventory

- run result `EXT-03` / `EXT-04` は `host_boundary` を返してよい。
  - `scope`
  - `adapter_entry`
  - `request_lane`
  - `receipt_lane`
  - `failure_lane`
  - `visualization_lane`
  - `non_collapse_lanes`
- helper closeout は次を返してよい。
  - `host_boundary_scope`
  - `host_boundary_lanes`
  - `non_collapse_lanes`
  - `host_family_gates`
  - `host_boundary_inventory`
- current reading は、host-facing adapter seam の最小 inventory を helper-local evidence として actualize した、というところで止める。
- `engine-abi` は placeholder のままに保ち、この package で crate-side adapter surface を claim しない。

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
- typed external helper
  - `samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir`
  - `samples/not_implemented/typed-external-boundary/04_adapter_failure_typed_result.mir`
  - `scripts/typed_external_boundary_samples.py`
  - helper closeout `residual_review_matrix`

## helper-local debug surface

- `summary`
  - sample / adapter / transport seam の最短要約
- `envelopes`
  - envelope id、transport、auth evidence、membership freshness、capability requirements、witness refs を見る
- `visualization`
  - label / authority / redaction を見る
- `failures`
  - typed adapter failure family、retryable/terminal、domain mutation committed の有無を見る

これらは helper-local synthetic preview であり、phase 9 `.mir` の direct semantic execution や final public adapter / visualization schema を意味しない。

## residual review matrix

- `EXT-01`
  - anchor kind:
    `provider_boundary`
  - expected debug surface:
    `summary`, `envelopes`
- `EXT-02`
  - anchor kind:
    `visualization_projection_bridge`
  - expected debug surface:
    `summary`, `visualization`
- `EXT-05`
  - anchor kind:
    `visualization_redaction_lane`
  - expected debug surface:
    `visualization`

## current command set

```bash
python3 scripts/typed_external_boundary_samples.py list
python3 scripts/typed_external_boundary_samples.py check-all
python3 scripts/typed_external_boundary_samples.py closeout
python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 -m unittest scripts.tests.test_typed_external_boundary_samples
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
```

## current decision

- standard I/O は Mir core primitive にしない。
- external world とは typed effect / adapter boundary で接続する。
- effect boundary、transport envelope、authentication / authorization / membership / capability / witness を collapse しない。
- helper-local `host_boundary` preview inventory により request / receipt / failure / visualization split を explicit に保つ。
- scenario label は helper-local working sample ID であり、final effect 名ではない。
- visualization も effect なので label / authority / redaction を保つ。

## mixed gate

- final public adapter API / FFI
- exact host schema
- browser / network / VR host family split
- final public `AuthEvidence` kind
- real transport widening
- final public visualization service contract

## stop line

- current helper cut を installed binary / final public host-facing adapter と読まない
- `P2` close を external adapter implementation complete と読まない
- local console / overlay / viewer schema を既成事実化しない
- transport / auth / membership / witness / visualization を 1 lane に潰さない
- phase 9 helper-local sample ID を final public effect name として固定しない
