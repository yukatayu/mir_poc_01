# visual debugger / viewer 01

## この文書の役割

この文書は `P16` visual debugger / viewer first public prototype の
**current typed prototype surface** を最短で確認するための hands-on です。

- final public viewer API ではありません
- helper-local debug pretty print の言い換えでもありません
- typed panel / typed telemetry inventory を current line として読む入口です

## まず実行するコマンド

```bash
python3 scripts/visual_debugger_viewer_samples.py list --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
```

## backing anchors

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
```

## 何を見るか

### `P16-VIEW-01`

- Sugoroku helper `turn_timeline`
- Sugoroku helper `message_route`
- Sugoroku helper `verification_summary`
- Sugoroku helper `projection_view`

ここでは panel ごとに `authority` / `redaction` / `retention_scope` /
`source_refs` が残っていることを確認します。

### `P16-VIEW-02`

- Sugoroku closeout catalog
- `membership_snapshot`
- `hotplug_lifecycle`
- `hotplug_view_ids`
- `hotplug_telemetry_row_ids`

ここでは current closeout catalog が viewer prototype の backing inventory として
読めることを確認します。

### `P16-VIEW-03`

- `NET-05` observer-safe `route_trace`

ここでは route trace が fail-closed であり、
raw auth / capability payload に fallback していないことを確認します。

### `P16-VIEW-04`

- clean near-end runtime canonical inventory
- `authority_trace_redacted_view`
- `provider_boundary_redacted_flow`
- `cross_place_projection`

ここでは runtime canonical inventory 側の retention scope が
`report_local_inventory` に残っていることを確認します。

### `P16-VIEW-05`

- typed external `room_message_route`
- `host_boundary_scope`
- `non_collapse_lanes`

ここでは host-boundary preview が console / overlay / viewer contract を freeze せずに
viewer pressure を示せることを確認します。

## これで確認できること

- helper/runtime surface を同じ typed panel / telemetry lane で読めること
- `authority` / `redaction` / `retention_scope` / `source_refs` が
  public-prototype boundary の主軸に残っていること
- projection / host-boundary / hot-plug / route-trace が
  untyped debug leak ではなく typed inventory として合流していること

## これではまだ確認できないこと

- final public viewer API
- final public visualization / telemetry schema
- place graph / effect route graph / proof obligation graph
- Event DAG / witness timeline / performance telemetry service
- IDE embedding / multi-tenant telemetry backend

## 関連

- `../../plan/26-visual-debugger-viewer-roadmap.md`
- `../research_abstract/visual_debugger_viewer_plan_01.md`
- `current_phase_closeout_01.md`
- `network_transport_canaries_01.md`
- `projection_placement_views_01.md`
