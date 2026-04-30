# 479 — current L2 theorem discharge actual-format probe

## 目的

`specs/examples/440`、
`446`、
`465`、
`470`、
`474`、
`475`
と
`e5-underdeclared-lineage`、
`samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
および historical compare-floor note としての
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- theorem discharge actual-format probe
- transport preview
- public-contract preview
- notebook-consumer-first boundary
- brand-neutral contract probe
- helper-local actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**row-local `proof_notebook_review_unit` principal と theorem-first pilot actualization を保ったまま、actual discharge transport / public theorem contract の final adoption に入る直前の actual-format probe を helper-local に置く current cut**
であり、

- actual discharge transport
- public theorem contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 1 actual adoption package
   - checker-adjacent principal
   - theorem-first external integration target
   - notebook-first theorem line
2. theorem discharge pre-floor
   - row-local review unit
   - abstract discharge-entry reserve
   - transport seam / public-contract seam distinction
3. theorem-first pilot actualization
   - symbolic `evidence_refs`
   - repo-local emitted artifact refs
4. theorem-prover experimental binding preflight
   - brand-neutral theorem request preflight
   - no concrete theorem brand
5. principal theory spine / Lean-first proof roadmap
   - theorem line を final public theorem contract へ premature promotion しない layered reading

したがって current open problem は、
theorem line を発明することではなく、
**actual discharge transport / public theorem contract mixed gate の直前まで、何を helper-local actualization として閉じてよいか**
である。

## current actualization cut

current package では、次を採る。

1. principal source は引き続き row-local `proof_notebook_review_unit` に置く
2. abstract discharge-entry reserve は actual-format probe の adjacent reserve に残す
3. transport side は
   - `theorem_discharge_transport_preview:<subject_ref>:<obligation_kind>`
   の preview refs として actualize し、
   actual transport contract へは上げない
4. public-contract side は
   - `theorem_public_contract_preview:<subject_ref>:review_unit_first`
   - `theorem_public_contract_preview:<subject_ref>:discharge_entry_adjacent`
   の preview refs に留め、
   public theorem contract へは上げない
5. consumer boundary は
   - notebook consumer first
   - abstract discharge entry only
   - brand-neutral contract probe
   の 3 ref に明示的に分ける
6. repo-local emitted artifact refs は theorem-first pilot actualization と同じ floor に保つ
7. current live subject は `e5-underdeclared-lineage` に取り、`05_delegated_rng_service` を runtime-adjacent compare floor に置く。`p05 / p06 / p07 / p08` は historical compare anchor としてのみ残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support` | row-local review-unit principal と formal-hook obligation rows が actual-format probe の current live floor であることを再確認する |
| `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder` | `e5-underdeclared-lineage` が current theorem-side source-backed subject として green であり、probe が runtime / static current floor と乖離していないことを再確認する |
| `cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe` | actual-format probe の downstream でも、現在の live bridge は non-production Lean stub までで止まり、dedicated public theorem transport symbol へは上がっていないことを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | clean-near-end representative output が theorem obligations / layer signatures を持ち、runtime-private compare floor を維持している |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | active clean suite と committed Lean roots が current bridge floor として closeout で再確認できる |

以下の `transport_preview_refs` / `public_contract_preview_refs` / `consumer_boundary_refs` は repository-memory 上の helper-local names であり、current CLI や test がそのまま field 名として expose しているわけではない。

## actualization shape

current helper-local probe manifest は少なくとも次を持つ。

- `probe_subject_kind` / `probe_subject_ref`
- `principal_review_unit_refs`
- `discharge_entry_reserve_refs`
- `symbolic_evidence_refs`
- `transport_preview_refs`
- `public_contract_preview_refs`
- `consumer_boundary_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### transport preview refs

current helper-local cut では、transport preview を

- `theorem_discharge_transport_preview:<subject_ref>:canonical_normalization_law`
- `theorem_discharge_transport_preview:<subject_ref>:no_re_promotion`
- `theorem_discharge_transport_preview:<subject_ref>:rollback_cut_non_interference`

の family として actualize する。

これは actual transport contract ではなく、
review-unit first line から public-contract mixed gate へ渡す preview floor である。

### public-contract preview refs

current helper-local cut では、public-contract preview を

- `theorem_public_contract_preview:<subject_ref>:review_unit_first`
- `theorem_public_contract_preview:<subject_ref>:discharge_entry_adjacent`

として actualize する。

これは final public theorem contract や proof object public schema ではない。

### consumer boundary refs

current helper-local cut では、consumer boundary を

- `consumer_boundary:notebook_consumer_first`
- `consumer_boundary:abstract_discharge_entry_only`
- `consumer_boundary:brand_neutral_contract_probe`

として actualize する。

これは theorem result を public theorem object として final adoption する cut ではない。

## current recommendation

1. theorem discharge mixed gate では、actual-format probe を self-driven mixed-gate helper cut として close してよい。
2. current cut は
   - row-local review unit principal
   - abstract discharge-entry reserve keep
   - transport preview only
   - public-contract preview only
   - notebook-consumer-first boundary
   に置くのが自然である。
3. current live subject を `e5` に限定し、`05_delegated_rng_service` を runtime-adjacent compare floor、`p05 / p06 / p07 / p08` を historical compare anchor に留める読みは semantically honest である。
4. theorem-first external integration target は brand-neutral preflight と actual-format probe を両立させるが、
   actual transport / public theorem contract adoption へは still later に留める。

## retained alternatives

- actual discharge transport first adoption
- public theorem contract first adoption
- theorem result object first adoption
- concrete theorem prover brand first adoption
- proof object public schema first adoption

## stop line

current package は次で止める。

- actual discharge transport
- public theorem contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

## historical package-local next reopen line

historical closeout memory では、
current package を close した後も
principal self-driven package を増やさない読みを保持してよい。

historical next reopen line としては、

1. settled property language / concrete tool seam
2. actual discharge transport / public theorem contract
3. final public witness schema / provider receipt optional attachment

の mixed gate として扱うのが自然である。

current queue authority は `progress.md` / `tasks.md` に残し、
この文書では historical reopen memory を active queue と混同しない。
