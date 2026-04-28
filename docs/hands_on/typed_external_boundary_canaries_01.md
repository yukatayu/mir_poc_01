# typed external boundary canaries 01

## この文書の役割

この文書は、phase 9 `Typed external boundary / adapter` の
**current synthetic preview helper cut** を最短コマンドで追うための hands-on landing page です。

- final public adapter API ではありません
- browser / network / VR host schema ではありません
- helper-local preview を public contract と混同しません

## まず実行するコマンド

```bash
python3 scripts/typed_external_boundary_samples.py list
python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 -m unittest scripts.tests.test_typed_external_boundary_samples
```

## current synthetic preview subset

- `EXT-03`
  local queue room-message synthetic preview lane。
  effect boundary、transport envelope、auth evidence、witness refs を collapse しない。
  current `host_boundary` preview では `request_lane = typed_effect_request`、`receipt_lane = typed_effect_receipt`、`visualization_lane = redacted_observer_view` を返す。
- `EXT-04`
  typed adapter failure synthetic preview lane。
  adapter failure を hidden retry や domain rejection に潰さない。
  current `host_boundary` preview では `failure_lane = typed_adapter_failure` を返す。

## current host-boundary preview inventory

- `host_boundary_scope`
  `helper_local_synthetic_preview`
- `host_boundary_lanes`
  `request / receipt / failure / visualization`
- `non_collapse_lanes`
  `transport / auth / membership / capability / witness / visualization`
- `host_family_gates`
  `final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`

per-sample `run EXT-03` / `run EXT-04` が返すのは `host_boundary` です。aggregate な `host_boundary_scope` / `host_boundary_lanes` / `host_family_gates` / `non_collapse_lanes` は helper closeout が返します。どちらも final public adapter ABI や exact host schema ではありません。

## residual planned family

- `EXT-01`
  local console scenario。
  stdio builtin 誤読を避けるため residual planned に保つ。
  current indirect anchor は clean near-end `05_delegated_rng_service` の `provider_boundary` である。
  reopen 条件は、typed adapter request / receipt を `provider_boundary` anchor 上に載せても
  Mir core standard I/O primitive と誤読されないことを `summary` / `envelopes` で示せること。
- `EXT-02`
  world overlay scenario。
  projection / visualization split と結びつくため residual planned に保つ。
  current indirect anchor は Sugoroku `visualization_views` / `telemetry_rows` と projection preview floor である。
  reopen 条件は、label / authority / redaction を保った overlay route を final host family split なしで記述できること。
- `EXT-05`
  standalone visualization restriction scenario。
  current helper cut では `EXT-03` の visualization view に吸収する。
  current indirect anchor は `EXT-03` の `visualization_view` と Sugoroku `visualization_views` である。
  reopen 条件は、standalone sample にすることで `EXT-03` では示せない redaction / authority case が増えること。

## residual family の indirect anchors

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
```

この 4 本は residual family 自体の direct execution ではなく、`EXT-01` / `EXT-02` / `EXT-05` が
依存する typed envelope / projection / visualization / provider-boundary anchor の current evidence です。

## これで確認できること

- standard I/O を Mir core primitive にせず、typed effect / adapter boundary に残す current reading
- local queue lane で、effect boundary、transport envelope、auth evidence、witness refs を separate に保つ synthetic preview helper evidence
- helper-local `host_boundary` preview inventory により request / receipt / failure / visualization split を explicit に保つ evidence
- typed adapter failure を explicit result として返し、hidden retry や hidden repair をしない synthetic preview helper evidence
- visualization も effect であり、label / authority / redaction を持つ synthetic preview helper evidence
- `EXT-01` / `EXT-02` / `EXT-05` が residual planned family のままでも、どの indirect anchor に依存しているかは current repo で確認できること

## これではまだ確認できないこと

- final public adapter API / FFI
- exact host schema
- browser / network / VR host family split
- real transport widening
- final public visualization service contract
- phase 9 `.mir` files の direct semantic execution

## current anchor

- `../../plan/25-typed-external-boundary-executable-roadmap.md`
- `../../samples/not_implemented/typed-external-boundary/README.md`
- `visual_debugger_viewer_01.md`
