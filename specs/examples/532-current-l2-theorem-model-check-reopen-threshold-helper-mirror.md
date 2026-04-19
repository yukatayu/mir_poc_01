# 532 — current L2 theorem/model-check reopen-threshold helper mirror

## 目的

`specs/examples/506`、
`507`、
`530`
と
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- theorem final public-contract reopen threshold
- model-check final public-contract reopen threshold
- `run-source-sample` helper summary
- reached / guard 非対称
- current recommendation
- kept-later public seam

を 1 本に束ねる。

ここで actualize するのは、
**docs 側で source-backed になっていた reopen-threshold package を `run-source-sample` helper summary に narrow に mirror する current cut**
であり、

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

は still later に残す。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem side
   - `specs/examples/506`
   - result-object and payload first
   - prover-brand and proof-schema second
   - final public verifier contract third
2. model-check side
   - `specs/examples/507`
   - property-language and tool-brand first
   - public-checker-artifact and migration second
   - verifier-handoff and runtime-policy-contract third
   - final public verifier contract fourth
3. helper-local operational CLI widening
   - `specs/examples/530`
   - theorem result-object preview helper mirror
   - model-check public-checker preview helper mirror
   - `p08` theorem reached / model-check guarded
   - `p09` theorem guarded / model-check reached

したがって current open problem は、
reopen-threshold docs を compare-floor に戻すことではなく、
**source-backed reopen order を `run-source-sample` helper summary でも sample-visible に保ち、remaining mixed gate を final public seam adoption だけに narrow に残せるか**
である。

## current helper mirror cut

current package では、次を採る。

1. theorem reopen-threshold helper mirror
   - reached sample は `e5 / p06 / p07 / p08`
   - `p09` は guard-only
   - `result_object_route_refs`
   - `payload_preview_keep_refs`
   - `proof_object_schema_candidate_refs`
   - `prover_brand_candidate_refs`
   - `final_public_contract_reopen_sequence_refs`
2. model-check reopen-threshold helper mirror
   - reached sample は `e5 / p06 / p07 / p09`
   - `p08` は guard-only
   - `checker_artifact_route_refs`
   - `migration_candidate_keep_refs`
   - `verifier_handoff_candidate_refs`
   - `tool_brand_candidate_refs`
   - `final_public_contract_reopen_sequence_refs`
3. current CLI wording は `helper_local_reopen_threshold_manifest` に留める
4. final public theorem/model-check contract adoption には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -p mir-runtime --test current_l2_operational_cli` | `run-source-sample` JSON/pretty summary に reopen-threshold helper mirror が現れることを machine-check する |
| `p06` | theorem/model-check の両 reopen-threshold helper mirror reached |
| `p08` | theorem reopen-threshold reached、model-check reopen-threshold guarded |
| `p09` | theorem reopen-threshold guarded、model-check reopen-threshold reached |

## current recommendation

1. theorem/model-check residual mixed gate は、まず reopen order を helper-local operational summary に mirror してよい。
2. `p08` と `p09` の非対称 reached/guard を collapse しないことが重要である。
3. theorem side は result-object and payload first を、model-check side は property-language and tool-brand first を current recommendation に保ったまま、public seam adoption は still later に残すのが自然である。
4. `run-source-sample` helper summary へ出したからといって、final public verifier contract や final public checker/theorem contract を fixed したとは読まない。

## retained alternatives

- theorem/model-check public seam compression だけを helper summary に出し、reopen-threshold を出さない cut
- theorem final public-contract reopen threshold だけを helper mirror し、model-check side は docs-only に残す cut
- model-check final public-contract reopen threshold だけを helper mirror し、theorem side は docs-only に残す cut
- unified theorem/model-check final public contract reopen order

## stop line

この package の先で still later に残すものは次である。

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

## non-goal

この package は、

- final public theorem/model-check contract adoption
- final public verifier contract adoption
- final parser grammar adoption
- final public parser/checker/runtime API adoption

ではない。
