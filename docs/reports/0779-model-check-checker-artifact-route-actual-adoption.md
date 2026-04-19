# 0779 — model-check checker-artifact route actual adoption

## Objective

model-check line の `final public checker artifact / actual public checker migration / emitted verifier handoff artifact`
mixed gate を compare-only のまま残さず、
`checker-artifact route first` の repo-local actual adoption package まで narrow に進める。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- row-local property route first / checker-boundary contract first は prior source-backed floor として再利用する。
- final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract はこの package で固定しない。

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
- `specs/examples/498-current-l2-model-check-public-checker-artifact-and-migration-coupled-later-gate.md`
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

1. model-check line の prior floor を `488`、`492`、`495`、`498` に限定して再確認した。
2. `crates/mir-runtime/tests/current_l2_model_check_checker_artifact_route_actual_adoption.rs` を追加し、missing struct/builder で RED を確認した。
3. `current_l2_source_sample_emitted_artifact_support.rs` に model-check checker-artifact route actual adoption helper を追加し、artifact/migration coupled-later gate を前段にした repo-local actual adoption route を actualize した。
4. compare-floor 継承に `property_tool_threshold` が残ることを test で確認し、期待値を修正した。
5. `specs/examples/501` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。
6. snapshot / roadmap / open-question / decision-register / traceability を `501` reading に同期する前提を整えた。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_model_check_checker_artifact_route_actual_adoption`
  - unresolved imports for `CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption` and builder
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_model_check_checker_artifact_route_actual_adoption`
  - `5 passed; 0 failed`

## What changed in understanding

- model-check line の next reopen candidate は、final public checker artifact へ即進むことではなく、`checker-artifact route first` を repo-local actual adoption に上げるところまでは self-driven に進められる、と整理できた。
- migration / verifier-handoff / tool-brand / runtime-policy contract は still later に残しつつ、checker-artifact route 自体は current recommendation に上げても捻れは生じない。

## Open questions

- final public checker artifact と actual public checker migration を同じ gate に残すか。
- concrete model-check tool brand と emitted verifier handoff artifact を同じ gate に残すか。
- model-check line と order-handoff modal/source-marker line のどちらを next mixed-gate reopen にするか。

## Suggested next prompt

`501` を full validation と snapshot に同期したうえで、order-handoff final source wording / modal marker line か、theorem/model-check public-object boundary line のどちらを next reopen candidate にするかを比較し、helper-local actualization か actual adoption を 1 本進めてください。
