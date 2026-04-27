# projection placement views 01

## この文書の役割

この文書は、phase 12 `Projection / placement` の **current helper/report-local preview floor** を
最短で確認するための hands-on です。

- final public projection API ではありません
- final emitted place program ではありません
- helper-local / report-local evidence を current line として読むための入口です

## まず実行するコマンド

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
```

## 何を見るか

### Sugoroku helper `projection_view`

`03_roll_publish_handoff` の `projection_view` では、少なくとも次を確認します。

- `system_source = SugorokuWorldSource#1`
- `server_places = ["WorldServerPlace", "SugorokuGamePlace#1"]`
- `authority_place = SugorokuGamePlace#1`
- `participant_places = ["ParticipantPlace[Alice]", "ParticipantPlace[Bob]", "ParticipantPlace[Carol]"]`
- `adapter_transport = local_queue`
- `observer_views = ["turn_timeline", "message_route", "verification_summary"]`
- `membership_frontier`

ここで見ているのは、system-wide source を early server/client split に潰さず、
authority place / participant place / observer view refs を separate lane で読めることです。

### clean near-end runtime `cross_place_projection`

`05_delegated_rng_service` の report-local inventory では、`cross_place_projection` を見ます。

- authority placement と provider placement を collapse していないこと
- `provider_request#1` / `provider_receipt#1` message envelope refs
- `projection_summary_only` / `auth_evidence:none_baseline` redaction rule refs
- `projection:provider_boundary_draw_route`

ここで見ているのは、projection を provider-boundary lane と結びつけても、
transport / auth / witness / placement を 1 つの implicit carrier に潰していないことです。

## これで確認できること

- helper-local preview でも `Place` の split を明示できること
- projection / placement が visualization / envelope / membership frontier と接続して読めること
- provider placement と authority placement を distinct に保てること
- projection preview を final emitted place program と混同しない current stop line

## これではまだ確認できないこと

- final projection IR
- generated place-specific program emitter
- placement optimizer
- cross-place equivalence checker
- production scheduler / deployment planner
- real network transport 上の projection execution

## 関連

- `../../plan/20-projection-and-placement-roadmap.md`
- `../research_abstract/projection_placement_plan_01.md`
- `current_phase_closeout_01.md`
- `../../docs/reports/0942-projection-placement-executable-widening.md`
