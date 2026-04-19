# 480 — current L2 model-check property-language and tool-seam probe

## 目的

`specs/examples/415`、
`420`、
`464`、
`475`、
`478`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- model-check property-language / tool-seam probe
- row-local property preview carry-over
- brand-neutral tool-seam probe
- checker-boundary probe
- helper-local actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**row-local `model_check_concrete_carrier` principal と second-line concretization を保ったまま、first settled property language / concrete tool seam mixed gate の直前に置く property-language / tool-seam probe を helper-local に actualize する current cut**
であり、

- first settled property language
- concrete model-check tool brand / schema
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker / runtime-policy contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 1 actual adoption package
   - checker-adjacent principal
   - row-local model-check carrier first
2. model-check projection pre-floor
   - row-local carrier
   - small-cluster projection reserve
   - property-language seam / tool-binding seam distinction
3. model-check second-line concretization
   - row-local property preview
   - brand-neutral request preflight
   - public-checker second reserve split
4. principal theory spine / Lean-first proof roadmap
   - theorem/model-check bridge line を public contract に premature promotion しない layered reading

したがって current open problem は、
model-check line を発明することではなく、
**first settled property language / concrete tool seam mixed gate の直前まで、何を helper-local actualization として閉じてよいか**
である。

## current actualization cut

current package では、次を採る。

1. principal source は引き続き row-local `model_check_concrete_carrier` に置く
2. row-local property preview と small-cluster projection second line はそのまま carry-over する
3. property-language side は
   - `property_language_probe:row_local:<subject_ref>:<obligation_kind>`
   の probe refs として actualize し、
   first settled property language へは上げない
4. tool-seam side は
   - `tool_seam_probe:<subject_ref>:brand_neutral_model_check_request`
   - `tool_seam_probe:<subject_ref>:small_cluster_semantic_projection`
   の probe refs に留め、
   concrete tool brand / schema へは上げない
5. checker side は
   - row-local property preview first
   - brand-neutral tool probe only
   - public checker contract later
   の boundary probe refs に分ける
6. repo-local emitted artifact refs は second-line concretization と同じ floor に保つ
7. reached sample は `e5 / p06 / p07 / p08 / p09` に取り、`p05` は guard-only に残す
8. theorem discharge actual-format、room protocol projection、provider receipt / fairness family は excluded family に保つ

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_model_check_property_tool_seam_probe` | `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の property-language / tool-seam probe manifest を machine-check する |
| `current_l2_model_check_second_line_concretization` | property/tool-seam probe が second-line concretization の上に乗っていることを再確認する |
| `current_l2_model_check_projection_prefloor` | row-local carrier / small-cluster projection reserve floor を維持していることを再確認する |
| `current_l2_theorem_discharge_actual_format_probe` | theorem line と model-check line が sibling mixed-gate probe として保てることを再確認する |
| `current_l2_source_sample_runner` / `current_l2_operational_cli` | representative runtime / static corpus の runnable floor 自体は引き続き green である |

## actualization shape

current helper-local probe manifest は少なくとも次を持つ。

- `probe_subject_kind` / `probe_subject_ref`
- `principal_machine_carrier_refs`
- `row_local_property_preview_refs`
- `secondary_projection_refs`
- `property_language_probe_refs`
- `tool_seam_probe_refs`
- `checker_boundary_probe_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `excluded_family_refs`
- `guard_refs`
- `kept_later_refs`

### property-language probe refs

current helper-local cut では、property-language probe を

- `property_language_probe:row_local:<subject_ref>:canonical_normalization_law`
- `property_language_probe:row_local:<subject_ref>:no_re_promotion`
- `property_language_probe:row_local:<subject_ref>:rollback_cut_non_interference`

の family として actualize する。

これは first settled property language ではなく、
row-local carrier first line から tool-seam mixed gate へ橋を掛ける probe floor である。

### tool-seam probe refs

current helper-local cut では、tool-seam probe を

- `tool_seam_probe:<subject_ref>:brand_neutral_model_check_request`
- `tool_seam_probe:<subject_ref>:small_cluster_semantic_projection`

として actualize する。

これは concrete model-check tool brand / schema ではない。

### checker-boundary probe refs

current helper-local cut では、checker boundary を

- `checker_boundary_probe:row_local_property_preview_first`
- `checker_boundary_probe:brand_neutral_tool_probe_only`
- `checker_boundary_probe:public_checker_contract_later`

として actualize する。

これは actual public checker migration や final public verifier contract ではない。

## current recommendation

1. model-check property-language / tool-seam mixed gate では、property/tool-seam probe を self-driven mixed-gate helper cut として close してよい。
2. current cut は
   - row-local carrier principal
   - property-language probe only
   - brand-neutral tool-seam probe only
   - checker-boundary probe only
   に置くのが自然である。
3. `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. theorem discharge actual-format と property/tool-seam probe を sibling helper cut に保つことで、
   theorem line と model-check line の mixed gate を public contract 前で narrow に切り分けられる。

## retained alternatives

- first settled property language first adoption
- concrete model-check tool brand / schema first adoption
- actual public checker migration first
- room protocol / fairness projection first
- consumer-shaped public checker schema first

## stop line

current package は次で止める。

- first settled property language
- concrete model-check tool brand / schema
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker / runtime-policy contract

## next line

current package を close した後も、
principal self-driven package は増やさない。

next reopen line は、

1. actual discharge transport / public theorem contract
2. first settled property language / concrete model-check tool brand
3. final public witness schema / provider receipt optional attachment

の mixed gate として扱うのが自然である。
