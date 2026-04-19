# Report 0762 — theorem contract shape threshold default

## 1. Title and identifier

- Report 0762
- theorem contract shape threshold default

## 2. Objective

`actual discharge transport / public theorem contract` mixed gateを compare-floor のままにせず、
`refs-only reserve schema first` を current default shape として helper-local actualization に上げる。
同時に、`plan/`、`specs/`、`Documentation.md`、`progress.md`、`tasks.md` の current reading を同期する。

## 3. Scope and assumptions

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を時系列証跡として扱う。
- current theorem line は
  - notebook-first theorem line
  - theorem-first external integration target
  - theorem discharge actual-format probe
  - theorem discharge / public-theorem-contract threshold default
  を前提にする。
- 今回は final public theorem contract、actual discharge transport、proof object public schema、concrete theorem prover brand を凍らせない。
- `e5 / p06 / p07 / p08` reached、`p05` guard-only を current adequacy corpus として使う。

## 4. Documents consulted

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
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `sub-agent-pro/codex_theory_handoff_2026-04-18.md`

## 5. Actions taken

1. 資源状況を確認した。
   - `df -h .`
   - `free -h`
2. theorem mixed gate の current support / tests を再読し、new helper-local cut を
   `refs-only reserve schema first`
   `review-unit transport anchor`
   `brand-neutral request manifest keep`
   `consumer-shaped payload later`
   に置くと決めた。
3. RED として [current_l2_theorem_contract_shape_threshold.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/tests/current_l2_theorem_contract_shape_threshold.rs) を追加し、未実装 import で落ちることを確認した。
4. support helper [current_l2_source_sample_emitted_artifact_support.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs) に
   - `CurrentL2SourceSampleTheoremContractShapeThreshold`
   - `build_current_l2_source_sample_theorem_contract_shape_threshold`
   - shape-threshold default / compare / contrast / guard / kept-later helper 群
   を追加した。
5. 規範側に [specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md](/home/yukatayu/dev/mir_poc_01/specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md) を追加した。
6. snapshot / memory / roadmap を同期した。
   - [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
   - [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)
   - [tasks.md](/home/yukatayu/dev/mir_poc_01/tasks.md)
   - [specs/00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
   - [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md)
   - [specs/11-roadmap-and-workstreams.md](/home/yukatayu/dev/mir_poc_01/specs/11-roadmap-and-workstreams.md)
   - [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md)
   - [plan/00-index.md](/home/yukatayu/dev/mir_poc_01/plan/00-index.md)
   - [plan/01-status-at-a-glance.md](/home/yukatayu/dev/mir_poc_01/plan/01-status-at-a-glance.md)
   - [plan/10-roadmap-overall.md](/home/yukatayu/dev/mir_poc_01/plan/10-roadmap-overall.md)
   - [plan/11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
   - [plan/12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
   - [plan/13-heavy-future-workstreams.md](/home/yukatayu/dev/mir_poc_01/plan/13-heavy-future-workstreams.md)
   - [plan/17-research-phases-and-autonomy-gates.md](/home/yukatayu/dev/mir_poc_01/plan/17-research-phases-and-autonomy-gates.md)
   - [plan/18-type-proof-modelcheck-and-ordering-research-program.md](/home/yukatayu/dev/mir_poc_01/plan/18-type-proof-modelcheck-and-ordering-research-program.md)
   - [plan/90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
   - [docs/research_abstract/phase6-compile-ready-minimal-actualization.md](/home/yukatayu/dev/mir_poc_01/docs/research_abstract/phase6-compile-ready-minimal-actualization.md)

## 6. Evidence / outputs / test results

- RED confirmation:
  - `cargo test -p mir-runtime --test current_l2_theorem_contract_shape_threshold`
  - initial result:
    `error[E0432]: unresolved imports ... CurrentL2SourceSampleTheoremContractShapeThreshold ... build_current_l2_source_sample_theorem_contract_shape_threshold`
- targeted GREEN:
  - `cargo test -p mir-runtime --test current_l2_theorem_contract_shape_threshold`
  - result:
    `5 passed; 0 failed`
- full runtime / helper verification:
  - `cargo test -p mir-runtime`
  - result:
    all suites green。
    代表値:
    `current_l2_source_sample_runner 22 passed`
    `current_l2_operational_cli 12 passed`
    `current_l2_theorem_contract_shape_threshold 5 passed`
    `current_l2_theorem_discharge_public_contract_threshold 5 passed`
- regression:
  - `python3 scripts/current_l2_source_sample_regression.py regression`
  - result:
    `all regression commands passed`
- docs validation:
  - `python3 scripts/validate_docs.py`
  - result:
    `Documentation scaffold looks complete.`
    `Found 761 numbered report(s).`
- diff hygiene:
  - `git diff --check`
  - result:
    no output

## 7. What changed in understanding

- theorem line は `threshold default` で止めるだけでなく、もう 1 段
  `shape threshold`
  まで narrow に actualize できると分かった。
- current safe reading は、
  `public theorem contract` を採ることではなく、
  `refs-only reserve schema first`
  を current default shape に固定することにある。
- これにより theorem mixed gate は
  - actual discharge transport / public theorem contract actual adoption
  - theorem result public object / consumer-shaped theorem payload
  - concrete theorem prover brand / proof object public schema
  へさらに狭まった。
- したがって current principal self-driven queue が live package none である読みは維持できるが、
  theorem solved や final public theorem contract completion を意味しない、という区別がさらに明確になった。

## 8. Open questions

- actual discharge transport をいつ actual adoption judgment に上げるか。
- public theorem contract を notebook-first line からどこまで切り離すか。
- theorem result public object と consumer-shaped theorem payload を同時に扱うか、段階分離するか。
- concrete theorem prover brand / proof object public schema を theorem-first pilot の後段 mixed gate としてどこで reopen するか。

## 9. Suggested next prompt

`theorem contract shape threshold` の current default を前提に、`actual discharge transport / public theorem contract actual adoption` を helper-local actualization まで押し、`refs-only reserve schema first` からどこまで public contract へ進められるかを narrow に検証してください。
