# 0764 — theorem review-unit transport and notebook-contract actual adoption

## Objective

theorem line の `actual discharge transport / public theorem contract` mixed gate を compare-floor のままにせず、
current repo-local actual adoption package を source-backed に選び、
helper-local actual adoption manifest と docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- final public theorem contract、proof object public schema、concrete theorem prover brand、final public verifier contract はこの package で固定しない。
- current default は review-unit anchor first、notebook-consumer contract first、brand-neutral binding reserve keep と読む。

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
- `specs/examples/446-current-l2-theorem-discharge-transport-and-public-contract-later-gate-framing-note.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- `specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md`
- `specs/examples/486-current-l2-theorem-transport-public-contract-coupled-later-gate.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. theorem helper stack と current docs の current line を再読し、transport-first / contract-first / coupled-actual-adoption のどれを actual package に上げるかを整理した。
2. RED として `crates/mir-runtime/tests/current_l2_theorem_review_unit_transport_actual_adoption.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper `CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption` と builder を追加し、review-unit transport first / notebook-consumer contract first / brand-neutral binding reserve keep を repo-local actual adoption manifest に actualize した。
4. `specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。
5. `Documentation.md`、`progress.md`、`tasks.md`、`specs/`、`plan/`、`docs/research_abstract/` を更新し、theorem mixed gate の current reading を repo-local actual adoption close 後の形へ揃えた。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_theorem_review_unit_transport_actual_adoption`
  - unresolved import `CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption` / builder missing で失敗
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_theorem_review_unit_transport_actual_adoption`
  - `5 passed; 0 failed`

## What changed in understanding

- theorem line の current actual adoption package は、`actual discharge transport` と `public theorem contract` を final public surface として固定することではなく、review-unit anchor first と notebook-consumer contract first を repo-local actual adoption floor に上げることだと整理できた。
- これにより theorem mixed gate は、
  - theorem result public object / consumer-shaped theorem payload
  - concrete theorem prover brand / proof object public schema
  - final public verifier contract
 へ narrow になった。

## Open questions

- theorem result public object を current helper-local actual adoption からどの public gate で切り出すか。
- consumer-shaped theorem payload と proof object public schema をどこまで同時に扱うか。
- concrete theorem prover brand を public theorem payload family と切り離して keep し続けるか。

## Suggested next prompt

`specs/examples/` と `samples/` の stale / superseded 監査を行い、traceability を壊さない安全な削除候補だけを整理し、その後 representative validation を再実行してください。`
