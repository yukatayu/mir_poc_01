# Report 0748 — order handoff surface narrowing after external advice

- Date: 2026-04-18T02:56:53.788589Z
- Author / agent: Codex
- Scope: GPT5.4-pro 由来の surface 助言のうち current boundary と両立する部分を docs-first に吸収し、order / handoff source-facing recommendation を principal / secondary / reserve へ狭める。
- Decision levels touched: L2

## 1. Objective

Problem 2 / syntax-modality line について、

- explicit edge-row / vertical continuation を principal に維持すること
- `stage` / `after` / `witness` family を strong secondary experimental candidate に落とすこと
- authoritative-room `serial` sugar を reserve に留めること

を source-backed に整理し、helper-local compare floor でも machine-check できる状態へ進める。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `faq_007.md`
- `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
- `specs/examples/468-current-l2-syntax-modality-convergence-and-current-recommendation.md`
- `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`
- `plan/06-surface-notation-status.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
- `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`

## 3. Actions taken

1. GPT5.4-pro 助言を current boundary に照らし、source principal / secondary experimental candidate / reserve sugar に切り分けた。
2. RED として `crates/mir-runtime/tests/current_l2_order_handoff_stage_block_surface.rs` を追加し、missing helper で失敗することを確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に stage-block secondary surface route を追加した。
4. `specs/examples/473-current-l2-order-handoff-surface-narrowing-and-stage-block-secondary-candidate.md` を追加し、principal / secondary / reserve の current recommendation を固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/06`、`plan/09`、`plan/16`、`plan/18`、`samples/prototype/README.md` へ current reading を反映した。

## 4. Files changed

- Added:
  - `crates/mir-runtime/tests/current_l2_order_handoff_stage_block_surface.rs`
  - `specs/examples/473-current-l2-order-handoff-surface-narrowing-and-stage-block-secondary-candidate.md`
- Modified:
  - `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/06-surface-notation-status.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/16-shared-space-membership-and-example-boundary.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `samples/prototype/README.md`

## 5. Commands run and exact outputs

- `df -h .`
  - `/dev/vda2 99G / 78G used / 17G avail / 83%`
- `free -h`
  - `Mem: 960Mi total / 695Mi used / 264Mi available`
- RED:
  - `cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface --test current_l2_theorem_prover_binding_preflight`
  - unresolved import for `CurrentL2SourceSampleStageBlockSurface` / `build_current_l2_source_sample_stage_block_surface`
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface --test current_l2_theorem_prover_binding_preflight`
  - `current_l2_order_handoff_stage_block_surface`: `3 passed`

## 6. Evidence / findings

- principal source wording を `memory_order` exact surface へ寄せる必要はなく、むしろ current docs と矛盾する。
- `stage-block + witness` は current principal ではないが、`p07/p08` を用いた helper-local compare floor では十分に semantically honest な secondary candidate になる。
- guarded sample `p05` は reached と偽装せず guard-only へ残せる。
- authoritative-room `serial` sugar は shared-memory lock 直感を招きやすく、current line では reserve が妥当だった。

## 7. Changes in understanding

- GPT5.4-pro 助言は新理論の発見ではなく、Problem 2 / syntax-modality の last-mile narrowing を速める材料として有効だった。
- current repo の principal は引き続き edge-row / vertical continuation であり、`stage` family は strong secondary candidate として扱うのが最も整合的だった。

## 8. Open questions

- `stage` / `after` / `witness` family を final public companion へどこまで mirror するか。
- authoritative-room `serial` sugar を本当に必要とするか。
- final source-surface handoff wording / emitted-artifact schema をどこで mixed gate から上げるか。

## 9. Suggested next prompt

`auditable_authority_witness` strengthening package を、`specs/examples/471` と `473` を前提に docs / helper / tests まで進めてください。
