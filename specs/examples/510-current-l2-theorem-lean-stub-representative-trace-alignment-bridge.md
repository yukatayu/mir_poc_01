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
**repo-local theorem artifact-conformance bridge を representative prototype corpus `p06 / p07 / p08` まで伸ばし、`e2 / e5` と同じ pair alignment cut で machine-check する bridge**
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
   - `e5 / p06 / p07 / p08` reached
   - `p05` guard-only
2. theorem review-unit to Lean-stub repo-local artifact-conformance bridge
   - `e2 / e5` representative source sample coverage
   - regression-integrated compare floor
3. prototype preview / theorem pilot helpers
   - prototype-side `proof_notebook_review_units`
   - prototype-side `lean_stub_artifacts`
   - guard-only reading for unreached rows

したがって current open problem は、
prototype trace alignment を compare-floor の外に置くことではなく、
**reached representative prototype corpus に対して review unit / Lean stub pair alignment を helper-local actualization floor に上げられるか**
である。

## current actualization cut

current package では、次を採る。

1. representative corpus は
   - runtime source sample `e2`
   - static source sample `e5`
   - typed prototype `p06`
   - order-handoff prototypes `p07 / p08`
   - guard-only contrast `p05`
   に置く
2. alignment predicate は
   - same `theorem_trace_alignment_pair:<subject_ref>:<obligation_kind>`
   - review unit side と Lean stub side が exact match
   に置く
3. actualization surface は
   - runtime support builder
   - focused runtime test
   に留める
4. public theorem contract / proof object public schema には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge` | preview route と Lean stub pilot route を束ね、representative corpus の pair alignment を helper-local に actualize する runtime support |
| `current_l2_theorem_lean_stub_trace_alignment_bridge` | `e2 / e5 / p06 / p07 / p08` reached、`p05` guard-only の representative theorem trace alignment bridge を machine-check する focused runtime test |
| `theorem_trace_alignment_pair:*` refs | review unit side と Lean stub side の obligation-pair alignment を subject-local に固定する repo-local ref family |

## current recommendation

1. theorem review-unit / Lean-stub line は、representative prototype corpus まで trace alignment bridge を actualize してよい。
2. reached representative corpus では review unit pair refs と Lean stub pair refs の exact match を first predicate に置く。
3. `p05` は guard-only contrast として保持する。
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
