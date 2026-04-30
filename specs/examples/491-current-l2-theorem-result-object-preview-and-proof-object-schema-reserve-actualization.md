# 491 — current L2 theorem result-object preview and proof-object-schema reserve actualization

## 目的

`specs/examples/470`、
`474`、
`479`、
`481`、
`485`、
`486`、
`487`
と
`e5-underdeclared-lineage`、
`samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
および historical compare-floor note としての
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- theorem result-object preview actualization
- notebook-consumer object-first cut
- consumer-shaped payload preview only
- proof-object-schema reserve keep
- brand-neutral binding reserve keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**theorem review-unit transport / notebook-contract actual adoption を保ったまま、repo-local theorem result-object preview と proof-object-schema reserve を helper-local manifest に actualize する current cut**
であり、

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem-first pilot actualization
   - `proof_notebook_review_unit` principal
   - symbolic `evidence_refs` only
   - repo-local emitted artifact refs
2. theorem-prover experimental binding preflight
   - brand-neutral theorem request manifest
   - adapter-boundary refs
3. theorem discharge actual-format probe
   - transport preview
   - public-contract preview
   - notebook-consumer-first boundary
4. theorem contract shape threshold default
   - refs-only reserve schema first
   - review-unit transport anchor
   - consumer-shaped theorem payload later
5. theorem transport/public-contract coupled later gate
   - transport/public-contract adjacent but distinct
   - review-unit anchor keep
   - refs-only reserve schema first
6. theorem review-unit transport / notebook-contract actual adoption
   - review-unit transport first
   - notebook-consumer contract first
   - brand-neutral binding reserve keep

したがって current open problem は、
theorem line を review-unit transport actual adoption で止め続けることではなく、
**repo-local result-object preview と proof-object-schema reserve をどこまで current recommendation に上げるか**
である。

## current actualization cut

current package では、次を採る。

1. result-object route は
   - notebook-consumer object first
   - review-unit anchor bundle
   - consumer-shaped payload preview only
   - repo-local emitted artifact refs
   に置く
2. payload-preview side は
   - notebook-consumer first
   - review-unit reference bundle
   - consumer-shaped payload preview only
   - proof-object-schema later
   に置く
3. proof-object-schema reserve side は
   - brand-neutral binding keep
   - proof-object-schema later
   - final public verifier contract later
   に置く
4. result-object route と proof-object-schema reserve は adjacent に読むが、collapse しない
5. current live subject は `e5-underdeclared-lineage` に取り、`05_delegated_rng_service` を runtime-adjacent compare floor に置く。`p05 / p06 / p07 / p08` は historical compare anchor としてのみ残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support` | notebook-consumer object-first / proof-object-schema reserve reading が current review-unit / formal-hook floor の上に乗っていることを再確認する |
| `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder` | `e5-underdeclared-lineage` が current theorem-side source-backed subject として green であり、result-object preview reading が accepted current-L2 source corpus と乖離していないことを再確認する |
| `cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe` | current live bridge は non-production Lean stub までで止まり、final public theorem result object / proof object public schema へは上がっていないことを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | clean-near-end representative output が theorem obligations / layer signatures を持ち、runtime-private compare floor を維持している |

以下の result-object preview manifest ref names は helper-local / repository-memory names であり、current CLI や test がそのまま field 名として expose しているわけではない。

## actualization shape

current helper-local actualization manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `result_object_route_refs`
- `notebook_payload_preview_refs`
- `proof_object_schema_reserve_refs`
- `actual_adoption_default_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### result object route refs

current helper-local cut では、result-object route を

- `theorem_result_object_route:<subject_ref>:notebook_consumer_object_first`
- `theorem_result_object_route:<subject_ref>:review_unit_anchor_bundle`
- `theorem_result_object_route:<subject_ref>:consumer_shaped_payload_preview_only`
- `theorem_result_object_route:<subject_ref>:repo_local_emitted_artifact_refs`

として actualize する。

これは final public theorem result object ではない。

### payload preview refs

current helper-local cut では、payload-preview route を

- `theorem_result_payload_preview:<subject_ref>:notebook_consumer_first`
- `theorem_result_payload_preview:<subject_ref>:review_unit_reference_bundle`
- `theorem_result_payload_preview:<subject_ref>:consumer_shaped_payload_preview_only`
- `theorem_result_payload_preview:<subject_ref>:proof_object_public_schema_later`

として actualize する。

これは consumer-shaped theorem payload public contract ではない。

### proof object schema reserve refs

current helper-local cut では、proof-object-schema reserve を

- `proof_object_schema_reserve:brand_neutral_binding_keep`
- `proof_object_schema_reserve:proof_object_public_schema_later`
- `proof_object_schema_reserve:final_public_verifier_contract_later`

として actualize する。

これは proof object public schema adoption や final public verifier contract ではない。

## current recommendation

1. theorem line は、review-unit transport first / notebook-consumer contract first を保ったまま、repo-local theorem result-object preview first を current recommendation に上げてよい。
2. current package は
   - notebook-consumer object first
   - consumer-shaped payload preview only
   - proof-object-schema reserve keep
   - brand-neutral binding reserve keep
   に置くのが自然である。
3. current live subject を `e5` に限定し、`05_delegated_rng_service` を runtime-adjacent compare floor、`p05 / p06 / p07 / p08` を historical compare anchor に留める読みは semantically honest である。
4. current package を final public theorem result object、proof object public schema、final public verifier contract に昇格させない。

## retained alternatives

- final public theorem result object first adoption
- consumer-shaped theorem payload public contract first adoption
- proof object public schema first adoption
- concrete theorem prover brand first adoption
- unified theorem result / proof object payload

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
