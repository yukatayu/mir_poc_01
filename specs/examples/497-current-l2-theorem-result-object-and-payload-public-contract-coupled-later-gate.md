# 497 — current L2 theorem result object and payload public contract coupled later gate

## 目的

`specs/examples/487`、
`491`、
`494`
と
`e5-underdeclared-lineage`、
`samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
および historical compare-floor note としての
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- final theorem result-object candidate side
- consumer-shaped payload public-contract candidate side
- notebook-consumer object-first keep
- proof-object-schema / prover-brand adjacent keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**theorem result-object preview actualization と theorem proof-object-schema / prover-brand coupled later gate を保ったまま、final theorem result-object side と consumer-shaped payload public-contract side を adjacent but distinct later gate として helper-local manifest に actualize する current cut**
であり、

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem review-unit transport / notebook-contract actual adoption
   - review-unit transport first
   - notebook-consumer contract first
2. theorem result-object preview / proof-object-schema reserve actualization
   - notebook-consumer object first
   - consumer-shaped payload preview only
   - proof-object-schema reserve keep
3. theorem proof-object-schema / prover-brand coupled later gate
   - proof-object-schema candidate only
   - prover-brand candidate only
   - final public contract later

したがって current open problem は、
theorem line を result-object preview actualization や proof-object-schema/prover-brand coupled-later gate で止め続けることではなく、
**final theorem result-object side と consumer-shaped payload public-contract side をどこまで current recommendation に上げるか**
である。

## current coupled-later cut

current package では、次を採る。

1. theorem result-object side は
   - notebook-consumer object first
   - review-unit transport anchor
   - repo-local emitted artifact refs first
   - final public result object later
   に置く
2. payload public-contract side は
   - consumer-shaped payload preview keep
   - notebook-consumer contract first
   - consumer-shaped payload candidate only
   - proof-object-schema / prover-brand adjacent but not collapsed
   に置く
3. current live subject は `e5-underdeclared-lineage` に取り、`05_delegated_rng_service` を runtime-adjacent compare floor に置く。`p05 / p06 / p07 / p08` は historical compare anchor としてのみ残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support` | theorem result-object side / payload public-contract side を adjacent but distinct に読む coupled-later gate が current review-unit / formal-hook floor の上に乗っていることを再確認する |
| `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder` | `e5-underdeclared-lineage` が current theorem-side source-backed subject として green であり、result/payload coupled-later reading が accepted current-L2 source corpus と乖離していないことを再確認する |
| `cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe` | current live bridge は non-production Lean stub までで止まり、final public theorem result object / payload public contract へは上がっていないことを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | clean-near-end representative output が theorem obligations / layer signatures を持ち、runtime-private compare floor を維持している |

以下の result/payload coupled-later ref names は helper-local / repository-memory names であり、current CLI や test がそのまま field 名として expose しているわけではない。

## actualization shape

current helper-local coupled-later manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `result_object_candidate_refs`
- `payload_public_contract_candidate_refs`
- `coupled_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### result object candidate refs

current helper-local cut では、theorem result-object side を

- `theorem_final_result_candidate:<subject_ref>:notebook_consumer_object_first`
- `theorem_final_result_candidate:<subject_ref>:review_unit_transport_anchor`
- `theorem_final_result_candidate:<subject_ref>:repo_local_emitted_artifact_refs_first`
- `theorem_final_result_candidate:<subject_ref>:final_public_result_object_later`

として actualize する。

これは final public theorem result object ではない。

### payload public-contract candidate refs

current helper-local cut では、payload public-contract side を

- `theorem_payload_public_contract_candidate:<subject_ref>:consumer_shaped_payload_preview_keep`
- `theorem_payload_public_contract_candidate:<subject_ref>:notebook_consumer_contract_first`
- `theorem_payload_public_contract_candidate:<subject_ref>:consumer_shaped_payload_candidate_only`
- `theorem_payload_public_contract_candidate:<subject_ref>:proof_object_schema_prover_brand_adjacent_not_collapsed`

として actualize する。

これは consumer-shaped theorem payload public contract ではない。

## current recommendation

1. theorem line は、final theorem result-object side と consumer-shaped payload public-contract side を coupled-later gate として current recommendation に上げてよい。
2. current package は
   - notebook-consumer object first
   - consumer-shaped payload candidate only
   - proof-object-schema / prover-brand adjacent keep
   - final public verifier contract later
   に置くのが自然である。
3. current live subject を `e5` に限定し、`05_delegated_rng_service` を runtime-adjacent compare floor、`p05 / p06 / p07 / p08` を historical compare anchor に留める読みは semantically honest である。
4. current package を final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contract に昇格させない。

## retained alternatives

- final public theorem result object first adoption
- consumer-shaped theorem payload public contract first adoption
- concrete theorem prover brand first adoption
- proof object public schema first adoption
- unified theorem result / payload / proof-object public contract

## stop line

current package は次で止める。

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

## current kept-later mixed-gate status

theorem line の remaining mixed gate は、

1. final public theorem result object / consumer-shaped theorem payload public contract
2. concrete theorem prover brand / proof object public schema
3. final public verifier contract

として kept-later に残す。
