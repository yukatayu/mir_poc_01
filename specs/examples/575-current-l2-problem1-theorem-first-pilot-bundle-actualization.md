# 575 — current L2 Problem 1 theorem-first pilot bundle actualization

## 目的

`specs/examples/466-current-l2-problem1-actual-adoption-package-and-theorem-first-pilot.md`、
`508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md`、
`509-current-l2-theorem-review-unit-to-lean-stub-repo-local-artifact-conformance-bridge.md`、
`568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`、
`572-current-l2-guided-problem-sample-entrypoints-and-runner.md`、
`573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
を前提に、

- guided sample `problem1`
- residual matrix `problem1`
- `samples/lean/current-l2/` representative artifact
- repo-local emitted theorem artifact route
- anchor になる spec / report

を、`bundle problem1` helper と sample README によって
**一本道の repo-local theorem-first pilot bundle**
として読む current cut を 1 本に束ねる。

ここで actualize するのは、
**Problem 1 の current first line と theorem-first pilot line を docs / helper / Lean artifact path まで bundle 化する current cut**
であり、

- final public theorem contract
- concrete theorem prover brand adoption
- final public verifier contract
- final public parser / checker / runtime API

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. Problem 1 actual adoption package
   - checker-adjacent first layer principal
   - theorem-first external integration target
   - notebook-first theorem line
   - row-local model-check carrier first
2. Lean-first non-production stub pilot
   - review-unit first
   - symbolic `evidence_refs` only
   - repo-local emitted stub artifact refs first
3. representative Lean sample corpus
   - `p06` は theorem-first representative sample
   - `p10 / p11 / p12 / p15 / p16` は checker-adjacent / bridge-floor 補助 sample
4. guided helper floor
   - `show problem1`
   - `run problem1`
   - `matrix problem1`

したがって current open problem は、
Problem 1 current first line そのものを追加比較することではなく、
**既に存在する representative sample / Lean artifact / residual matrix / spec-report anchor をどう一本道で辿れる形にするか**
である。

## current actualization cut

current package では、次を採る。

1. `scripts/current_l2_guided_samples.py`
   に `bundle problem1` を追加する
2. `bundle problem1` は
   - `show problem1`
   - representative sample 実行コマンド
   - `matrix problem1`
   - `run problem1 --all --format json`
   のおすすめ順を出す
3. representative sample として
   - `p06`
   - `samples/lean/current-l2/p06-typed-proof-owner-handoff/`
   を bundle の中心に置く
4. 補助 sample として
   - `p10 / p11 / p12 / p15 / p16`
   - 対応する `samples/lean/current-l2/`
   を同じ bundle 内で辿れるようにする
5. anchor docs / reports と stop line を bundle 本文に明示する

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py bundle problem1` | Problem 1 representative sample、Lean artifact、anchor docs / reports、stop line を 1 画面で辿れる |
| `python3 scripts/current_l2_guided_samples.py bundle problem1 --format json` | bundle manifest を repo-local machine-readable shape で inspect できる |
| `python3 -m unittest scripts.tests.test_current_l2_guided_samples` | bundle helper と residual matrix helper の current behavior を machine-check する |
| `samples/prototype/current-l2-typed-proof-model-check/README.md` | Problem 1 README から `bundle problem1` 導線を日本語で辿れる |

## current recommendation

1. Problem 1 theorem-first pilot line は、public theorem contract を採る前に
   guided helper / Lean artifact / spec-report anchor の bundle 化まで actualize してよい。
2. bundle の中心は `p06` に置き、
   `p10 / p11 / p12 / p15 / p16` は checker-adjacent bridge-floor 補助 bundle として読む。
3. bundle 化は adoption debt を減らすための repo-local helper actualization であり、
   concrete theorem prover brand や final public theorem contract を意味しない。

## retained alternatives

- `show` / `matrix` を separate のままにして bundle を足さない
- concrete theorem prover brand adoption を先に行う
- final public theorem contract を先に詰める
- exhaustive theorem sample catalog を先に作る

## stop line

current package は次で止める。

- final public theorem contract
- concrete theorem prover brand adoption
- final public verifier contract
- final public parser / checker / runtime API

## next self-driven line

current package を close した後の next self-driven line は、

1. experimental parser-side companion surface bundle
2. parser-side carrier から current helper summary への thin bridge

に置くのが自然である。
