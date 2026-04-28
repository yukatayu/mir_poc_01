# plan/26 — visual debugger / viewer roadmap

## 目的

この文書は `P16` visual debugger / viewer first public prototype の
**current repository memory** を整理する。

- 規範正本ではない
- final public viewer API を固定する文書ではない
- helper-local / report-local typed inventory を public prototype boundary へ
  どう widen するかを current line として保つ

## current status

- `P7` で visualization / telemetry security envelope 自体は close 済み
- `P16` current first cut の主眼は、新しい untyped debug leak を増やすことではなく、
  既存 helper/runtime surface を **typed panel / typed telemetry** として
  public-prototype shape へ正規化すること
- current public prototype helper は `scripts/visual_debugger_viewer_samples.py`
- `P16` current first-cut closeout 自体は close 済みであり、
  repo-global queue は `P17` / `P18` を経て
  post-`P18` true user-spec hold line へ進んでいる。
  この文書は viewer 固有 memory に留め、
  repo-global queue 正本は `progress.md`、`tasks.md`、
  `plan/27-public-api-parser-gate-roadmap.md` を参照する

## current input surfaces

### helper-local

- `scripts/sugoroku_world_samples.py`
  - `visualization_views`
  - `telemetry_rows`
  - `--debug visualization`
  - `closeout`
- `scripts/network_transport_samples.py`
  - `NET-05`
  - `observer_route_trace`
  - `visualization_view`
  - `--debug route-trace`
- `scripts/typed_external_boundary_samples.py`
  - `EXT-03`
  - `host_boundary`
  - `route_trace`
  - `visualization_view`

### report-local / runtime canonical

- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - `visualization_views`
  - `telemetry_rows`
  - `visualization_view_lanes`
  - `telemetry_row_lanes`
  - `retention_scope_names`

## current prototype boundary

current first cut は browser UI や IDE integration ではなく、
**typed public prototype inventory** として読む。

- helper/local preview をそのまま final public viewer API と呼ばない
- report-local canonical inventory を final public telemetry service と呼ばない
- projection preview、host-boundary preview、route-trace canary を
  later package の public contract と混同しない

prototype scope 名:

```text
first_public_prototype_over_typed_inventories
```

prototype boundary wording:

```text
typed public prototype inventory over helper/runtime surfaces;
not a final public viewer API
```

## current normalized schema

### panel lanes

- `panel_id`
- `panel_kind`
- `label`
- `authority`
- `redaction`
- `retention_scope`
- `source_refs`
- `focus_refs`
- `notes`

### telemetry lanes

- `telemetry_id`
- `telemetry_kind`
- `label`
- `authority`
- `redaction`
- `retention_scope`
- `source_refs`
- `channel`
- `value_summary`
- `notes`

## current actualized subset

### actualized panel families

- `turn_timeline`
- `message_route`
- `verification_summary`
- `projection_view`
- `membership_snapshot`
- `hotplug_lifecycle`
- `route_trace`
- `audit_trace`

### actualized telemetry families

- `message_dispatch`
- `published_roll`
- `membership_update`
- `history_visibility`
- `model_check_summary`
- `hotplug_activation`
- `hotplug_detach`
- `route_hop`
- runtime `message_dispatch`
- typed external `typed_effect_request` / `typed_effect_receipt`

## validation floor

```bash
python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples
python3 scripts/visual_debugger_viewer_samples.py list --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
```

## kept-later gates

- `final_public_viewer_api`
- `final_public_visualization_schema`
- `final_public_telemetry_schema`
- `retention_policy`
- `multi_tenant_telemetry_service`
- `event_dag_runtime`
- `place_graph_view`
- `effect_route_graph_view`
- `proof_obligation_graph_view`
- `witness_timeline_view`
- `performance_telemetry_service`
- `ide_embedding`

## adjacent boundaries

- projection / placement:
  `plan/20-projection-and-placement-roadmap.md`
- typed external / host boundary:
  `plan/25-typed-external-boundary-executable-roadmap.md`
- helper stack / boundary wording:
  `plan/09-helper-stack-and-responsibility-map.md`
  `plan/14-glossary-and-boundary-rules.md`

## stop line

- `mir_hilight.html` を `P16` public prototype と言わない
- helper-local debug pretty print を final public viewer API と呼ばない
- report-local canonical inventory を production telemetry backend と呼ばない
- `projection_view` / `cross_place_projection` を final public projection viewer と呼ばない
- `host_boundary` / `route_trace` canary を final console / overlay / viewer contract と呼ばない
