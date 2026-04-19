# Report 0807 — package58 order-handoff surface-preview CLI actualization

- Date: 2026-04-19T08:05:00Z
- Author / agent: Codex
- Scope: Package 58 の helper / CLI hardening として `run-source-sample` に order-handoff `surface_preview` を actualize する
- Decision levels touched: L1 / L2 / L3

## 1. Objective

Package 58 の remaining helper hardening を docs-only wording にせず、

- current order-handoff surface recommendation を CLI から inspectable にすること
- `p07 / p08 / p09` で reached / guarded の違いを helper-local preview に落とすこと
- final grammar / final public API を凍らせずに current recommendation を sample-visible にすること

を目的とした。

## 2. Inputs consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`
- `specs/examples/473-current-l2-order-handoff-surface-narrowing-and-stage-block-secondary-candidate.md`
- `specs/examples/511-current-l2-order-handoff-serial-scope-reserve-surface.md`
- `specs/examples/520-current-l2-final-layer-closeout-defaults-and-reopened-selfdriven-queue.md`
- `specs/examples/525-current-l2-delegated-rng-provider-placement-representative-lean-sample-set-carryover.md`
- `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
- `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`
- `samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt`

## 3. Actions taken

1. TDD として `current_l2_order_handoff_surface_preview_cli.rs` を追加し、CLI JSON に `surface_preview` 欄が無い RED を確認した。
2. `crates/mir-runtime/src/current_l2_cli.rs` に helper-local `surface_preview` summary を追加した。
3. `minimal_companion`、`stage_block_secondary`、`serial_scope_reserve` の 3 section を sample-id-aware に actualize した。
4. `p07 / p08` は principal companion と stage-block secondary と serial reserve を reached にし、`p09` は serial reserve のみ reached、他 2 つを guarded にした。
5. docs / roadmap / queue snapshot を `specs/examples/526` anchor へ同期した。

## 4. Files changed

- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_order_handoff_surface_preview_cli.rs`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/526-current-l2-order-handoff-helper-cli-surface-preview-actualization.md`
- `samples/prototype/README.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_order_handoff_surface_preview_cli order_handoff_surface_preview_cli_matches_builders_for_late_join_sample -- --exact`
  - RED: `left: Null / right: "reached"`
- `cargo test -p mir-runtime --test current_l2_order_handoff_surface_preview_cli`
  - pass
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
  - `surface_preview.serial_scope_reserve.status = "reached"`
  - `surface_preview.minimal_companion.status = "guarded_not_reached"`
  - `surface_preview.stage_block_secondary.status = "guarded_not_reached"`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - whitespace / conflict marker issue なし

## 6. Evidence / findings

- CLI helper preview に surface family を actualize することで、current syntax recommendation が docs-only でなく sample-local JSON / pretty output に下りてきた。
- `p09` では serial reserve のみ reached とすることで、delegated provider practical cut と authoritative-room baseline companion surfaceを無理に collapse せずに保てた。
- current implementation は helper-local preview に留まり、final grammar / final public API / final emitted schema の adoption を主張しない。

## 7. Changes in understanding

- Package 58 の helper hardening は、新しい sample を増やすだけでなく、既存 recommendation を inspectable helper output に actualize することで実質的に前進できる。
- order-handoff surface family は compare-floor に留めるより、CLI preview で reached / guarded を見せた方が queue drift を起こしにくい。

## 8. Open questions

- helper/CLI hardening の次段を broader theorem-side widening と order-handoff negative corpus tighteningのどちらから切るか。
- `missing witness static stop` と `handoff before publish static stop` を current closeout corpus にどう actualize するか。
- final source wording / final emitted-artifact schema / final public witness-provider contract は still mixed gate のままである。

## 9. Suggested next prompt

Package 58 の次段として、order-handoff negative corpus tightening か broader theorem-side helper widening を 1 本選び、`surface_preview` と同じく helper-local actualization + docs sync + focused validation まで進める。
