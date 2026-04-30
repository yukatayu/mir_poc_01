# 510 — current L2 theorem Lean-stub representative trace-alignment bridge

## 目的

`specs/examples/508` と `509` を前提に、

- theorem Lean-stub representative trace alignment
- runtime/static/prototype representative corpus
- current recommendation
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**repo-local theorem artifact-conformance bridge の current live floor を `e2 / e5` に保ちつつ、historical `p06 / p07 / p08` trace-alignment widening を retained compare-floor memory に冷やす current cut**
であり、

- actual Lean tool execution
- prototype-wide exhaustive alignment
- public theorem contract
- proof object public schema
- cross-tool public artifact-conformance contract
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem Lean-first non-production stub pilot actualization
   - current live subject は `e5`
   - clean-near-end `05_delegated_rng_service` は runtime-adjacent compare floor
   - `p05 / p06 / p07 / p08` は historical compare anchor
2. theorem review-unit to Lean-stub repo-local artifact-conformance bridge
   - `e2 / e5` representative source sample coverage
   - regression-integrated compare floor
3. committed Lean bridge floor
   - `samples/lean/foundations/`
   - `samples/lean/clean-near-end/`
   - `samples/lean/manifest.json`

したがって current open problem は、
prototype trace alignment widening を current live runtime test と誤読させないまま、
**review unit / Lean stub pair alignment の live floor と historical compare-floor memory をどう分けて保つか**
である。

## current actualization cut

current package では、次を採る。

1. representative corpus は
   - runtime source sample `e2`
   - static source sample `e5`
2. runtime-adjacent compare floor は
   - clean-near-end `05_delegated_rng_service`
3. historical compare anchor は
   - typed prototype `p06`
   - order-handoff prototypes `p07 / p08`
   - historical guard anchor `p05`
   に置く
4. alignment predicate は
   - same `theorem_trace_alignment_pair:<subject_ref>:<obligation_kind>`
   - review unit side と Lean stub side が exact match
   に置く
5. actualization surface は
   - current review-unit / formal-hook / Lean-stub tests
   - committed `samples/lean/clean-near-end/` corpus
   に留める
6. public theorem contract / proof object public schema には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-semantics --test current_l2_formal_hook_support --test current_l2_proof_notebook_review_unit_support` | `e2 / e5` の review-unit / formal-hook floor が current live theorem bridge であることを再確認する |
| `cargo test -q -p mir-semantics --test current_l2_lean_theorem_stub_support --test current_l2_lean_theorem_stub_actual_probe` | review-unit から Lean stub への current live bridge と static `e5` actual probe を再確認する |
| `cargo test -q -p mir-runtime --test current_l2_source_sample_runner --test current_l2_source_sample_verification_ladder` | accepted current-L2 source corpus で `e2 / e5` bridge floor が current runtime/static floor と乖離していないことを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | clean-near-end representative output が theorem obligations / layer signatures を持ち、runtime-adjacent compare floor を維持している |
| `samples/lean/README.md` / `samples/lean/manifest.json` | committed Lean bridge floor は `foundations/` + `clean-near-end/` corpus であり、prototype-wide trace alignment current test ではない |

`theorem_trace_alignment_pair:*` refs は repository-memory / helper-local names であり、current test がそのまま live field 名として expose しているわけではない。

## current recommendation

1. theorem review-unit / Lean-stub line の current live bridge は `e2 / e5` と committed clean-near-end corpus に留めるのが自然である。
2. `05_delegated_rng_service` は runtime-adjacent compare floor として保持し、historical `p06 / p07 / p08` widening は compare-floor memory に冷やす。
3. `p05` は historical guard anchor として保持する。
4. actual Lean tool execution、prototype-wide exhaustive alignment、public theorem contract は still later に残す。

## retained alternatives

- source sample only で止める
- actual Lean execution first
- public theorem contract first
- proof object public schema first
- cross-tool public artifact-conformance contract first

## stop line

current package は次で止める。

- actual Lean tool execution
- prototype-wide exhaustive trace alignment
- public theorem contract
- proof object public schema
- cross-tool public artifact-conformance contract
- final public verifier contract

## non-goal

この package は、

- theorem discharge transport adoption
- concrete theorem prover brand adoption
- public theorem result object adoption
- consumer-shaped theorem payload public-contract adoption
- final public verifier contract adoption

ではない。
