# 573 — current L2 Problem 1 public-seam residual bundle matrix

## 目的

Problem 1 の representative sample bundle `p06 / p10 / p11 / p12 / p15 / p16` について、theorem/model-check public seam residual を helper-local matrix として読めるようにする。

## current recommendation

- `p06-typed-proof-owner-handoff` を Problem 1 public-seam representative に置く。
- `p10 / p11 / p12 / p15 / p16` は checker-adjacent / Lean-first theorem bridge / row-local model-check bridge の residual bundle として読む。
- `typed_checker_hint_preview` は checker-adjacent first layer の到達度であり、theorem/model-check public seam そのものとして読まない。
- `theorem_result_object_preview`、`theorem_final_public_contract_reopen_threshold`、`model_check_public_checker_preview`、`model_check_final_public_contract_reopen_threshold` は final public contract ではなく helper-local residual matrix として読む。

## actualization

- `scripts/current_l2_guided_samples.py matrix problem1`
  - `p06` を `public-seam representative`
  - `p10 / p11 / p12 / p15 / p16` を `checker-adjacent bridge-floor`
  として sample-facing に表示してよい。
- pretty output は current recommendation を人間向けに示し、json output は repo-local compare / report 用に使ってよい。

## stop line

- final public theorem result object
- consumer-shaped theorem payload public contract
- first settled property language
- concrete theorem/model-check tool binding
- final public checker artifact
- final public verifier contract

## non-goals

- stronger typed surface を source principal へ昇格しない。
- final public theorem/model-check contract を採らない。
- concrete theorem prover brand / proof object public schema / final public checker artifact を凍らせない。

## evidence

| command | expectation |
|---|---|
| `python3 -m unittest scripts.tests.test_current_l2_guided_samples` | Problem 1 residual row classification と matrix text が固定される |
| `python3 scripts/current_l2_guided_samples.py matrix problem1` | `p06` representative と `p10 / p11 / p12 / p15 / p16` residual bridge-floor bundle が見える |
| `python3 scripts/current_l2_guided_samples.py matrix problem1 --format json` | six-row residual bundle が machine-readable に出る |

## rationale

- Package 94 までで `bridge_floor_refs` は source-backed に actualize 済みであり、remaining work は public seam residual の読み分けである。
- representative sample bundle matrix は compare-floor を増やさず residual mixed gate だけを narrow に読むのに十分である。
