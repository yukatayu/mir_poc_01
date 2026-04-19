# 0781 — order-handoff source wording route actual adoption

## Objective

order-handoff line の later mixed gate を 1 本進め、
`order-handoff surface actual adoption` と
`order-handoff source-wording / emitted-artifact coupled-later gate`
を prior floor にしたまま、
principal source wording route first を helper-local actual-adoption floor へ actualize する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- final parser grammar / final public parser-checker-runtime API / final source-surface handoff wording / final emitted-artifact schema は今回 fixed しない。
- representative corpus は `p07 / p08` reached、`p05` guard-only に置く。
- surface narrowing は edge-row principal を保ち、stage-block は secondary に留める。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md`
- `specs/examples/496-current-l2-order-handoff-source-wording-and-emitted-artifact-coupled-later-gate.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. order-handoff line の current floor を再読し、`source-wording candidate only` を compare-floor に留めず、principal source wording route first を actual-adoption package に切り出した。
2. RED 先行で `crates/mir-runtime/tests/current_l2_order_handoff_source_wording_route_actual_adoption.rs` を追加し、未実装 struct/builder import による compile failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に
   - `CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption`
   - `build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption`
   - source wording / emitted keep / default / compare / guard / kept-later helper
   を追加した。
4. `specs/examples/503-current-l2-order-handoff-source-wording-route-actual-adoption.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を actual-adoption package として固定した。
5. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `docs/research_abstract/` を同期し、traceability addendum を追加した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_order_handoff_source_wording_route_actual_adoption`
  - unresolved imports
    - `CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption`
    - `build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_order_handoff_source_wording_route_actual_adoption`
  - `3 passed`
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

- order-handoff line の next ratchet は、`source-wording candidate only` をさらに comparison doc として増やすことではなく、principal source wording route first を helper-local actual-adoption cut に上げることだった。
- `p07 / p08` reached と `p05` guard-only の corpus で、edge-row principal / readable relation vocabulary keep / stage-block secondary keep / emitted-artifact schema candidate keep を semantically honest に固定できる。
- current package を final source wording adoption と読まずに済ませるには、emitted-artifact side を `candidate keep` に留めるのが必要だった。

## Open questions

- final source-surface handoff wording をどの package で actual adoption judgment に上げるか。
- final emitted-artifact schema と final emitted-handoff contract を分離して扱うか、同一 package で close するか。
- authoritative-room `serial` sugar と low-level exact source wording をどこまで reserve に留めるか。
- final modal foundation adoption と final source marker をどの mixed gate で判断するか。

## Suggested next prompt

`order-handoff source wording route actual adoption の次として、final source-surface handoff wording と final emitted-artifact schema のどちらを先に narrow するかを source-backed に比較しつつ、必要なら helper-local actualization を追加してください。`
