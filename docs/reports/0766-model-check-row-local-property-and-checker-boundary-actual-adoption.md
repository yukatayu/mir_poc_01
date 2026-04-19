# 0766 — model-check row-local property and checker-boundary actual adoption

## Objective

model-check line の `first settled property language / concrete tool brand` mixed gate を compare-floor のままにせず、
current repo-local actual adoption package を source-backed に選び、
helper-local actual adoption manifest と docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- first settled property language、concrete model-check tool brand、consumer-shaped public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract はこの package で固定しない。
- current default は row-local property route first、checker-boundary contract first、brand-neutral tool-binding reserve keep と読む。

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
- `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. model-check helper stack と current docs の current line を再読し、projection pre-floor / seam probe / threshold default のどこまでを actual adoption に上げるかを整理した。
2. RED として `crates/mir-runtime/tests/current_l2_model_check_row_local_property_actual_adoption.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper `CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption` と builder を追加し、row-local property route first / checker-boundary contract first / brand-neutral tool-binding reserve keep を repo-local actual adoption manifest に actualize した。
4. `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。
5. `Documentation.md`、`progress.md`、`tasks.md`、`specs/`、`plan/`、`docs/research_abstract/` を更新し、model-check mixed gate の current reading を repo-local actual adoption close 後の形へ揃えた。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_model_check_row_local_property_actual_adoption`
  - unresolved import `CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption` / builder missing で失敗
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_model_check_row_local_property_actual_adoption`
  - `5 passed; 0 failed`

## What changed in understanding

- model-check line の current actual adoption package は、first settled property language や concrete tool brand を public surface として固定することではなく、row-local property route first と checker-boundary contract first を repo-local actual adoption floor に上げることだと整理できた。
- これにより model-check mixed gate は、
  - first settled property language / concrete model-check tool brand
  - consumer-shaped public checker artifact
  - actual public checker migration / emitted verifier handoff artifact
  - production checker/runtime-policy contract
  へ narrow になった。

## Open questions

- first settled property language を concrete tool brand より先に reopen するか。
- consumer-shaped public checker artifact と emitted verifier handoff artifact をどの public gate で切り出すか。
- production checker/runtime-policy contract を model-check line の public artifact family とどこまで分離したまま keep するか。

## Suggested next prompt

`witness/provider/artifact public-shape line を threshold default から actual adoption に上げられるかを再監査し、repo-local actual adoption cut が選べるなら helper-local actualization / docs / snapshot まで閉じてください。`
