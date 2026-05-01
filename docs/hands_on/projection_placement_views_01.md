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
python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
find samples/generated -maxdepth 3 -type f | sort
```

`check-all` は committed manifest と live anchors の alignment validation です。
`closeout` は manifest inventory evidence であり、final emitted executable family、generated place-program synthesis、placement optimizer、deployment planner、equivalence checker / proof completion ではありません。

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

### generated artifact reserve

`find samples/generated -maxdepth 3 -type f | sort` は、
current repo が final emitted executable program family をまだ committed sample として持っていないこと、
`samples/generated/projection-placement/manifest.json` が committed generated bridge evidence であり、
`samples/generated/README.md` が reserve policy を示していることを確認するための current guard です。

### projection/codegen current first cut

`python3 scripts/projection_codegen_samples.py closeout --format json` では、少なくとも次を確認します。

- `projection_scope = generated_reserve_bridge_evidence`
- `artifact_boundary = committed manifest bridge evidence only; not a final emitted executable program`
- `P15-GEN-01..04`
- `generated_bridge_artifact_inventory`
- `generated_reserve_inventory`
- `equivalence_review_categories`
- `validation_floor`

ここで見ているのは、helper/runtime anchor と committed generated manifest の alignment であって、
final emitted executable family や optimizer / deployment planner ではありません。

## これで確認できること

- helper-local preview でも `Place` の split を明示できること
- projection / placement が visualization / envelope / membership frontier と接続して読めること
- provider placement と authority placement を distinct に保てること
- projection preview を final emitted place program と混同しない current stop line
- generated place-specific program family が reserve path であり、actual emitted place-specific program family は later package だと読めること
- `P15` current first cut が committed generated bridge evidence only であり、final emitted executable family を過大主張していないこと

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
- `visual_debugger_viewer_01.md`
- `current_phase_closeout_01.md`
- `../../docs/reports/0942-projection-placement-executable-widening.md`
- `../../docs/reports/0970-p15-projection-codegen-generated-bridge-first-cut-closeout.md`
