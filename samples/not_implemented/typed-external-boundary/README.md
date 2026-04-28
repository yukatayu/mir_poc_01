# typed-external-boundary planned family

この directory は phase 9 `Typed external boundary / adapter` の **planned source family** です。

- current repo で runnable なのは
  `scripts/typed_external_boundary_samples.py` による **synthetic preview helper subset**
  `EXT-03` / `EXT-04` だけです。
  `P12` current first-cut closeout では、per-sample run が `host_boundary` split を返し、helper closeout が aggregate `host_boundary` preview inventory を返します。
- その helper はこの directory の source stub を参照しますが、
  current parser / runtime がこれらの `.mir` を直接 semantic execution しているわけではありません。
- `EXT-01` / `EXT-02` / `EXT-05` は current parser / runner ではまだ扱いません。
- final public adapter API、browser/network/VR host schema、real transport 実装を意味しません。
- current repo では、`provider_boundary` と Sugoroku helper の `local_queue` carrier を
  docs-first evidence anchor として使います。

## planned source family

- `EXT-01` `LogText` adapter local console
- `EXT-02` `ShowFloatingText` world overlay
- `EXT-03` `SendRoomMessage` local queue
- `EXT-04` adapter failure typed result
- `EXT-05` debug visualization label restriction

これらの name は 2026-04-24 handoff に基づく **working sample ID / scenario label** です。
final effect 名、exact host schema、final adapter contract はまだ `OPEN` です。
effect boundary、transport envelope、auth evidence は collapse せずに分けて読みます。

## current evidence anchors

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
```

## intended reading

- `EXT-01`
  typed `LogText` scenario を local console adapter に route する最小線。
  stdio builtin 誤読を避けるため、current phase 9 helper cut では residual planned に保つ。
  current indirect anchor は clean near-end `provider_boundary` / `provider_boundary_dispatch` である。
  reopen には、typed adapter request / receipt を `provider_boundary` 上に載せても
  Mir core standard I/O primitive と誤読されないことを `summary` / `envelopes` で示す必要がある。
- `EXT-02`
  host/world overlay 側へ floating text を出すが、visualization label / authority / redaction を保つ線
  current indirect anchor は Sugoroku `visualization_views` / `telemetry_rows` と projection preview floor である。
  reopen には、label / authority / redaction を保った overlay route を final host family split なしで記述できる必要がある。
- `EXT-03`
  room-level message publication を local queue の message carrier として扱う線。
  current synthetic preview helper subset の first positive lane。
- `EXT-04`
  adapter failure を typed result / explicit failure reason として返す線。
  current synthetic preview helper subset の typed negative lane。
- `EXT-05`
  debug / visualization 出力でも label restriction を破らない線。
  current helper cut では `EXT-03` の visualization view に吸収し、standalone sample としては residual planned に保つ。
  current indirect anchor は `EXT-03` `visualization_view` と Sugoroku `visualization_views` である。
  reopen には、standalone sample にすることで `EXT-03` では示せない redaction / authority case が増える必要がある。

## current synthetic preview subset

- `EXT-03`
  `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json`
  `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json`
- `EXT-04`
  `python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json`
  `python3 scripts/typed_external_boundary_samples.py run EXT-04 --format json`

helper closeout では、`host_boundary_scope = helper_local_synthetic_preview`、
`host_boundary_lanes = request / receipt / failure / visualization`、
`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、
`host_family_gates = final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`
を evidence-oriented inventory として読めます。per-sample run result が返す aggregate field は `host_boundary` のみです。

この subset は helper self-consistency と evidence-shape preview を確認するための thin facade です。
phase 9 `.mir` files の direct semantic execution ではありません。

## stop line

- concrete browser/network/VR adapter 実装
- final public serialization schema
- final public adapter ABI / FFI
- multi-process / real transport widening
