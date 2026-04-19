# 492 — current L2 model-check public-checker artifact preview and verifier-handoff reserve actualization

## 目的

`specs/examples/478`、
`480`、
`482`、
`488`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- public-checker artifact preview actualization
- checker-boundary bundle keep
- verifier-handoff reserve keep
- brand-neutral tool-binding reserve keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**model-check row-local property / checker-boundary actual adoption を保ったまま、repo-local public-checker artifact preview と verifier-handoff reserve を helper-local manifest に actualize する current cut**
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

1. model-check second-line concretization
   - row-local property preview
   - brand-neutral request preflight
   - public-checker second reserve split
2. model-check property/tool-seam probe
   - property-language probe
   - tool-seam probe
   - checker-boundary probe
3. model-check property/tool threshold default
   - row-local property preview first
   - small-cluster semantic projection second
   - brand-neutral model-check request keep
   - public-checker contract later
4. model-check row-local property / checker-boundary actual adoption
   - row-local property route first
   - checker-boundary contract first
   - brand-neutral tool-binding reserve keep

したがって current open problem は、
model-check line を row-local actual adoption で止めることではなく、
**repo-local public-checker artifact preview と verifier-handoff reserve をどこまで current recommendation に上げるか**
である。

## current actualization cut

current package では、次を採る。

1. public-checker artifact preview route は
   - consumer-shaped artifact preview only
   - checker-boundary bundle
   - row-local property route bundle
   - repo-local emitted artifact refs
   に置く
2. verifier-handoff reserve side は
   - public-checker migration later
   - emitted handoff artifact later
   - runtime-policy contract later
   に置く
3. tool-binding reserve side は
   - brand-neutral request manifest keep
   - concrete tool brand later
   - runtime-policy contract later
   に置く
4. public-checker artifact preview と verifier-handoff reserve は adjacent に読むが、collapse しない
5. reached sample は `e5 / p06 / p07 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_model_check_public_checker_artifact_preview_actualization` | `e5 / p06 / p07 / p09` reached、`p05` guard-only の actualization manifest を machine-check する |
| `current_l2_model_check_row_local_property_actual_adoption` | row-local property route first / checker-boundary contract first を compare floor として carry-over する |
| `current_l2_model_check_second_line_concretization` | row-local property preview / request preflight / public-checker second reserve split を compare floor として carry-over する |
| `current_l2_source_sample_runner` / `current_l2_operational_cli` | representative runtime / static corpus の runnable floor 自体は引き続き green である |

## actualization shape

current helper-local actualization manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `checker_artifact_preview_refs`
- `verifier_handoff_reserve_refs`
- `tool_binding_reserve_refs`
- `actual_adoption_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### checker artifact preview refs

current helper-local cut では、checker-artifact preview route を

- `model_check_public_checker_preview:<subject_ref>:consumer_shaped_artifact_preview_only`
- `model_check_public_checker_preview:<subject_ref>:checker_boundary_bundle`
- `model_check_public_checker_preview:<subject_ref>:row_local_property_route_bundle`
- `model_check_public_checker_preview:<subject_ref>:repo_local_emitted_artifact_refs`

として actualize する。

これは final public checker artifact ではない。

### verifier handoff reserve refs

current helper-local cut では、verifier-handoff reserve を

- `model_check_verifier_handoff_reserve:public_checker_migration_later`
- `model_check_verifier_handoff_reserve:emitted_handoff_artifact_later`
- `model_check_verifier_handoff_reserve:runtime_policy_contract_later`

として actualize する。

これは actual public checker migration や actual emitted verifier handoff artifact ではない。

## current recommendation

1. model-check line は、row-local property route first / checker-boundary contract first を保ったまま、repo-local public-checker artifact preview first を current recommendation に上げてよい。
2. current package は
   - consumer-shaped artifact preview only
   - verifier-handoff reserve keep
   - brand-neutral tool-binding reserve keep
   - runtime-policy contract later
   に置くのが自然である。
3. `e5 / p06 / p07 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final public checker artifact、actual public checker migration、final public verifier contract に昇格させない。

## retained alternatives

- final public checker artifact first adoption
- actual public checker migration first adoption
- actual emitted verifier handoff artifact first adoption
- concrete model-check tool brand first adoption
- unified public checker / verifier handoff artifact

## stop line

current package は次で止める。

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

## next line

model-check line の remaining mixed gate は、

1. first settled property language / concrete model-check tool brand
2. final public checker artifact / actual public checker migration
3. actual emitted verifier handoff artifact / production checker-runtime-policy contract
4. final public verifier contract

として kept-later に残す。
