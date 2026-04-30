# 486 — current L2 theorem transport/public-contract coupled later gate

## 目的

`specs/examples/446`、
`479`、
`481`、
`485`
と
`e5-underdeclared-lineage`、
`samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
および historical compare-floor note としての
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- theorem transport/public-contract coupled later gate
- adjacent but distinct seam keep
- review-unit anchor keep
- refs-only reserve schema first
- consumer-shaped payload later
- helper-local actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**actual discharge transport seam と public theorem contract seam を distinct に保ったまま adjacent later gate として helper-local に actualize する current cut**
であり、

- actual discharge transport adoption
- public theorem contract adoption
- theorem result public object
- consumer-shaped theorem payload
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem discharge transport/public-contract later-gate framing
   - coupled later gate
   - transport seam と public-contract seam を distinct に keep
2. theorem discharge actual-format probe
   - transport preview
   - public-contract preview
3. theorem discharge / public-theorem-contract threshold default
   - review-unit first
   - discharge-entry adjacent
4. theorem contract shape threshold default
   - refs-only reserve schema first
   - consumer-shaped payload later

したがって current open problem は、
theorem transport/public-contract line を actual adoption へ上げるかどうかではなく、
**adjacent but distinct later gate をどこまで helper-local に actualize してよいか**
である。

## current actualization cut

current package では、次を採る。

1. transport side は
   - review-unit anchor
   - discharge-entry adjacent
   - symbolic evidence refs only
   - repo-local emitted artifact refs first
   に置く
2. public-contract side は
   - notebook-consumer adjacent
   - refs-only reserve schema
   - brand-neutral request manifest keep
   - consumer-shaped payload later
   に置く
3. transport seam と public-contract seam は adjacent に読むが、collapse しない
4. current live subject は `e5-underdeclared-lineage` に取り、`05_delegated_rng_service` を runtime-adjacent compare floor に置く。`p05 / p06 / p07 / p08` は historical compare anchor としてのみ残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support` | transport/public-contract seam を adjacent but distinct に読む coupled-later gate が current review-unit / formal-hook floor の上に乗っていることを再確認する |
| `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder` | `e5-underdeclared-lineage` が current theorem-side source-backed subject として green であり、coupled-later gate reading が accepted current-L2 source corpus と乖離していないことを再確認する |
| `cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe` | coupled-later gate の downstream でも、current live bridge は non-production Lean stub までで止まり、actual discharge transport / public theorem contract へは上がっていないことを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | clean-near-end representative output が theorem obligations / layer signatures を持ち、runtime-private compare floor を維持している |

以下の coupled-later manifest ref names は helper-local / repository-memory names であり、current CLI や test がそのまま field 名として expose しているわけではない。

## actualization shape

current helper-local manifest は少なくとも次を持つ。

- `actualization_subject_kind` / `actualization_subject_ref`
- `transport_candidate_refs`
- `public_contract_candidate_refs`
- `coupled_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### coupled default refs

current helper-local cut では、coupled default を

- `theorem_coupled_later_gate_default:transport_and_public_contract_adjacent_distinct`
- `theorem_coupled_later_gate_default:review_unit_anchor`
- `theorem_coupled_later_gate_default:refs_only_reserve_schema_first`
- `theorem_coupled_later_gate_default:proof_object_public_schema_later`

として actualize する。

これは actual adoption ではなく、
actual discharge transport seam と public theorem contract seam の coupled later gate を helper-local に固定する current cut である。

## current recommendation

1. theorem transport/public-contract line は、coupled later gate として helper-local actualization まで上げてよい。
2. current default は
   - transport/public-contract adjacent but distinct
   - review-unit anchor keep
   - refs-only reserve schema first
   - consumer-shaped payload later
   に置くのが自然である。
3. current live subject を `e5` に限定し、`05_delegated_rng_service` を runtime-adjacent compare floor、`p05 / p06 / p07 / p08` を historical compare anchor に留める読みは semantically honest である。
4. actual discharge transport adoption や public theorem contract adoption 自体は still later mixed gate に残し、この manifest を final public theorem contract や final public verifier contract に昇格させない。

## retained alternatives

- transport-first adoption
- public-contract-first adoption
- consumer-shaped theorem payload first adoption
- proof-object-public-schema first adoption
- concrete prover-facing payload first adoption

## stop line

current package は次で止める。

- actual discharge transport adoption
- public theorem contract adoption
- theorem result public object
- consumer-shaped theorem payload
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

## next line

theorem line の next mixed gate は、

1. actual discharge transport / public theorem contract actual adoption
2. theorem result public object / consumer-shaped theorem payload
3. concrete theorem prover brand / proof object public schema

として kept-later に残す。
