# plan/25 — typed external boundary executable roadmap

## 目的

phase 9 `Typed external boundary / adapter` を、
**repo-local synthetic preview helper widening** と
**final public host-facing gate** に分けて読むための repository memory を置く。

ここで固定するのは、current synthetic preview helper subset、evidence anchor、
debug surface、stop line である。
final public adapter API、exact host schema、browser / engine / VR family への配分は固定しない。

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
  - `EXT-02`
    world overlay scenario。
    projection / visualization split と結びつくため current helper cut では residual planned に保つ。
  - `EXT-05`
    standalone visualization restriction scenario。
    current helper cut では `EXT-03` の visualization view に吸収し、standalone sample としては residual planned に保つ。

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

## current command set

```bash
python3 scripts/typed_external_boundary_samples.py list
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 -m unittest scripts.tests.test_typed_external_boundary_samples
```

## current decision

- standard I/O は Mir core primitive にしない。
- external world とは typed effect / adapter boundary で接続する。
- effect boundary、transport envelope、authentication / authorization / membership / capability / witness を collapse しない。
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
- local console / overlay / viewer schema を既成事実化しない
- transport / auth / membership / witness / visualization を 1 lane に潰さない
- phase 9 helper-local sample ID を final public effect name として固定しない
