# 501 — current L2 model-check checker-artifact route actual adoption

## 目的

`specs/examples/488`、
`492`、
`495`、
`498`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- model-check checker-artifact route actual adoption
- row-local property route first
- checker-boundary contract anchor
- consumer-shaped checker-artifact candidate only
- migration candidate adjacent keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**model-check public-checker artifact / migration coupled-later gate を保ったまま、repo-local checker-artifact route first を helper-local actual adoption として actualize する current cut**
であり、

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. model-check row-local property / checker-boundary actual adoption
   - row-local property route first
   - checker-boundary contract first
2. model-check public-checker artifact preview / verifier-handoff reserve actualization
   - consumer-shaped artifact preview only
   - verifier-handoff reserve keep
   - brand-neutral tool-binding reserve keep
3. model-check tool-brand / verifier-handoff coupled later gate
   - tool-brand candidate only
   - verifier-handoff candidate only
4. model-check public-checker artifact / migration coupled later gate
   - consumer-shaped artifact candidate only
   - migration candidate only
   - tool-brand / verifier-handoff adjacent keep

したがって current open problem は、
model-check line を artifact preview や artifact/migration coupled-later gate で止め続けることではなく、
**repo-local checker-artifact route first を current recommendation に上げつつ、tool-brand / migration / verifier-handoff / runtime-policy contract を still later に残せるか**
である。

## current actual adoption cut

current package では、次を採る。

1. checker-artifact route は
   - row-local property route first
   - checker-boundary contract anchor
   - consumer-shaped checker-artifact candidate only
   - repo-local emitted artifact refs first
   - final public checker artifact later
   に置く
2. migration side は
   - verifier-handoff candidate adjacent keep
   - tool-brand candidate adjacent keep
   - actual public checker migration candidate only
   - runtime-policy contract later
   に置く
3. reached sample は `e5 / p06 / p07 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_model_check_checker_artifact_route_actual_adoption` | `e5 / p06 / p07 / p09` reached、`p05` guard-only の actual-adoption manifest を machine-check する |
| `current_l2_model_check_public_checker_artifact_migration_coupled_later_gate` | artifact/migration coupled-later gate を prior floor として carry-over する |
| `current_l2_model_check_public_checker_artifact_preview_actualization` | consumer-shaped checker-artifact preview / verifier-handoff reserve keep を prior floor として carry-over する |
| `current_l2_model_check_tool_brand_verifier_handoff_coupled_later_gate` | tool-brand / verifier-handoff adjacent keep を prior floor として carry-over する |

## actualization shape

current helper-local actual-adoption manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `checker_artifact_route_refs`
- `migration_candidate_keep_refs`
- `actual_adoption_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### checker artifact route refs

current helper-local cut では、checker-artifact route を

- `model_check_checker_artifact_actual_route:<subject_ref>:row_local_property_route_first`
- `model_check_checker_artifact_actual_route:<subject_ref>:checker_boundary_contract_anchor`
- `model_check_checker_artifact_actual_route:<subject_ref>:consumer_shaped_checker_artifact_candidate_only`
- `model_check_checker_artifact_actual_route:<subject_ref>:repo_local_emitted_artifact_refs_first`
- `model_check_checker_artifact_actual_route:<subject_ref>:final_public_checker_artifact_later`

として actualize する。

これは final public checker artifact ではない。

### migration candidate keep refs

current helper-local cut では、migration keep side を

- `model_check_checker_artifact_migration_keep:<subject_ref>:verifier_handoff_candidate_adjacent_keep`
- `model_check_checker_artifact_migration_keep:<subject_ref>:tool_brand_candidate_adjacent_keep`
- `model_check_checker_artifact_migration_keep:<subject_ref>:actual_public_checker_migration_candidate_only`
- `model_check_checker_artifact_migration_keep:<subject_ref>:runtime_policy_contract_later`

として actualize する。

これは actual public checker migration でも actual emitted verifier handoff artifact でもない。

## current recommendation

1. model-check line は、row-local property route first / checker-boundary contract anchor を保ったまま、repo-local checker-artifact route first を current recommendation に上げてよい。
2. current package は
   - row-local property route first
   - checker-boundary contract anchor
   - consumer-shaped checker-artifact candidate only
   - migration candidate adjacent keep
   に置くのが自然である。
3. `e5 / p06 / p07 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract に昇格させない。

## retained alternatives

- final public checker artifact first adoption
- actual public checker migration first adoption
- concrete model-check tool brand first adoption
- emitted verifier handoff artifact first adoption
- production checker/runtime-policy contract first adoption
- unified public checker / migration / handoff public bundle

## stop line

この package の先でまだ止めるものは次である。

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

## non-goal

この package は、

- final public checker artifact adoption
- actual public checker migration adoption
- actual emitted verifier handoff artifact adoption
- production checker/runtime-policy contract adoption
- final public verifier contract adoption

ではない。
