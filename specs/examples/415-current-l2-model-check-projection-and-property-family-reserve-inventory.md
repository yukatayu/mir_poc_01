# 415 — current L2 model-check projection and property-family reserve inventory

## 目的

current model-check line について、

- projection grain
- first property-family reserve order
- tool-binding stop line

を docs-first に整理する。

ここで固定するのは **concrete model-check tool binding** ではなく、
current row-local carrier から次にどこまで進めるかの reserve inventory である。

## source-backed floor

- `formal_hook -> model_check_concrete_carrier` の row-local machine-facing line は actual code anchor を持つ。
- source-sample emitted artifact wiring は model-check carrier line まで fan-out している。
- sample-facing summary も docs / README / regression flow に反映済みである。

## projection grain ladder

| projection grain | reading | current use | current risk |
|---|---|---|---|
| row-local carrier | 1 obligation row = 1 machine-facing case | current floor | higher-level protocol familyはまだ持てない |
| small-cluster semantic projection | 複数 row / 1 subject family を小さな transition cluster として扱う | next reserve | theorem line と protocol line の境界が曖昧になりやすい |
| room protocol projection | replay / provider receipt / fairness を含む room-level projection | later reserve | order / handoff line と混線しやすい |
| public checker / runtime-policy projection | public surface や runtime policy contract まで含む | still later | final public contract を先取りしやすい |

## first property-family reserve order

| order | property family | current status |
|---|---|---|
| 1 | row-local `canonical_normalization_law` / `no_re_promotion` / `rollback_cut_non_interference` | current floor |
| 2 | small-cluster semantic relation family | next reserve |
| 3 | publication / witness / replay / provider receipt / fairness family | package 4 reserve |
| 4 | public checker / runtime policy contract family | still later |

## current judgment

1. current first projection floor は **row-local machine-facing carrier** に維持する。
2. next reserve は **small-cluster semantic projection** に置く。
3. order / handoff / fairness / provider receipt family は package 4 側に残し、package 3 の事実にしない。
4. concrete model-check tool binding と final property language は still later に残す。

## current first choice details

- current package では、
  - current floor
  - next reserve
  - later reserve
  - stop line
  を明示できれば十分である。
- room protocol / fairness / replay family を row-local carrier の natural extension と short-hand しない。
- theorem-side review unit current first pilot を巻き戻さず、model-check side は sibling reserve line として保つ。

## what is not decided here

- concrete model-check tool binding
- first settled property language
- full protocol family
- production checker / runtime-policy contract
- room protocol property を package 3 の current fact に昇格すること
