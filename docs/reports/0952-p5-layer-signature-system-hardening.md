# Report 0952 — P5 LayerSignature system hardening

## 1. Title and identifier

- Identifier: `0952`
- Title: `P5 LayerSignature system hardening`

## 2. Objective

`P5` `LayerSignature` system hardening を close し、helper/runtime の row schema、
closeout scope、inventory naming、`VerificationLayer` stop line を current repo へ同期する。

## 3. Scope and assumptions

- この package は final public layer law schema ではない。
- current scope は helper representative inventory と clean near-end canonical inventory の
  schema hardening と docs / snapshot / report synchronization に限る。
- `P6` `MessageEnvelope / Auth seam`、`P7` `VisualizationProtocol / VisualizationSecurity`、
  final public verifier contract、theorem bridge emitted layer widening は対象外とする。

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
- `docs/reports/0919-layer-signature-system-first-cut.md`
- `docs/reports/0950-p4-term-signature-registry-hardening.md`
- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`

## 5. Actions taken

- Sugoroku helper `LayerSignature` row key を `layer` から `name` に変更した。
- helper/runtime の current `LayerSignature` carrier を
  `name / requires / provides / transforms / checks / emits / obligations / laws`
  に揃えた。
- helper representative inventory を
  `verification_handoff_witness` / `runtime_turn_trace` /
  `membership_late_join_boundary` / `membership_leave_invalidation` /
  `membership_owner_reassignment` / `hotplug_activation_boundary` /
  `hotplug_detach_boundary`
  に整理した。
- runtime canonical inventory を
  `transport_provider_boundary` / `auth_authority_witness` /
  `verification_model_check`
  のまま維持し、`obligations` lane を追加した。
- helper closeout に `layer_signatures` aggregate、`layer_signature_scope`、
  `layer_signature_lanes`、`layer_signature_names`、`reserved_layer_signature_names`
  を追加した。
- clean near-end closeout に `layer_signature_scope` を追加し、
  `layer_signature_lanes` を `name` と `obligations` を含む shape に更新した。
- helper/runtime ともに closeout aggregate が duplicate `LayerSignature.name` drift を
  silent shadowing しないよう fail-closed merge guard を追加した。
- helper unittest と runtime tests に、`obligations` lane と duplicate-name drift
  rejection guard の regression test を追加した。
- `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、
  `specs/10`、`specs/11`、`plan/01`、`plan/09`、`plan/11`、`plan/12`、`plan/14`、
  `plan/17`、`plan/90`、reader-facing docs を同期し、`P5` close / `P6` next /
  `P7` reopen-next に揃えた。

### Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
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
- `docs/reports/0952-p5-layer-signature-system-hardening.md`

### Commands run

- `df -h .`
- `free -h`
- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_peterson_sc_pass --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `cargo test -p mir-runtime --test clean_near_end_samples`
- `cargo test -p mir-runtime`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 6. Evidence / outputs / test results

- helper focused sample:
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`
  - pass
  - evidence:
    - `layer_signatures[0].name = verification_handoff_witness`
    - `layer_signatures[0].obligations = [handoff_witness_schema_remains_sample_defined]`
    - `layer_signatures[1].name = runtime_turn_trace`
- helper closeout:
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - evidence:
    - `layer_signature_scope = representative_slice`
    - `layer_signature_lanes = [name, requires, provides, transforms, checks, emits, obligations, laws]`
    - `layer_signature_names` includes `verification_handoff_witness`, `membership_late_join_boundary`, `hotplug_activation_boundary`
- helper tests:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  - pass
  - evidence:
    - `Ran 36 tests`
    - `OK`
    - duplicate-name drift regression is covered
- runtime focused sample:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
  - evidence:
    - `layer_signatures[0].name = transport_provider_boundary`
    - `layer_signatures[0].obligations = [provider_returns_draw_not_room_commit]`
- runtime authority-witness sample:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
  - pass
  - evidence:
    - `layer_signatures[0].name = auth_authority_witness`
    - `layer_signatures[0].obligations = [authority_witness_preserves_subject_identity]`
- runtime verification sample:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_peterson_sc_pass --format json`
  - pass
  - evidence:
    - `layer_signatures[0].name = verification_model_check`
    - `layer_signatures[0].obligations = [peterson_sc_mutual_exclusion]`
- runtime closeout:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass
  - evidence:
    - `layer_signature_scope = clean_near_end_canonical_inventory`
    - `layer_signature_lanes = [name, requires, provides, transforms, checks, emits, obligations, laws]`
    - `layer_signatures` contains `auth_authority_witness`, `transport_provider_boundary`, `verification_model_check`
- runtime regression:
  - `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass
  - evidence:
    - `26 passed; 0 failed`
- broader runtime regression:
  - `cargo test -p mir-runtime`
  - pass
  - evidence:
    - integration tests across `current_l2_*` and `clean_near_end_samples` all pass
- docs / hierarchy / diff:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - all pass

## 7. What changed in understanding

- `LayerSignature` hardening の中心は inventory の完全統一ではなく、
  helper representative inventory と runtime canonical inventory の scope split を
  hidden drift ではなく explicit repository memory に変えることだった。
- `VerificationLayer` composition は current emitted floor と later widening line を
  分けて書かないと overclaim になる。現時点の emitted floor は
  helper `verification_handoff_witness` と runtime `verification_model_check` である。
- `LayerSignature.name` を authoritative key にするなら、closeout aggregate は
  duplicate-name conflict を fail-closed にしないと hardening の意味が薄れる。

## 8. Open questions

- UNRESOLVED: `LayerSignature` value grammar をどこまで typed prefix family に揃えるか。
- UNRESOLVED: helper representative inventory と runtime canonical inventory の scope split を
  どの package で解除し、shared public law vocabulary へ寄せるか。
- UNRESOLVED: theorem bridge / runtime policy / visualization lane を
  emitted `VerificationLayer` inventory に上げる widening threshold。

## 9. Suggested next prompt

`P6` `MessageEnvelope / Auth seam` hardening を進め、helper/runtime closeout の
`auth_evidence` / `transport_seams` / `membership_epoch` / `member_incarnation` /
`capability_requirements` / `authorization_checks` / `witness_refs` を比較しつつ、
transport-auth collapse を起こさない current public stop line を `plan/09`、`plan/14`、
`progress.md`、`tasks.md`、`samples_progress.md`、new report に同期してください。
