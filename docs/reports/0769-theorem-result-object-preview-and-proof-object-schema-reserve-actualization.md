# 0769 — theorem result-object preview and proof-object-schema reserve actualization

## Objective

theorem line の次 reopen candidate だった `theorem result public object / proof object public schema` を、
premature な final public contract adoption にせず、
repo-local result-object preview と proof-object-schema reserve keep の helper-local actualization package へ narrow に進める。

## Scope and assumptions

- `proof_notebook_review_unit` は引き続き principal review artifact に留める。
- theorem review-unit transport / notebook-contract actual adoption は source-backed floor として再利用する。
- final public theorem result object、proof object public schema、final public verifier contract は今回 fixed しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- `specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md`
- `specs/examples/486-current-l2-theorem-transport-public-contract-coupled-later-gate.md`
- `specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`

## Actions taken

1. `current_l2_theorem_result_object_preview_actualization` failing test を追加し、missing struct/builder で RED を確認した。
2. `current_l2_source_sample_emitted_artifact_support.rs` に theorem result-object preview actualization helper を追加した。
3. `specs/examples/491` を追加し、repo-local result-object preview / proof-object-schema reserve keep を current recommendation として明文化した。
4. snapshot / plan / roadmap / open-question / decision-register を `491` reading に同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_theorem_result_object_preview_actualization`
  - unresolved imports for `CurrentL2SourceSampleTheoremResultObjectPreviewActualization` and builder
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_theorem_result_object_preview_actualization`
  - `5 passed; 0 failed`

## What changed in understanding

- theorem review-unit transport / notebook-contract actual adoption の次 reopen candidate は、
  すぐに final public theorem result object へ進む必要はない。
- `notebook_consumer_object_first + consumer_shaped_payload_preview_only + proof_object_schema_reserve_keep`
  という repo-local helper-local cut を 1 段 actualize すると、
  theorem mixed gate を public contract 直前ではなく narrow reserve まで縮められる。
- remaining theorem mixed gate は、
  final public theorem result object / consumer-shaped payload public contract、
  concrete theorem prover brand / proof object public schema、
  final public verifier contract、
  の 3 本に読み替えた方が current state を正確に表せる。

## Open questions

- theorem result-object preview を public theorem payload family に上げるとき、consumer-shaped payload public contract と同時 package にするか。
- concrete theorem prover brand と proof object public schema を同一 package に保つか、proof object public schema だけ先に narrow actualization するか。
- model-check 側の `public checker artifact` line と theorem 側の result-object line のどちらを先に reopen するか。

## Suggested next prompt

`specs/examples/491` と `docs/reports/0769` を起点に、model-check public checker artifact line か theorem proof-object-schema reserve line のどちらを先に reopen するかを比較し、chosen line を helper-local actualization package まで進めてください。
