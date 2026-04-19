# 495 — current L2 model-check tool-brand and verifier-handoff coupled later gate

## 目的

`specs/examples/482`、
`488`、
`492`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- tool-brand coupled later gate
- verifier-handoff coupled later gate
- public-checker artifact preview keep
- brand-neutral request keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**model-check public-checker artifact preview actualization と model-check property/tool threshold default を保ったまま、tool-brand side と verifier-handoff side を adjacent but distinct later gate として helper-local manifest に actualize する current cut**
であり、

- first settled property language
- concrete model-check tool brand adoption
- final public checker artifact adoption
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. model-check property/tool threshold default
   - row-local property preview first
   - brand-neutral model-check request keep
   - public-checker contract later
2. model-check row-local property / checker-boundary actual adoption
   - row-local property route first
   - checker-boundary contract first
   - brand-neutral tool-binding reserve keep
3. model-check public-checker artifact preview / verifier-handoff reserve actualization
   - consumer-shaped artifact preview only
   - verifier-handoff reserve keep
   - brand-neutral tool-binding reserve keep
4. model-check second-line concretization
   - row-local property preview
   - request preflight
   - public-checker second reserve split

したがって current open problem は、
tool-brand / verifier-handoff line を reserve-only の later bundle に止めることではなく、
**tool-brand side と verifier-handoff side をどこまで coupled-later gate として current recommendation に上げるか**
である。

## current coupled-later cut

current package では、次を採る。

1. verifier-handoff side は
   - public-checker preview adjacent
   - emitted handoff artifact candidate
   - runtime-policy contract candidate
   に置く
2. tool-brand side は
   - brand-neutral request manifest keep
   - concrete tool-brand candidate
   - public-checker artifact not adopted
   に置く
3. verifier-handoff side と tool-brand side は adjacent に読むが、collapse しない
4. public-checker artifact preview keep は current compare floor として carry-over してよいが、
   final public checker artifact と同一視しない
5. reached sample は `e5 / p06 / p07 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_model_check_tool_brand_verifier_handoff_coupled_later_gate` | `e5 / p06 / p07 / p09` reached、`p05` guard-only の coupled-later manifest を machine-check する |
| `current_l2_model_check_public_checker_artifact_preview_actualization` | public-checker artifact preview keep と verifier-handoff reserve keep を compare floor として carry-over する |
| `current_l2_model_check_property_tool_threshold` | brand-neutral request keep と tool-brand threshold default を compare floor として carry-over する |
| `current_l2_model_check_row_local_property_actual_adoption` | row-local property route first / checker-boundary contract first を model-check source-backed floor として再確認する |
| `current_l2_source_sample_runner` / `current_l2_operational_cli` | representative runtime / static corpus の runnable floor 自体は引き続き green である |

## actualization shape

current helper-local coupled-later manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `verifier_handoff_candidate_refs`
- `tool_brand_candidate_refs`
- `coupled_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### verifier handoff candidate refs

current helper-local cut では、verifier-handoff side を

- `model_check_verifier_handoff_candidate:<subject_ref>:public_checker_preview_adjacent`
- `model_check_verifier_handoff_candidate:<subject_ref>:emitted_handoff_artifact_candidate`
- `model_check_verifier_handoff_candidate:<subject_ref>:runtime_policy_contract_candidate`

として actualize する。

これは actual emitted verifier handoff artifact ではない。

### tool brand candidate refs

current helper-local cut では、tool-brand side を

- `model_check_tool_brand_candidate:<subject_ref>:brand_neutral_request_manifest_keep`
- `model_check_tool_brand_candidate:<subject_ref>:concrete_tool_brand_candidate`
- `model_check_tool_brand_candidate:<subject_ref>:public_checker_artifact_not_adopted`

として actualize する。

これは concrete model-check tool brand adoption ではない。

## current recommendation

1. model-check line は、tool-brand side と verifier-handoff side を coupled-later gate として current recommendation に上げてよい。
2. current package は
   - public-checker artifact preview keep
   - verifier-handoff candidate only
   - tool-brand candidate only
   - final public contract later
   に置くのが自然である。
3. `e5 / p06 / p07 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を first settled property language、concrete model-check tool brand adoption、final public checker artifact adoption、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract に昇格させない。

## retained alternatives

- first settled property language first adoption
- concrete model-check tool brand first adoption
- final public checker artifact first adoption
- actual public checker migration first adoption
- emitted verifier handoff first adoption
- unified tool-brand / verifier-handoff / public-checker artifact public bundle

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
