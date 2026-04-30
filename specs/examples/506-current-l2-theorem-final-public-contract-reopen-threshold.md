# 506 — current L2 theorem final public-contract reopen threshold

## 目的

`specs/examples/494`、
`497`、
`500`
と
`e5-underdeclared-lineage`、
`samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
および historical compare-floor note としての
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- theorem final public-contract reopen threshold
- result-object and payload first
- prover-brand and proof-schema second
- final public verifier contract third
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで固定するのは、
**theorem result-object route actual adoption と theorem proof-object schema / prover-brand coupled-later gate を保ったまま、remaining theorem final public-contract gate の reopen 順を helper-local threshold として actualize する current cut**
であり、

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem proof-object schema / prover-brand coupled-later gate
   - proof-object-schema candidate only
   - prover-brand candidate only
   - final public contract later
2. theorem result object / payload public-contract coupled-later gate
   - final theorem result-object candidate only
   - consumer-shaped theorem payload public-contract candidate only
   - proof-object-schema / prover-brand adjacent keep
3. theorem result-object route actual adoption
   - review-unit transport first
   - notebook-consumer object first
   - repo-local emitted artifact refs first
   - consumer-shaped payload preview keep
   - proof-object-schema / prover-brand later

したがって current open problem は、
remaining theorem final public-contract gate を unordered mixed gate として残すことではなく、
**result-object-and-payload first / prover-brand-and-proof-schema second / final public verifier contract third の reopen 順を current recommendation に上げつつ、final public theorem contract adoption そのものは still later に残せるか**
である。

## current threshold cut

current package では、次を採る。

1. first reopen pair は
   - final public theorem result object
   - consumer-shaped theorem payload public contract
   に置く
2. second reopen pair は
   - concrete theorem prover brand
   - proof object public schema
   に置く
3. third reopen は
   - final public verifier contract
   に置く
4. current live subject は `e5-underdeclared-lineage` に取り、`05_delegated_rng_service` を runtime-adjacent compare floor に置く。`p05 / p06 / p07 / p08` は historical compare anchor としてのみ残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support` | result-object/payload first、prover-brand/proof-schema second の reopen threshold reading が current review-unit / formal-hook floor の上に乗っていることを再確認する |
| `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder` | `e5-underdeclared-lineage` が current theorem-side source-backed subject として green であり、reopen-threshold reading が accepted current-L2 source corpus と乖離していないことを再確認する |
| `cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe` | current live bridge は non-production Lean stub までで止まり、final public theorem result object / public contract reopen 自体には入っていないことを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | clean-near-end representative output が theorem obligations / layer signatures を持ち、runtime-private compare floor を維持している |

以下の reopen-threshold ref names は helper-local / repository-memory names であり、current CLI や test がそのまま field 名として expose しているわけではない。

## threshold shape

current helper-local threshold manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `result_object_route_refs`
- `payload_preview_keep_refs`
- `proof_object_schema_candidate_refs`
- `prover_brand_candidate_refs`
- `final_public_contract_reopen_sequence_refs`
- `threshold_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### reopen sequence refs

current helper-local cut では、reopen 順を

- `theorem_final_public_contract_reopen:<subject_ref>:result_object_and_payload_first`
- `theorem_final_public_contract_reopen:<subject_ref>:prover_brand_and_proof_schema_second`
- `theorem_final_public_contract_reopen:<subject_ref>:final_public_verifier_contract_third`

として actualize する。

これは final public theorem contract adoption ではない。

## current recommendation

1. theorem final public-contract line は、result-object and payload first を current recommendation に上げてよい。
2. current package は
   - result-object and payload first
   - prover-brand and proof-schema second
   - final public verifier contract third
   に置くのが自然である。
3. current live subject を `e5` に限定し、`05_delegated_rng_service` を runtime-adjacent compare floor、`p05 / p06 / p07 / p08` を historical compare anchor に留める読みは semantically honest である。
4. current package を final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contract に昇格させない。

## retained alternatives

- final public theorem result object first single adoption
- consumer-shaped theorem payload public contract first single adoption
- concrete theorem prover brand first adoption
- proof object public schema first adoption
- final public verifier contract first adoption
- unified theorem result / payload / proof-object / verifier public contract

## stop line

この package の先でまだ止めるものは次である。

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

## non-goal

この package は、

- final public theorem result object adoption
- consumer-shaped theorem payload public contract adoption
- concrete theorem prover brand adoption
- proof object public schema adoption
- final public verifier contract adoption

ではない。
