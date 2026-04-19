# 0782 — witness/provider schema route actual adoption

## Objective

shared-space line の later mixed gate を 1 本進め、
`witness/provider route actual adoption` と
`witness/provider public-schema coupled-later gate`
を prior floor にしたまま、
witness/provider schema route first を helper-local actual-adoption floor へ actualize する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- final public witness schema / final public provider receipt schema / combined provider+witness public contract / final emitted-handoff contract は今回 fixed しない。
- representative corpus は `p07 / p08 / p09` reached、`p05` guard-only に置く。
- schema-route actualization は repo-local emitted-artifact refs first を維持し、combined public contract は candidate keep に留める。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md`
- `specs/examples/499-current-l2-witness-provider-public-schema-coupled-later-gate.md`
- `specs/examples/502-current-l2-witness-provider-route-actual-adoption.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. shared-space line の current floor を再読し、public-schema candidate only と route actual adoption の間に残っていた schema-route actual-adoption cut を独立 package として切り出した。
2. RED 先行で `crates/mir-runtime/tests/current_l2_witness_provider_schema_route_actual_adoption.rs` を追加し、未実装 struct/builder import による compile failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に
   - `CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption`
   - `build_current_l2_source_sample_witness_provider_schema_route_actual_adoption`
   - witness/provider schema-route actual-adoption helper 群
   を追加した。
4. `specs/examples/504-current-l2-witness-provider-schema-route-actual-adoption.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を actual-adoption package として固定した。
5. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `docs/research_abstract/` を同期し、traceability addendum を追加した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_schema_route_actual_adoption`
  - unresolved imports
    - `CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption`
    - `build_current_l2_source_sample_witness_provider_schema_route_actual_adoption`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_schema_route_actual_adoption`
  - `4 passed`
- repo-wide validation:
  - `cargo test -p mir-runtime`
  - rerun after docs sync
- regression:
  - `python3 scripts/current_l2_source_sample_regression.py regression`
- docs validation:
  - `python3 scripts/validate_docs.py`
  - rerun after adding this report
- diff hygiene:
  - `git diff --check`

## What changed in understanding

- shared-space line の next ratchet は、public-schema candidate only を増やすことではなく、schema-route first を repo-local actual-adoption cut に上げることだった。
- `p07 / p08 / p09` reached と `p05` guard-only の corpus で、witness-schema candidate keep / provider-receipt candidate keep / combined public-contract candidate keep を route-first current recommendation として固定できる。
- current package を final public witness/provider schema adoption と読まずに済ませるには、combined public contract 側を `candidate keep` に留め、final emitted-handoff contract を adjacent keep に止めるのが必要だった。

## Open questions

- final public witness schema と final public provider receipt schema を同一 package で close するか、分けるか。
- delegated provider attestation と combined provider+witness public contract をどこまで同時に扱うか。
- final emitted-handoff contract を shared-space final public contract package と同一 reopen に置くか、order-handoff emitted-artifact schema 側と合わせて扱うか。
- exhaustive shared-space catalog を current route/schema cut と混ぜずにどこで reopen するか。

## Suggested next prompt

`witness/provider schema route actual adoption の次として、final public witness schema / provider receipt schema / combined public contract / final emitted-handoff contract の reopen 順を narrow に比較し、必要なら helper-local actualization を追加してください。`
