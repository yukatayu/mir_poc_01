# 0954 — P7 visualization security hardening

## Objective

`P7` `VisualizationProtocol / VisualizationSecurity` hardening として、helper/runtime の view / telemetry carrier を security boundary として tighten し、current snapshot / queue / dashboard / reader-facing docs を `P8` promoted line に同期する。

## Scope and assumptions

- current scope は repo-local current layer / helper-local preview / report-local inventory の hardening である。
- final public viewer contract、retention policy、multi-tenant telemetry service、cross-place visualization public schema はこの task では固定しない。
- `transport` compatibility alias は current closeout で維持されるが、consumer が見る primary lane は `transport_medium` / `transport_seam` とする。

## Documents consulted

- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
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
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/network_transport_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- helper/runtime test files under `scripts/tests/` and `crates/mir-runtime/tests/`

## Actions taken

1. Reviewed the current P7 code/doc drift against the handoff, `tasks.md`, `samples_progress.md`, and the relevant plan/spec mirrors.
2. Kept the TDD path already in progress and confirmed the helper/runtime carriers now expose `label` / `authority` / `redaction` / `retention_scope` / `source_refs`.
3. Fixed the NET-05 observer route-trace path so observer-facing formatting is fail-closed and does not fall back to raw principal/auth/freshness detail.
4. Added helper/runtime closeout inventory for retention scope and security-lane visibility.
5. Advanced snapshot docs and queue state from `P7` promoted / `P8` reopen to `P8` promoted / `P9` reopen.
6. Added this report and prepared the next-package handoff toward `P8` Sugoroku runtime attach hardening.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/network_transport_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `scripts/tests/test_network_transport_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`

## Evidence / outputs / test results

- `df -h .`
  - `/dev/vda2` 99G total, 32G free
- `free -h`
  - `960Mi` memory, `386Mi` available at probe time
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - pass; helper view / telemetry carrier exposes `retention_scope = helper_local_ephemeral` and `source_refs`
- `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
  - pass; observer route trace exposes `authority` / `retention_scope` / `source_refs` and omits raw principal/auth/freshness/witness detail
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass; helper closeout exposes `retention_scope_names = ["helper_local_ephemeral"]`
- `python3 scripts/network_transport_samples.py closeout --format json`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass; runtime provider-boundary view carries the P7 security envelope
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
  - pass; runtime audit-trace view / telemetry row carries the P7 security envelope
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass; runtime closeout exposes visualization / telemetry lane inventories plus `retention_scope_names = ["report_local_inventory"]`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  - pass; `36/36`
- `python3 -m unittest scripts.tests.test_network_transport_samples -v`
  - pass; `10/10`, includes fail-closed regression coverage when `observer_route_trace` is absent
- `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass; `26/26`
- `cargo test -p mir-runtime`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass; required hierarchy intact after doc/report sync
- `python3 scripts/validate_docs.py`
  - pass; front-door / snapshot scaffolding remains complete after queue promotion
- `git diff --check`
  - pass; whitespace-clean after final doc/report sync

## What changed in understanding

- P7 should be read as security-boundary hardening, not as “viewer feature work”.
- Typed telemetry belongs inside the same label / authority / redaction / retention conversation as visualization.
- The meaningful stop line is not just “public viewer deferred”; it is specifically the deferral of final public schema, retention policy, and multi-tenant telemetry service.
- The next promoted line can now move to `P8` because the remaining visualization gap is no longer current-scope runtime/helper drift but later public-surface design.

## Open questions

- What should the final public visualization schema look like across helper-local preview, report-local inventory, and later emitted place-specific viewer surfaces?
- How should retention policy graduate from current `helper_local_ephemeral` / `report_local_inventory` floor to a multi-tenant or durable telemetry service?
- At what later package should cross-place projection viewer composition become part of a public visualization contract?

## Suggested next prompt

`P8` Sugoroku runtime attach hardening を進め、attach / membership / handoff / late join / detach TODO boundary の residual drift、world sugar wording、membership registry contract、hot-plug lifecycle stop line を helper/runtime/docs/report で同期してください。validation は Sugoroku helper closeout、`09_detach_todo --debug hotplug`、関連 unittest、source-hierarchy/doc checks を最低線にしてください。
