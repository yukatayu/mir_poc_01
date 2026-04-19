# 0776 — model-check public checker artifact and migration coupled later gate

## Objective

model-check line の `final public checker artifact / actual public checker migration`
mixed gate を compare-only のまま残さず、
helper-local coupled-later gate として source-backed に actualize し、
docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract はこの package で固定しない。
- current default は consumer-shaped artifact candidate only、actual public checker migration candidate only、tool-brand / verifier-handoff adjacent keep、final public verifier contract later と読む。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md`
- `specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md`
- `specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `488`、`492`、`495` を再読し、model-check line の next mixed gate を `public checker artifact / migration coupled later gate` として切る方針を確認した。
2. RED として `crates/mir-runtime/tests/current_l2_model_check_public_checker_artifact_migration_coupled_later_gate.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper に `CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate` と builder を追加し、public checker artifact side と checker migration side を adjacent but distinct later gate として actualize した。
4. compare-floor carry-over は `tool_brand_verifier_handoff_coupled_later_gate` の chain を維持する方が current repo reading と整合するため、expectation をその current chain に合わせて修正した。
5. `specs/examples/498-current-l2-model-check-public-checker-artifact-and-migration-coupled-later-gate.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。
6. `Documentation.md`、`progress.md`、`tasks.md`、relevant `specs/` / `plan/` / `plan/90-source-traceability.md` を current reading へ同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_model_check_public_checker_artifact_migration_coupled_later_gate`
  - unresolved import `CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate` / builder missing で失敗
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_model_check_public_checker_artifact_migration_coupled_later_gate`
  - `5 passed; 0 failed`

## What changed in understanding

- model-check line の next mixed gate は、final public checker artifact や actual public checker migration を即採ることではなく、public checker artifact side と migration side を `adjacent but distinct later gate` として helper-local manifest に actualize するところまでは self-driven に進められると整理できた。
- tool-brand / verifier-handoff coupled-later gate は、public checker artifact / migration side の next cut と collapse せず compare floor として carry-over する方が current model-check reading と整合する、と明確になった。

## Open questions

- final public checker artifact と actual public checker migration を same gate のまま reopen するか。
- actual emitted verifier handoff artifact を checker migration adoption より先に reopen するか。
- first settled property language / concrete tool brand を checker artifact adoption と独立に reopen するか。

## Suggested next prompt

`498` を representative validation と traceability まで同期したうえで、final public witness/provider/artifact contract mixed gate か final modal foundation / source marker mixed gate のどちらを next reopen candidate として actualize するかを narrow に比較し、helper-local actualization を 1 本進めてください。
