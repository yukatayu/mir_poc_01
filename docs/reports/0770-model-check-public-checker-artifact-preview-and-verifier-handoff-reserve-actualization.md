# 0770 — model-check public-checker artifact preview and verifier-handoff reserve actualization

## Objective

model-check line の次 reopen candidate だった `public checker artifact / emitted verifier handoff artifact` を、
premature な final public checker migration にせず、
repo-local public-checker artifact preview と verifier-handoff reserve keep の helper-local actualization package へ narrow に進める。

## Scope and assumptions

- row-local property route first / checker-boundary contract first は current source-backed floor として維持する。
- final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、final public verifier contract は今回 fixed しない。

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
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`

## Actions taken

1. `current_l2_model_check_public_checker_artifact_preview_actualization` failing test を追加し、missing struct/builder で RED を確認した。
2. `current_l2_source_sample_emitted_artifact_support.rs` に model-check public-checker artifact preview actualization helper を追加した。
3. `specs/examples/492` を追加し、repo-local public-checker artifact preview / verifier-handoff reserve keep を current recommendation として明文化した。
4. snapshot / plan / roadmap / open-question / decision-register を `492` reading に同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_model_check_public_checker_artifact_preview_actualization`
  - unresolved imports for `CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization` and builder
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_model_check_public_checker_artifact_preview_actualization`
  - `5 passed; 0 failed`

## What changed in understanding

- model-check row-local property / checker-boundary actual adoption の次 reopen candidate は、
  すぐに final public checker artifact や actual public checker migration へ進む必要はない。
- `consumer_shaped_artifact_preview_only + verifier_handoff_reserve_keep + brand_neutral_tool_binding_reserve_keep`
  という repo-local helper-local cut を 1 段 actualize すると、
  model-check mixed gate を public contract 直前ではなく narrow preview/reserve まで縮められる。
- remaining model-check mixed gate は、
  first settled property language / concrete model-check tool brand、
  final public checker artifact / actual public checker migration、
  actual emitted verifier handoff artifact / production checker-runtime-policy contract、
  final public verifier contract、
  の順に読み替えた方が current state を正確に表せる。

## Open questions

- public-checker artifact preview を final public checker artifact に上げるとき、actual public checker migration と同時 package にするか。
- emitted verifier handoff artifact と runtime-policy contract を同一 package に保つか、verifier handoff だけ先に narrow actualization するか。
- theorem 側の proof-object-schema reserve line と model-check 側の public-checker preview line のどちらを next reopen candidate にするか。

## Suggested next prompt

`specs/examples/492` と `docs/reports/0770` を起点に、theorem proof-object-schema reserve line か order/shared-space final emitted-artifact contract line のどちらを先に reopen するかを比較し、chosen line を helper-local actualization package まで進めてください。
