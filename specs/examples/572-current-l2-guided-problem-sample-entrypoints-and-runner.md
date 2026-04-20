# 572 — current L2 guided problem sample entrypoints and runner

## 目的

`samples/prototype/current-l2-typed-proof-model-check/`、
`samples/prototype/current-l2-order-handoff/`、
`specs/examples/571`
を前提に、

- Problem 1 / Problem 2 の guided sample entrypoint
- sample-local Japanese explanation
- repo-local helper runner
- current first line / reserve / negative の読み分け
- public surface と helper-local surface の boundary

を 1 本に束ねる。

ここで actualize するのは、
**既存 corrected prototype を複製せずに、どの sample をどの順で見れば current first line と reserve line が分かるかを `samples/` README と repo-local helper script に actualize する current cut**
であり、

- final public CLI
- final public tutorial surface
- final public parser / checker / runtime API
- final public verifier contract
- exhaustive sample catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 1
   - representative typed / theorem / model-check bridge sample は `p06`
   - IFC / capture / cost 補助 sample は `p10 / p11 / p12 / p15 / p16`
2. Problem 2
   - authoritative-room first line の representative pair は `p07 / p08`
   - delegated RNG reserve practical route は `p09`
   - late-join negative static-stop pair は `p13 / p14`
3. current operational helper floor
   - `run-source-sample` は pretty/json の両方で current helper summary を出せる
   - sample guide 自体は helper-local / repo-local convenience に留めてよい

したがって current open problem は、
sample corpus を増やすことではなく、
**現在すでに動く corrected prototype をどう入口化して誤読なく辿れるようにするか**
である。

## current actualization cut

current package では、次を採る。

1. `samples/prototype/current-l2-typed-proof-model-check/README.md`
   で Problem 1 の representative sample と補助 sample を日本語で案内する
2. `samples/prototype/current-l2-order-handoff/README.md`
   で Problem 2 の representative pair / reserve / negative pair を日本語で案内する
3. `scripts/current_l2_guided_samples.py`
   を repo-local helper として追加し、
   - `list`
   - `show problem1|problem2`
   - `run problem1|problem2 [--all]`
   を提供する
4. `run problem1`
   は primary sample として `p06` を流す
5. `run problem2`
   は primary sample として `p07 / p08` representative pair を流す
6. `--all`
   を付けたときだけ reserve / negative sample まで広げる

## actual runnable evidence

| evidence | current reading |
|---|---|
| `samples/prototype/current-l2-typed-proof-model-check/README.md` | Problem 1 の sample-visible reading を日本語で辿れる |
| `samples/prototype/current-l2-order-handoff/README.md` | Problem 2 の first line / reserve / negative を日本語で辿れる |
| `scripts/current_l2_guided_samples.py list` | problem id と要約を repo-local helper で一覧できる |
| `scripts/current_l2_guided_samples.py show problem1` | representative sample / focus point / command を 1 画面で見られる |
| `scripts/current_l2_guided_samples.py run problem1` | current representative sample を repo-local command でそのまま流せる |

## current recommendation

1. guided problem sample entrypoint and runner は current helper package として close してよい。
2. current cut は
   - existing corrected prototype reuse
   - Japanese explanation first
   - repo-local helper runner only
   - public surface promotion later
   に置くのが自然である。
3. sample guide と helper runner は current first line / reserve / negative の読み分けを助けるが、final public contract や final public tutorial surface を意味しない。

## retained alternatives

- final public CLI first
- exhaustive sample catalog first
- sample corpus duplicate files first
- installed binary tutorial first

## stop line

current package は次で止める。

- final public CLI
- final public tutorial surface
- exhaustive sample catalog
- final public parser / checker / runtime API
- final public verifier contract

## next self-driven line

current package を close した後の self-driven line は、

1. theorem / model-check public-seam residual bundle
2. witness/provider/public-shape residual bundle

に narrow に戻すのが自然である。
