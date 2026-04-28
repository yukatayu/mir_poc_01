# hands-on: Sugoroku VisualizationProtocol first cut

## 目的

この文書は、Sugoroku helper と clean near-end report / closeout に追加した
helper-local / report-local `VisualizationView` / `TelemetryRow` first cut を読むための短い hands-on です。

ここで見るのは **repo-local current layer の evidence carrier** です。
final public visualization API、multi-tenant telemetry service、retention policy を意味しません。

## 何を actualize したか

current first cut では、visualization と telemetry を untyped debug leak にせず、
少なくとも次を持つ record として見せます。

- view / row kind
- label
- authority
- redaction
- source refs
- summary / fields

Sugoroku helper では `visualization_views` と `telemetry_rows` を sample result に含め、
`--debug visualization` で evidence-oriented な pretty view を出します。

clean near-end report / closeout では、runtime-rich な current sample に対して
report-local `VisualizationView` / `TelemetryRow` inventory を返します。

`P16` current first-cut closeout では、これら helper/runtime surface を
`scripts/visual_debugger_viewer_samples.py` が `viewer_panel_lanes` /
`viewer_telemetry_lanes` に正規化し、typed public prototype inventory として読む入口を追加しました。
これは final public viewer API ではありません。

## 実行コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
```

## 読み方

`03_roll_publish_handoff` では、少なくとも次の helper-local view が見えます。

- `turn_timeline`
  - label は `helper:published-history`
  - authority は `ObservePublishedHistory(SugorokuGame#1)`
  - redaction は `published_history_only`
- `message_route`
  - label は `helper:local-queue`
  - authority は `InspectLocalQueue(SugorokuGame#1)`
  - redaction は `omit_auth_evidence_payload`
- `verification_summary`
  - verification property の summary だけを見せる

対応する telemetry row では、`roll_request#1` と `handoff_notice#1` が見えます。
ここでも membership freshness、dispatch outcome、published witness count などを
separate lane で読み、auth / witness / visualization を潰しません。

`05_late_join_history_visible` では、membership timeline が
`pending_turn_order_only` と `published_history_only` の redaction を伴って出ます。

## stop line

この package で fixed にしないもの:

- final public `VisualizationProtocol` schema
- final public `TelemetryRow` schema
- retention / export / tenancy policy
- cross-place projection viewer
- final IDE integration
- production trace service

## 関連

- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/visual_debugger_viewer_plan_01.md`
- `docs/hands_on/visual_debugger_viewer_01.md`
- `docs/reports/0922-visualization-protocol-first-cut.md`
