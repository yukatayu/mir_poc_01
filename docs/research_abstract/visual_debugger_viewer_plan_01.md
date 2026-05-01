# visual debugger / viewer plan 01

## 要約

`P16` の current first cut は、new browser UI を final public product として出すことではなく、
existing helper/runtime inventory を **typed public prototype inventory** として
まとめ直すことです。

current scope では `scripts/visual_debugger_viewer_samples.py` が
次を同じ schema で読みます。

- Sugoroku helper `visualization_views` / `telemetry_rows`
- Sugoroku closeout catalog
- `NET-05` observer-safe `route_trace`
- clean near-end runtime canonical inventory
- typed external `EXT-03` host-boundary preview

## current boundary

current prototype scope:

```text
first_public_prototype_over_typed_inventories
```

current prototype boundary:

```text
typed public prototype inventory over helper/runtime surfaces;
not a final public viewer API
```

これにより、public prototype と言っても
helper-local debug pretty print や report-local inventory を
そのまま final public contract に昇格させないことを明示します。

## actualized subset

### panel

- `turn_timeline`
- `message_route`
- `verification_summary`
- `projection_view`
- `membership_snapshot`
- `hotplug_lifecycle`
- `route_trace`
- `audit_trace`

### telemetry

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

## stop line

この package で fixed にしないもの:

- final public viewer API
- final public visualization / telemetry schema
- retention policy
- multi-tenant telemetry service
- Event DAG / place graph / effect route graph / proof obligation graph
- witness timeline / performance telemetry service / IDE embedding
- `mir_hilight.html` の役割変更

## historical next line

`P16` current first cut close 時点では、
historical follow-up は `P17` storage / LLVM / backend preparation でした。
`P17`、`R7`、`P21` は historical closeout chain として残し、
live status / next reopen point は `../../progress.md` と `../../tasks.md` を参照します。

`P18` final public API / parser grammar gate は引き続き later mixed gate に残します。
