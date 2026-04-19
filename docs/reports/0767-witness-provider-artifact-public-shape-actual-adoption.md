# 0767 — witness/provider/artifact public-shape actual adoption

## Objective

shared-space line の `final public witness/provider/artifact contract` mixed gate を threshold default のままにせず、
current repo-local actual adoption package を source-backed に選び、
helper-local actual adoption manifest と docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted handoff contract、exhaustive shared-space catalog はこの package で固定しない。
- current default は claim/payload split first、witness route / provider route non-collapse、repo-local emitted artifact refs first と読む。

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
- `specs/examples/476-current-l2-auditable-authority-witness-strengthening-actualization.md`
- `specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md`
- `specs/examples/483-current-l2-witness-provider-artifact-public-shape-threshold-default.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. public-shape threshold default と witness/provider reserve lines を再読し、witness route / provider route をどこまで actual adoption に上げられるかを整理した。
2. RED として `crates/mir-runtime/tests/current_l2_witness_provider_artifact_public_shape_actual_adoption.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper `CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption` と builder を追加し、claim/payload split first、optional attachment refs only、repo-local emitted artifact refs first を repo-local actual adoption manifest に actualize した。
4. `specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_artifact_public_shape_actual_adoption`
  - unresolved import `CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption` / builder missing で失敗
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_artifact_public_shape_actual_adoption`
  - `4 passed; 0 failed`

## What changed in understanding

- public-shape line の current actual adoption package は、final public schema を決めることではなく、witness route と provider route を non-collapse のまま repo-local actual adoption floor に上げることだと整理できた。
- これにより shared-space mixed gate は、
  - final public witness schema / provider receipt schema
  - delegated provider attestation
  - combined provider+witness public contract / final emitted handoff contract
  - exhaustive shared-space catalog
 へ narrow になった。

## Open questions

- final public witness schema と final provider receipt schema を別 gate にするか。
- delegated provider attestation を combined public contract と切り離して扱い続けるか。
- final emitted handoff contract を witness/provider public contract と同時に reopen するか。

## Suggested next prompt

`order-handoff source surface を threshold default から actual adoption に上げられるかを再監査し、edge-row principal / stage-block secondary / repo-local emitted artifact refs first の actual package を docs / plan / snapshot へ同期してください。`
