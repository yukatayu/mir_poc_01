# 0780 — witness/provider route actual adoption

## Objective

shared-space line の later mixed gate を 1 本進め、
`witness/provider public-shape actual adoption` と
`witness/provider public-schema coupled-later gate`
を prior floor にしたまま、
repo-local `witness/provider route first` を helper-local actual-adoption floor へ actualize する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- final public witness schema / provider receipt schema / combined provider+witness public contract / final emitted-handoff contract は今回 fixed しない。
- representative corpus は `p07 / p08 / p09` reached、`p05` guard-only に置く。
- mainline public surface は widen せず、test/support helper と docs-first actual-adoption package に留める。

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
- `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md`
- `specs/examples/499-current-l2-witness-provider-public-schema-coupled-later-gate.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. `witness/provider public-shape actual adoption` と `witness/provider public-schema coupled-later gate` の gap を再読し、route-first actual adoption cut を current package として切り出した。
2. RED 先行で `crates/mir-runtime/tests/current_l2_witness_provider_route_actual_adoption.rs` を追加し、未実装 struct/builder import による compile failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に
   - `CurrentL2SourceSampleWitnessProviderRouteActualAdoption`
   - `build_current_l2_source_sample_witness_provider_route_actual_adoption`
   - route/schema/default/compare/guard/kept-later helper
   を追加した。
4. `specs/examples/502-current-l2-witness-provider-route-actual-adoption.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を actual-adoption package として固定した。
5. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `docs/research_abstract/` を同期し、traceability addendum も追加した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_route_actual_adoption`
  - unresolved imports
    - `CurrentL2SourceSampleWitnessProviderRouteActualAdoption`
    - `build_current_l2_source_sample_witness_provider_route_actual_adoption`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_route_actual_adoption`
  - `4 passed`
- repo-wide validation:
  - `cargo test -p mir-runtime`
  - green
  - representative new suite:
    - `current_l2_witness_provider_route_actual_adoption`
    - `4 passed`
- regression:
  - `python3 scripts/current_l2_source_sample_regression.py regression`
  - `all regression commands passed`
- docs validation:
  - `python3 scripts/validate_docs.py`
  - rerun after adding this report
- diff hygiene:
  - `git diff --check`

## What changed in understanding

- shared-space line の next useful ratchet は、`public schema candidate only` をさらに比較で引っ張ることではなく、`witness/provider route first` を repo-local actual-adoption cut に上げることだった。
- `p08` は witness/provider route 自体が空でも、repo-local emitted-artifact route と combined public-contract candidate keep を持つ baseline として actual-adoption floor に残してよい。
- `witness/provider route actual adoption` を追加しても、final public witness/provider schema adoption や exhaustive shared-space catalog adoption を prematurely imply しない。

## Open questions

- final public witness schema をどの package で actual adoption judgment に上げるか。
- final public provider receipt schema と delegated provider attestation を分離したまま進めるか、同一 package で判断するか。
- combined provider+witness public contract / final emitted-handoff contract をどの mixed gate で close するか。
- exhaustive shared-space catalog を minimal subset からどの時点で reopen するか。

## Suggested next prompt

`shared-space witness/provider route actual adoption の次として、final public witness/provider schema と combined public contract のどちらを先に narrow するかを source-backed に比較しつつ、必要なら helper-local actualization を追加してください。`
