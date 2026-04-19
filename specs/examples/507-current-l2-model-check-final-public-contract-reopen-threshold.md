# 507 — current L2 model-check final public-contract reopen threshold

## 目的

`specs/examples/495`、
`498`、
`501`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- model-check final public-contract reopen threshold
- property-language and tool-brand first
- public-checker-artifact and migration second
- verifier-handoff and runtime-policy-contract third
- final public verifier contract fourth
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで固定するのは、
**model-check checker-artifact route actual adoption と model-check tool-brand / verifier-handoff coupled-later gate を保ったまま、remaining model-check final public-contract gate の reopen 順を helper-local threshold として actualize する current cut**
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

1. model-check tool-brand / verifier-handoff coupled-later gate
   - verifier-handoff candidate only
   - tool-brand candidate only
   - final public contract later
   - next line:
     1. first settled property language / concrete model-check tool brand
     2. final public checker artifact / actual public checker migration
     3. actual emitted verifier handoff artifact / production checker-runtime-policy contract
     4. final public verifier contract
2. model-check public checker artifact / migration coupled-later gate
   - final public checker artifact candidate only
   - actual public checker migration candidate only
   - tool-brand / verifier-handoff adjacent keep
3. model-check checker-artifact route actual adoption
   - row-local property route first
   - checker-boundary contract anchor
   - consumer-shaped checker-artifact candidate only
   - migration candidate adjacent keep

したがって current open problem は、
remaining model-check final public-contract gate を unordered mixed gate として残すことではなく、
**property-language-and-tool-brand first / public-checker-artifact-and-migration second / verifier-handoff-and-runtime-policy-contract third / final public verifier contract fourth の reopen 順を current recommendation に上げつつ、final public model-check contract adoption そのものは still later に残せるか**
である。

## current threshold cut

current package では、次を採る。

1. first reopen pair は
   - first settled property language
   - concrete model-check tool brand
   に置く
2. second reopen pair は
   - final public checker artifact
   - actual public checker migration
   に置く
3. third reopen pair は
   - actual emitted verifier handoff artifact
   - production checker-runtime-policy contract
   に置く
4. fourth reopen は
   - final public verifier contract
   に置く
5. reached sample は `e5 / p06 / p07 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_model_check_final_public_contract_reopen_threshold` | `e5 / p06 / p07 / p09` reached、`p05` guard-only の threshold manifest を machine-check する |
| `current_l2_model_check_checker_artifact_route_actual_adoption` | checker-artifact route first / migration candidate adjacent keep を prior floor として carry-over する |
| `current_l2_model_check_tool_brand_verifier_handoff_coupled_later_gate` | tool-brand candidate only / verifier-handoff candidate only を prior floor として carry-over する |

## threshold shape

current helper-local threshold manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `checker_artifact_route_refs`
- `migration_candidate_keep_refs`
- `verifier_handoff_candidate_refs`
- `tool_brand_candidate_refs`
- `final_public_contract_reopen_sequence_refs`
- `threshold_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### reopen sequence refs

current helper-local cut では、reopen 順を

- `model_check_final_public_contract_reopen:<subject_ref>:property_language_and_tool_brand_first`
- `model_check_final_public_contract_reopen:<subject_ref>:public_checker_artifact_and_migration_second`
- `model_check_final_public_contract_reopen:<subject_ref>:verifier_handoff_and_runtime_policy_contract_third`
- `model_check_final_public_contract_reopen:<subject_ref>:final_public_verifier_contract_fourth`

として actualize する。

これは final public model-check contract adoption ではない。

## current recommendation

1. model-check final public-contract line は、property-language and tool-brand first を current recommendation に上げてよい。
2. current package は
   - property-language and tool-brand first
   - public-checker-artifact and migration second
   - verifier-handoff and runtime-policy-contract third
   - final public verifier contract fourth
   に置くのが自然である。
3. `e5 / p06 / p07 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker-runtime-policy contract、final public verifier contract に昇格させない。

## retained alternatives

- first settled property language first single adoption
- concrete model-check tool brand first single adoption
- final public checker artifact first single adoption
- actual public checker migration first single adoption
- actual emitted verifier handoff artifact first single adoption
- production checker-runtime-policy contract first single adoption
- final public verifier contract first single adoption
- unified model-check public checker / migration / handoff / verifier contract

## stop line

この package の先でまだ止めるものは次である。

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker-runtime-policy contract
- final public verifier contract

## non-goal

この package は、

- first settled property language adoption
- concrete model-check tool brand adoption
- final public checker artifact adoption
- actual public checker migration adoption
- actual emitted verifier handoff artifact adoption
- production checker-runtime-policy contract adoption
- final public verifier contract adoption

ではない。
