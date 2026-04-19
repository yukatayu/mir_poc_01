# Report 0759 — model check property tool threshold default

- Date: 2026-04-18T13:11:44.881552Z
- Author / agent: Codex (GPT-5)
- Scope: `M2` model-check property-language / tool-brand threshold package の close、helper-local threshold actualization、`specs/` / `plan/` / snapshot 同期
- Decision levels touched: L2

## 1. Objective

`specs/examples/480` と `478` の間にある model-check mixed gate を compare-floor のままにせず、

- row-local property preview first
- small-cluster semantic projection second
- brand-neutral model-check request
- public checker contract later

を current default threshold として helper-local actualization し、
`M2` package を close する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md`
- `specs/examples/466-current-l2-problem1-actual-adoption-package-and-theorem-first-pilot.md`
- `specs/examples/475-current-l2-principal-theory-spine-and-lean-first-proof-roadmap.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `crates/mir-runtime/tests/current_l2_model_check_second_line_concretization.rs`
- `crates/mir-runtime/tests/current_l2_model_check_property_tool_seam_probe.rs`

## 3. Actions taken

1. model-check second-line concretization と property/tool-seam probe の gap を監査し、helper-local threshold manifest が必要と判断した。
2. `CurrentL2SourceSampleModelCheckPropertyToolThreshold` と builder を support helper に追加した。
3. `current_l2_model_check_property_tool_threshold.rs` を新設し、`e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only を machine-check した。
4. threshold test の expectation drift を root-cause まで追い、threshold builder が second-line concretization の excluded family を carry-over することを確認したうえで、test expectation を整合させた。
5. `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md` を追加し、current default / retained alternatives / stop line を source-backed に固定した。
6. `specs/10`、`specs/11`、`specs/12` を更新し、`M2` close と `D-075` judgment を反映した。
7. `Documentation.md`、`progress.md`、`tasks.md`、`plan/00`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/18`、`plan/90`、`docs/research_abstract/*` を同期し、current queue を `M3` に narrowed した。

## 4. Files changed

- Added:
  - `crates/mir-runtime/tests/current_l2_model_check_property_tool_threshold.rs`
  - `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- Updated:
  - `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug model-check-property-tool-threshold-default`
  - `docs/reports/0759-model-check-property-tool-threshold-default.md`
- `cargo test -p mir-runtime --test current_l2_model_check_property_tool_threshold`
  - first run: `5 failed; 1 passed`
  - second run after expectation fix: `6 passed; 0 failed`

## 6. Evidence / findings

- new helper-local threshold manifest can carry:
  - row-local property preview first
  - small-cluster semantic projection second
  - brand-neutral model-check request
  - public checker contract later
- representative corpus is stable at this threshold:
  - reached: `e5`, `p06`, `p07`, `p08`, `p09`
  - guard-only: `p05`
- the threshold builder intentionally inherits excluded family refs from model-check second-line concretization:
  - `excluded_family:theorem_discharge_transport`
  - `excluded_family:room_protocol_projection`
  - `excluded_family:provider_receipt_fairness_family`
- this closes `M2` as a threshold package without promoting:
  - first settled property language
  - concrete model-check tool brand
  - actual public checker migration
  - actual emitted verifier handoff artifact
  - production checker/runtime-policy contract

## 7. Changes in understanding

- `M2` was not waiting for a final property language or concrete tool brand decision; it was waiting for a source-backed threshold default that could stop comparison debt.
- the natural default is not a concrete checker brand or public checker artifact.
- the natural default is a row-local property-preview-first threshold with semantic projection second, brand-neutral request keep, and public checker contract later.
- once that threshold is explicit and executable, the immediate self-driven queue narrows to `M3`.

## 8. Open questions

- first settled property language を actual adoption に送るなら、row-local property preview からどの normalization/property family を first public cut に採るか。
- concrete model-check tool brand を actual adoption に送るなら、brand-neutral request keep をどの schema seam で concretize するか。
- actual public checker migration と emitted verifier handoff artifact を same gate に置くか、separate gate に分けるか。
- room protocol projection と provider receipt/fairness family を checker public-shape mixed gate へどこで reconnect するか。

## 9. Suggested next prompt

`M3` witness-provider-artifact public-shape threshold package を進め、`p07/p08/p09` と witness/provider actualization を保ったまま、public witness schema / provider receipt optional attachment / emitted-artifact reserve shape の current default / retained alternatives / stop line を source-backed に詰めたうえで、snapshot と plan/spec を再同期してください。
