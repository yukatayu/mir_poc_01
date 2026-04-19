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
7. reached sample は `e5 / p06 / p07 / p08` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_theorem_discharge_actual_format_probe` | `e5 / p06 / p07 / p08` reached、`p05` guard-only の actual-format probe manifest を machine-check する |
| `current_l2_theorem_discharge_prefloor` | actual-format probe が row-local review-unit / reserve floor の上に乗っていることを再確認する |
| `current_l2_theorem_first_pilot_actualization` | symbolic evidence refs / repo-local emitted artifact refs を probe がそのまま受け継いでいることを再確認する |
| `current_l2_theorem_prover_binding_preflight` | theorem-first external integration target は brand-neutral preflight に留まり、actual-format probe と collapse しないことを再確認する |
| `current_l2_source_sample_runner` / `current_l2_operational_cli` | representative runtime / static corpus の runnable floor 自体は引き続き green である |

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
3. `e5 / p06 / p07 / p08` reached、`p05` guard-only の組み合わせは semantically honest である。
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

## next line

current package を close した後も、
principal self-driven package は増やさない。

next reopen line は、

1. settled property language / concrete tool seam
2. actual discharge transport / public theorem contract
3. final public witness schema / provider receipt optional attachment

の mixed gate として扱うのが自然である。
