# 509 — current L2 theorem review-unit to Lean-stub repo-local artifact-conformance bridge

## 目的

`specs/examples/470`、
`474`、
`475`、
`508`
を前提に、

- theorem review-unit to Lean-stub repo-local artifact-conformance bridge
- authored current-L2 representative runtime/static sample coverage
- reproducible regression integration
- current recommendation
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**actual Lean tool execution を要求せずに、source sample -> formal hook -> proof notebook review unit -> Lean stub artifact の pair alignment を repo-local に再現する bridge**
であり、

- actual Lean tool execution
- prototype-wide trace alignment
- public theorem contract
- proof object public schema
- cross-tool public artifact-conformance contract
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem-first external integration target
   - theorem route は theorem-first を current default に取る
   - notebook-first review unit principal を保つ
2. theorem Lean-first non-production stub pilot actualization
   - review-unit first
   - brand-neutral preflight anchor keep
   - Lean-first non-production stub only
3. Lean-first proof roadmap
   - artifact-conformance bridge は second stage
   - actual Lean tool execution / public proof contract は later
4. current authored sample regression floor
   - runtime/static representative source samplesを repo-local helper で再現できる

したがって current open problem は、
artifact conformance を abstract comparison に留めることではなく、
**runtime/static representative source sample を使って review unit と Lean stub artifact の pair alignment を reproducible compare floor に actualize できるか**
である。

## current actualization cut

current package では、次を採る。

1. representative runtime/static sample は
   - `e2-try-fallback`
   - `e5-underdeclared-lineage`
   に置く
2. repo-local pipeline は
   - source sample regression inventory
   - detached formal-hook smoke
   - `current_l2_emit_proof_notebook_review_unit`
   - `current_l2_emit_lean_theorem_stub`
   の 4 段で束ねる
3. conformance predicate は
   - same `(subject_ref, obligation_kind)` pair
   - `tool_family = lean-first`
   - non-empty `source_text`
   に置く
4. reproducible compare floor は
   - dedicated pipeline helper
   - unit tests
   - `scripts/current_l2_source_sample_regression.py regression`
   の 3 点で支える

## actual runnable evidence

| evidence | current reading |
|---|---|
| `scripts/current_l2_theorem_lean_stub_pipeline.py` | representative source sample 1 件に対して formal hook / review unit / Lean stub emission と pair conformance を実行する repo-local helper |
| `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py` | runtime/static planning、guarded rejection、pair drift rejection を unit test する |
| `scripts/current_l2_source_sample_regression.py` | authored-sample regression bundleの末尾に theorem artifact-conformance bridge を組み込み、`e2` / `e5` representative coverage を one-command floor に上げる |
| `scripts/tests/test_current_l2_source_sample_regression.py` | regression helper が theorem artifact-conformance commands を含むことを planning/CLI test で固定する |
| `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e2-try-fallback ...` | runtime representative sample で review unit 1 件と Lean stub 1 件の matched pair を実行確認する |
| `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e5-underdeclared-lineage ...` | static representative sample で review unit 2 件と Lean stub 2 件の matched pair を実行確認する |
| `python3 scripts/current_l2_source_sample_regression.py regression ...` | 22-command regression bundleの一部として theorem artifact-conformance bridge が green であることを再確認する |

## current recommendation

1. theorem artifact-conformance bridge は、actual Lean tool execution より前の repo-local second stage として actualize してよい。
2. principal transport anchor は review-unit first に維持する。
3. representative runtime/static sample coverage を regression bundle に繋いで、reproducible compare floor に上げてよい。
4. actual Lean tool execution、prototype-wide trace alignment、public theorem contract は still later に残す。

## retained alternatives

- Lean tool execution first
- prototype-wide / room-wide trace alignment first
- public theorem contract first
- proof object public schema first
- cross-tool public artifact-conformance contract first

## stop line

current package は次で止める。

- actual Lean tool execution
- `p06 / p07 / p08` 全面 trace alignment
- concrete theorem prover brand adoption
- public theorem contract
- proof object public schema
- cross-tool public artifact-conformance contract
- final public verifier contract

## non-goal

この package は、

- theorem discharge transport adoption
- final public theorem result object adoption
- consumer-shaped theorem payload public-contract adoption
- proof object public schema adoption
- concrete theorem prover production binding adoption
- final public verifier contract adoption

ではない。
