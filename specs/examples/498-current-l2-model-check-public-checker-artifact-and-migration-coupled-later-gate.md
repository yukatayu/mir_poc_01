# 498 — current L2 model-check public checker artifact and migration coupled later gate

## 目的

`specs/examples/488`、
`492`、
`495`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- final public checker artifact candidate side
- actual public checker migration candidate side
- public-checker preview keep
- tool-brand / verifier-handoff adjacent keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**model-check public-checker artifact preview actualization と model-check tool-brand / verifier-handoff coupled later gate を保ったまま、final public checker artifact side と actual public checker migration side を adjacent but distinct later gate として helper-local manifest に actualize する current cut**
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
   - public-checker artifact preview keep
   - verifier-handoff candidate only
   - tool-brand candidate only
   - final public contract later

したがって current open problem は、
model-check line を public-checker preview actualization や tool-brand/verifier-handoff coupled-later gate で止め続けることではなく、
**final public checker artifact side と actual public checker migration side をどこまで current recommendation に上げるか**
である。

## current coupled-later cut

current package では、次を採る。

1. final public checker artifact side は
   - consumer-shaped artifact preview keep
   - checker-boundary contract anchor
   - repo-local emitted artifact refs first
   - final public checker artifact later
   に置く
2. actual public checker migration side は
   - verifier-handoff candidate keep
   - tool-brand candidate adjacent keep
   - actual public checker migration candidate only
   - runtime-policy contract adjacent but not collapsed
   に置く
3. reached sample は `e5 / p06 / p07 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_model_check_public_checker_artifact_migration_coupled_later_gate` | `e5 / p06 / p07 / p09` reached、`p05` guard-only の coupled-later manifest を machine-check する |
| `current_l2_model_check_public_checker_artifact_preview_actualization` | public-checker preview keep / checker-boundary anchor を compare floor として carry-over する |
| `current_l2_model_check_tool_brand_verifier_handoff_coupled_later_gate` | verifier-handoff/tool-brand adjacent keep を compare floor として carry-over する |
| `current_l2_model_check_row_local_property_actual_adoption` | row-local property route first / checker-boundary contract first を model-check-side source-backed floor として再確認する |

## actualization shape

current helper-local coupled-later manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `public_checker_artifact_candidate_refs`
- `checker_migration_candidate_refs`
- `coupled_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### public checker artifact candidate refs

current helper-local cut では、final public checker artifact side を

- `model_check_final_public_checker_candidate:<subject_ref>:consumer_shaped_artifact_preview_keep`
- `model_check_final_public_checker_candidate:<subject_ref>:checker_boundary_contract_anchor`
- `model_check_final_public_checker_candidate:<subject_ref>:repo_local_emitted_artifact_refs_first`
- `model_check_final_public_checker_candidate:<subject_ref>:final_public_checker_artifact_later`

として actualize する。

これは final public checker artifact ではない。

### checker migration candidate refs

current helper-local cut では、actual public checker migration side を

- `model_check_checker_migration_candidate:<subject_ref>:verifier_handoff_candidate_keep`
- `model_check_checker_migration_candidate:<subject_ref>:tool_brand_candidate_adjacent_keep`
- `model_check_checker_migration_candidate:<subject_ref>:actual_public_checker_migration_candidate_only`
- `model_check_checker_migration_candidate:<subject_ref>:runtime_policy_contract_adjacent_not_collapsed`

として actualize する。

これは actual public checker migration でも actual emitted verifier handoff artifact でもない。

## current recommendation

1. model-check line は、final public checker artifact side と actual public checker migration side を coupled-later gate として current recommendation に上げてよい。
2. current package は
   - consumer-shaped artifact candidate only
   - actual public checker migration candidate only
   - tool-brand / verifier-handoff adjacent keep
   - final public verifier contract later
   に置くのが自然である。
3. `e5 / p06 / p07 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract に昇格させない。

## retained alternatives

- final public checker artifact first adoption
- actual public checker migration first adoption
- tool-brand-first adoption
- emitted verifier handoff artifact first adoption
- production checker/runtime-policy contract first adoption

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
