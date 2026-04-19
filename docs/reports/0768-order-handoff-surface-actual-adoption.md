# 0768 — order-handoff surface actual adoption

## Objective

order-handoff line の `final source-surface wording / final emitted-artifact schema` mixed gate を threshold default のままにせず、
current repo-local actual adoption package を source-backed に選び、
helper-local actual adoption manifest と docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- final parser grammar、final public parser/checker/runtime API、final source-surface handoff wording、final emitted-handoff contract、final public witness schema、authoritative-room `serial` sugar adoption、low-level exact source wording、final modal foundation adoption はこの package で固定しない。
- current default は edge-row principal、stage-block secondary keep、repo-local emitted artifact refs first と読む。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/471-current-l2-authoritative-room-vertical-slice-emitted-artifact-ratchet.md`
- `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`
- `specs/examples/473-current-l2-order-handoff-surface-narrowing-and-stage-block-secondary-candidate.md`
- `specs/examples/483-current-l2-witness-provider-artifact-public-shape-threshold-default.md`
- `specs/examples/484-current-l2-order-handoff-surface-artifact-threshold-default.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. order-handoff surface threshold default と companion / stage-block helper surfaces を再読し、surface principal をどこまで actual adoption に上げるかを整理した。
2. RED として `crates/mir-runtime/tests/current_l2_order_handoff_surface_actual_adoption.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper `CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption` と builder を追加し、edge-row principal、stage-block secondary keep、repo-local emitted artifact refs first を repo-local actual adoption manifest に actualize した。
4. `specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_order_handoff_surface_actual_adoption`
  - unresolved import `CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption` / builder missing で失敗
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_order_handoff_surface_actual_adoption`
  - `3 passed; 0 failed`

## What changed in understanding

- order-handoff line の current actual adoption package は、final wording を決めることではなく、principal surface と secondary surface を repo-local actual adoption floor に上げることだと整理できた。
- これにより order-handoff mixed gate は、
  - final source-surface handoff wording
  - final emitted-handoff contract / final public witness-provider-artifact contract
  - authoritative-room `serial` sugar adoption / low-level exact source wording
  - final modal foundation adoption
 へ narrow になった。

## Open questions

- final source-surface handoff wording を principal edge-row line の refinement として採るか。
- final emitted-handoff contract を witness/provider public contract と同時に reopen するか。
- `serial` sugar を order-handoff line で reopen するか、shared-space line に残すか。

## Suggested next prompt

`489 / 490 を progress.md / tasks.md / plan / specs snapshot に同期し、representative validation を再実行したうえで、theorem result public object か final public witness/provider/artifact contract のどちらを先に reopen するかを narrow に比較してください。`
