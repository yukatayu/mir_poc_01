# 478 — current L2 model-check second-line concretization

## 目的

`specs/examples/415`、
`420`、
`464`、
`474`、
`475`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- model-check second-line package
- row-local property preview
- small-cluster semantic projection second line
- brand-neutral model-check request preflight
- public-checker second reserve split
- helper-local actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**row-local machine-facing carrier を principal に維持したまま、property preview / request preflight / public-checker reserve split を helper-local second line に actualize する current cut**
であり、

- first settled property language
- concrete model-check tool brand / tool schema
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker / runtime-policy contract
- room protocol / fairness / provider receipt family の collapse

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 1 actual adoption package
   - checker-adjacent principal
   - theorem-first external integration target
   - row-local model-check carrier first
2. model-check projection pre-floor
   - row-local carrier
   - small-cluster semantic projection reserve
   - property-language seam / tool-binding seam distinction
3. theorem-first pilot / theorem binding preflight
   - theorem line は notebook-first / brand-neutral preflight に留める
   - model-check lineと collapse しない
4. property-family reserve order
   - row-local `canonical_normalization_law`
   - row-local `no_re_promotion`
   - row-local `rollback_cut_non_interference`
   を current floor に置き、
   small-cluster semantic relation family を next reserve に置く
5. historical package reading では
   - `e5` static reached anchor
   - `p05` guarded not reached anchor
   - `p06 / p07 / p08 / p09` runtime reached anchors
   を使っていたが、current runnable floor は clean-near-end model-check family へ置き換える

したがって current open problem は、
model-check line を discovery することではなく、
**property-language / tool seam mixed gate の直前まで、何を helper-local actualization として閉じてよいか**
である。

## current actualization cut

current package では、次を採る。

1. principal source は引き続き row-local `model_check_concrete_carrier` に置く
2. row-local property family は
   - `canonical_normalization_law`
   - `no_re_promotion`
   - `rollback_cut_non_interference`
   の preview refs として actualize し、
   final settled property language には上げない
3. small-cluster semantic projection は second line に残し、
   room protocol / fairness / provider receipt family とは collapse しない
4. concrete tool brand ではなく、
   subject-bound brand-neutral model-check request preflight までを helper-local に actualize する
5. public-checker side は
   - payload schema
   - API read relation
   - command surface
   - shared output contract
   - boundary
   - verifier handoff surface
   の second reserve refs として explicit に残す
6. actual runnable reached floor は current clean-near-end model-check family
   `01_peterson_sc_pass / 02_peterson_relaxed_counterexample / 03_broken_mutex_counterexample`
   に取り、historical `e5 / p05 / p06 / p07 / p08 / p09` labels は current runnable target に使わない
7. order-handoff side を adjacent に読む場合でも provider / fairness family は excluded family に保ち、
   model-check second line と shared-space stronger claim を collapse しない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run model-check --format json` | clean near-end model-check family の representative runtime inventory を読み、active second-line runtime floor を doc-level に再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | canonical runtime inventory と current emitted rows を与える。projection pre-floor relation 自体は helper-local / doc-level judgment に残り、closeout が直接 emitted するわけではない |
| `python3 scripts/current_l2_lean_sample_sync.py` | theorem-first line と model-check second line を sibling adjacent cut として読むための proof-bridge sync anchor を与える。1:1 theorem-discharge target ではない |
| `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | active clean-near-end representative runtime / static corpus floor が compatibility front door から green であることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-current-l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json` | clean model-check sample を CLI-shaped current surface からも読めることを再確認する。ただし old helper manifest の 1:1 proof surface ではない |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | delegated provider practical line が actualized 済みでも、model-check second line は provider / fairness family を excluded に保てることを読むための adjacent runtime evidence を与える |

## actualization shape

current helper-local second-line manifest は少なくとも次を持つ。

- `concretization_subject_kind` / `concretization_subject_ref`
- `principal_machine_carrier_refs`
- `row_local_property_preview_refs`
- `secondary_projection_refs`
- `request_preflight_refs`
- `public_checker_reserve_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `excluded_family_refs`
- `guard_refs`
- `kept_later_refs`

### row-local property preview refs

current helper-local cut では、row-local property preview を

- `property_preview:row_local:<subject_ref>:canonical_normalization_law`
- `property_preview:row_local:<subject_ref>:no_re_promotion`
- `property_preview:row_local:<subject_ref>:rollback_cut_non_interference`

の family として actualize する。

これは final property-language surface ではなく、
current carrier floor から next mixed gate へ橋を掛ける helper-local preview である。

### request preflight refs

current helper-local cut では、brand-neutral request preflight を

- `model_check_request_preflight:<subject_ref>:row_local_property_preview`
- `model_check_request_preflight:<subject_ref>:small_cluster_semantic_projection`

として actualize する。

これは concrete tool brand / concrete schema adoption ではない。

### public-checker second reserve refs

current helper-local cut では、public-checker side を

- `public_checker_second_reserve:payload_schema`
- `public_checker_second_reserve:api_read_relation`
- `public_checker_second_reserve:command_surface`
- `public_checker_second_reserve:shared_output_contract`
- `public_checker_second_reserve:boundary`
- `public_checker_second_reserve:verifier_handoff_surface`

として explicit reserve に残す。

これは actual public checker migration や final public verifier contract ではない。

## current recommendation

1. model-check second line は self-driven package として close してよい。
2. current cut は
   - row-local carrier principal
   - row-local property preview only
   - brand-neutral request preflight only
   - public-checker chain docs-only reserve
   に置くのが自然である。
3. historical package reading としての `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。ただし current runnable floor は clean-near-end model-check family に置く。
4. adjacent order-handoff evidence として `05_delegated_rng_service` を読んでも provider receipt / fairness family を excluded に保つことで、
   Problem 2 stronger line と Problem 1 second line を collapse しない。

## retained alternatives

- property-language surface first adoption
- concrete model-check tool brand first adoption
- actual public-checker migration first
- room protocol / fairness projection first
- consumer-shaped model-check schema first

## stop line

current package は次で止める。

- first settled property language
- concrete model-check tool brand / tool schema
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker / runtime-policy contract
- room protocol / fairness / provider receipt family の model-check side collapse

## next line

current package を close した後は、
live な principal self-driven package はいったん空になる。

ただし、これは theory solved を意味しない。

next reopen line は、

1. theorem discharge public-contract actual format
2. settled property language / concrete tool seam
3. final public witness schema / provider receipt optional attachment

の mixed gate として扱うのが自然である。
