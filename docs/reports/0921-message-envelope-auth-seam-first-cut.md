# 0921 MessageEnvelope / Auth seam first cut

## Objective

`MessageEnvelope / Auth seam` package を repo-local current layer で close する。
transport、authentication、authorization、membership、capability、witness を 1 つの opaque tunnel に潰さず、
helper-local / report-local first cut として visible にする。

## Scope and assumptions

- current scope は **repo-local helper / report-local evidence carrier** に限る。
- current auth baseline は `auth none` とする。
- final public `AuthEvidence` kind、session / signature protocol、real network transport はこの task で固定しない。
- `samples/clean-near-end/sugoroku-world/` と `crates/mir-runtime` の current runnable path を壊さないことを優先する。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`
- `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`

## Actions taken

1. Sugoroku helper に `PrincipalClaim`、`AuthEvidence`、`MessageEnvelope` を追加した。
2. `scripts/sugoroku_world_samples.py` に `message_envelopes` payload と `--debug envelopes` を追加した。
3. helper 側の current cut では `local_queue` transport、`auth none`、`membership_epoch` / `member_incarnation`、`capability_requirements`、`authorization_checks`、`witness_refs` を separate lane で保持した。
4. clean near-end runtime report-local lane に `MessageEnvelope` inventory を追加した。
   - `05_delegated_rng_service` は `provider_boundary`
   - `06_auditable_authority_witness` は `audit_trace_boundary`
5. clean near-end closeout に `message_envelope_lanes`、`auth_evidence_kinds`、`transport_seams` を追加した。
6. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`specs/10`、`specs/11`、`plan/01`、`plan/09`、`plan/11`、`plan/12`、`plan/16`、reader-facing docs を同期した。
7. hands-on note `docs/research_abstract/hands_on_sugoroku_07_message_envelope_auth.md` を追加した。

## Files changed

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
- `plan/16-shared-space-membership-and-example-boundary.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/hands_on_sugoroku_07_message_envelope_auth.md`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass, `Ran 21 tests`
- `python3 scripts/sugoroku_world_samples.py check-all`
  - pass, `sample_count = 10`, `failed = []`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  - pass, `message_envelopes` に `roll_request#1` と `handoff_notice#1`
  - helper-local current cut は `transport = local_queue`, `auth_evidence = null`, `membership_epoch`, `member_incarnation`, `capability_requirements`, `authorization_checks`, `witness_refs` を出力
- `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass, `21 passed`
- `cargo test -p mir-runtime`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass, `message_envelopes` に `provider_request#1` と `provider_receipt#1`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass, `message_envelope_lanes`, `auth_evidence_kinds = [none]`, `transport_seams = [provider_boundary, audit_trace_boundary]`
- `python3 scripts/check_source_hierarchy.py`
  - pass, `required 23 / present 23 / missing 0`
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- `membership_epoch` / `member_incarnation` は freshness carrier であり、auth/session proof そのものではないことを current carrier で明示できた。
- `provider_receipt`、`draw_pub`、`AuthorityDrawWitness` は witness/evidence であり、authentication ではないことを helper/runtime の両 lane で明示できた。
- shared-space helper の `local_queue` baseline と clean near-end runtime の `provider_boundary` / `audit_trace_boundary` は、どちらも final public transport contract ではなく current report-local seam として扱うのが安全である。

## Open questions

- final public `AuthEvidence` kind を `session_token` / `signature` / 他の形へどう widen するか。
- real network transport と `MessageEnvelope` をどう接続するか。
- helper-local `message_envelopes` を `VisualizationProtocol` の view / telemetry surface にどう持ち上げるか。
- session / signature / federation / audit retention をどの package で reopen するか。

## Suggested next prompt

`VisualizationProtocol` first implementation を進めてください。Sugoroku helper の `summary` / `turn-trace` / `membership` / `verification` / `signatures` / `envelopes` と、clean near-end runtime closeout の `layer_signatures` / `message_envelopes` を参照する thin protocol を置き、typed telemetry / redaction-aware view の helper-local / report-local first cut を追加してください。
