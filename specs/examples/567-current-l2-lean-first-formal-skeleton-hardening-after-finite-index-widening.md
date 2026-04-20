# 567. current L2 Lean-first formal skeleton hardening after finite-index widening

## 目的

Package 92 で widened した
source-side first strong typing sample set
`p10 / p11 / p12 / p15 / p16`
を、Lean side の current explanation と generated corpus に
**歪みなく接続し直す**。

ここで actualize するのは、

- final public theorem contract
- concrete production prover binding
- final public verifier contract

ではない。

current cut の目的は、
`samples/lean/foundations/` と
`samples/lean/current-l2/`
の役割差を source-backed に保ったまま、
finite-index first layer と theorem-side placeholder 群の
current reading を close することである。

## current first line

- `samples/lean/foundations/` は
  actual small proof fragment を置く mechanization-ready core として読む。
- `samples/lean/current-l2/` は
  representative sample set から生成した theorem stub corpus として読む。
- generated stub は Lean に受理されても `sorry` を含みうるため、
  completed theorem discharge ではなく
  artifact well-formedness / bridge alignment evidence として扱う。
- finite-index first strong typing line は
  IFC だけでなく capture / lifetime / simple cost まで
  Lean-first explanation に carry over してよい。

## 今回 harden したもの

### foundation 追加

- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  - lifetime preorder
  - capture-set subset
  - simple remote-call budget
  に関する小さな自己完結 proof fragment を actual small proof として追加した。

この file は final typed calculus ではなく、
first strong typing layer を支える
**最小の mechanization-ready fact cluster**
に留める。

### representative generated corpus widening

`scripts/current_l2_lean_sample_sync.py` の representative set を次へ widen した。

- `p15-typed-capture-escape-rejected`
- `p16-typed-remote-call-budget-exceeded`

これにより current L2 generated theorem stub corpus は、

- `e5`
- `p06`
- `p10 / p11 / p12 / p15 / p16`
- `p07 / p08 / p09 / p13 / p14`

の current first-line evidence を
1 つの committed Lean corpus として保持する。

### 日本語 explanation hardening

`samples/lean/README.md` と
各 generated sample README は、

- foundation = actual small proof
- current-L2 = generated stub
- generated stub は `sorry` を含む non-production bridge evidence

という役割差が誤読されないように更新した。

## evidence

- sync test
  - `scripts/tests/test_current_l2_lean_sample_sync.py`
- sync helper
  - `scripts/current_l2_lean_sample_sync.py`
- foundation
  - `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  - `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md`
- representative generated corpus
  - `samples/lean/current-l2/p15-typed-capture-escape-rejected/`
  - `samples/lean/current-l2/p16-typed-remote-call-budget-exceeded/`
  - `samples/lean/manifest.json`

## retained alternatives

- proof object public schema を先に public surface へ上げる案
- stronger typed source principal を先に Lean side へ寄せる案
- concrete theorem prover brand / discharge transport を先に publicize する案

## stop line

今回ここで止めてよいもの:

- production prover binding
- final proof object public contract
- final public theorem result contract
- final public verifier contract

## next package

次の current package は Package 94 である。

- theorem-first notebook line
- row-local model-check second line
- current samples / artifacts / previews

の reconnect を、Lean-first explanation と
current sample corpus に沿って tighten する。
