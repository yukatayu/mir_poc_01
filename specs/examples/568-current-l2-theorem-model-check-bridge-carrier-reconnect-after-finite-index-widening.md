# 568. current L2 theorem/model-check bridge carrier reconnect after finite-index widening

## 目的

`specs/examples/566` と `specs/examples/567` により、

- source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16`
- `CurrentL2FiniteIndexFirstLayer.lean`
- representative generated theorem stub corpus `p15 / p16`

は source-backed に actualize 済みである。

この文書の目的は、その widening を compare-floor に戻さず、

- theorem result-object / theorem public-seam helper mirror
- model-check public-checker / model-check public-seam helper mirror
- model-check second-line / row-local carrier actual adoption

へ **無理のない current reconnect** として落とし直すことである。

ここで actualize するのは helper-local / repo-local bridge visibility だけであり、

- final public theorem result object
- final public checker artifact
- first settled property language
- concrete theorem/model-check production binding
- final public verifier contract

は fixed しない。

## current first line

current repo では、次を current first line に置く。

1. theorem side
   - representative theorem quartet `e5 / p06 / p07 / p08` を theorem result-object preview / reopen-threshold の reached 集合に保つ
   - `p10 / p11 / p12 / p15 / p16` は theorem public seam に premature に入れず、
     checker-adjacent + Lean-first theorem bridge floor に留める
2. model-check side
   - public-checker preview / reopen-threshold の representative checker quartet `e5 / p06 / p07 / p09` は維持する
   - ただし row-local carrier / property-tool threshold / row-local actual adoption は
     source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` まで widened してよい
3. helper-local CLI
   - theorem/model-check public seam summary が guard の場合でも、
     その手前で到達済みの bridge floor を `bridge_floor_refs` として sample-visible にしてよい

## actualization cut

current package では、次を actualize してよい。

### theorem helper mirror

- `theorem_result_object_preview`
- `theorem_final_public_contract_reopen_threshold`

は representative theorem quartet の outside では `guarded_not_reached` に保つ。

ただし source-side first strong typing sample set については、
次の bridge floor を `bridge_floor_refs` として出してよい。

- `typed_checker_hint_preview`
- finite-index / IFC source compare floor
- `proof_notebook_review_unit:*:rollback_cut_non_interference`
- `theorem_binding_preflight:*:rollback_cut_non_interference`
- `theorem_lean_stub_pilot:*:lean_first_principal`
- `repo_local_emitted_artifact:lean_theorem_stub:*:rollback_cut_non_interference`

これは theorem public seam の reached を意味しない。

### model-check helper mirror

- `model_check_public_checker_preview`
- `model_check_final_public_contract_reopen_threshold`

は representative checker quartet の outside では `guarded_not_reached` に保つ。

ただし source-side first strong typing sample set については、
次の reached bridge floor を `bridge_floor_refs` として出してよい。

- `compare_floor:current_l2.model_check_projection_prefloor`
- `compare_floor:current_l2.model_check.second_line_concretization`
- `small_cluster_projection:*:runtime_try_cut_local`
- `model_check_request_preflight:*:row_local_property_preview`
- `model_check_property_route:*:row_local_preview_bundle`
- `model_check_checker_contract_route:*:checker_boundary_contract_anchor`

これは public-checker preview や final public-contract reopen threshold の reached を意味しない。

### model-check second-line carrier widening

current package では、

- `current_l2_model_check_second_line_concretization`
- `current_l2_model_check_row_local_property_actual_adoption`

の actual runnable evidence に
`p15-typed-capture-escape-rejected` と
`p16-typed-remote-call-budget-exceeded`
を加えてよい。

これにより、source-side finite-index widening と
model-check row-local carrier line の drift を閉じる。

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -p mir-runtime --test current_l2_model_check_second_line_concretization` | `p15 / p16` が second-line carrier まで reached であることを machine-check する |
| `cargo test -p mir-runtime --test current_l2_model_check_row_local_property_actual_adoption` | `p15 / p16` が row-local property actual adoption まで reached であることを machine-check する |
| `cargo test -p mir-runtime --test current_l2_operational_cli` | `p15 / p16` の theorem/model-check helper summary が still guarded でありつつ、bridge floor を `bridge_floor_refs` で可視化することを machine-check する |

## current recommendation

1. `p15 / p16` を theorem public seam や model-check public-checker preview の reached 集合に混ぜない。
2. theorem side は checker-adjacent + Lean-first theorem bridge floor を visible にする。
3. model-check side は row-local carrier / property-tool / checker-boundary bridge floor を visible にする。
4. `p10 / p11 / p12 / p15 / p16` を source-side first strong typing sample setとして保ちつつ、
   theorem/model-check public seam と collapse しない。

## retained alternatives

- theorem representative quartet を widening して `p10 / p11 / p12 / p15 / p16` を theorem result-object preview reached に入れる
- model-check public-checker representative quartet を widening して `p10 / p11 / p12 / p15 / p16` を public-checker preview reached に入れる
- theorem/model-check bridge floor を helper-local `bridge_floor_refs` で出さず、docs-only のままに留める
- stronger typed source principal を theorem/model-check public seam widening と同時に進める

## stop line

この package の先で still later に残すものは次である。

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- final public verifier contract

## next line

current package を close した後の active queue は、

1. Package 95 order/handoff source surface and artifacts
2. Package 96 authoritative-room first scenario
3. Package 97 reserve strengthening
4. Package 98 documentation/report closeout

として読むのが自然である。
