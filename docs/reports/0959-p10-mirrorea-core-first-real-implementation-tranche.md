# 0959 — P10 mirrorea-core first real implementation tranche

## Objective

`P10` `mirrorea-core` first real implementation tranche として、`crates/mirrorea-core` を pure placeholder のままにせず、`LayerSignature`、`PrincipalClaim`、`AuthEvidence`、`MessageEnvelope`、lane inventory、duplicate-name merge helper、carrier validation の narrow ownership cut を actualize する。helper-local / report-local preview と final public contract は混同せず、queue を `P11` promoted / `P12` reopen next へ進める。

## Scope and assumptions

- current scope は narrow Rust carrier extraction と docs/report/snapshot synchronization であり、real multi-place runtime、real transport、final public API、public verifier、projection object model、viewer catalog、hot-plug runtime ABI は固定しない。
- Sugoroku / avatar Python helpers は representative evidence floor のまま保ち、crate-side ownership cut を入れたからといって helper-local visualization / membership / hot-plug catalog を `mirrorea-core` へ昇格させない。
- authentication / authorization / membership / capability / witness split、typed visualization / telemetry、standard I/O 非 primitive rule は維持する。
- worktree には unrelated local modifications として `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs` と `crates/mir-ast/src/current_l2.rs` があるため、この package の commit には含めない。

## Documents consulted

- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0957-p10-mirrorea-core-first-tranche-extraction-candidates.md`
- `docs/reports/0958-p10-mirrorea-core-runtime-carrier-boundary-memo.md`
- `crates/mirrorea-core/Cargo.toml`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mir-runtime/Cargo.toml`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`

## Actions taken

1. Read the handoff, front-door docs, relevant specs, and relevant `plan/` files to keep the source hierarchy and `P10 -> P11 -> P12` queue reading explicit.
2. Used parallel explorer sub-agents to audit safe extraction candidates and runtime-side stop lines, then recorded their analysis in reports `0957` and `0958`.
3. Converted `crates/mirrorea-core` from placeholder-only to a current minimal carrier crate by adding:
   - `MirroreaCoreError`
   - `LayerSignature`
   - `PrincipalClaim`
   - `AuthEvidence`
   - `MessageEnvelope`
   - `layer_signature_lanes()`
   - `message_envelope_lanes()`
   - `auth_evidence_lanes()`
   - `insert_layer_signature(...)`
   - per-carrier validation helpers
4. Wired `crates/mir-runtime/src/clean_near_end.rs` to consume the `mirrorea-core` carriers and helper functions rather than keeping local duplicate definitions, while preserving clean-near-end closeout as report-local canonical inventory.
5. Added `mirrorea-core` tests and expanded runtime sample tests so current lane inventories are checked against the shared crate.
6. Ran reviewer follow-up, then tightened the legacy `transport` alias into a compatibility-only invariant `transport == transport_seam`, added regression coverage, and fixed the remaining `0958` report path inconsistency.
7. Synchronized front-door docs, snapshot docs, repository memory, and reports to state that `P10` is close 済み, `P11` is promoted next, and `P12` is reopen next.
8. Kept stale/current/planned/helper/final-public boundaries explicit and did not widen the package into visualization catalog, membership registry, projection object model, hot-plug runtime, or final auth/transport ABI.

## Files changed

- `crates/mirrorea-core/Cargo.toml`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mirrorea-core/src/error.rs`
- `crates/mirrorea-core/src/fabric.rs`
- `crates/mirrorea-core/src/layer.rs`
- `crates/mirrorea-core/tests/carriers.rs`
- `crates/mir-runtime/Cargo.toml`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0957-p10-mirrorea-core-first-tranche-extraction-candidates.md`
- `docs/reports/0958-p10-mirrorea-core-runtime-carrier-boundary-memo.md`
- `docs/reports/0959-p10-mirrorea-core-first-real-implementation-tranche.md`

## Commands run and exact outputs

- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo test -p mirrorea-core`
  - output summary:
    - `test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo test -p mir-runtime`
  - output summary:
    - crate floor green
    - `clean_near_end_samples`: `27 passed`
    - `current_l2_runtime_skeleton`: `6 passed`
    - `current_l2_source_lowering`: `18 passed`
    - `current_l2_source_sample_verification_ladder`: `16 passed`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples scripts.tests.test_avatar_follow_samples`
  - exact output:
    - `Ran 52 tests in 0.108s`
    - `OK`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - output anchors:
    - `place_model.description = "single OS process logical multi-place emulator"`
    - `membership_model.source_of_truth = "MembershipRegistry"`
    - `layer_signature_scope = "representative_slice"`
- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - output anchors:
    - `message_envelope_scope = "clean_near_end_canonical_inventory"`
    - `auth_evidence_lanes = ["kind", "subject", "issuer", "bindings", "notes"]`
    - `layer_signature_scope = "clean_near_end_canonical_inventory"`
- `python3 scripts/check_source_hierarchy.py`
  - exact output:
    - `required: 23`
    - `present: 23`
    - `missing: 0`
    - `all required paths present`
- `python3 scripts/validate_docs.py`
  - exact output:
    - `Documentation scaffold looks complete.`
    - `Found 957 numbered report(s).`
- `git diff --check`
  - exact output:
    - no output

## Evidence / outputs / test results

- `mirrorea-core` now contains executable carrier tests rather than only placeholder text, and those tests verify current `LayerSignature` / `MessageEnvelope` / lane-inventory invariants.
- `MessageEnvelope` now treats legacy `transport` as a compatibility-only alias that must match `transport_seam`; alias drift is rejected in `mirrorea-core` and asserted by runtime sample tests.
- `mir-runtime` reuses the shared crate shape while preserving the existing clean-near-end closeout distinction:
  - helper side remains `representative_slice`
  - runtime side remains `clean_near_end_canonical_inventory`
- `P10` close does not overclaim:
  - `transport_mediums` remain deferred on the runtime canonical side
  - `auth_evidence_kinds` still keeps only `none` active
  - helper-local visualization / projection / hot-plug catalogs remain helper/report-local evidence surfaces
- `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and relevant `plan/` / `specs/` docs now agree that:
  - `P10` is closed for current scope
  - `P11` is the promoted next package
  - `P12` is the next reopen point
  - `mirrorea-core` is a current minimal carrier substrate, not a final public crate shape

## What changed in understanding

- The first real `mirrorea-core` cut is viable as a narrow ownership move over already-stabilized carriers; it does not require premature extraction of runtime lifecycle semantics.
- Compatibility aliases also need hard invariants once they move into a shared crate; otherwise a repo-local normalization rule can silently drift back into an accidental public lane.
- The most stable cross-surface agreement today is the lane shape and validation rule, not the full multi-place runtime catalog.
- Preserving separate helper/runtime inventory scopes is important: sharing carrier structs is useful now, but collapsing representative helper views into canonical runtime protocol would still be a shortcut.

## Open questions

- For `P11`, which logical multi-place runtime carriers should move first after the current lane-shape cut: place execution state, message queue rows, membership freshness rows, or a smaller event/state timeline substrate?
- Should any part of `MembershipRegistry` move crate-side in `P11`, or should the first runtime tranche stop at queue/event/state carriers and keep full membership registry semantics helper-local?
- When `P12` begins, what is the smallest host-facing adapter seam that can be made real without collapsing transport/auth/visualization into a single interface?

## Suggested next prompt

`P11` logical multi-place runtime tranche に入り、helper-local logical multi-place emulator から crate-side runtime substrate へ寄せてよい最小 carrier を source-backed に選別し、`event` / `message` / `membership freshness` / `state timeline` のうちどこまでを current scope とするかを report、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `plan/` に反映してください。helper-local visualization catalog、full `MembershipRegistry` semantics、real transport、durable replay、final public API はまだ固定しないでください。
