# 0966 — P12 host-boundary preview inventory first cut

## Objective

`P12` external adapter / host boundary tranche の最小 safe package として、typed external helper subset / closeout に helper-local `host_boundary` preview inventory を actualize し、host-facing adapter seam の request / receipt / failure / visualization split を docs / dashboard / repository memory まで同期する。

## Scope and assumptions

- この package は helper-local inventory hardening であり、crate-side adapter ABI / FFI actualization ではない。
- `engine-abi` は placeholder のまま保つ。
- final public adapter ABI、exact host schema、browser / network / VR family split、real transport widening は current scope 外に残す。
- phase 9 `.mir` files の direct semantic execution は still not claimed である。

## Documents consulted

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
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/typed_external_boundary_canaries_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `crates/engine-abi/src/lib.rs`
- `crates/mir-runtime/src/lib.rs`
- `scripts/typed_external_boundary_samples.py`
- `scripts/tests/test_typed_external_boundary_samples.py`
- sub-agent read-only findings for `P12` scope / stop line

## Actions taken

1. Defined the smallest safe `P12` package as helper-local `host_boundary` inventory hardening, not crate-side seam activation.
2. Added RED tests that required `host_boundary` on `EXT-03` / `EXT-04` results and `host_boundary_scope` / `host_boundary_lanes` / `non_collapse_lanes` / `host_family_gates` / `host_boundary_inventory` on helper closeout.
3. Implemented shared helper-side host-boundary preview inventory in `scripts/typed_external_boundary_samples.py` and kept `engine-abi` untouched.
4. Updated snapshot docs, plan memory, hands-on docs, research abstract, sample README, and queue wording so the repo reads `P12 current first cut actualized`.
5. Closed the package by moving snapshot wording to `P12 current first cut closed` / `P13 promoted next` / `P14 reopen next`, and tightened the `P13` stop line to helper-local `process_boundary` closeout rather than premature real transport alpha.

## Files changed

- `scripts/typed_external_boundary_samples.py`
- `scripts/tests/test_typed_external_boundary_samples.py`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/typed_external_boundary_canaries_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `docs/reports/0966-p12-host-boundary-preview-inventory-first-cut.md`

## Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_typed_external_boundary_samples`
  - RED first:
    - `KeyError: 'host_boundary'`
    - `KeyError: 'host_boundary_scope'`
  - GREEN after implementation:
    - `Ran 17 tests in 0.001s`
    - `OK`
- `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json`
  - output anchors:
    - `host_boundary.scope = "helper_local_synthetic_preview"`
    - `host_boundary.request_lane = "typed_effect_request"`
    - `host_boundary.receipt_lane = "typed_effect_receipt"`
    - `host_boundary.visualization_lane = "redacted_observer_view"`
- `python3 scripts/typed_external_boundary_samples.py run EXT-04 --format json`
  - output anchors:
    - `host_boundary.failure_lane = "typed_adapter_failure"`
    - `host_boundary.receipt_lane = null`
    - `terminal_outcome = "typed_failure"`
- `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  - output anchors:
    - `host_boundary_scope = "helper_local_synthetic_preview"`
    - `host_boundary_lanes = ["request", "receipt", "failure", "visualization"]`
    - `non_collapse_lanes = ["transport", "auth", "membership", "capability", "witness", "visualization"]`
    - `host_family_gates = ["final_public_adapter_api", "exact_host_schema", "browser_network_vr_host_family_split"]`
    - `host_boundary_inventory` contains `EXT-03` and `EXT-04`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - output anchors:
    - `visualization_views`
    - `retention_scope = "helper_local_ephemeral"`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - output anchors:
    - `transport_seam = "provider_boundary"`
    - `layer_signatures[0].name = "transport_provider_boundary"`
    - `message_envelopes` include `provider_request#1` and `provider_receipt#1`

## Evidence / outputs / test results

- `EXT-03` now exposes host-boundary request / receipt / visualization split without collapsing transport/auth/membership/capability/witness.
- `EXT-04` now exposes host-boundary request / failure split and keeps domain mutation uncommitted.
- helper closeout now states the current host-boundary inventory explicitly instead of leaving the validation floor as an unnamed helper shape.
- `engine-abi` remains a named placeholder; no crate-side adapter seam was claimed.
- runtime/sample anchors still come from Sugoroku helper visualization and clean near-end `provider_boundary`, which matches the existing stop line.
- snapshot queue now reads `P12` first cut closed, `P13` promoted next, `P14` reopen next.

## What changed in understanding

- The next safe `P12` move was not new crate code in `engine-abi`; it was making the host-boundary inventory explicit in the already-running helper subset and promoting that inventory into repository memory.
- The concrete validation floor for `P12` can be narrowed to helper unittest + typed-external closeout + Sugoroku visualization anchor + clean near-end provider-boundary anchor without pretending that final adapter ABI exists.

## Open questions

- What is the smallest crate-side host-facing adapter seam that should follow this helper-local inventory, and should it live in `engine-abi` or a narrower `mir-runtime` proof-of-concept boundary first?
- When `P13` eventually widens beyond helper-local `process_boundary`, which carrier should own the first real session / reconnect runtime without collapsing transport into auth or membership?

## Suggested next prompt

`P13` network transport minimal alpha の first cut として、helper-local `process_boundary` closeout を実施し、`NET-01..05` canary の validation floor、`transport_scope = helper_local_process_boundary`、stale reconnect reject、typed failure family、observer-safe redacted route trace を docs / dashboard / report に同期してください。real socket / broker / crypto session / durable replay は still stop line に残してください。
