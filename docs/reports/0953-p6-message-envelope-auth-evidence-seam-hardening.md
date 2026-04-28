# Report 0953 — P6 MessageEnvelope / AuthEvidence seam hardening

## 1. Title and identifier

- Identifier: `0953`
- Title: `P6 MessageEnvelope / AuthEvidence seam hardening`

## 2. Objective

`P6` `MessageEnvelope / AuthEvidence` seam hardening を close し、helper/runtime の
message/auth carrier を current scope で tighten する。

## 3. Scope and assumptions

- この package は final public `AuthEvidence` schema ではない。
- current scope は helper/runtime `message_envelope_scope`、`transport_medium` /
  `transport_seam` split、`emitter_principal`、`freshness_checks`、shared
  `auth_evidence_lanes` の hardening に限る。
- `session_token` / `signature`、real network transport、final public transport ABI、
  final public `witness_refs` role taxonomy は対象外とする。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0921-message-envelope-auth-seam-first-cut.md`
- `docs/reports/0952-p5-layer-signature-system-hardening.md`
- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`

## 5. Actions taken

- helper/runtime `MessageEnvelope` current carrier を `transport_medium` /
  `transport_seam` / `emitter_principal` / `freshness_checks` を含む shape に widen した。
- helper/runtime closeout に `message_envelope_scope` と shared
  `auth_evidence_lanes = kind / subject / issuer / bindings / notes` を追加した。
- helper closeout の `auth_evidence_modes` naming drift を `auth_evidence_kinds` へ揃え、
  compatibility alias は残した。
- helper active transport medium inventory と runtime canonical transport seam inventory を
  別 lane として固定した。
- helper/runtime/network helper で legacy `transport` alias を seam 意味へ正規化し、
  `transport_medium` を medium lane として分離した。
- helper sample と runtime sample で `principal_claim.claimed_capabilities` と
  `capability_requirements` が常に同値にならない representative evidence を追加した。
- membership freshness を `authorization_checks` free-form text に潰さず、
  `freshness_checks` lane に上げた。
- network helper canary `NET-02` / `NET-05` でも medium / seam split が subprocess JSON
  bridge と observer-safe route trace を越えて保持されるようにした。
- front-door docs / snapshot docs / plan / specs を同期し、`P6` close / `P7` next /
  `P8` reopen-next に揃えた。

### Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `scripts/network_transport_samples.py`
- `scripts/tests/test_network_transport_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0953-p6-message-envelope-auth-evidence-seam-hardening.md`

### Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `df -h .`
- `free -h`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/network_transport_samples.py run NET-02 --format json`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples.SugorokuWorldSamplesTests.test_roll_publish_handoff_exposes_message_envelopes scripts.tests.test_sugoroku_world_samples.SugorokuWorldSamplesTests.test_closeout_records_message_envelope_debug_mode scripts.tests.test_sugoroku_world_samples.SugorokuWorldSamplesTests.test_runtime_attach_loopback_transport_preserves_attach_request_parity`
- `python3 -m unittest scripts.tests.test_network_transport_samples.NetworkTransportSamplesTests.test_net_02_process_boundary_keeps_envelope_and_witness_trace scripts.tests.test_network_transport_samples.NetworkTransportSamplesTests.test_net_05_emits_redacted_observer_safe_route_trace`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `cargo test -p mir-runtime --test clean_near_end_samples clean_sample_delegated_rng_service_emits_message_envelopes`
- `cargo test -p mir-runtime --test clean_near_end_samples clean_sample_authority_witness_emits_auth_envelope`
- `cargo test -p mir-runtime --test clean_near_end_samples clean_near_end_closeout_records_message_envelope_inventory`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
- `python3 -m unittest scripts.tests.test_network_transport_samples -v`
- `cargo test -p mir-runtime --test clean_near_end_samples`
- `cargo test -p mir-runtime`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 6. Evidence / outputs / test results

- helper focused sample:
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --format json`
  - pass
  - evidence:
    - `roll_request#1.transport = game_action_boundary`
    - `roll_request#1.transport_medium = local_queue`
    - `roll_request#1.transport_seam = game_action_boundary`
    - `handoff_notice#1.emitter_principal = SugorokuGame#1`
    - `freshness_checks` is separate from `authorization_checks`
- helper closeout:
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - evidence:
    - `message_envelope_scope = representative_slice`
    - `auth_evidence_lanes = [kind, subject, issuer, bindings, notes]`
    - `transport_mediums = [local_queue, loopback_socket]`
    - `transport_seams` includes `attach_point_boundary`, `game_action_boundary`, `published_history_boundary`
- runtime focused sample:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
  - evidence:
    - `provider_request#1.transport_medium = null`
    - `provider_request#1.transport_seam = provider_boundary`
    - `provider_receipt#1.emitter_principal = AuthorityRng`
- network canary:
  - `python3 scripts/network_transport_samples.py run NET-02 --format json`
  - pass
  - evidence:
    - `route_trace[*].transport = transport_seam`
    - `route_trace[*].transport_medium = loopback_socket`
    - `route_trace[*].transport_seam` stays explicit across subprocess JSON bridge
- full validation:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v` pass
  - `python3 -m unittest scripts.tests.test_network_transport_samples -v` pass
  - `cargo test -p mir-runtime --test clean_near_end_samples` pass
  - `cargo test -p mir-runtime` pass
  - `python3 scripts/check_source_hierarchy.py` pass
  - `python3 scripts/validate_docs.py` pass
  - `git diff --check` pass

## 7. What changed in understanding

- `transport` という 1 field に medium と seam を載せたままでは、helper/runtime split は
  intentional でも carrier ontology drift が残る。current scope では medium/seam split を
  additive に入れるのが最も安全だった。
- `auth none` baseline のままでも、shared `AuthEvidence` lane inventory を先に固定しておくと
  later `session_token` / `signature` reserve path が fork しにくくなる。
- membership freshness と subject/emitter distinction は final public auth protocol を
  固めなくても current carrier の中で先に分けられる。

## 8. Open questions

- UNRESOLVED: final public `AuthEvidence` kind と session / signature protocol。
- UNRESOLVED: helper medium inventory と runtime seam inventory を actual transport binding へ
  どう接続するか。
- UNRESOLVED: `witness_refs` role taxonomy を `requires` / `carries` / `produces` / `history`
  にどの package で split するか。

## 9. Suggested next prompt

`P7` `VisualizationProtocol / VisualizationSecurity` hardening を進め、Sugoroku helper と
clean near-end runtime の `visualization_views` / `telemetry_rows` / route-trace redaction を
比較しつつ、label / authority / redaction / retention / typed telemetry の current stop line を
`plan/14`、`progress.md`、`tasks.md`、`samples_progress.md`、new report に同期してください。
