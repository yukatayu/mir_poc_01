# 0853 — Package 96 authoritative-room first scenario tightening

## Objective

authoritative-room first default profile を、`run-source-sample` helper summary・tests・docs で drift なく読めるようにし、Package 96 を close する。

## Scope and assumptions

- `specs/` を規範正本とし、`plan/` を repository memory、`progress.md` / `tasks.md` を snapshot として扱う。
- final public witness/provider/artifact contract、distributed fairness theorem、exhaustive shared-space catalog は今回 fixed しない。
- `p07 / p08` を representative reached pair、`p09` を reserve practical route、`p13 / p14` を negative static-stop pair として扱う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
- `specs/examples/471-current-l2-authoritative-room-vertical-slice-emitted-artifact-ratchet.md`
- `specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`

## Actions taken

1. authoritative-room / shared-space current docs と `p07 / p08 / p09 / p13 / p14` prototype 群、`current_l2_authoritative_room_vertical_slice_actualization` test を照合した。
2. `crates/mir-runtime/src/current_l2_cli.rs` に `authoritative_room_first_scenario_actual_adoption` helper summary を追加した。
3. reached / reserve / negative static-stop を JSON/pretty で検証するため、`crates/mir-runtime/tests/current_l2_operational_cli.rs` に Package 96 用テストを追加した。
4. `specs/examples/570` を追加し、Package 96 current recommendation / retained alternatives / stop line を source-backed に整理した。
5. `Documentation.md`、`specs/00`、`specs/11`、`specs/12`、`plan/01`、`plan/11`、`plan/16`、`plan/17`、`plan/18`、`plan/90`、`progress.md`、`tasks.md` を Package 96 close / Package 97...98 active queue 読みに更新した。

## Files changed

- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/570-current-l2-authoritative-room-first-scenario-helper-summary-tightening.md`
- `docs/reports/0853-package96-authoritative-room-first-scenario-tightening.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## Commands run

- `df -h .`
- `free -h`
- `cargo test -p mir-runtime --test current_l2_operational_cli 'authoritative_room_first_scenario' -- --nocapture`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test current_l2_operational_cli 'authoritative_room_first_scenario' -- --nocapture`
  - 4 tests passed
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format json`
  - `authoritative_room_first_scenario_actual_adoption.status = reached`
  - `profile_axis_refs` / `relation_refs` / `authority_handoff_refs` / `runtime_evidence_refs` / `repo_local_emitted_artifact_refs` を確認
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
  - `authoritative_room_first_scenario_actual_adoption.status = guarded_not_reached`
  - `reserve_route_refs` に delegated RNG practical route が載ることを確認

## What changed in understanding

- authoritative-room first default profile は、vertical-slice test だけでなく `run-source-sample` helper summary でも direct に追える段階まで actualize 済みと読める。
- `p09` は first default profile の reached line ではなく、reserve practical route として visible に残す読みが、helper summary 上でも明確になった。
- `p13 / p14` late-join visibility drift pair は negative static-stop pair として first scenario summary の隣で読めるため、Package 96 の stop line を narrow に保ちやすくなった。

## Open questions

- `auditable_authority_witness` を first scenario の strengthening としてどこまで repo-local summary に寄せるか。
- `delegated_rng_service` を reserve practical route からどこで public promotion 候補に上げるか。
- final public witness/provider/artifact contract と exhaustive shared-space catalog を、どの mixed gate で reopen するか。

## Suggested next prompt

Package 97 として、`auditable_authority_witness` / `delegated_rng_service` / model-check second-line concretization を reserve strengthening lane として整理し、first completion line と reserve line の境界を docs / plan / tasks / progress に同期してください。
